use crate::area::Area;

#[test]
fn ix_to_pt_roundtrip() {
    let area = Area::new(5, 7);
    dbg!(area);
    for ix in 0..area.cell_count() {
        let pt = area.ix_to_pt(ix);
        dbg!(ix, pt);
        assert!(pt.col() < area.width());
        assert!(pt.row() < area.height());
        let ix2 = area.pt_to_ix(pt);
        assert_eq!(ix, ix2);
    }
}
