
use bytemuck::{Pod, Zeroable};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, NUMBER_OF_CELLS, CELL_WIDTH};
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
// Define sprite types as constant arrays
pub const TOP_BUN: [f32; 4] = [0.0, 0.0, 0.2, 0.05];
pub const CHEESE: [f32; 4] = [0.0, 0.105, 0.2, 0.04];
pub const MEAT: [f32; 4] = [0.0, 0.145, 0.21, 0.04];
pub const LETTUCE: [f32; 4] = [0.0, 0.24, 0.2, 0.05];

//pub const ball_size: f32 =CELL_WIDTH - 20.0 as f32;

pub fn create_sprites() -> Vec<GPUSprite> {
    let mut sprites: Vec<GPUSprite> = vec![
        GPUSprite { //0 somethin weird is happenign where 1-4 are not showing up 
            screen_region: [WINDOW_WIDTH/2.0, -16.0, 64.0, 16.0],
            sheet_region: [0.0, 0.057, 0.2, 0.04], // bottom bun platform that moves
        },

        GPUSprite { // target image
            screen_region: [0.0,WINDOW_HEIGHT-248.0, 152.0, 248.0],
            sheet_region: [0.0, 0.3, 0.19, 0.5], // bottom bun platform that moves
        },
    ];

    for row in 0..5 {
        for col in 0..(NUMBER_OF_CELLS+1) as usize {
            let x_position = col as f32 * CELL_WIDTH;
            // let y_position = WINDOW_HEIGHT - (row as f32 * CELL_HEIGHT);   
            let y_position = WINDOW_HEIGHT+70.0;        

            let color_region = match (row % 2, col % 2) {
                (0, 0) => TOP_BUN,
                (0, 1) => CHEESE,
                (1, 0) => MEAT,
                (1, 1) => LETTUCE,
                _ => unreachable!(),
            };
    
            sprites.push(GPUSprite {
                screen_region: [x_position, y_position, 64.0, 16.0],
                sheet_region: color_region,
            });


        }
    }

    sprites
}


pub fn move_platform(input: &Input, mut platform_position: [f32; 2]) -> [f32; 2] {
    if input.is_key_down(winit::event::VirtualKeyCode::Left) {
        platform_position[0] -= 5.0;
    }
    if input.is_key_down(winit::event::VirtualKeyCode::Right) {
        platform_position[0] += 5.0;
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

