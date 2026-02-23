#[test]
fn architecture_valid() {
    assert!(std::process::Command::new("cargo")
        .args(["run", "-p", "arch_guard"])
        .status()
        .unwrap()
        .success());
}