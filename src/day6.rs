use std::{collections::HashSet, fs};

use itertools::Itertools;

enum State {
    Run,
    Done,
}

enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn rotate(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

type Pos = (usize, usize);

struct Board {
    guard: Pos,
    guard_dir: Direction,
    board: Vec<Vec<bool>>,
    been: HashSet<Pos>,
    width: usize,
    height: usize,
    state: State,
}

impl Board {
    fn new(width: usize, height: usize, guard: Pos, board: Vec<Vec<bool>>) -> Board {
        Board {
            guard,
            guard_dir: Direction::N,
            board,
            been: HashSet::new(),
            width,
            height,
            state: State::Run,
        }
    }

    /*
     * Returns pos the guard is looking at
     * or none if they are looking off the
     * board
     */
    fn looking_at(&self) -> Option<Pos> {
        Some(match self.guard_dir {
            Direction::N => (self.guard.0, self.guard.1.checked_sub(1)?),
            Direction::E => {
                if self.guard.0 >= self.width {
                    None?
                } else {
                    (self.guard.0 + 1, self.guard.1)
                }
            }
            Direction::S => (
                self.guard.0,
                if self.guard.1 >= self.height {
                    None?
                } else {
                    self.guard.1 + 1
                },
            ),
            Direction::W => (self.guard.0.checked_sub(1)?, self.guard.1),
        })
    }

    /*
     * Returns true if there is a box in the given location
     */
    fn is_box(&self, pos: Pos) -> bool {
        *self.board.get(pos.1).unwrap().get(pos.0).unwrap()
    }

    /*
     * Returns the board after one step has been taken
     */
    fn next(mut self) -> Self {
        match self.looking_at() {
            Some(next) => {
                if self.is_box(next) {
                    self.guard_dir = self.guard_dir.rotate();
                } else {
                    self.been.insert(next);
                    self.guard = next;
                }
            }
            None => {
                self.state = State::Done;
            }
        }
        self
    }

    fn display(&self) -> String {
        for y in (0..self.height) {
            print!("|")
            for x in (0..self.width) {
                if self.is_box((x,y)) {
                    print!("#")
                } else {
                    print!("")
                }
            }
        }
        return String::new();
    }
}

fn get_all_unique_locations(mut board: Board) {
    while (board.state != State::Done) {}
}

fn part_1() -> u32 {}

fn part_2() -> u32 {
    42
}

pub fn run() {
    let contents =
        fs::read_to_string("src/day6.txt").expect("Should have been able to read the file");
    let part1 = part_1();
    let part2 = part_2();
    println!("part 1 -> {part1}");
    // Too low -> 217
    // Too low -> 6251
    println!("part 2 -> {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
    }
}
