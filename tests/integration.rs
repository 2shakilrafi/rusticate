#[test]
fn runs_with_help() {
    let output = std::process::Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
}

