// Renamed from `bind_address_test.rs`.
#[test]
fn config_keeps_advertise_address() {
    let c = coverage_gate_validation::config::NodeConfig::new("10.0.0.5:9000".to_string());
    assert_eq!(c.advertise_address, "10.0.0.5:9000");
    assert_eq!(c.max_batch, 64);
}
