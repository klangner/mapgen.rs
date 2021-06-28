//! Example generator usage:
//! ```
//! use rand::prelude::*;
//! use mapgen::{Map, MapFilter};
//! use mapgen::filter::MazeBuilder;
//! 
//! let mut rng = StdRng::seed_from_u64(100);
//! let gen = MazeBuilder::new();
//! let map = gen.modify_map(&mut rng, &Map::new(80, 50));
//! 
//! assert_eq!(map.width, 80);
//! assert_eq!(map.height, 50);
//! ```
//! 

use rand::prelude::*;
use crate::MapFilter;
use crate::{
    map::{Map, Tile},
    random::Rng
};


pub struct MazeBuilder {}

impl MapFilter for MazeBuilder {
    fn modify_map(&self, rng: &mut StdRng, map: &Map)  -> Map {
        self.build(rng, map)
    }
}

impl MazeBuilder {
    pub fn new() -> Box<MazeBuilder> {
        Box::new(MazeBuilder{})
    }

    #[allow(clippy::map_entry)]
    fn build(&self, rng: &mut StdRng, map: &Map) -> Map {
        let mut new_map = map.clone();
        let mut maze = Grid::new((map.width as i32/ 2)-2, (map.height as i32/ 2)-2, rng);
        maze.generate_maze(&mut new_map);
        new_map
    }
}

/* Maze code taken under MIT from https://github.com/cyucelen/mazeGenerator/ */

const TOP : usize = 0;
const RIGHT : usize = 1;
const BOTTOM : usize = 2;
const LEFT : usize = 3;

#[derive(Copy, Clone)]
struct Cell {
    row: i32,
    column: i32,
    walls: [bool; 4],
    visited: bool,
}

impl Cell {
    fn new(row: i32, column: i32) -> Cell {
        Cell{
            row,
            column,
            walls: [true, true, true, true],
            visited: false
        }
    }

    fn remove_walls(&mut self, next : &mut Cell) {
        let x = self.column - next.column;
        let y = self.row - next.row;

        if x == 1 {
            self.walls[LEFT] = false;
            next.walls[RIGHT] = false;
        }
        else if x == -1 {
            self.walls[RIGHT] = false;
            next.walls[LEFT] = false;
        }
        else if y == 1 {
            self.walls[TOP] = false;
            next.walls[BOTTOM] = false;
        }
        else if y == -1 {
            self.walls[BOTTOM] = false;
            next.walls[TOP] = false;
        }
    }
}

struct Grid<'a> {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    backtrace: Vec<usize>,
    current: usize,
    rng : &'a mut StdRng
}

impl<'a> Grid<'a> {
    fn new(width: i32, height:i32, rng: &mut StdRng) -> Grid {
        let mut grid = Grid{
            width,
            height,
            cells: Vec::new(),
            backtrace: Vec::new(),
            current: 0,
            rng
        };

        for row in 0..height {
            for column in 0..width {
                grid.cells.push(Cell::new(row, column));
            }
        }

        grid
    }

    fn calculate_index(&self, row: i32, column: i32) -> i32 {
        if row < 0 || column < 0 || column > self.width-1 || row > self.height-1 {
            -1
        } else {
            column + (row * self.width)
        }
    }

    fn get_available_neighbors(&self) -> Vec<usize> {
        let mut neighbors : Vec<usize> = Vec::new();

        let current_row = self.cells[self.current].row;
        let current_column = self.cells[self.current].column;

        let neighbor_indices : [i32; 4] = [
            self.calculate_index(current_row -1, current_column),
            self.calculate_index(current_row, current_column + 1),
            self.calculate_index(current_row + 1, current_column),
            self.calculate_index(current_row, current_column - 1)
        ];

        for i in neighbor_indices.iter() {
            if *i != -1 && !self.cells[*i as usize].visited {
                neighbors.push(*i as usize);
            }
        }

        neighbors
    }

    fn find_next_cell(&mut self) -> Option<usize> {
        let neighbors = self.get_available_neighbors();
        if !neighbors.is_empty() {
            if neighbors.len() == 1 {
                return Some(neighbors[0]);
            } else {
                return Some(neighbors[(self.rng.roll_dice(1, neighbors.len())-1) as usize]);
            }
        }
        None
    }

    fn generate_maze(&mut self, map: &mut Map) {
        let mut i = 0;
        loop {
            self.cells[self.current].visited = true;
            let next = self.find_next_cell();

            match next {
                Some(next) => {
                    self.cells[next].visited = true;
                    self.backtrace.push(self.current);
                    //   __lower_part__      __higher_part_
                    //   /            \      /            \
                    // --------cell1------ | cell2-----------
                    let (lower_part, higher_part) =
                        self.cells.split_at_mut(std::cmp::max(self.current, next));
                    let cell1 = &mut lower_part[std::cmp::min(self.current, next)];
                    let cell2 = &mut higher_part[0];
                    cell1.remove_walls(cell2);
                    self.current = next;
                }
                None => {
                    if !self.backtrace.is_empty() {
                        self.current = self.backtrace[0];
                        self.backtrace.remove(0);
                    } else {
                        break;
                    }
                }
            }

            if i % 50 == 0 {
                self.copy_to_map(map);
            }
            i += 1;
        }
    }

    fn copy_to_map(&self, map: &mut Map) {
        // Clear the map
        for i in map.tiles.iter_mut() { *i = Tile::wall(); }

        for cell in self.cells.iter() {
            let x = (cell.column as usize + 1) * 2;
            let y = (cell.row as usize + 1) * 2;

            map.set_tile(x, y, Tile::floor());
            if !cell.walls[TOP] { map.set_tile(x, y-1, Tile::floor()) }
            if !cell.walls[RIGHT] { map.set_tile(x+1, y, Tile::floor()) }
            if !cell.walls[BOTTOM] { map.set_tile(x, y+1, Tile::floor()) }
            if !cell.walls[LEFT] { map.set_tile(x-1, y, Tile::floor()) }
        }
    }
}