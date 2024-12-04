use crate::{Direction, Field};
use anyhow::Result;

impl<T> Field<T> {
    // Read-Only //

    //Use special overloads for horizontal and vertical lines, as they are trivial to calculate

    pub fn get_all_lines_iter(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = Box<dyn Iterator<Item = &T> + '_>> + '_ {
        (0..self.number_of_lines_in_direction(direction)).map(move |index| {
            self.get_line_iter(direction, index)
                .unwrap_or(Box::new(std::iter::empty())) //As indeces are generated by the range, this should never be reached
        })
    }

    pub fn get_line_iter<'a>(
        &'a self,
        direction: Direction,
        index: usize,
    ) -> Option<Box<dyn Iterator<Item = &'a T> + 'a>> {
        if index >= self.number_of_lines_in_direction(direction) {
            return None;
        }
        match direction {
            Direction::Horizontal => Some(Box::new(self.horizontal_line_iter(index))),
            Direction::Vertical => Some(Box::new(self.vertical_line_iter(index))),
            _ => Some(Box::new(self.generic_line_iter(direction, index))),
        }
    }

    fn generic_line_iter(&self, direction: Direction, index: usize) -> impl Iterator<Item = &T> {
        let start_coord = self
            .line_start_coordinates(direction)
            .nth(index)
            .unwrap_or_default();
        let line_coord_iter = start_coord.line_in_direction(direction, self.width(), self.height());
        self.coodinate_iter_access(line_coord_iter)
    }

    //Special overloads for horizontal and vertical lines, as they are trivial to calculate
    fn horizontal_line_iter(&self, index: usize) -> impl Iterator<Item = &T> {
        self.data
            .get(index)
            .map(|line| line.iter())
            .unwrap_or_default()
    }

    fn vertical_line_iter(&self, index: usize) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(move |line| line.get(index))
    }

    // Write-Only //

    pub fn set_all_lines_iter(
        &mut self,
        direction: Direction,
        lines: impl Iterator<Item = impl Iterator<Item = T>>,
    ) -> Result<()> {
        lines
            .enumerate()
            .try_for_each(|(index, line)| self.set_line_iter(direction, index, line))
    }

    pub fn set_line_iter(
        &mut self,
        direction: Direction,
        index: usize,
        line: impl Iterator<Item = T>,
    ) -> Result<()> {
        match direction {
            Direction::Horizontal => self.set_horizontal_line(index, line),
            Direction::Vertical => self.set_vertical_line(index, line),
            _ => self.set_generic_line(direction, index, line),
        }
    }

    fn set_generic_line(
        &mut self,
        direction: Direction,
        index: usize,
        line: impl Iterator<Item = T>,
    ) -> Result<()> {
        let start_coord = self
            .line_start_coordinates(direction)
            .nth(index)
            .ok_or(anyhow::anyhow!("Index out of bounds while writing line."))?;
        let line_coord_iter = start_coord.line_in_direction(direction, self.width(), self.height());
        self.set_coordinate_iter(line_coord_iter.zip(line))
    }

    fn set_horizontal_line(&mut self, index: usize, line: impl Iterator<Item = T>) -> Result<()> {
        self.data
            .get_mut(index)
            .ok_or(anyhow::anyhow!("Index out of bounds while writing line."))?
            .iter_mut()
            .zip(line)
            .for_each(|(cell, value)| {
                *cell = value;
            });
        Ok(())
    }

    fn set_vertical_line(&mut self, index: usize, mut line: impl Iterator<Item = T>) -> Result<()> {
        self.data.iter_mut().try_for_each(|row| {
            if let Some(value) = line.next() {
                *row.get_mut(index)
                    .ok_or(anyhow::anyhow!("Index out of bounds while writing line."))? = value;
                Ok(())
            } else {
                Err(anyhow::anyhow!("Line length mismatch while writing line."))
            }
        })
    }
}
