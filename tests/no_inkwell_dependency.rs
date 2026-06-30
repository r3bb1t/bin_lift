#[test]
fn cargo_manifest_no_longer_depends_on_inkwell() {
    let manifest = std::fs::read_to_string("Cargo.toml").expect("read Cargo.toml");
    assert!(!manifest.contains("inkwell"), "{manifest}");
    assert!(manifest.contains("llvmkit"), "{manifest}");
}
