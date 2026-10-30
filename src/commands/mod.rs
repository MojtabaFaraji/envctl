use crate::cli::*;
use crate::config::Locations;
use crate::error::{EnvctlError, Result};
use crate::log;
use crate::log_err;
use crate::model::{validate_key, validate_name, looks_like_secret, Environment, SCHEMA_VERSION};
use crate::output::{OutputFormat, render_env, render_list};
use crate::secrets::{is_secret_ref, resolve_value, secret_get, secret_remove, secret_set};
use crate::shell::{emit_activate, emit_deactivate, active_name, quote, Shell};
use crate::storage::Store;
use clap::CommandFactory;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::fmt::Write as _;

pub struct Context {
    pub locations: Locations,
    pub verbose: u8,
    pub quiet: bool,
    pub yes: bool,
    pub dry_run: bool,
    pub no_color: bool,
    pub no_input: bool,
    pub system: bool,
    pub debug: bool,
    pub output: OutputFormat,
}

impl Context {
    pub fn from_cli(global: &GlobalArgs) -> Result<Self> {
        let system = global.system;
        let locations = Locations::resolve(system, global.config.clone(), global.profile.clone())?;
        let verbose = if global.debug { 2 } else { global.verbose };
        Ok(Self {
            locations,
            verbose,
            quiet: global.quiet,
            yes: global.yes,
            dry_run: global.dry_run,
            no_color: global.no_color,
            no_input: global.no_input,
            system,
            debug: global.debug,
            output: global.output,
        })
    }

    pub fn store(&self) -> Store {
        Store::new(self.locations.clone())
    }

    pub fn confirm(&self, prompt: &str) -> Result<bool> {
        if self.yes || self.no_input {
            return Ok(self.yes);
        }
        if !atty::is(atty::Stream::Stdout) && !atty::is(atty::Stream::Stderr) {
            // Not a TTY and not --yes/--no-input -> fail.
            if self.no_input {
                return Ok(false);
            }
            return Err(EnvctlError::Usage(format!(
                "refusing to prompt in non-interactive mode: {prompt} (use --yes or --no-input)"
            )));
        }
        print!("{prompt} [y/N]: ");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        Ok(line.trim().eq_ignore_ascii_case("y") || line.trim().eq_ignore_ascii_case("yes"))
    }
}

/// Entry point: run the parsed CLI.
pub fn run(cli: Cli) -> Result<i32> {
    let ctx = Context::from_cli(&cli.global)?;
    crate::set_verbose(ctx.verbose);
    if ctx.debug {
        crate::set_verbose(2);
    }
    match cli.command {
        Commands::Init { force } => cmd_init(&ctx, force),
        Commands::Add { env, name } => cmd_add(&ctx, &env, &name),
        Commands::Remove { names, force, dry_run, no_backup } => {
            cmd_remove(&ctx, &names, force, dry_run, no_backup)
        }
        Commands::Edit { name, editor, set, unset, description } => {
            cmd_edit(&ctx, &name, editor, set, unset, description)
        }
        Commands::View { name, raw } => cmd_view(&ctx, &name, raw),
        Commands::List { filter, sort, limit, offset, tags, all } => {
            cmd_list(&ctx, filter, sort, limit, offset, tags, all)
        }
        Commands::Set { name, key, value, secret } => cmd_set(&ctx, &name, &key, value, secret),
        Commands::Unset { name, keys } => cmd_unset(&ctx, &name, &keys),
        Commands::Rename { old, new, force } => {
            cmd_rename(&ctx, &old, &new, force)
        }
        Commands::Clone { src, dst, force } => {
            cmd_clone(&ctx, &src, &dst, force)
        }
        Commands::Import { file, name, format, overwrite } => {
            cmd_import(&ctx, &file, name, format, overwrite)
        }
        Commands::Export { name, format, file, reveal } => {
            cmd_export(&ctx, &name, format, file, reveal)
        }
        Commands::Diff { a, b } => cmd_diff(&ctx, &a, &b),
        Commands::Validate { names, all } => cmd_validate(&ctx, names, all),
        Commands::Activate { name, shell, no_functions, no_aliases, no_cwd } => {
            cmd_activate(&ctx, &name, shell, no_functions, no_aliases, no_cwd)
        }
        Commands::Deactivate {} => cmd_deactivate(&ctx),
        Commands::Which {} => cmd_which(&ctx),
        Commands::Completion { shell } => cmd_completion(shell),
        Commands::Secret(sub) => cmd_secret(&ctx, sub),
        Commands::Version {} => {
            println!("envctl {}", env!("CARGO_PKG_VERSION"));
            Ok(0)
        }
    }
}

