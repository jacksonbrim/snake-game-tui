use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::fmt::{Display, Formatter};
use std::io;

use rand::{thread_rng, Rng};
use std::collections::{HashSet, VecDeque};
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameState {
    Won,
    Playing,
    Lost,
    Paused,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug)]
pub struct SnakeGameViewModel {
    pub head: (i16, i16),
    pub snake: VecDeque<(i16, i16)>,
    pub dot: (i16, i16),
    pub score: u32,
    pub state: GameState,
    pub direction: Direction,
    pub available_spaces: HashSet<(i16, i16)>,
    pub boost_turns: u16,
    pub speed: u64,
}

impl SnakeGameViewModel {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let initial_dot = (rng.gen_range(0..50), rng.gen_range(0..50));
        let mut initial_location = (rng.gen_range(0..50), rng.gen_range(0..50));

        while initial_dot == initial_location {
            initial_location = (rng.gen_range(0..50), rng.gen_range(0..50));
        }

        let initial_directions = vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ];
        let index = rng.gen_range(0..initial_directions.len()); // Generate a random index
        let initial_direction: Direction = initial_directions[index];

        let mut available_spaces = HashSet::new();
        for x in 0..50 {
            for y in 0..50 {
                if initial_dot != (x, y) && initial_location != (x, y) {
                    available_spaces.insert((x, y));
                }
            }
        }

        SnakeGameViewModel {
            head: initial_location,
            snake: VecDeque::new(),
            dot: initial_dot,
            score: 0,
            state: GameState::Playing,
            direction: initial_direction,
            available_spaces,
            speed: 100,
            boost_turns: 0,
        }
    }
    pub fn handle_events(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(self.speed))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match (key.code, key.modifiers) {
                        (KeyCode::Char('q'), _) => return Ok(true),
                        (KeyCode::Char('n'), _) if self.state != GameState::Playing => {
                            self.reset_game()
                        }
                        (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::CONTROL) => {
                            self.up();
                            self.up()
                        }
                        (KeyCode::Char('k') | KeyCode::Up, _) => self.up(),
                        (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::CONTROL) => {
                            self.down();
                            self.down();
                        }
                        (KeyCode::Char('j') | KeyCode::Down, _) => self.down(),
                        (KeyCode::Char('h') | KeyCode::Left, KeyModifiers::CONTROL) => {
                            self.left();
                            self.left();
                        }
                        (KeyCode::Char('h') | KeyCode::Left, _) => self.left(),
                        (KeyCode::Char('l') | KeyCode::Right, KeyModifiers::CONTROL) => {
                            self.right();
                            self.right();
                        }
                        (KeyCode::Char('l') | KeyCode::Right, _) => self.right(),
                        (KeyCode::Char(' '), _) => {
                            // navigate stack up
                            match self.state {
                                GameState::Playing => self.state = GameState::Paused,
                                GameState::Paused => self.state = GameState::Playing,
                                _ => (),
                            }
                        }
                        (KeyCode::Char('b'), _) => {
                            self.boost();
                        }
                        (KeyCode::Enter, _) => {
                            if self.state != GameState::Playing {
                                self.reset_game();
                            }
                        }
                        _ => (),
                    }
                }
            }
        } else {
            self.same_direction();
        }

        Ok(false)
    }
    fn reset_game(&mut self) {
        let mut rng = thread_rng();

        // Generate initial dot and location within 0..50 bounds
        let initial_dot = (rng.gen_range(0..50), rng.gen_range(0..50));
        let mut initial_location = (rng.gen_range(0..50), rng.gen_range(0..50));

        // Ensure initial dot and location are not the same; if so, regenerate the location
        while initial_dot == initial_location {
            initial_location = (rng.gen_range(0..50), rng.gen_range(0..50));
        }

        let initial_directions = vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ];
        let index = rng.gen_range(0..initial_directions.len()); // Generate a random index
        let initial_direction: Direction = initial_directions[index];

        let mut available_spaces = HashSet::new();
        for x in 0..50 {
            for y in 0..50 {
                if initial_dot != (x, y) && initial_location != (x, y) {
                    available_spaces.insert((x, y));
                }
            }
        }
        self.head = initial_location;
        self.snake = VecDeque::new();
        self.dot = initial_dot;
        self.score = 0;
        self.state = GameState::Playing;
        self.direction = initial_direction;
        self.available_spaces = available_spaces;
        self.speed = 100;
        self.boost_turns = 0;
    }
    fn boost(&mut self) {
        if self.speed > 50 && self.boost_turns == 0 && self.score > 0 {
            self.boost_turns = 300;
            self.speed = self.speed - 50;
        }
    }

    fn update_game(&mut self) {
        // (x, y)
        if self.snake.contains(&self.head)
            || !(0..50).contains(&self.head.0)
            || !(0..50).contains(&self.head.1)
        {
            // Game over
            self.state = GameState::Lost;
        } else if self.dot == self.head {
            // Snake grows by one
            self.score = self.score + 1;
            self.snake.push_front(self.head);
            self.available_spaces.remove(&self.head);
            // increase speed
            match self.score {
                // speed: 100 -> 76 (-20)
                // score: 0..30
                score if score > 0 && score <= 30 && score % 5 == 0 => self.speed = self.speed - 4,
                // speed: 76 -> 51 (-25)
                // score: 25..2500
                score if score > 0 && score % 100 == 0 => self.speed = self.speed - 1,
                _ => {}
            }
            if self.score == 2500 {
                self.state = GameState::Won;
            } else {
                let new_point = self.get_random_point_from_set();
                if let Some(point) = new_point {
                    self.available_spaces.remove(&point);
                    self.dot = point;
                }
            }
        } else if self.score <= 50 && self.state == GameState::Playing {
            // Snake moves
            self.snake.push_front(self.head);
            self.available_spaces.remove(&self.head);
            let tail = self.snake.pop_back();
            // put the space back in the available spaces
            if let Some(space) = tail {
                self.available_spaces.insert(space);
            }
        }
        match self.boost_turns {
            1 => {
                self.boost_turns = 0;
                self.speed = self.speed + 50;
            }
            2..=300 => {
                self.boost_turns = self.boost_turns - 1;
            }
            _ => (),
        }
    }
    fn get_random_point_from_set(&mut self) -> Option<(i16, i16)> {
        if self.available_spaces.is_empty() {
            return None; // Return None if the HashSet is empty
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.available_spaces.len()); // Generate a random index
        self.available_spaces.iter().nth(index).copied() // Access the element at the random index
    }
    fn up(&mut self) {
        if self.state != GameState::Playing || self.direction == Direction::Down {
            return {};
        }
        self.direction = Direction::Up;
        let y = self.head.1;
        let x = self.head.0;
        self.head = (x, y + 1);
        self.update_game();
    }
    fn down(&mut self) {
        let y = self.head.1;
        let x = self.head.0;

        if self.state != GameState::Playing || self.direction == Direction::Up {
            return {};
        }
        self.direction = Direction::Down;
        self.head = (x, y - 1);
        self.update_game();
    }
    fn right(&mut self) {
        let y = self.head.1;
        let x = self.head.0;

        if self.state != GameState::Playing || self.direction == Direction::Left {
            return {};
        }
        self.direction = Direction::Right;
        self.head = (x + 1, y);
        self.update_game();
    }
    fn left(&mut self) {
        let y = self.head.1;
        let x = self.head.0;

        if self.state != GameState::Playing || self.direction == Direction::Right {
            return {};
        }
        self.direction = Direction::Left;
        self.head = (x - 1, y);
        self.update_game();
    }
    fn same_direction(&mut self) {
        if self.state != GameState::Playing {
            return {};
        }
        match self.direction {
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Up => self.up(),
            Direction::Down => self.down(),
        }
    }
}
impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for SnakeGameViewModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Head: ({}, {})\n", self.head.0, self.head.1)?;
        write!(f, "Snake: [")?;
        for (x, y) in &self.snake {
            write!(f, "({}, {}), ", x, y)?;
        }
        write!(f, "]\nDot: ({}, {})\n", self.dot.0, self.dot.1)?;
        write!(f, "Score: {}\n", self.score)?;
        write!(f, "State: {}\n", self.state)?;
        write!(f, "Direction: {}\n", self.direction)?;
        write!(f, "]")
    }
}
