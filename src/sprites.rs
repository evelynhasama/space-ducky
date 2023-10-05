use bytemuck::{Pod, Zeroable};
use rand::Rng;
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

pub fn create_sprites() ->  Vec<GPUSprite> {
    
    return vec![
        GPUSprite {
            screen_region: [WINDOW_WIDTH/2.0, 32.0, 64.0, 64.0],
            sheet_region: [0.0, 0.0, 0.5, 0.5], // duck
        },
        GPUSprite {
            screen_region: [WINDOW_WIDTH, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            sheet_region: [0.5, 0.0, 0.5, 0.5], //bomb
        },
        GPUSprite {
            screen_region: [0.0, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            sheet_region: [0.5, 0.5, 0.5, 0.5], // asteroid
        },
        GPUSprite {
            screen_region: [WINDOW_WIDTH, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            // screen_region: [128.0, 128.0, 64.0, 64.0],
            sheet_region: [0.5, 0.0, 0.5, 0.5], //bomb
        },
        GPUSprite {
            screen_region: [0.0, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            sheet_region: [0.0, 0.5, 0.5, 0.5], // star
        },
        GPUSprite {
            screen_region: [WINDOW_WIDTH, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            sheet_region: [0.5, 0.0, 0.5, 0.5], //bomb
        },
        GPUSprite {
            screen_region: [0.0, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            sheet_region: [0.5, 0.5, 0.5, 0.5], // asteroid
        },
        GPUSprite {
            screen_region: [WINDOW_WIDTH, rand::thread_rng().gen_range(1..NUMBER_OF_CELLS) as f32 * CELL_HEIGHT, 64.0, 64.0],
            sheet_region: [0.5, 0.0, 0.5, 0.5], //bomb
        },
    ];
}

pub fn move_sprite_input(input: &Input, mut sprite_position: [f32; 2]) -> [f32; 2] {
        // Update sprite position based on keyboard input
        if input.is_key_pressed(winit::event::VirtualKeyCode::Up) {
            if sprite_position[1] + CELL_HEIGHT < WINDOW_HEIGHT {
                sprite_position[1] += CELL_HEIGHT;
            } else {
                sprite_position[1] = WINDOW_HEIGHT - CELL_HEIGHT;
            }
        }
        
        if input.is_key_pressed(winit::event::VirtualKeyCode::Down) {
            sprite_position[1] -= CELL_HEIGHT;

            if sprite_position[1] < 0.0 {
                sprite_position[1] = 0.0;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Left) {
            sprite_position[0] -= CELL_WIDTH;

            if sprite_position[0] < 0.0 {
                sprite_position[0] = 0.0;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Right) {
            if sprite_position[0] + CELL_WIDTH < WINDOW_WIDTH {
                sprite_position[0] += CELL_WIDTH;
            } else {
                sprite_position[0] = WINDOW_WIDTH - CELL_WIDTH;
            }
        }  
        sprite_position
}

