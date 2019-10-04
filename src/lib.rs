mod utils;

use wasm_bindgen::prelude::*;

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
    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row & self.width) + column) as usize
    }

    /**
     * Two loops which  
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