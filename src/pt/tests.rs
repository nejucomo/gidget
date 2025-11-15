use itertools::Itertools as _;

use crate::area::Area;
use crate::direction::Direction;
use crate::pt::Pt;

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

#[test]
fn clipped_add() {
    use Direction::*;

    let area = Area::new(5, 7);
    let (left, right) = (0, area.width() - 1);
    let (top, bottom) = (0, area.height() - 1);
    let (midcol, midrow) = (right / 2, bottom / 2);

    for pt in [left, midcol, right]
        .into_iter()
        .cartesian_product([top, midrow, bottom])
    {
        let pt = Pt::from(pt);

        for d in Direction::each() {
            let neighbor = area.clip(pt + d);

            assert!(match d {
                Up if pt.row() == top => neighbor.is_none(),
                Down if pt.row() == bottom => neighbor.is_none(),
                Left if pt.col() == left => neighbor.is_none(),
                Right if pt.col() == right => neighbor.is_none(),
                _ => neighbor.is_some(),
            });
        }
    }
}