// ---------------------------------------------------------------------------
// Command handlers
// ---------------------------------------------------------------------------

fn cmd_init(ctx: &Context, force: bool) -> Result<i32> {
    if ctx.dry_run {
        println!("(dry-run) would initialize store at {}", ctx.locations.config_dir.display());
        return Ok(0);
    }
    ctx.locations.ensure_dirs()?;
    let cfg_path = ctx.locations.config_file();
    if cfg_path.exists() && !force {
        println!("config already exists at {}", cfg_path.display());
        return Ok(0);
    }
    let default_cfg = format!(
        r#"# envctl global configuration
default_shell = ""
color = {}
editor = ""
allow_plaintext_secrets = false
system = {}
"#,
        !ctx.no_color, ctx.system
    );
    let tmp = atomic_write_tmp(&cfg_path, default_cfg.as_bytes())?;
    atomic_rename(&tmp, &cfg_path)?;
    println!("initialized envctl store at {}", ctx.locations.config_dir.display());
    Ok(0)
}

fn cmd_add(ctx: &Context, args: &EnvEditArgs, name: &str) -> Result<i32> {
    validate_name(name).map_err(|e| EnvctlError::Usage(e))?;
    let store = ctx.store();
    if store.exists(name) {
        return Err(EnvctlError::Conflict(format!("environment '{name}' already exists")));
    }
    let mut env = build_environment(args, name)?;
    if ctx.dry_run {
        println!("(dry-run) would add environment '{name}'");
        return Ok(0);
    }
    store.save(&env)?;
    println!("added environment '{name}'");
    Ok(0)
}

fn cmd_remove(ctx: &Context, names: &[String], force: bool, dry_run: bool, no_backup: bool) -> Result<i32> {
    let store = ctx.store();
    for name in names {
        if !store.exists(name) {
            return Err(EnvctlError::NotFound(format!("environment '{name}' not found")));
        }
    }
    if !force && !dry_run {
        let list = names.join(", ");
        let prompt = format!("delete environment(s) {list}?");
        if !ctx.confirm(&prompt)? {
            println!("aborted.");
            return Ok(0);
        }
    }
    if dry_run {
        for name in names {
            println!("(dry-run) would delete '{name}' (backup={})", !no_backup);
        }
        return Ok(0);
    }
    let mut any_removed = false;
    for name in names {
        if let Err(e) = store.delete(name, !no_backup) {
            log_err(&format!("{name}: {e}"));
            continue;
        }
        any_removed = true;
        println!("deleted environment '{name}'");
    }
    if !any_removed && !names.is_empty() {
        return Ok(1);
    }
    Ok(0)
}

