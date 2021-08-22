pub struct GameConf {
    pub alive: char,
    pub dead: char,
    pub millis: u64,
    pub starting_value: Vec<String>,
}

impl GameConf {
    pub fn new(contents: &String) -> Self {
        let info: Vec<&str> = contents
            .split("\n")
            .take(1)
            .next()
            .expect("Unable to load game information, usage dead:alive:milliseconds")
            .split(":")
            .collect();

        let alive = match info[0].chars().next() {
            Some(val) => val,
            None => 'X',
        };
        let dead = match info[1].chars().next() {
            Some(val) => val,
            None => ' ',
        };
        let millis = match info[2].parse::<u64>() {
            Ok(val) => val,
            Err(_) => 500,
        };
        let starting_value = contents
            .split("\n")
            .skip(1)
            .map(|s| s.to_string())
            .collect();
        GameConf {
            alive: alive,
            dead: dead,
            millis: millis,
            starting_value: starting_value,
        }
    }
}
