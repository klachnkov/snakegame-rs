use raylib::prelude::*;
use crate::raylib::{config::Configure,config};
use crate::raylib::init::{snake::Snake,food::Food};

pub static mut CONFIG:Configure = Configure::default();

struct Game {
    title:String,
    size:[i32;2],
    score:i32,
    highscore:i32,
    _version:f32
}
impl Game {
    const VERSION:f32 = 0.1;

    fn new(win_title:&str,win_size:[i32;2]) -> Self {
        let game = Self {
            title: win_title.to_string(),
            size:  win_size,score: 0,
            highscore: config::read_highscore(),
            _version: Self::VERSION
        }; return game;
    }

    fn draw(
        root:&mut RaylibHandle,thread:&RaylibThread,
        config:&Configure,snake:&Snake,food:&Food
    ) {
        let mut world = root.begin_drawing(thread);
        world.clear_background(Color::BLACK);

        food.draw(&mut world,config);
        snake.draw(&mut world,config);
    }

    fn update(
        &mut self,root:&RaylibHandle,thread:&RaylibThread,config:&Configure,
        snake:&mut Snake,food:&mut Food
    ) {
        if snake.is_alive {
            snake.wall_collision(config);
            snake.controls(root);
            snake.start_clock();
            snake.movement();
            snake.body_collision();
            snake.food_collision(root,thread,config,food,&self.highscore,&mut self.score);
        }
        if !snake.is_alive && root.is_key_pressed(KeyboardKey::KEY_SPACE) {
            snake.death_reset();
            if self.score > self.highscore {
                config::update_highscore(self.score);
                self.highscore = self.score;
            }
            self.score = 0;
            food.position = Food::respawn(snake,config);
            root.set_window_title(thread,&format!("snake-rs || highscore: {} || score: {}",self.highscore,self.score));
        }
    }

    fn run_start(&mut self,config:&Configure) {
        let (mut root,thread) = raylib::init()
            .size(self.size[0],self.size[1])
            .title(&self.title).build();
        root.set_target_fps(60);
        root.set_window_icon(config::load_icon());
        root.set_window_title(&thread,&format!("snake-rs || highscore: {} || score: {}",
                                               self.highscore,self.score));

        let mut snake = Snake::new();
        let mut food = Food::new(Color::MAROON,&snake,config);

        while !root.window_should_close() {
            Self::draw(&mut root,&thread,config,&snake,&food);
            self.update(&root,&thread,config,&mut snake,&mut food);
        }
    }
}

pub fn run_game(){
    // NOTE: DISABLE RAYLIB LOGS
    set_trace_log(TraceLogLevel::LOG_NONE);
    unsafe {
        let mut game = Game::new(
            &format!("snake-rs || highscore: 0 || score: 0")
            ,[
                CONFIG.cell_info[0]*CONFIG.cell_info[1],
                CONFIG.cell_info[0]*CONFIG.cell_info[1]
            ]);
        game.run_start(&CONFIG);
    }
}
