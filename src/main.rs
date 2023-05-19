use crossterm::{cursor, ExecutableCommand};
use rand::Rng;
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

const WIDTH: i64 = 80;
const HEIGHT: i64 = 70;
const DELAY_TIME_IN_MS: u64 = 50;
const SKIP_DRAWING_FIRST_N_STEPS: u64 = 10000;
const AMOUNT_OF_ANTS: u8 = 1;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Ant {
    position: Point,
    direction: Direction,
    id: u8,
}

impl Ant {
    fn new(id: u8) -> Ant {
        let mut rng = rand::thread_rng();
        Ant {
            position: Point {
                x: rng.gen_range(0..WIDTH),
                y: rng.gen_range(0..HEIGHT),
            },
            direction: Direction::Up,
            id, // Ustawiamy identyfikator
        }
    }

    fn step(&mut self, black_cells: &mut HashSet<Point>) {
        if black_cells.contains(&self.position) {
            black_cells.remove(&self.position);
            self.turn_right();
        } else {
            black_cells.insert(self.position);
            self.turn_left();
        }

        self.move_forward();
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        };
    }

    fn move_forward(&mut self) {
        if self.position.x == 0 && self.direction == Direction::Left {
            self.position.x = WIDTH - 1;
        } else if self.position.x == WIDTH - 1 && self.direction == Direction::Right {
            self.position.x = 0;
        } else if self.position.y == 0 && self.direction == Direction::Up {
            self.position.y = HEIGHT - 1;
        } else if self.position.y == HEIGHT - 1 && self.direction == Direction::Down {
            self.position.y = 0;
        }

        match self.direction {
            Direction::Up => self.position.y -= 1,
            Direction::Right => self.position.x += 1,
            Direction::Down => self.position.y += 1,
            Direction::Left => self.position.x -= 1,
        }
    }
}

fn main() {
    let mut stdout = stdout();
    stdout.execute(cursor::Hide).unwrap();

    let mut ants = vec![];
    for id in 0..AMOUNT_OF_ANTS {
        ants.push(Ant::new(id));
    }
    let mut black_cells: HashSet<Point> = HashSet::new();
    let mut steps = 0;

    loop {
        steps += 1;
        for ant in &mut ants {
            ant.step(&mut black_cells);
        }

        if steps < SKIP_DRAWING_FIRST_N_STEPS {
            continue;
        }

        for ant in &mut ants {
            // Drawing ants
            stdout
                .execute(cursor::MoveTo(
                    (ant.position.x) as u16,
                    (ant.position.y) as u16,
                ))
                .unwrap();
            stdout.write(format!("{}", ant.id).as_bytes()).unwrap();
        }

        // Limit display to cells within this bounding box
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = Point { x, y };

                let symbol = if black_cells.contains(&cell) {
                    '█'
                } else {
                    '.'
                };

                stdout.execute(cursor::MoveTo(x as u16, y as u16)).unwrap();
                stdout.write(symbol.to_string().as_bytes()).unwrap();
            }
        }

        stdout
            .execute(cursor::MoveTo(0 as u16, HEIGHT as u16))
            .unwrap();
        stdout.write("Steps: ".as_bytes()).unwrap();
        stdout.write(steps.to_string().as_bytes()).unwrap();

        if steps >= 20000 {
            break;
        }

        let _ = stdout.flush();
        thread::sleep(Duration::from_millis(DELAY_TIME_IN_MS));
    }
}