fn cmd_edit(
    ctx: &Context,
    name: &str,
    editor: bool,
    set: Vec<String>,
    unset: Vec<String>,
    description: Option<String>,
) -> Result<i32> {
    let store = ctx.store();
    let mut env = store.load(name)?;
    if editor {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        let toml_str = toml::to_string_pretty(&env)?;
        let tmp = tempfile_path()?;
        fs::write(&tmp, &toml_str)?;
        let status = std::process::Command::new(&editor).arg(&tmp).status()?;
        if !status.success() {
            return Err(EnvctlError::Generic(format!("editor '{editor}' failed")));
        }
        let new_str = fs::read_to_string(&tmp)?;
        let updated: Environment = toml::from_str(&new_str)
            .map_err(|e| EnvctlError::Parse(format!("edited config: {e}")))?;
        env = updated;
        let _ = fs::remove_file(&tmp);
        if ctx.dry_run {
            println!("(dry-run) would save edited environment '{name}'");
            return Ok(0);
        }
        store.save(&env)?;
        println!("edited environment '{name}'");
        return Ok(0);
    }
    let mut changed = false;
    for s in &set {
        let (k, v) = parse_kv(s)?;
        validate_key(&k)?;
        if looks_like_secret(&k) && !ctx.system && !std::env::var("ENVCTL_ALLOW_PLAINTEXT_SECRETS").is_ok() {
            // store as keyring ref unless already a ref
            if !is_secret_ref(&v) {
                let secret_name = format!("{}/{}", name, k);
                secret_set(&secret_name, &v)?;
                env.vars.insert(k, format!("secret://keyring/{secret_name}"));
                changed = true;
                continue;
            }
        }
        env.vars.insert(k, v.to_string());
        changed = true;
    }
    for k in &unset {
        validate_key(k)?;
        if env.vars.remove(k).is_some() {
            changed = true;
        }
    }
    if let Some(d) = description {
        env.description = d;
        changed = true;
    }
    if !changed && !ctx.dry_run {
        println!("no changes for '{name}'");
        return Ok(0);
    }
    env.touch();
    if ctx.dry_run {
        println!("(dry-run) would update environment '{name}'");
        return Ok(0);
    }
    store.save(&env)?;
    println!("updated environment '{name}'");
    Ok(0)
}

fn cmd_view(ctx: &Context, name: &str, raw: bool) -> Result<i32> {
    let store = ctx.store();
    let env = store.load(name)?;
    if raw {
        let toml_str = toml::to_string_pretty(&env)?;
        print!("{toml_str}");
    } else {
        print!("{}", render_env(&env, ctx.output)?);
    }
    Ok(0)
}

fn cmd_list(
    ctx: &Context,
    filter: Option<String>,
    sort: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
    tags: Option<String>,
    _all: bool,
) -> Result<i32> {
    let store = ctx.store();
    let mut names = store.list_names()?;
    if let Some(f) = filter {
        names.retain(|n| n.contains(&f));
    }
    if let Some(t) = tags {
        let wanted: Vec<&str> = t.split(',').map(|s| s.trim()).collect();
        names = names
            .into_iter()
            .filter(|n| {
                let env = store.load(n).ok();
                env.map(|e| wanted.iter().all(|w| e.tags.contains(&w.to_string()))).unwrap_or(false)
            })
            .collect();
    }
    match sort.as_deref() {
        None | Some("name") => {}
        Some("updated") | Some("created") => {
            // Sort by mtime of env file.
            names.sort_by(|a, b| {
                let pa = store.locations.env_file(a);
                let pb = store.locations.env_file(b);
                let ma = fs::metadata(&pa).ok().and_then(|m| m.modified().ok());
                let mb = fs::metadata(&pb).ok().and_then(|m| m.modified().ok());
                match (ma, mb) {
                    (Some(a), Some(b)) => a.cmp(&b),
                    _ => std::cmp::Ordering::Equal,
                }
            });
        }
        Some(other) => return Err(EnvctlError::Usage(format!("unknown sort '{other}'"))),
    }
    let off = offset.unwrap_or(0);
    if off < names.len() {
        names.drain(..off);
    }
    if let Some(l) = limit {
        names.truncate(l);
    }
    let rows: Vec<(String, Option<String>)> = names
        .iter()
        .map(|n| {
            let updated = fs::metadata(store.locations.env_file(n))
                .ok()
                .and_then(|m| m.modified().ok())
                .map(|t| {
                    let dt: chrono::DateTime<chrono::Utc> = t.into();
                    dt.to_rfc3339()
                });
            (n.clone(), updated)
        })
        .collect();
    print!("{}", render_list(&rows, ctx.output)?);
    Ok(0)
}

