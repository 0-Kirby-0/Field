use crate::helpers::Direction;
use crate::Field;
use anyhow::Result;
impl<T> Field<T> {
    pub fn transform_all<F, R>(self, transform: F) -> Field<R>
    where
        F: Fn(T) -> R,
    {
        //theoretically, one could check if R implements into::<T> and transform in-place, rather than allocating a new field
        //and discarding the old. However, to my knowledge under Rust's type system that would require duplicate functions, which isn't exactly DRY
        //So: This is good enough :)
        let transformed_grid = self
            .into_grid()
            .into_iter()
            .map(|line| line.into_iter().map(&transform).collect())
            .collect();

        Field::new_from_grid(transformed_grid)
    }

    pub fn transform_by_line<F, R>(self, direction: Direction, transform: F) -> Option<Field<R>>
    where
        F: Fn(Vec<&T>) -> Vec<R>,
    {
        let transformed_lines: Vec<Vec<R>> = self
            .get_all_lines_iter(direction)
            .map(|line| transform(line.collect()))
            .collect();

        let new_field = Field::<R>::new_from_grid(transformed_lines);

        Some(new_field)
    }

    // Merging //

    pub fn merge_field<F, O, R>(self, other: Field<O>, merge: F) -> Field<R>
    where
        F: Fn(T, O) -> R,
    {
        let merged = self
            .into_grid()
            .into_iter()
            .zip(other.into_grid())
            .map(|(self_line, other_line)| {
                self_line
                    .into_iter()
                    .zip(other_line)
                    .map(|(self_val, other_val)| merge(self_val, other_val))
                    .collect()
            })
            .collect();

        Field::new_from_grid(merged)
    }

    pub fn merge_line<F>(
        &mut self,
        direction: Direction,
        index: usize,
        line: impl Iterator<Item = T>,
        transform: F,
    ) -> Result<()>
    where
        F: Fn(&T, T) -> T,
    {
        let tranformed_line = self
            .get_line_iter(direction, index)
            .ok_or_else(|| anyhow::anyhow!("Unable to retrieve line when merging."))?
            .zip(line)
            .map(|(self_val, other_val)| transform(self_val, other_val))
            .collect::<Vec<_>>();

        self.set_line_iter(direction, index, tranformed_line.into_iter())
    }
}
