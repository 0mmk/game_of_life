use std::{thread, time, env, fs};

fn main() {
    let filename = "game.txt";
    let contents = fs::read_to_string(filename)
                     .expect(format!("file `{}` not found", filename).as_str());
    let starting_value = contents.split("\n").skip(1).collect();
    let info = get_info(&contents);
    let mut game = Game::new(starting_value, info.alive, info.dead);

    clear_terminal();
    println!("{}", game.to_string());

    loop {
        thread::sleep(time::Duration::from_millis(info.millis));
        game.next();
        clear_terminal();
        print!("{}", game.to_string());
    }
}

struct Info {
    alive: char,
    dead: char,
    millis: u64
}

fn get_info(contents: &String) -> Info {
    let info: Vec<&str> = contents.split("\n").take(1).next().unwrap().split(":").collect();
    let alive  = info[0].chars().next().unwrap();
    let dead   = info[1].chars().next().unwrap();
    let millis  = info[2].parse::<u64>().unwrap();
    Info { alive: alive, dead: dead, millis: millis }
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
    fn new(starting_value: Vec<&str>, alive: char, dead: char) -> Self {
        let mut game = Game {
            grid: vec![],
            alive: alive,
            dead: dead
        };

        for row in starting_value {
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