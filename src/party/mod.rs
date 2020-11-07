use std::fmt; // On importe le module `fmt`

pub mod display;
pub mod file;
pub mod normal_game;
pub mod random_game;
pub mod rules;

#[derive(Debug)]
pub enum Orientation {
    North,
    West,
    South,
    Est,
}

#[derive(Debug)]
pub enum Instruction {
    F,
    L,
    R,
}
#[derive(Debug)]
pub struct Robot<'a> {
    id: i32,
    x: i32,
    y: i32,
    orientation: Orientation,
    instruction: Vec<&'a Instruction>,
}

#[derive(Debug)]
pub struct Terrain {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Obstacle {
    x: i32,
    y: i32,
    id: i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Crash {
    Collision(String),
    Obstacle(String),
    Attention(String),
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x_max = {}, y_max = {}", self.x, self.y)
    }
}
impl fmt::Display for Crash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Crash::Collision(e) => write!(f, "{}", e),
            Crash::Obstacle(e) => write!(f, "{}", e),
            Crash::Attention(e) => write!(f, "{}", e),
        }
    }
}
impl<'a> fmt::Display for Robot<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id = {}, x = {}; y = {}, orientation = {:?}, instruction = {:?}",
            self.id, self.x, self.y, self.orientation, self.instruction
        )
    }
}
