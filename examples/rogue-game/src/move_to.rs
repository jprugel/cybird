use crate::Player;
use rogue_lib::prelude::*;

pub trait MoveTo {
    fn move_to(&mut self, x: u16, y: u16);
}

impl MoveTo for &mut Enemy {
    fn move_to(&mut self, x: u16, y: u16) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl MoveTo for &mut Player {
    fn move_to(&mut self, x: u16, y: u16) {
        self.position.x = x;
        self.position.y = y;
    }
}