fn cmd_set(ctx: &Context, name: &str, key: &str, value: Option<String>, secret: bool) -> Result<i32> {
    // Support `set NAME KEY=VALUE` form.
    let (key, value) = if value.is_none() && key.contains('=') {
        let pos = key.find('=').unwrap();
        (key[..pos].to_string(), Some(key[pos + 1..].to_string()))
    } else {
        (key.to_string(), value)
    };
    validate_key(&key)?;
    let store = ctx.store();
    let mut env = store.load(name)?;
    let value = value.unwrap_or_else(|| {
        // read from stdin if no value given
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap_or_default();
        s.trim_end().to_string()
    });
    let final_value = if secret || looks_like_secret(&key) {
        if !is_secret_ref(&value) {
            let secret_name = format!("{name}/{key}");
            secret_set(&secret_name, &value)?;
            format!("secret://keyring/{secret_name}")
        } else {
            value
        }
    } else {
        value
    };
    if ctx.dry_run {
        println!("(dry-run) would set {key} in '{name}'");
        return Ok(0);
    }
    env.vars.insert(key.to_string(), final_value);
    env.touch();
    store.save(&env)?;
    println!("set {key} in environment '{name}'");
    Ok(0)
}

fn cmd_unset(ctx: &Context, name: &str, keys: &[String]) -> Result<i32> {
    let store = ctx.store();
    let mut env = store.load(name)?;
    let mut changed = false;
    for k in keys {
        validate_key(k)?;
        if env.vars.remove(k).is_some() {
            changed = true;
        }
    }
    if !changed {
        println!("no matching keys removed");
        return Ok(0);
    }
    env.touch();
    if ctx.dry_run {
        println!("(dry-run) would unset keys in '{name}'");
        return Ok(0);
    }
    store.save(&env)?;
    println!("unset keys in environment '{name}'");
    Ok(0)
}

fn cmd_rename(ctx: &Context, old: &str, new: &str, force: bool) -> Result<i32> {
    validate_name(new).map_err(|e| EnvctlError::Usage(e))?;
    let store = ctx.store();
    if !store.exists(old) {
        return Err(EnvctlError::NotFound(format!("environment '{old}' not found")));
    }
    if store.exists(new) && !force {
        return Err(EnvctlError::Conflict(format!("environment '{new}' already exists")));
    }
    if ctx.dry_run {
        println!("(dry-run) would rename '{old}' -> '{new}'");
        return Ok(0);
    }
    let mut env = store.load(old)?;
    env.name = new.to_string();
    env.touch();
    store.save(&env)?;
    store.delete(old, true)?;
    println!("renamed '{old}' -> '{new}'");
    Ok(0)
}

fn cmd_clone(ctx: &Context, src: &str, dst: &str, force: bool) -> Result<i32> {
    validate_name(dst).map_err(|e| EnvctlError::Usage(e))?;
    let store = ctx.store();
    if !store.exists(src) {
        return Err(EnvctlError::NotFound(format!("environment '{src}' not found")));
    }
    if store.exists(dst) && !force {
        return Err(EnvctlError::Conflict(format!("environment '{dst}' already exists")));
    }
    let mut env = store.load(src)?;
    env.name = dst.to_string();
    env.touch();
    if ctx.dry_run {
        println!("(dry-run) would clone '{src}' -> '{dst}'");
        return Ok(0);
    }
    store.save(&env)?;
    println!("cloned '{src}' -> '{dst}'");
    Ok(0)
}

fn cmd_import(
    ctx: &Context,
    file: &str,
    name: Option<String>,
    format: Option<ImportFormat>,
    overwrite: bool,
) -> Result<i32> {
    let store = ctx.store();
    let content = if file == "-" {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s)?;
        s
    } else {
        fs::read_to_string(file).map_err(|e| EnvctlError::Io(e.to_string()))?
    };
    let fmt = format.unwrap_or(ImportFormat::Auto);
    let env = parse_import(&content, fmt, file, name.as_deref())?;
    let env_name = &env.name;
    if store.exists(env_name) && !overwrite && !ctx.dry_run {
        return Err(EnvctlError::Conflict(format!("environment '{env_name}' already exists")));
    }
    if ctx.dry_run {
        println!("(dry-run) would import environment '{env_name}'");
        return Ok(0);
    }
    store.save(&env)?;
    println!("imported environment '{env_name}'");
    Ok(0)
}

