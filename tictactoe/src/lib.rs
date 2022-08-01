use std::collections::HashSet;
use std::ops::Add;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    X,
    O,
}

impl ToString for Shape {
    fn to_string(&self) -> String {
        match self {
            Shape::X => String::from("X"),
            Shape::O => String::from("O"),
        }
    }
}

pub struct TicTacToe {
    pub size: i8,
    victorynum: i8,
    exes: HashSet<Position>,
    owes: HashSet<Position>,
    pub winner: Option<Shape>,
}

impl TicTacToe {
    pub fn new(size: i8, victorynum: i8) -> TicTacToe {
        TicTacToe {
            size,
            victorynum,
            exes: HashSet::new(),
            owes: HashSet::new(),
            winner: Option::None,
        }
    }

    pub fn set_square(&mut self, x: i8, y: i8, shape: Shape) {
        if self.winner != None {
            return;
        }
        if self.exes.contains(&Position { x, y }) {
            return;
        }
        if self.owes.contains(&Position { x, y }) {
            return;
        }

        match shape {
            Shape::X => self.exes.insert(Position { x, y }),
            Shape::O => self.owes.insert(Position { x, y }),
        };
        if self.winner == None{
            self.check_victory(Shape::X);
        }
        if self.winner == None{
            self.check_victory(Shape::O);
        }
    }

    pub fn get_square(&self, x: i8, y: i8) -> &str {
        if self.exes.contains(&Position { x, y }) {
            "❌"
        } else if self.owes.contains(&Position { x, y }) {
            "⭕️"
        } else {
            "⬜️"
        }
    }

    fn set_victor(&mut self, shape: Shape) {
        self.winner = Some(shape);
    }

    pub fn display_victor(&self) -> String {
        return format!("The winner is {}!", self.winner.unwrap().to_string());
    }

    pub fn check_victory(&mut self, shape: Shape) {
        
        
        let list = match shape {
            Shape::X => self.exes.clone(),
            Shape::O => self.owes.clone(),
        };

        let mut count = 0;
        //Check Vertically
        for x in 1..=self.size {
            for y in 1..=self.size {
                if list.contains(&Position { x, y }) {
                    count += 1;
                }
            }
            if count == self.victorynum {
                self.set_victor(shape);
    
            };
            count = 0;
        }
        //Check Hoizontally
        for y in 1..=self.size {
            for x in 1..=self.size {
                if list.contains(&Position { x, y }) {
                    count += 1;
                }
            }
            if count == self.victorynum {
                self.set_victor(shape);
            };
            count = 0;
        }
        //Check Diagonally
        for y in 1..=self.size {
            for x in 1..=self.size {
                let new_count =
                    self.check_diag(Position { x, y }, &Position { x: 1, y: 1 }, 0, &list);
                println!("{}", new_count);
                if new_count == self.victorynum {
                    self.set_victor(shape);
                }

                let new_count =
                    self.check_diag(Position { x, y }, &Position { x: -1, y: 1 }, 0, &list);
                if new_count == self.victorynum {
                    self.set_victor(shape);
                }
            }
        }
    }

    fn check_diag(
        &self,
        true_pos: Position,
        added_pos: &Position,
        mut count: i8,
        shapes: &HashSet<Position>,
    ) -> i8 {
        if shapes.contains(&true_pos) {
            // println!("{}{}",true_pos.x, true_pos.y);
            count += 1;
        }

        let new_pos = true_pos + *added_pos;

        if shapes.contains(&new_pos) {
            count = self.check_diag(new_pos, added_pos, count, shapes);
        }
        // println!("{}", count);
        count
    }
}
