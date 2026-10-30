use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn envctl() -> (TempDir, Command) {
    let tmp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"));
    (tmp, cmd)
}

#[test]
fn init_and_add_list() {
    let (tmp, mut cmd) = envctl();
    cmd.args(["init"]).assert().success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args(["add", "foo", "--var", "FOO=bar"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("foo"));
}

#[test]
fn exit_codes() {
    let (tmp, mut cmd) = envctl();
    cmd.args(["init"]).assert().success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args(["view", "nope"])
        .assert()
        .code(3);
}

#[test]
fn activate_eval() {
    let (tmp, _) = envctl();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args(["init"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args(["add", "foo", "--var", "FOO=hello world"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"));
    let out = cmd.args(["activate", "foo"]).output().unwrap();
    let code = String::from_utf8(out.stdout).unwrap();
    assert!(code.contains("export FOO='hello world'"));
}

#[test]
fn shell_compat_zsh() {
    let (tmp, _) = envctl();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args(["init"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"))
        .args([
            "add",
            "foo",
            "--var",
            "FOO=hello world",
            "--var",
            "BAR=it's",
        ])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("envctl").unwrap();
    cmd.env("HOME", tmp.path())
        .env("XDG_CONFIG_HOME", tmp.path().join(".config"))
        .env("XDG_STATE_HOME", tmp.path().join(".state"))
        .env("XDG_CACHE_HOME", tmp.path().join(".cache"));
    let out = cmd
        .args(["activate", "foo", "--shell", "zsh"])
        .output()
        .unwrap();
    let code = String::from_utf8(out.stdout).unwrap();
    std::fs::write(tmp.path().join("act.zsh"), &code).unwrap();
    // Skip if zsh is not installed (e.g. minimal CI containers)
    if std::process::Command::new("zsh")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipping shell_compat_zsh: zsh not installed");
        return;
    }

    let status = std::process::Command::new("zsh")
        .arg("-c")
        .arg(format!(
            "source {} && test \"$FOO\" = \"hello world\" && test \"$BAR\" = \"it's\"",
            tmp.path().join("act.zsh").display()
        ))
        .status()
        .unwrap();
    assert!(status.success());
}
