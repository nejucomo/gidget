use crate::pt::Pt;

#[test]
fn ix_to_pt_roundtrip() {
    let area = Pt(5, 7);
    dbg!(area);
    for ix in 0..area.area() {
        let pt = area.ix_to_pt(ix);
        dbg!(ix, pt);
        assert!(pt.0 < area.0);
        assert!(pt.1 < area.1);
        let ix2 = area.pt_to_ix(pt);
        assert_eq!(ix, ix2);
    }
}
