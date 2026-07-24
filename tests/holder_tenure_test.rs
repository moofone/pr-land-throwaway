// Renamed from `election_tenure_test.rs` — leadership wording removed.
#[test]
fn registry_records_holder_incarnation() {
    let mut r = coverage_gate_validation::registry::Registry::default();
    r.record("node-a", 7);
    assert_eq!(r.incarnation("node-a"), Some(7));
}
