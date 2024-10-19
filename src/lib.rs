#![deny(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
pub mod helpers;

use crate::helpers::{Axis, Coordinate};
use anyhow::{Ok, Result};

#[derive(Clone)]
pub struct Field<T> {
    data: Vec<Vec<T>>,
}

impl<T> Field<T> {
    // Constructors //

    pub fn new(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![vec![value; width]; height],
        }
    }

    pub fn new_default(width: usize, height: usize) -> Self
    where
        T: Clone + Default,
    {
        Self {
            data: vec![vec![T::default(); width]; height],
        }
    }

    pub fn new_from_grid(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    // Dimensions //

    pub fn width(&self) -> usize {
        self.data.first().unwrap_or(&vec![]).len()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn length_of_axis(&self, axis: Axis) -> usize {
        match axis {
            Axis::Row => self.width(),
            Axis::Column => self.height(),
        }
    }

    pub fn number_of_lines_in_axis(&self, axis: Axis) -> usize {
        match axis {
            Axis::Row => self.height(),
            Axis::Column => self.width(),
        }
    }

    // Direct Access //

    pub fn get_grid(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn set_grid(&mut self, grid: Vec<Vec<T>>) {
        self.data = grid;
    }

    // Line Access //

    pub fn get_line(&self, axis: Axis, index: usize) -> Option<Vec<T>>
    where
        T: Clone,
    {
        if index >= self.number_of_lines_in_axis(axis) {
            return None;
        }
        match axis {
            Axis::Row => self.data.get(index).cloned(),
            Axis::Column => self
                .data
                .iter()
                .map(|row| row.get(index).cloned())
                .collect::<Option<Vec<_>>>(),
        }
    }

    pub fn get_lines_context(&self, coord: Coordinate) -> Option<(Vec<T>, Vec<T>)>
    where
        T: Clone,
    {
        Some((
            self.get_line(Axis::Row, coord.row)?,
            self.get_line(Axis::Column, coord.column)?,
        ))
    }

    pub fn set_line(&mut self, axis: Axis, index: usize, line: Vec<T>) -> Result<()>
    where
        T: Clone,
    {
        if index >= self.number_of_lines_in_axis(axis) {
            return Err(anyhow::anyhow!("Attempted to set line out of bounds."));
        }
        match axis {
            Axis::Row => {
                *self
                    .data
                    .get_mut(index)
                    .ok_or_else(|| anyhow::anyhow!("Unable to set row."))? = line;
                Ok(())
            }
            Axis::Column => {
                self.data
                    .iter_mut()
                    .zip(line.iter())
                    .try_for_each(|(row, val)| {
                        *row.get_mut(index)
                            .ok_or_else(|| anyhow::anyhow!("Unable to set column."))? = val.clone();
                        Ok(())
                    })?;
                Ok(())
            }
        }
    }

    pub fn line_iterator(&self, axis: Axis) -> impl Iterator<Item = Vec<T>> + '_
    where
        T: Clone,
    {
        (0..self.number_of_lines_in_axis(axis)).filter_map(move |index| self.get_line(axis, index))
    }

    // Value Access //
    pub fn get_value(&self, coord: Coordinate) -> Option<&T> {
        self.data.get(coord.row)?.get(coord.column)
    }

    pub fn set_value(&mut self, coord: Coordinate, value: &T) -> Result<()>
    where
        T: Clone,
    {
        *self
            .data
            .get_mut(coord.row)
            .ok_or_else(|| anyhow::anyhow!("Row index out of bounds."))?
            .get_mut(coord.column)
            .ok_or_else(|| anyhow::anyhow!("Column index out of bounds."))? = value.clone();

        Ok(())
    }
    pub fn value_iterator<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        //honestly not all that useful because flattening loses any 2d structure, but might as well
        self.get_grid().iter().flat_map(|line| line.iter())
    }

    // Transformations //

    pub fn transform_all<F, R>(&self, transform: F) -> Field<R>
    where
        F: Fn(&T) -> R,
    {
        //theoretically, one could check if R implements into::<T> and transform in-place, rather than allocating a new field
        //and discarding the old. However, under the current type system that would require duplicate functions, which isn't exactly DRY
        //So: This is good enough :)
        let transformed_grid = self
            .get_grid()
            .iter()
            .map(|line| line.iter().map(&transform).collect())
            .collect();

        Field::new_from_grid(transformed_grid)
    }

    pub fn transform_by_line<F, R>(&self, axis: Axis, transform: F) -> Option<Field<R>>
    where
        T: Clone,
        R: Clone,
        F: Fn(Vec<T>) -> Vec<R>,
    {
        match axis {
            Axis::Row => {
                let transformed = self
                    .line_iterator(Axis::Row)
                    .map(&transform)
                    .collect::<Vec<_>>();

                Some(Field::new_from_grid(transformed))
            }

            Axis::Column => {
                let default_value = transform(self.get_line(axis, 0)?).first()?.clone();
                // Probably the dumbest way of getting a default value in the history of default values.
                // *But* this means R doesn't need to actually implement Default, only Clone, which it had to anyway.
                let mut transformed = Field::new(self.width(), self.height(), default_value);
                for (index, column) in self.line_iterator(Axis::Column).map(&transform).enumerate()
                {
                    transformed.set_line(Axis::Column, index, column).ok();
                }
                Some(transformed)
            }
        }
    }

    // Merging //

    pub fn merge_field<F, O, R>(&self, other: &Field<O>, merge: F) -> Field<R>
    where
        F: Fn(&T, &O) -> R,
    {
        let merged = self
            .get_grid()
            .iter()
            .zip(other.get_grid().iter())
            .map(|(self_line, other_line)| {
                self_line
                    .iter()
                    .zip(other_line.iter())
                    .map(|(self_val, other_val)| merge(self_val, other_val))
                    .collect()
            })
            .collect();
        Field::new_from_grid(merged)
    }

    pub fn merge_line<F>(
        &mut self,
        axis: Axis,
        index: usize,
        line: &[T],
        transform: F,
    ) -> Result<()>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
    {
        let merged_line = self
            .get_line(axis, index)
            .ok_or_else(|| anyhow::anyhow!("Unable to retrieve line when merging."))?
            .iter()
            .zip(line.iter())
            .map(|(self_val, other_val)| transform(self_val, other_val))
            .collect();

        self.set_line(axis, index, merged_line)
    }

    // Other //

    pub fn find_all<P>(&self, predicate: P) -> Vec<Coordinate>
    where
        P: Fn(&T) -> bool,
    {
        self.get_grid()
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, val)| predicate(val))
                    .map(move |(column, _)| Coordinate { row, column })
            })
            .collect()
    }

    pub fn debug_print(&self)
    where
        T: std::fmt::Debug,
    {
        println!("Debug Printing:");
        for line in self.get_grid() {
            for val in line {
                print!("[{:?}]", val)
            }
            println!();
        }
    }
}
