use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::{Drawable, Frame};

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    // will move at `timer` ms interval
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        // let's expand the timer so we can see the explosion
        self.timer = Timer::from_millis(250);
    }

    pub fn dead(&self) -> bool {
        // exploded and explosion shown, or out from the screen
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}
