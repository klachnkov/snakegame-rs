use raylib::prelude::*;
use std::{collections::VecDeque, ops::Add};
use crate::raylib::init::food::Food;
use crate::raylib::config::Configure;

pub static mut DRAW_BODY_CONNECTED:bool = true;

#[derive(PartialEq)]
enum Direction {
    Right,Left,Up,Down,None
}

pub struct Snake {
    pub body:VecDeque<Vector2>,
    direction:Direction,
    should_take_input:bool,
    pub is_alive:bool,
    inner_clock:f64,
    interval:f64
}
impl Snake {
    pub fn new() -> Self {
        let snake = Self{
            body:default_spawn(),
            direction:Direction::None,
            should_take_input:true,
            is_alive:true,
            inner_clock:0.0,
            interval:0.0
        }; return snake;
    }

    pub fn death_reset(&mut self) {
        if self.is_alive == false {
            self.body = default_spawn();
            self.direction = Direction::None;
            self.should_take_input = true;
            self.is_alive = true;
            self.interval = 0.0;
            self.inner_clock = 0.0;
        }
    }

    pub fn draw(&self,world:&mut RaylibDrawHandle,config:&Configure) {
        for (i,body_part) in self.body.iter().enumerate() {
            if i != 0 {
                unsafe {
                    if DRAW_BODY_CONNECTED {
                        world.draw_rectangle(
                            config.cell_info[0] * body_part.x as i32,
                            config.cell_info[0] * body_part.y as i32,
                            config.cell_info[0], config.cell_info[0],
                            config.snake_color[0]
                        );
                    } else {
                        world.draw_rectangle(
                            config.cell_info[0] * body_part.x as i32,
                            config.cell_info[0] * body_part.y as i32,
                            config.cell_info[0]-2, config.cell_info[0]-2,
                            config.snake_color[0]
                        );
                    }
                }
            }
        }
        unsafe {
            if DRAW_BODY_CONNECTED {
                world.draw_rectangle(
                    config.cell_info[0] * self.body[0].x as i32,
                    config.cell_info[0] * self.body[0].y as i32,
                    config.cell_info[0], config.cell_info[0],
                    config.snake_color[1]
                );
            } else {
                world.draw_rectangle(
                    config.cell_info[0] * self.body[0].x as i32,
                    config.cell_info[0] * self.body[0].y as i32,
                    config.cell_info[0]-2, config.cell_info[0]-2,
                    config.snake_color[1]
                );
            }
        }
    }

    pub fn controls(&mut self, root: &RaylibHandle) {
        if root.is_key_pressed(KeyboardKey::KEY_RIGHT)
            && self.direction != Direction::Left
            && self.direction != Direction::Right
            && self.should_take_input
        {
            self.direction = Direction::Right;
            self.should_take_input = false;
        } else if root.is_key_pressed(KeyboardKey::KEY_LEFT)
            && self.direction != Direction::Right
            && self.direction != Direction::Left
            && self.direction != Direction::None
            && self.should_take_input
        {
            self.direction = Direction::Left;
            self.should_take_input = false;
        } else if root.is_key_pressed(KeyboardKey::KEY_UP)
            && self.direction != Direction::Down
            && self.direction != Direction::Up
            && self.should_take_input
        {
            self.direction = Direction::Up;
            self.should_take_input = false;
        } else if root.is_key_pressed(KeyboardKey::KEY_DOWN)
            && self.direction != Direction::Up
            && self.direction != Direction::Down
            && self.should_take_input
        {
            self.direction = Direction::Down;
            self.should_take_input = false;
        }
    }

    pub fn wall_collision(&mut self,config:&Configure) {
        if self.body[0].x as i32 > config.cell_info[1] - 1 {
            self.is_alive = false;
        }
        if (self.body[0].x as i32) < 0 {
            self.is_alive = false;
        }
        if self.body[0].y as i32 > config.cell_info[1] - 1 {
            self.is_alive = false;
        }
        if (self.body[0].y as i32) < 0 {
            self.is_alive = false;
        }
    }

    pub fn body_collision(&mut self) {
        for (i, body_part) in self.body.iter().enumerate() {
            if i != 0 && self.body[0] == *body_part {
                self.is_alive = false;
            }
        }
    }

    pub fn food_collision(
        &mut self,root:&RaylibHandle,thread:&RaylibThread,config:&Configure,
        food:&mut Food,highscore:&i32,score:&mut i32
    ) {
        if self.body[0] == food.position {
            match self.direction {
                Direction::Right => {
                    self.body
                        .push_front(Vector2 { x: 1.0, y: 0.0 }.add(self.body[0]));
                }
                Direction::Left => {
                    self.body
                        .push_front(Vector2 { x: -1.0, y: 0.0 }.add(self.body[0]));
                }
                Direction::Up => {
                    self.body
                        .push_front(Vector2 { x: 0.0, y: -1.0 }.add(self.body[0]));
                }
                Direction::Down => {
                    self.body
                        .push_front(Vector2 { x: 0.0, y: 1.0 }.add(self.body[0]));
                }
                _ => {}
            }
            food.position = Food::respawn(self,config);
            *score += 1;
            root.set_window_title(thread,&format!("snake-rs || highscore: {} || score: {}",highscore,score));
        }
    }

    pub fn start_clock(&mut self) {
        self.inner_clock += 0.017;
    }

    pub fn movement(&mut self) {
        if self.inner_clock > self.interval {
            self.movement_init();
            self.interval += 0.1;
        }
    }

    fn movement_init(&mut self) {
        match self.direction {
            Direction::Right => {
                self.body.pop_back();
                self.body
                    .push_front(Vector2 { x: 1.0, y: 0.0 }.add(self.body[0]));
                if !self.should_take_input {
                    self.should_take_input = true;
                }
            }
            Direction::Left => {
                self.body.pop_back();
                self.body
                    .push_front(Vector2 { x: -1.0, y: 0.0 }.add(self.body[0]));
                if !self.should_take_input {
                    self.should_take_input = true;
                }
            }
            Direction::Up => {
                self.body.pop_back();
                self.body
                    .push_front(Vector2 { x: 0.0, y: -1.0 }.add(self.body[0]));
                if !self.should_take_input {
                    self.should_take_input = true;
                }
            }
            Direction::Down => {
                self.body.pop_back();
                self.body
                    .push_front(Vector2 { x: 0.0, y: 1.0 }.add(self.body[0]));
                if !self.should_take_input {
                    self.should_take_input = true;
                }
            }
            _ => {}
        }
    }
}

fn default_spawn() -> VecDeque<Vector2> {
    let mut default_spawn = VecDeque::new();
    default_spawn.push_back(Vector2 { x: 3.0, y: 1.0 });
    default_spawn.push_back(Vector2 { x: 2.0, y: 1.0 });
    default_spawn.push_back(Vector2 { x: 1.0, y: 1.0 });
    return default_spawn;
}
