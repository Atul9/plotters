use super::{AsRangedCoord, DiscreteRanged, Ranged};
use std::ops::Range;

pub struct RangedSlice<'a, T: PartialEq>(&'a [T]);

impl<'a, T: PartialEq> Ranged for RangedSlice<'a, T> {
    type ValueType = &'a T;

    fn range(&self) -> Range<&'a T> {
        &self.0[0]..&self.0[self.0.len() - 1]
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        match self.0.iter().position(|x| &x == value) {
            Some(pos) => {
                let pixel_span = limit.1 - limit.0;
                let value_span = self.0.len() - 1;
                (f64::from(limit.0)
                    + f64::from(pixel_span)
                        * (f64::from(pos as u32) / f64::from(value_span as u32)))
                .round() as i32
            }
            None => limit.0,
        }
    }

    fn key_points(&self, max_points: usize) -> Vec<Self::ValueType> {
        let mut ret = vec![];
        let intervals = (self.0.len() - 1) as f64;
        let step = (intervals / max_points as f64 + 1.0) as usize;
        for idx in (0..self.0.len()).step_by(step) {
            ret.push(&self.0[idx]);
        }
        ret
    }
}

impl<'a, T: PartialEq> DiscreteRanged for RangedSlice<'a, T> {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn index_of(&self, value: &&'a T) -> Option<usize> {
        self.0.iter().position(|x| &x == value)
    }

    fn from_index(&self, index: usize) -> Option<&'a T> {
        if self.0.len() <= index {
            return None;
        }
        Some(&self.0[index])
    }
}

impl<'a, T: PartialEq> From<&'a [T]> for RangedSlice<'a, T> {
    fn from(range: &'a [T]) -> Self {
        RangedSlice(range)
    }
}

impl<'a, T: PartialEq> AsRangedCoord for &'a [T] {
    type CoordDescType = RangedSlice<'a, T>;
    type Value = &'a T;
}
