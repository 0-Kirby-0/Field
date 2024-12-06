# Field 

A set of utilities for manipulating 2D fields of data. 
This library is designed to abstractly handle two-dimensional structures, enabling intuitive operations on datasets that inherently belong to a 2D space.

It enables access and manipulation in the following patterns:

- **Rows**
- **Columns**
- **Diagonals**
- **Kernel Patterns** with customisable kernels


#### Constructors
```rust
//Populate the field with clones of value.
pub fn new(width: usize, height: usize, value: T) -> Self;

//Populate the field with T::Default
pub fn new_default(width: usize, height: usize) -> Self;

// Directly build the field from the given data
pub fn new_from_grid(data: Vec<Vec<T>>) -> Self;
```

#### Access

```rust
// Direct Access
pub fn into_grid(self) -> Vec<Vec<T>>;

// Read-Only
pub fn get_grid(&self) -> &Vec<Vec<T>>;
pub fn get_value(&self, coord: Coordinate) -> Option<&T>;
pub fn flat_value_iter(&self) -> impl Iterator<Item = &T>;
pub fn coodinate_iter_access(&self, coords: impl Iterator<Item = Coordinate>) -> impl Iterator<Item = &T>;
pub fn kernel_iter(&self, coord: Coordinate, offsets: &[Offset]) -> impl Iterator<Item = &T>;

// Write-Only
pub fn set_grid(&mut self, grid: Vec<Vec<T>>);
pub fn set_value(&mut self, coord: Coordinate, value: T) -> Result<()>;
// Populating from a flat iter is not yet implemented
pub fn set_coordinate_iter(&mut self, coord_value_pairs: impl Iterator<Item = (Coordinate, T)>) -> Result<()>;
// Setting values from a kernel iterator is not yet implemented


// Read-Write
// Mutable access to the entire grid is not yet implemented
pub fn value_mut(&mut self, coord: Coordinate) -> Option<&mut T>;
pub fn flat_value_iter_mut(&mut self) -> impl Iterator<Item = &mut T>;
// Mutable access to multiple arbitrary values is not possible due to Rust's borrowing rules
```

#### Dimensions

```rust
pub fn width(&self) -> usize;
pub fn height(&self) -> usize;
pub fn number_of_lines_in_direction(&self, direction: Direction) -> usize;
pub fn all_coordinates(&self) -> impl Iterator<Item = Coordinate>;
```

#### Lines

```rust
// Read-Only
pub fn get_all_lines_iter(&self, direction: Direction) -> impl Iterator<Item = Box<dyn Iterator<Item = &T>>>;
pub fn get_line_iter(&self, direction: Direction, index: usize) -> Option<Box<dyn Iterator<Item = &T>>>;

// Write-Only
pub fn set_all_lines_iter(&mut self, direction: Direction, lines: impl Iterator<Item = impl Iterator<Item = T>>) -> Result<()>;
pub fn set_line_iter(&mut self, direction: Direction, index: usize, line: impl Iterator<Item = T>) -> Result<()>;
```

#### Other

```rust
// Transformations
//Transform all values individually
pub fn transform_all<F, R>(self, transform: F) -> Field<R>;   

//Transform all values, line by line                                
pub fn transform_by_line<F, R>(self, direction: Direction, transform: F) -> Option<Field<R>>; 

// Merging
//Merges two fields into each other
pub fn merge_field<F, O, R>(self, other: Field<O>, merge: F) -> Field<R>; 

//Merges a line interator into the field
pub fn merge_line<F>(&mut self, direction: Direction, index: usize, line: impl Iterator<Item = T>, transform: F) -> Result<()>;

// Finding
pub fn find_all(&self, predicate: Fn(&T) -> bool) -> Vec<Coordinate>
```
