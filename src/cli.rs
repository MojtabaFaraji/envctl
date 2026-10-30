use crate::output::OutputFormat;
use crate::shell::Shell;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "envctl", version, about = "Manage named shell environments")]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Args, Debug)]
pub struct GlobalArgs {
    /// Path to envctl config file (default: XDG_CONFIG_HOME/envctl/config.toml)
    #[arg(long, global = true, short = 'c')]
    pub config: Option<PathBuf>,

    /// Named config profile (alternate config dir suffix)
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Verbose logging (repeat for more)
    #[arg(long, short = 'v', global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Quiet mode (errors only)
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Assume yes to prompts
    #[arg(long, short = 'y', global = true)]
    pub yes: bool,

    /// Print actions without applying
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Disable color
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Output format
    #[arg(long, short = 'o', global = true, value_enum, default_value = "table")]
    pub output: OutputFormat,

    /// Non-interactive: fail instead of prompting
    #[arg(long, global = true)]
    pub no_input: bool,

    /// Operate on the system-wide store (requires root)
    #[arg(long, global = true)]
    pub system: bool,

    /// Debug logging
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize config dir and defaults
    Init {
        #[arg(long)]
        force: bool,
    },
    /// Create a new environment
    #[command(alias = "create")]
    Add {
        #[command(flatten)]
        env: EnvEditArgs,
        name: String,
    },
    /// Delete one or more environments
    #[command(alias = "delete")]
    Remove {
        names: Vec<String>,
        #[arg(long)]
        force: bool,
        #[arg(long)]
        dry_run: bool,
        #[arg(long)]
        no_backup: bool,
    },
    /// Edit an environment (open $EDITOR or mutate via flags)
    Edit {
        name: String,
        #[arg(long)]
        editor: bool,
        #[arg(long)]
        set: Vec<String>,
        #[arg(long)]
        unset: Vec<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Show a single environment in full
    #[command(alias = "show")]
    View {
        name: String,
        #[arg(long)]
        raw: bool,
    },
    /// List environments
    #[command(alias = "ls")]
    List {
        #[arg(short = 'f', long)]
        filter: Option<String>,
        #[arg(long)]
        sort: Option<String>,
        #[arg(short = 'l', long)]
        limit: Option<usize>,
        #[arg(long)]
        offset: Option<usize>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long)]
        all: bool,
    },
    /// Set a variable (KEY=VALUE or KEY VALUE)
    Set {
        name: String,
        key: String,
        value: Option<String>,
        #[arg(long)]
        secret: bool,
    },
    /// Unset one or more variables
    Unset { name: String, keys: Vec<String> },
    /// Rename an environment
    #[command(alias = "mv")]
    Rename {
        old: String,
        new: String,
        #[arg(long)]
        force: bool,
    },
    /// Clone (duplicate) an environment
    #[command(alias = "cp")]
    Clone {
        src: String,
        dst: String,
        #[arg(long)]
        force: bool,
    },
    /// Import an environment from file or stdin
    Import {
        file: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        format: Option<ImportFormat>,
        #[arg(long)]
        overwrite: bool,
    },
    /// Export an environment to file or stdout
    Export {
        name: String,
        #[arg(long, value_enum)]
        format: Option<ExportFormat>,
        #[arg(long = "file")]
        file: Option<PathBuf>,
        #[arg(long)]
        reveal: bool,
    },
    /// Diff two environments
    Diff { a: String, b: String },
    /// Validate environment(s)
    Validate {
        names: Vec<String>,
        #[arg(long)]
        all: bool,
    },
    /// Emit shell code to activate an environment
    #[command(alias = "use")]
    Activate {
        name: String,
        #[arg(long)]
        shell: Option<Shell>,
        #[arg(long)]
        no_functions: bool,
        #[arg(long)]
        no_aliases: bool,
        #[arg(long)]
        no_cwd: bool,
    },
    /// Emit shell code to deactivate the active environment
    Deactivate {},
    /// Show the active environment
    #[command(alias = "current")]
    Which {},
    /// Generate shell completions
    Completion {
        #[arg(value_enum)]
        shell: CompletionShell,
    },
    /// Manage secrets in the system keyring
    #[command(subcommand)]
    Secret(SecretCmd),
    /// Print version
    Version {},
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ImportFormat {
    Auto,
    Dotenv,
    Json,
    Yaml,
    Toml,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExportFormat {
    Dotenv,
    Json,
    Yaml,
    Toml,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
}

#[derive(Subcommand, Debug)]
pub enum SecretCmd {
    /// Store a secret
    Set {
        key: String,
        value: Option<String>,
        #[arg(long)]
        from_stdin: bool,
    },
    /// Retrieve a secret (masked by default)
    Get {
        key: String,
        #[arg(long)]
        reveal: bool,
    },
    /// Remove a secret
    Rm { key: String },
    /// List known secrets (names only)
    List {},
}

/// Common env mutation flags.
#[derive(clap::Args, Debug)]
pub struct EnvEditArgs {
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long = "var", value_name = "KEY=VALUE")]
    pub var: Vec<String>,
    #[arg(long = "path-prepend")]
    pub path_prepend: Vec<String>,
    #[arg(long = "path-append")]
    pub path_append: Vec<String>,
    #[arg(long = "alias", value_name = "NAME=CMD")]
    pub alias: Vec<String>,
    #[arg(long = "function", value_name = "NAME=BODY")]
    pub function: Vec<String>,
    #[arg(long)]
    pub cwd: Option<String>,
    #[arg(long = "tags", value_name = "A,B")]
    pub tags: Option<String>,
    #[arg(long = "from")]
    pub from: Option<PathBuf>,
}
