use bytemuck::{Pod, Zeroable};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, NUMBER_OF_CELLS, CELL_WIDTH, CELL_HEIGHT};
use crate::input::Input;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUSprite {
    pub screen_region: [f32; 4],
    pub sheet_region: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUCamera {
    pub screen_pos: [f32; 2],
    pub screen_size: [f32; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpriteOption {
    Storage,
    Uniform,
    VertexBuffer,
}

pub const ball_size: f32 =CELL_WIDTH - 20.0 as f32;

pub fn create_sprites() -> Vec<GPUSprite> {
    let mut sprites: Vec<GPUSprite> = vec![
        GPUSprite { //0 somethin weird is happenign where 1-4 are not showing up 
            screen_region: [WINDOW_WIDTH/2.0, -16.0, 128.0, 128.0],
            sheet_region: [0.0,0.0, 0.2, 0.2], // orange ball (platformer)
        },
        GPUSprite { //1
            screen_region: [128.0, 500.0, ball_size, ball_size],
            sheet_region: [0.0,0.0, 0.2, 0.2], // yellow ball
        },
    ];

    for row in 0..5 {
        for col in 0..(NUMBER_OF_CELLS+1) as usize {
            let x_position = col as f32 * CELL_WIDTH;
            let y_position = WINDOW_HEIGHT - (row as f32 * CELL_HEIGHT);

            let top_bun =[0.0,0.0, 0.2, 0.05];
            let bottom_bun= [0.0, 0.05, 0.2, 0.04];
            let cheese = [0.0, 0.1, 0.2, 0.04];
            let meat = [0.0, 0.14, 0.2, 0.04];
            let sauce = [0.0, 0.18, 0.2, 0.05];
            let lettuce = [0.0, 0.23, 0.2, 0.05];


            sprites.push(GPUSprite {
                screen_region: [x_position, y_position, 64.0, 16.0],
                sheet_region: [0.0, 0.23, 0.2, 0.05],
            });
        }
    }
    
    

    
    sprites
}




pub fn move_platform(input: &Input, mut platform_position: [f32; 2]) -> [f32; 2] {
    if input.is_key_down(winit::event::VirtualKeyCode::Left) {
        platform_position[0] -= 5.0;
// change platform angle
    }
    if input.is_key_down(winit::event::VirtualKeyCode::Right) {
        platform_position[0] += 5.0;
// change platform angle
    }  
    if input.is_key_pressed(winit::event::VirtualKeyCode::Space) {
        // shoot ball
    }  

    // prevent from going off screen
    if platform_position[0] < 0.0 {
        platform_position[0] = 0.0;
    }
    if platform_position[0] + CELL_WIDTH > WINDOW_WIDTH {
        platform_position[0] = WINDOW_WIDTH - CELL_WIDTH;
    }
    platform_position
}

