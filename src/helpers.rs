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

impl std::ops::Add<Offset> for Coordinate {
    type Output = Option<Coordinate>;

    fn add(self, offset: Offset) -> Self::Output {
        let mut out = Coordinate::default();

        for axis in [Axis::Row, Axis::Column] {
            let coord = self.get_axis_index(axis);
            let off = offset.get_axis_index(axis);

            if off > 0 && usize::MAX - coord < off as usize {
                return None; // Overflow
            }
            if off < 0 && coord < off.unsigned_abs() {
                return None; // Underflow
            }

            match off.cmp(&0) {
                std::cmp::Ordering::Less => out.set_axis_index(axis, coord - off.unsigned_abs()),
                std::cmp::Ordering::Equal => out.set_axis_index(axis, coord),
                std::cmp::Ordering::Greater => out.set_axis_index(axis, coord + off as usize),
            }
        }
        Some(out)
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Offset {
    pub row: isize,
    pub column: isize,
}

impl Offset {
    pub fn get_axis_index(self, axis: Axis) -> isize {
        match axis {
            Axis::Row => self.row,
            Axis::Column => self.column,
        }
    }
    pub fn set_axis_index(&mut self, axis: Axis, index: isize) {
        match axis {
            Axis::Row => self.row = index,
            Axis::Column => self.column = index,
        }
    }

    pub fn square_kernel(radius: usize, include_center: bool) -> Vec<Offset> {
        let radius = radius as isize;

        (-radius..=radius)
            .flat_map(|row| {
                (-radius..=radius).filter_map(move |column| {
                    if row == 0 && column == 0 && !include_center {
                        None
                    } else {
                        Some(Offset { row, column })
                    }
                })
            })
            .collect()
    }
}

impl std::ops::Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            row: self.row.add(rhs.row),
            column: self.column.add(rhs.column),
        }
    }
}
