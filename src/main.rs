use std::{thread, time};

fn main() {
    let initial_value = "   \n●●●\n   ";
    let mut game = Game::new(initial_value, '●', ' ');

    println!("{}", game.to_string());

    loop {
        thread::sleep(time::Duration::from_millis(250));
        game.next();
        clear_terminal();
        print!("{}", game.to_string());
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

struct Game {
    grid: Vec<Vec<bool>>,
    alive: char,
    dead: char
}

#[derive(Debug)]
struct Pos {
    row: isize,
    col: isize
}

impl Pos {
    fn from_usize(row: usize, col: usize) -> Self {
        Pos {row: row as isize, col: col as isize}
    }
    fn from(row: isize, col: isize) -> Self {
        Pos {row: row, col: col}
    }
}

impl Game {
    fn new(starting_value: &str, alive: char, dead: char) -> Self {
        let mut game = Game {
            grid: vec![],
            alive: alive,
            dead: dead
        };

        let rows = starting_value.split("\n");
        for row in rows {
            let row = row.chars().map(|c| c == alive).collect();
            game.grid.push(row);
        }
        game
    }

    fn next(&mut self) {
        let next_positions = self.calc_next_positions();
        self.change_grid(&next_positions);
    }

    fn calc_next_positions(&self, ) -> Vec<Pos> {
        let mut change_positions:Vec<Pos> = Vec::new();

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let curr_alive = self.grid[i][j];
                let next_alive = self.is_next_alive(i, j);
                if next_alive != curr_alive {
                    change_positions.push(Pos::from_usize(i, j));
                }
            }
        }

        change_positions
    }

    fn change_grid(&mut self, next_positions: &Vec<Pos>) {
        for pos in next_positions {
            let (row, col) = (pos.row as usize, pos.col as usize);
            self.grid[row][col] = !self.grid[row][col];
        }
    }

    fn is_next_alive(&self, row: usize, col: usize) -> bool {
        let curr_pos = Pos::from_usize(row, col);
        let neighbors = self.get_neighbors(&curr_pos);
        let num_of_neighbors = neighbors.iter()
                                        .map(|p| self.is_alive(p))
                                        .filter(|&e| e == true)
                                        .count();
        
        num_of_neighbors == 3 || (self.is_alive(&curr_pos) && num_of_neighbors == 2)
    }

    fn get_neighbors(&self, pos: &Pos) -> Vec<Pos> {
        let (row, col) = (pos.row, pos.col);

        vec![
            Pos::from(row-1, col-1), Pos::from(row-1, col), Pos::from(row-1, col+1),
            Pos::from(row,   col-1),                        Pos::from(row,   col+1),
            Pos::from(row+1, col-1), Pos::from(row+1, col), Pos::from(row+1, col+1),
        ]
    }

    fn is_alive(&self, pos: &Pos) -> bool {
        let (row, col) = (pos.row, pos.col);
        
        if row < 0 || row >= self.grid.len() as isize || col < 0 || col >= self.grid[row as usize].len() as isize {
            false
        } else {
            self.grid[row as usize][col as usize]
        }
    }
    
    fn to_string(&self) -> String {
        let mut ret = String::new();
        for row in &self.grid {
            for unit in row {
                ret.push(match unit {
                    true => self.alive,
                    false => self.dead,
                });
            }
            ret.push_str("\n");
        }
        ret
    }
}