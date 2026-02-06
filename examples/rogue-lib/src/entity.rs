use bon::Builder;

#[derive(Builder)]
pub struct Enemy {
    pub x: u16,
    pub y: u16,
    icon: char,
    name: String,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
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
