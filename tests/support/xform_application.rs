// Historical decoder tests retain their legacy manifest refusals. CCA-101 adds
// a separate application authority only for the authenticated CONTROL sequence.
pub fn assert_capability(row: &phoenix::app_contract::SequenceSummary, qualified: bool) {
    use phoenix::app_contract::Readiness;
    assert!(row.export_capability.is_none());
    if qualified {
        assert_eq!(row.readiness, Readiness::Ready);
        let capability = row.bounded_export_capability.as_ref().unwrap();
        assert_eq!(capability.contract_id, "authenticated_xform_recovery");
        assert_eq!(capability.contract_revision, 1);
    } else {
        assert_eq!(row.readiness, Readiness::PartiallySupported);
        assert!(row.bounded_export_capability.is_none());
    }
}
