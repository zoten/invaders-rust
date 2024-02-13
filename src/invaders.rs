use std::cmp::max;
use std::time::Duration;

use rusty_time::timer::Timer;

use crate::{
    frame::{self, Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

pub struct Invader {
    x: usize,
    y: usize,
}

// invaders manager
pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // let's not have invaders on the borders and let's them stop somewhere (9)
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y });
                }
            }
        }

        Self {
            army,
            move_timer: Timer::from_millis(2000),
            // positive, to the right
            direction: 1,
        }
    }

    // if they move, we need to play a sound
    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            // timer is ready, elt's reset
            self.move_timer.reset();

            let mut downwards = false;
            // if I am moving left (-1)
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    // we have to move down and change direction
                    downwards = true;
                    self.direction = 1;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    // we have to move down and change direction
                    downwards = true;
                    self.direction = -1;
                }
            }

            if downwards {
                // each time we move down the army speeds up
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);

                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            true
        } else {
            false
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            // roughly half of the time show different characters
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            }
        }
    }
}