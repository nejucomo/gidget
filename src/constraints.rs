use std::ops::{Index, IndexMut};

use itertools::Itertools;
use rand::Rng;
use rand::seq::IndexedRandom as _;
use unicode_box_drawing::Width;

use crate::boxchar::BoxChar;
use crate::direction::Direction;
use crate::dirpack::DirPack;

#[derive(Copy, Clone, Debug, Default)]
pub struct Constraints(DirPack<Option<Width>>);

impl Constraints {
    pub fn is_constrained(self) -> bool {
        Direction::each().any(|d| self.0[d].is_some())
    }

    pub fn random_boxchar<R: Rng>(self, rng: &mut R) -> BoxChar {
        let any_width = [Width::None, Width::Single, Width::Double];

        let choices: Vec<BoxChar> = Direction::each()
            .map(|d| {
                self[d]
                    .map(|w| vec![(d, w)])
                    .unwrap_or(any_width.iter().map(|&w| (d, w)).collect())
            })
            .multi_cartesian_product()
            .map(|edges| {
                let mut bc = BoxChar::default();
                for (d, w) in edges {
                    bc[d] = w;
                }
                bc
            })
            .collect();

        *choices.choose(rng).unwrap()
    }
}

impl Index<Direction> for Constraints {
    type Output = Option<Width>;

    fn index(&self, d: Direction) -> &Self::Output {
        self.0.index(d)
    }
}

impl IndexMut<Direction> for Constraints {
    fn index_mut(&mut self, d: Direction) -> &mut Self::Output {
        self.0.index_mut(d)
    }
}
