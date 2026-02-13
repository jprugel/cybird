use crate::vector::Vector2;
use bon::Builder;

#[derive(Builder)]
pub struct Enemy {
    pub position: Vector2<u16>,
    icon: char,
    name: String,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            position: Vector2 { x: 0, y: 0 },
            icon: 'E',
            name: "Enemy".to_string(),
        }
    }
}

impl Enemy {
    pub fn icon(&self) -> char {
        self.icon
    }
}
