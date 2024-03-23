use std::ops::{Add, Sub};

pub(crate) struct Tape<T> {
    cells: Vec<T>,
    pointer: usize,
}

impl<T> Tape<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T>,
{
    // Create a new Tape with specified memory length
    fn new(memory_length: usize) -> Self {
        Self {
            cells: vec![T::default(); memory_length],
            pointer: 0,
        }
    }
    // Increment the cell at the pointer
    fn increment(&mut self) {
        self.cells[self.pointer] = self.cells[self.pointer] + T::default();
    }
    // Decrement the cell at the pointer
    fn decrement(&mut self) {
        self.cells[self.pointer] = self.cells[self.pointer] - T::default();
    }
    // Move the pointer to the right
    fn move_right(&mut self) {
        if self.pointer < self.cells.len() - 1 {
            self.pointer += 1;
        }
    }
    // Move the pointer to the left
    fn move_left(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
    }
    // Get the value of the cell at the pointer
    fn get(&self) -> T {
        self.cells[self.pointer]
    }
    // Set the value of the cell at the pointer
    fn set(&mut self, value: T) {
        self.cells[self.pointer] = value;
    }
}