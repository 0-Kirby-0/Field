use crate::{Coordinate, Field};
use anyhow::Result;
impl<T> Field<T> {
    // Direct Access //

    pub fn into_grid(self) -> Vec<Vec<T>> {
        self.data
    }

    // Read Only //

    pub fn get_grid(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn get_value(&self, coord: Coordinate) -> Option<&T> {
        self.data.get(coord.row)?.get(coord.column)
    }

    //honestly not all that useful because flattening loses any 2d structure, but might as well
    pub fn flat_value_iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.get_grid().iter().flat_map(|line| line.iter())
    }

    ///Takes an iterator od coordinates and returns an iterator of the values at those coordinates
    pub fn coodinate_iter_access(
        &self,
        coords: impl Iterator<Item = Coordinate>,
    ) -> impl Iterator<Item = &T> {
        coords.flat_map(move |coord| self.get_value(coord))
    }

    // Write-Only //

    pub fn set_grid(&mut self, grid: Vec<Vec<T>>) {
        self.data = grid;
    }

    pub fn set_value(&mut self, coord: Coordinate, value: T) -> Result<()> {
        *self
            .data
            .get_mut(coord.row)
            .ok_or_else(|| anyhow::anyhow!("Row index out of bounds."))?
            .get_mut(coord.column)
            .ok_or_else(|| anyhow::anyhow!("Column index out of bounds."))? = value;

        Ok(())
    }

    ///Takes an iterator of Coordinate-Value pairs and sets the values at those coordinates
    pub fn set_coordinate_iter(
        &mut self,
        mut coord_value_pairs: impl Iterator<Item = (Coordinate, T)>,
    ) -> Result<()> {
        coord_value_pairs.try_for_each(|(coord, value)| self.set_value(coord, value))
    }

    // Read-Write //

    pub fn value_mut(&mut self, coord: Coordinate) -> Option<&mut T> {
        self.data.get_mut(coord.row)?.get_mut(coord.column)
    }

    pub fn flat_value_iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flat_map(|line| line.iter_mut())
    }
}
