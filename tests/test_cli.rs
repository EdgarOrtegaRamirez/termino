use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .arg("--help")
        .output()
        .expect("Failed to run termino --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("termino"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .arg("--version")
        .output()
        .expect("Failed to run termino --version");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_cli_countdown_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["countdown", "--help"])
        .output()
        .expect("Failed to run termino countdown --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("countdown"));
}

#[test]
fn test_cli_stopwatch_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["stopwatch", "--help"])
        .output()
        .expect("Failed to run termino stopwatch --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("stopwatch"));
}

#[test]
fn test_cli_pomodoro_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["pomodoro", "--help"])
        .output()
        .expect("Failed to run termino pomodoro --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pomodoro"));
}

#[test]
fn test_cli_history_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["history", "--help"])
        .output()
        .expect("Failed to run termino history --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("history"));
}

#[test]
fn test_cli_history_no_sessions() {
    // Set a temp home for this test
    let tmpdir = std::env::temp_dir().join("termino_cli_test");
    let _ = std::fs::remove_dir_all(&tmpdir);
    std::fs::create_dir_all(&tmpdir).ok();

    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["history"])
        .env("HOME", &tmpdir)
        .output()
        .expect("Failed to run termino history");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No sessions found"));
}

#[test]
fn test_cli_history_with_limit() {
    let tmpdir = std::env::temp_dir().join("termino_cli_limit");
    let _ = std::fs::remove_dir_all(&tmpdir);
    std::fs::create_dir_all(&tmpdir).ok();

    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["history", "--limit", "5"])
        .env("HOME", &tmpdir)
        .output()
        .expect("Failed to run termino history --limit");
    assert!(output.status.success());
}

#[test]
fn test_cli_history_invalid_type() {
    let output = Command::new(env!("CARGO_BIN_EXE_termino"))
        .args(["history", "--type", "invalid"])
        .output()
        .expect("Failed to run termino history --type invalid");
    assert!(!output.status.success());
}