fn cmd_export(
    ctx: &Context,
    name: &str,
    format: Option<ExportFormat>,
    output: Option<std::path::PathBuf>,
    reveal: bool,
) -> Result<i32> {
    let store = ctx.store();
    let mut env = store.load(name)?;
    if !reveal {
        // mask secret refs: replace values with ref placeholder
        for v in env.vars.values_mut() {
            if is_secret_ref(v) {
                *v = format!("secret://keyring/<{}>", v.split('/').last().unwrap_or("key"));
            }
        }
    }
    let fmt = format.unwrap_or(ExportFormat::Dotenv);
    let text = match fmt {
        ExportFormat::Dotenv => dotenv_export(&env),
        ExportFormat::Json => serde_json::to_string_pretty(&env)?,
        ExportFormat::Yaml => serde_yaml::to_string(&env)?,
        ExportFormat::Toml => toml::to_string_pretty(&env)?,
    };
    match output {
        Some(p) => {
            if ctx.dry_run {
                println!("(dry-run) would write export of '{name}' to {}", p.display());
                return Ok(0);
            }
            fs::write(&p, &text)?;
            println!("exported '{name}' to {}", p.display());
        }
        None => {
            if ctx.dry_run {
                println!("(dry-run) would export '{name}'");
                return Ok(0);
            }
            print!("{text}");
        }
    }
    Ok(0)
}

fn cmd_diff(ctx: &Context, a: &str, b: &str) -> Result<i32> {
    let store = ctx.store();
    let ea = store.load(a)?;
    let eb = store.load(b)?;
    let mut diffs = Vec::new();
    let all_keys: std::collections::BTreeSet<&str> =
        ea.vars.keys().map(|s| s.as_str()).chain(eb.vars.keys().map(|s| s.as_str())).collect();
    for k in all_keys {
        match (ea.vars.get(k), eb.vars.get(k)) {
            (Some(va), Some(vb)) if va == vb => {}
            (Some(va), Some(vb)) => diffs.push(format!("  {k}: '{va}' -> '{vb}'")),
            (Some(va), None) => diffs.push(format!("  {k}: '{va}' -> (removed)")),
            (None, Some(vb)) => diffs.push(format!("  {k}: (added) -> '{vb}'")),
            (None, None) => {}
        }
    }
    if diffs.is_empty() {
        println!("environments '{a}' and '{b}' are identical");
    } else {
        println!("diff {a} -> {b}:");
        for d in diffs {
            println!("{d}");
        }
    }
    Ok(0)
}

fn cmd_validate(ctx: &Context, names: Vec<String>, all: bool) -> Result<i32> {
    let store = ctx.store();
    let names = if all { store.list_names()? } else { names };
    if names.is_empty() {
        println!("nothing to validate");
        return Ok(0);
    }
    let mut problems = 0u64;
    for name in &names {
        match store.load(name) {
            Ok(env) => {
                let mut p = Vec::new();
                if validate_name(name).is_err() {
                    p.push(format!("  invalid name '{name}'"));
                }
                for k in env.vars.keys() {
                    if validate_key(k).is_err() {
                        p.push(format!("  invalid key '{k}' in '{name}'"));
                    }
                }
                for (k, v) in &env.vars {
                    if is_secret_ref(v) {
                        let rname = v.trim_start_matches("secret://keyring/");
                        if secret_get(rname).ok().flatten().is_none() {
                            p.push(format!("  unresolved secret ref '{k}' -> {v} in '{name}'"));
                            problems += 1;
                        }
                    }
                }
                if !p.is_empty() {
                    println!("{name}:");
                    for line in p {
                        println!("{line}");
                    }
                    problems += 1;
                } else {
                    println!("{name}: OK");
                }
            }
            Err(e) => {
                println!("{name}: ERROR {e}");
                problems += 1;
            }
        }
    }
    if problems > 0 {
        return Ok(1);
    }
    Ok(0)
}

