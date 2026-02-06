use crate::Player;
use rogue_lib::prelude::*;

trait MoveTo {
    fn move_to(&mut self, x: u16, y: u16);
}

impl MoveTo for Enemy {
    fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

impl MoveTo for Player {
    fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
