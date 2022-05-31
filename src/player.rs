use quad::{ecs::Component, timing::Time, ty::Vec2};

use crate::{
    constant::{ANIMATION_COUNT, ANIMATION_SPEED, UPDATE_SPEED},
    hit_map::HitMap,
};

pub enum PlayerOrientation {
    Left,
    Right,
}

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
        if !hit_map.check_collision(x, self.position.y) {
            self.position.x = x;
            self.animate(time);    
        }
    }

    pub fn move_right(&mut self, time: &Time, hit_map: &HitMap) {
        self.orientation = PlayerOrientation::Right;
        let x = self.position.x + UPDATE_SPEED * time.delta_seconds();        
        if !hit_map.check_collision(x, self.position.y) {
            self.position.x = x;
            self.animate(time);    
        }
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
