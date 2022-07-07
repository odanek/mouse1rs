use quad::prelude::*;

use crate::{constant::*, hit_map::HitMap};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayerOrientation {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayerState {
    Standing,
    Jumping,
    Falling,
}

#[derive(Component)]
pub struct Player {
    pub orientation: PlayerOrientation,
    pub state: PlayerState,
    pub position: Vec2,
    pub jump_phase: f32,
    pub animation_phase: f32,
}

impl Player {
    pub fn move_left(&mut self, time: &Time, hit_map: &HitMap) {
        self.orientation = PlayerOrientation::Left;
        let x = self.position.x - UPDATE_SPEED * time.delta_seconds();
        if !hit_map.check_left(x, self.position.y) {
            self.position.x = x;
            self.animate(time);
        }
    }

    pub fn move_right(&mut self, time: &Time, hit_map: &HitMap) {
        self.orientation = PlayerOrientation::Right;
        let x = self.position.x + UPDATE_SPEED * time.delta_seconds();
        if !hit_map.check_right(x, self.position.y) {
            self.position.x = x;
            self.animate(time);
        }
    }

    pub fn move_up(&mut self, time: &Time, hit_map: &HitMap) {
        let y = self.position.y - UPDATE_SPEED * time.delta_seconds();
        if self.jump_phase >= PLAYER_JUMP_MAX || hit_map.check_top(self.position.x, y) {
            self.state = PlayerState::Falling;
        } else {
            self.position.y = y;
            self.jump_phase += UPDATE_SPEED * time.delta_seconds();
        }
    }

    pub fn move_down(&mut self, time: &Time, hit_map: &HitMap) {
        let y = self.position.y + UPDATE_SPEED * time.delta_seconds();
        if hit_map.check_bottom(self.position.x, y) {
            self.state = PlayerState::Standing;
        } else {
            self.position.y = y;
        }
    }

    pub fn jump(&mut self, hit_map: &HitMap) {
        if self.state == PlayerState::Standing
            && !hit_map.check_top(self.position.x, self.position.y - 1.0)
        {
            self.state = PlayerState::Jumping;
            self.jump_phase = 0.0;
        }
    }

    pub fn can_fall(&self, hit_map: &HitMap) -> bool {
        !hit_map.check_bottom(self.position.x, self.position.y + 1.0)
    }

    pub fn is_dead(&self, hit_map: &HitMap) -> bool {
        hit_map.check_dead(self.position.x, self.position.y)
    }

    pub fn is_next_level(&self, hit_map: &HitMap) -> bool {
        hit_map.check_next_level(self.position.x, self.position.y)
    }

    pub fn sprite_index(&self) -> usize {
        let index = match self.state {
            PlayerState::Falling | PlayerState::Jumping => 0,
            PlayerState::Standing => 1 + self.animation_phase as usize,
        };
        match self.orientation {
            PlayerOrientation::Left => 9 + index,
            PlayerOrientation::Right => index,
        }
    }

    fn animate(&mut self, time: &Time) {
        self.animation_phase += UPDATE_SPEED * ANIMATION_SPEED * time.delta_seconds();
        if self.animation_phase >= ANIMATION_COUNT {
            self.animation_phase = 0.0;
        }
    }
}
