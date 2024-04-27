// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {x, y, d}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
        };
        Robot {d, ..self}
    }
 
    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
        };
        Robot {d, ..self}
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        match self.d {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1, 
        }
        Robot {x, y, ..self}
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |direction, instructions| {
            match instructions {
                'R' => direction.turn_right(),
                'L' => direction.turn_left(),
                'A' => direction.advance(),
                _ => direction,
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
