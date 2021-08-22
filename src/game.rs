use crate::game_conf::GameConf;

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
        let mut game = Game {
            grid: vec![],
            conf: conf,
        };

        for row in &game.conf.starting_value {
            let row = row.chars().map(|c| c == game.conf.alive).collect();
            game.grid.push(row);
        }

        game
    }

    pub fn start(&mut self) {
        Game::clear_terminal();
        println!("{}", self.to_string());

        loop {
            std::thread::sleep(std::time::Duration::from_millis(self.conf.millis));
            self.next();
            Game::clear_terminal();
            println!("{}", self.to_string());
        }
    }

    fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn next(&mut self) {
        let next_positions = self.calc_next_positions();
        self.change_grid(&next_positions);
    }

    fn calc_next_positions(&self) -> Vec<Pos> {
        let mut next_positions: Vec<Pos> = Vec::new();

        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let curr_alive = self.grid[i][j];
                let next_alive = self.is_next_alive(i, j);
                if next_alive != curr_alive {
                    next_positions.push(Pos::from_usize(i, j));
                }
            }
        }

        next_positions
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
        let num_of_neighbors = neighbors
            .iter()
            .map(|p| self.is_alive(p))
            .filter(|&e| e == true)
            .count();

        num_of_neighbors == 3 || (self.is_alive(&curr_pos) && num_of_neighbors == 2)
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
        let (row, col) = (pos.row, pos.col);

        if row < 0
            || row >= self.grid.len() as isize
            || col < 0
            || col >= self.grid[row as usize].len() as isize
        {
            false
        } else {
            self.grid[row as usize][col as usize]
        }
    }

    fn row_to_string(&self, row: &Vec<bool>) -> String {
        row.iter()
            .map(|b| match b {
                true => self.conf.alive,
                false => self.conf.dead,
            })
            .collect::<String>()
    }

    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| self.row_to_string(row))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
