#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Horizontal,   // Left to right
    Vertical,     // Top to bottom
    Diagonal,     // Top-left to bottom-right
    AntiDiagonal, // Top-right to bottom-left
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
            Direction::Diagonal => Direction::AntiDiagonal,
            Direction::AntiDiagonal => Direction::Diagonal,
        }
    }

    pub fn is_axial(&self) -> bool {
        match self {
            Direction::Horizontal | Direction::Vertical => true,
            Direction::Diagonal | Direction::AntiDiagonal => false,
        }
    }

    pub fn axis(&self) -> Option<Axis> {
        match self {
            Direction::Horizontal => Some(Axis::Row),
            Direction::Vertical => Some(Axis::Column),
            Direction::Diagonal | Direction::AntiDiagonal => None,
        }
    }
}

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

    pub fn direction(&self) -> Direction {
        match self {
            Axis::Row => Direction::Horizontal,
            Axis::Column => Direction::Vertical,
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug)]
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
    pub fn line_in_direction(
        self,
        direction: Direction,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = Coordinate> {
        let offset = Offset::from_direction(direction);
        let mut maybe_next = Some(self);
        std::iter::from_fn(move || {
            let valid = maybe_next?;
            if valid.row >= height || valid.column >= width {
                return None;
            }
            maybe_next = valid + offset;
            Some(valid)
        })
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
    pub fn from_direction(direction: Direction) -> Self {
        match direction {
            Direction::Horizontal => Offset { row: 0, column: 1 },
            Direction::Vertical => Offset { row: 1, column: 0 },
            Direction::Diagonal => Offset { row: 1, column: 1 },
            Direction::AntiDiagonal => Offset { row: 1, column: -1 },
        }
    }

    pub fn square_kernel(radius: usize, include_center: bool) -> impl Iterator<Item = Offset> {
        let radius = radius as isize;

        (-radius..=radius).flat_map(move |row| {
            (-radius..=radius).filter_map(move |column| {
                if row == 0 && column == 0 && !include_center {
                    None
                } else {
                    Some(Offset { row, column })
                }
            })
        })
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
