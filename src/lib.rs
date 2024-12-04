#![deny(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
#![allow(dead_code)]

mod helpers;

mod access;
mod dimensions;
mod lines;
mod transform;

pub use crate::helpers::{Coordinate, Direction, Offset};

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
        T: Default,
    {
        Self {
            data: (0..height)
                .map(|_| (0..width).map(|_| T::default()).collect())
                .collect(),
        }
    }

    pub fn new_from_grid(data: Vec<Vec<T>>) -> Self {
        Self { data }
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

    pub fn kernel_iter<'a>(
        &'a self,
        coord: Coordinate,
        offsets: &'a [Offset],
    ) -> impl Iterator<Item = &'a T> + 'a {
        offsets
            .iter()
            .filter_map(move |&off| self.get_value((coord + off)?))
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

impl<T> std::fmt::Debug for Field<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().try_for_each(|line| {
            line.iter().try_for_each(|val| write!(f, "[{:?}]", val))?;
            writeln!(f)
        })
    }
}
