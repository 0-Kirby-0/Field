use crate::{Coordinate, Direction, Field};
impl<T> Field<T> {
    pub fn width(&self) -> usize {
        self.data.first().unwrap_or(&vec![]).len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn number_of_lines_in_direction(&self, direction: Direction) -> usize {
        match direction {
            Direction::Horizontal => self.height(),
            Direction::Vertical => self.width(),
            Direction::Diagonal | Direction::AntiDiagonal => self.width() + self.height() - 1,
        }
    }

    pub fn all_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let height = self.height();
        let width = self.width();

        (0..height).flat_map(move |row| (0..width).map(move |column| Coordinate { row, column }))
    }

    // Line start coordinates

    pub(super) fn line_start_coordinates(
        &self,
        direction: Direction,
    ) -> Box<dyn Iterator<Item = Coordinate> + '_> {
        // Lines are arranged thusly:
        // Horizontal   Vertical    Diagonal    Anti-Diagonal
        //   0 1 2        0 1 2       0 1 2       0 1 2
        // 0 - - -      0 | | |     0 \ \ \     0 / / /
        // 1 - - -      1 | | |     1 \ \ \     1 / / /
        // 2 - - -      2 | | |     2 \ \ \     2 / / /
        // 3 - - -      3 | | |     3 \ \ \     3 / / /

        // Diagonal counts starting on the bottom-left
        // Anti-Diagonal counts starting on the top-left

        // Therefore the line indeces read thusly:
        // Horizontal   Vertical    Diagonal    Anti-Diagonal
        //              0 1 2       3 4 5         0 1 2
        // 0 - - -      | | |       2 \ \ \     / / / 3
        // 1 - - -      | | |       1 \ \ \     / / / 4
        // 2 - - -      | | |       0 \ \ \     / / / 5
        // 3 - - -      | | |         \ \ \     / / /

        match direction {
            Direction::Horizontal => Box::new(self.horizontal_start_coordinates()),
            Direction::Vertical => Box::new(self.vertical_start_coordinates()),
            Direction::Diagonal => Box::new(self.diagonal_start_coordinates()),
            Direction::AntiDiagonal => Box::new(self.anti_diagonal_start_coordinates()),
        }
    }
    // Horizontal and vertical start coordinates are trivial to calculate, so these functions should never be used
    // They are still implemented for completeness
    fn horizontal_start_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let max = self.height();
        Box::new((0..max).map(|row| Coordinate { row, column: 0 }))
    }

    fn vertical_start_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let max = self.width();
        Box::new((0..max).map(|column| Coordinate { row: 0, column }))
    }

    fn diagonal_start_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let (height, width) = (self.height(), self.width());
        let max = height + width;

        (1..height)
            .map(move |index| Coordinate {
                row: height - index,
                column: 0,
            })
            .chain((height..max).map(move |index| Coordinate {
                row: 0,
                column: index - height,
            }))
    }

    fn anti_diagonal_start_coordinates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        let (height, width) = (self.height(), self.width());

        (0..width)
            .map(|index| Coordinate {
                row: 0,
                column: index,
            })
            .chain((1..height).map(move |index| Coordinate {
                row: index,
                column: width - 1,
            }))
    }
}
