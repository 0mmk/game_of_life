use crate::game_conf::GameConf;
use std::{thread, time};

pub struct Game {
    grid: Vec<Vec<bool>>,
    conf: GameConf,
}

struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    fn from_usize(row: usize, col: usize) -> Self {
        Pos {
            row: row as isize,
            col: col as isize,
        }
    }
    fn from(row: isize, col: isize) -> Self {
        Pos { row: row, col: col }
    }
}

impl Game {
    pub fn new(conf: GameConf) -> Self {
        let string_to_live_row = |row: &String| {
            row.chars()
               .map(|c| c == conf.alive)
               .collect()
        };

        let grid = conf.starting_value
                       .iter()
                       .map(|row| string_to_live_row(row))
                       .collect();

        Game { grid: grid, conf: conf }
    }

    pub fn start(&mut self) {
        Game::clear_terminal();
        println!("{}", self.to_string());

        loop {
            self.sleep();
            self.next();
            Game::clear_terminal();
            println!("{}", self.to_string());
        }
    }

    fn sleep(&self) {
        thread::sleep(time::Duration::from_millis(self.conf.millis));
    }

    fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn next(&mut self) {
        let next_positions = self.calc_next_positions();
        self.change_liveness_of_grid(&next_positions);
    }

    fn calc_next_positions(&self) -> Vec<Pos> {
        (0..self.grid.len())
            .map(|row| self.calc_next_positions_for_a_row(row))
            .into_iter()
            .flatten()
            .collect()
    }

    fn calc_next_positions_for_a_row(&self, row: usize) -> Vec<Pos> {
        (0..self.grid[row].len())
            .filter(|&col| {
                let curr_pos = Pos::from_usize(row, col);
                self.is_pos_alive(&curr_pos) != self.is_next_alive(&curr_pos)
            })
            .map(|col| Pos::from_usize(row, col))
            .collect()
    }

    fn change_liveness_of_grid(&mut self, next_positions: &Vec<Pos>) {
        for pos in next_positions {
            let (row, col) = (pos.row as usize, pos.col as usize);
            self.grid[row][col] = !self.grid[row][col];
        }
    }

    fn is_next_alive(&self, pos: &Pos) -> bool {
        let neighbors = self.get_neighbors(pos);
        let num_of_neighbors = neighbors
            .iter()
            .filter(|p| self.is_alive(p))
            .count();

        num_of_neighbors == 3 || (self.is_alive(pos) && num_of_neighbors == 2)
    }

    fn get_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let (row, col) = (pos.row, pos.col);

        vec![
            Pos::from(row - 1, col - 1),
            Pos::from(row - 1, col),
            Pos::from(row - 1, col + 1),
            Pos::from(row, col - 1),
            Pos::from(row, col + 1),
            Pos::from(row + 1, col - 1),
            Pos::from(row + 1, col),
            Pos::from(row + 1, col + 1),
        ]
    }

    fn is_alive(&self, pos: &Pos) -> bool {
        if self.is_outside_of_grid(pos) {
            false
        } else {
            self.is_pos_alive(pos)
        }
    }

    fn is_pos_alive(&self, pos: &Pos) -> bool {
        self.grid[pos.row as usize][pos.col as usize]
    }

    fn is_outside_of_grid(&self, pos: &Pos) -> bool {
        let (row, col) = (pos.row, pos.col);

        row < 0 
            || row >= self.grid.len() as isize
            || col < 0
            || col >= self.grid[row as usize].len() as isize
    }

    fn row_to_string(&self, row: &Vec<bool>) -> String {
        row.iter()
            .map(|alive| match alive {
                true => self.conf.alive,
                false => self.conf.dead,
            })
            .collect()
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| self.row_to_string(row))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
