#![allow(dead_code)]
/*
* A collection of tiny helper enums.
*/
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Row,
    Column,
}

impl Axis {
    pub fn opposite(&self) -> Self {
        match self {
            Axis::Row => Axis::Column,
            Axis::Column => Axis::Row,
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Coordinate {
    pub row: usize,
    pub column: usize,
}

impl Coordinate {
    pub fn get_axis_index(self, axis: Axis) -> usize {
        match axis {
            Axis::Row => self.row,
            Axis::Column => self.column,
        }
    }
    pub fn set_axis_index(&mut self, axis: Axis, index: usize) {
        match axis {
            Axis::Row => self.row = index,
            Axis::Column => self.column = index,
        }
    }
}