fn cmd_activate(
    ctx: &Context,
    name: &str,
    shell_opt: Option<Shell>,
    no_functions: bool,
    no_aliases: bool,
    no_cwd: bool,
) -> Result<i32> {
    let shell = shell_opt.unwrap_or(Shell::detect());
    let store = ctx.store();
    let env = store.load(name)?;
    let code = emit_activate(&ctx.locations, shell, &env, no_functions, no_aliases, no_cwd)?;
    print!("{code}");
    Ok(0)
}

fn cmd_deactivate(ctx: &Context) -> Result<i32> {
    let shell = Shell::detect();
    let code = emit_deactivate(&ctx.locations, shell)?;
    print!("{code}");
    Ok(0)
}

fn cmd_which(ctx: &Context) -> Result<i32> {
    match active_name(&ctx.locations)? {
        Some(n) => {
            println!("{n}");
            Ok(0)
        }
        None => {
            println!("no active environment");
            Ok(3)
        }
    }
}

fn cmd_completion(shell: CompletionShell) -> Result<i32> {
    let mut cmd = Cli::command();
    let name = "envctl";
    match shell {
        CompletionShell::Bash => clap_complete::generate(clap_complete::Shell::Bash, &mut cmd, name, &mut io::stdout()),
        CompletionShell::Zsh => clap_complete::generate(clap_complete::Shell::Zsh, &mut cmd, name, &mut io::stdout()),
        CompletionShell::Fish => clap_complete::generate(clap_complete::Shell::Fish, &mut cmd, name, &mut io::stdout()),
    }
    Ok(0)
}

