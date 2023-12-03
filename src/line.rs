use winit::event::VirtualKeyCode;
use bytemuck::{Pod, Zeroable};
use nalgebra::Vector2;

use crate::input;


pub struct Line {
    pub position: [f32; 2],
    pub angle: f32,
    pub length: f32,
}

impl Line {
    pub fn new() -> Self {
        Line {
            position: [super::WINDOW_WIDTH / 2.0, 0.0],
            angle: 0.0,
            length: super::WINDOW_HEIGHT,
        }
    }

    pub fn update(&mut self, input: &input::Input) {
        const ANGLE_SPEED: f32 = 0.1;
        const POSITION_SPEED: f32 = 5.0;

        if input.is_key_pressed(VirtualKeyCode::Left) {
            self.angle += ANGLE_SPEED;
        }

        if input.is_key_pressed(VirtualKeyCode::Right) {
            self.angle -= ANGLE_SPEED;
        }

        let direction = nalgebra::Vector2::new(self.angle.cos(), self.angle.sin());

        if input.is_key_pressed(VirtualKeyCode::Up) {
            self.position[0] += direction.x * POSITION_SPEED;
            self.position[1] += direction.y * POSITION_SPEED;
        }

        if input.is_key_pressed(VirtualKeyCode::Down) {
            self.position[0] -= direction.x * POSITION_SPEED;
            self.position[1] -= direction.y * POSITION_SPEED;
        }
    }

    pub fn get_points(&self) -> ([f32; 2], [f32; 2]) {
        let end_x = self.position[0] + self.length * self.angle.cos();
        let end_y = self.position[1] + self.length * self.angle.sin();

        (self.position, [end_x, end_y])
    }
}
