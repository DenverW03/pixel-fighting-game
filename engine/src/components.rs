pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Size {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

// Player component, used to identify... well, the player lol
pub struct Player {}
