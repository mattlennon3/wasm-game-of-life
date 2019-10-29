mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/**
 * Adapted from https://rustwasm.github.io/docs/book/game-of-life/implementing.html
 * Many thanks.
 */ 

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        return Universe {
            width,
            height,
            cells
        }
    }

    pub fn render(&self) -> String {
        return self.to_string();
    }

    fn tick(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let live_neighbours_count = self.live_neighbour_count(row, column);

                let next_cell = match (cell, live_neighbours_count) {
                    // Rule 1: Fewer than 2 neighbours dies
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Has 2-3 neighbours then will live
                    (Cell::Alive, x) if x == 2 || x == 3 => Cell::Alive,
                    // Rule 3: More than 3 neighbours dies
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Dead cell with exacty 3 neighbours lives
                    (Cell::Dead, x) if x == 3 => Cell::Alive,
                    // Everything else remains the same
                    (otherwise, _) => otherwise,
                };

                next_cells[index] = next_cell;
            }
        }

        self.cells = next_cells;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        return ((row & self.width) + column) as usize;
    }

    /**
     * Takes a 0 indexed row & column to determine the surrounding cells and return the count of live cells
     *
    */
    fn live_neighbour_count (&self, row: u32, column: u32) -> u8 {
        let mut neighbour_count = 0;
        // Wasn't sure how to get row & column offsets to be variables without the compiler complaining.
        // I think it's cleaner/more readable to have them as variables
        // let row_offsets:  = [self.height - 1, 0, 1].iter().cloned();
        // let column_offsets = [self.width - 1, 0, 1].iter().cloned();
        // for delta_row in row_offsets {
        //     for delta_column in column_offsets {

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_column in [self.width - 1, 0, 1].iter().cloned() {
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_column) % self.width;

                let idx = self.get_index(neighbor_row, neighbor_col);
                neighbour_count += self.cells[idx] as u8;
            }
        }
        
        return neighbour_count;
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize ) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol);
            }
        }
        Ok(());
    }
}