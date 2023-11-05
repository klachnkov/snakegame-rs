use crate::raylib::init::snake::Snake;
use crate::raylib::config::Configure;
use raylib::prelude::*;

pub struct Food {
    pub color: Color,
    pub position: Vector2,
}
impl Food {
    pub fn new(food_clr:Color,snake:&Snake,config:&Configure) -> Self {
        let food = Self {
            color: food_clr,
            position: Self::respawn(snake,config)
        }; return food;
    }

    pub fn draw(&self,world:&mut RaylibDrawHandle,config:&Configure) {
        world.draw_rectangle(
            config.cell_info[0] * self.position.x as i32,
            config.cell_info[0] * self.position.y as i32,
            config.cell_info[0],config.cell_info[0],self.color,
        );
    }

    pub fn respawn(snake:&Snake,config:&Configure) -> Vector2 {
        loop {
            let mut should_spawn = true;
            let rand_vec2 = Vector2 {
                x: get_random_value::<i32>(0, config.cell_info[1] - 1) as f32,
                y: get_random_value::<i32>(1, config.cell_info[1] - 1) as f32,
            };
            for body_part in snake.body.iter() {
                if *body_part == rand_vec2 {
                    should_spawn = false;
                }
            }
            // NOTE: idk how to fix the corner spawn ama just hard code remove corners spawn
            if rand_vec2 == (Vector2 { x: 19.0, y: 19.0 })
            || rand_vec2 == (Vector2 { x: 0.0, y: 19.0 }) {
                should_spawn = false;
            }
            if should_spawn {
                return rand_vec2;
            } else { continue; }
        }
    }
}