fn cmd_secret(ctx: &Context, sub: SecretCmd) -> Result<i32> {
    match sub {
        SecretCmd::Set { key, value, from_stdin } => {
            let value = if from_stdin {
                let mut s = String::new();
                io::stdin().read_to_string(&mut s)?;
                s.trim_end().to_string()
            } else {
                value.unwrap_or_else(|| {
                    let mut s = String::new();
                    io::stdin().read_to_string(&mut s).unwrap_or_default();
                    s.trim_end().to_string()
                })
            };
            if ctx.dry_run {
                println!("(dry-run) would store secret {key}");
                return Ok(0);
            }
            secret_set(&key, &value)?;
            println!("stored secret {key}");
            Ok(0)
        }
        SecretCmd::Get { key, reveal } => {
            match secret_get(&key)? {
                Some(v) => {
                    if reveal {
                        println!("{v}");
                    } else {
                        println!("{}", "*".repeat(v.len().max(1)));
                    }
                    Ok(0)
                }
                None => Err(EnvctlError::NotFound(format!("secret '{key}' not found"))),
            }
        }
        SecretCmd::Rm { key } => {
            if ctx.dry_run {
                println!("(dry-run) would remove secret {key}");
                return Ok(0);
            }
            secret_remove(&key)?;
            println!("removed secret {key}");
            Ok(0)
        }
        SecretCmd::List {} => {
            // keyring doesn't support listing; print hint.
            println!("secret listing not supported by keyring backend; use `envctl secret get <key>`");
            Ok(0)
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn build_environment(args: &EnvEditArgs, name: &str) -> Result<Environment> {
    let mut env = Environment::new(name.to_string());
    if let Some(d) = &args.description {
        env.description = d.clone();
    }
    if let Some(tags) = &args.tags {
        env.tags = tags.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    }
    if let Some(cwd) = &args.cwd {
        env.cwd = Some(crate::model::Cwd { path: cwd.clone() });
    }
    for s in &args.var {
        let (k, v) = parse_kv(s)?;
        validate_key(&k)?;
        if looks_like_secret(&k) && !is_secret_ref(&v) {
            let secret_name = format!("{name}/{k}");
            secret_set(&secret_name, &v)?;
            env.vars.insert(k, format!("secret://keyring/{secret_name}"));
        } else {
            env.vars.insert(k, v);
        }
    }
    for p in &args.path_prepend {
        env.paths.prepend.push(p.clone());
    }
    for p in &args.path_append {
        env.paths.append.push(p.clone());
    }
    for s in &args.alias {
        let (k, v) = parse_kv(s)?;
        env.aliases.insert(k, v);
    }
    for s in &args.function {
        let (k, v) = parse_kv(s)?;
        env.functions.insert(k, v);
    }
    if let Some(from) = &args.from {
        let content = fs::read_to_string(from).map_err(|e| EnvctlError::Io(e.to_string()))?;
        let imported = parse_import(&content, ImportFormat::Auto, from.to_str().unwrap_or("file"), None)?;
        env.vars.extend(imported.vars);
        env.paths.prepend.extend(imported.paths.prepend);
        env.paths.append.extend(imported.paths.append);
        env.aliases.extend(imported.aliases);
        env.functions.extend(imported.functions);
    }
    env.touch();
    Ok(env)
}

fn parse_kv(s: &str) -> Result<(String, String)> {
    let pos = s.find('=').ok_or_else(|| EnvctlError::Usage(format!("expected KEY=VALUE, got '{s}'")))?;
    let k = s[..pos].to_string();
    let v = s[pos + 1..].to_string();
    if k.is_empty() {
        return Err(EnvctlError::Usage("empty key".into()));
    }
    Ok((k, v))
}

fn parse_import(content: &str, fmt: ImportFormat, source: &str, name: Option<&str>) -> Result<Environment> {
    match fmt {
        ImportFormat::Auto => {
            let ext = std::path::Path::new(source).extension().and_then(|e| e.to_str()).unwrap_or("");
            let env = match ext {
                "json" => parse_import(content, ImportFormat::Json, source, name)?,
                "yaml" | "yml" => parse_import(content, ImportFormat::Yaml, source, name)?,
                "toml" => parse_import(content, ImportFormat::Toml, source, name)?,
                _ => parse_dotenv(content, name)?,
            };
            Ok(env)
        }
        ImportFormat::Json => {
            let mut env: Environment = serde_json::from_str(content).map_err(|e| EnvctlError::Parse(e.to_string()))?;
            if let Some(n) = name { env.name = n.to_string(); }
            Ok(env)
        }
        ImportFormat::Yaml => {
            let mut env: Environment = serde_yaml::from_str(content).map_err(|e| EnvctlError::Parse(e.to_string()))?;
            if let Some(n) = name { env.name = n.to_string(); }
            Ok(env)
        }
        ImportFormat::Toml => {
            let mut env: Environment = toml::from_str(content).map_err(|e| EnvctlError::Parse(e.to_string()))?;
            if let Some(n) = name { env.name = n.to_string(); }
            Ok(env)
        }
        ImportFormat::Dotenv => parse_dotenv(content, name),
    }
}

fn parse_dotenv(content: &str, name: Option<&str>) -> Result<Environment> {
    let n = name.unwrap_or("imported").to_string();
    let mut env = Environment::new(n.clone());
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(pos) = line.find('=') {
            let k = line[..pos].trim().to_string();
            let v = line[pos + 1..].trim().trim_matches('"').to_string();
            if !k.is_empty() {
                env.vars.insert(k, v);
            }
        }
    }
    env.name = n;
    env.touch();
    Ok(env)
}

fn dotenv_export(env: &Environment) -> String {
    let mut s = String::new();
    for (k, v) in &env.vars {
        writeln!(s, "{k}={v}").ok();
    }
    s
}

fn tempfile_path() -> Result<std::path::PathBuf> {
    Ok(std::env::temp_dir().join(format!("envctl-edit-{}", std::process::id())))
}

fn atomic_write_tmp(target: &std::path::Path, data: &[u8]) -> Result<std::path::PathBuf> {
    let dir = target.parent().unwrap_or(std::path::Path::new("."));
    std::fs::create_dir_all(dir)?;
    let tmp = dir.join(format!(
        ".{}.{}.tmp",
        target.file_name().and_then(|s| s.to_str()).unwrap_or("env"),
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
    let mut f = std::fs::File::create(&tmp)?;
    f.write_all(data)?;
    f.sync_all()?;
    Ok(tmp)
}

fn atomic_rename(tmp: &std::path::Path, target: &std::path::Path) -> Result<()> {
    std::fs::rename(tmp, target).map_err(|e| EnvctlError::Io(e.to_string()))
}
