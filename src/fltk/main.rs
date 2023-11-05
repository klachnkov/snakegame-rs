use fltk::{
    app,prelude::*,window::Window,
    input::Input,input::IntInput,button::Button
};
use raylib::prelude::Color;
use crate::raylib::main::{CONFIG,run_game};
use crate::raylib::init::snake::DRAW_BODY_CONNECTED;
use crate::raylib::config::Configure;

static mut GAME_RUNNING:bool = false;

struct Gui;
impl Gui {
    fn head_clr_section() {
        let head_clr = Input::new(20,20,90,40,"");
        let mut head_btn = Button::new(120,20,90,40,"HEAD CLR");
        head_btn.set_callback(move |_| {
            match head_clr.value().to_lowercase().as_str() {
                "pink" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::PINK {
                            CONFIG.snake_color[1] = Color::PINK;
                        }
                    }
                }
                "purple" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::PURPLE {
                            CONFIG.snake_color[1] = Color::PURPLE;
                        }
                    }
                }
                "darkpurple" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::DARKPURPLE {
                            CONFIG.snake_color[1] = Color::DARKPURPLE;
                        }
                    }
                }
                "yellow" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::YELLOW {
                            CONFIG.snake_color[1] = Color::YELLOW;
                        }
                    }
                }
                "cyan" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::SKYBLUE {
                            CONFIG.snake_color[1] = Color::SKYBLUE;
                        }
                    }
                }
                "blue" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::BLUE {
                            CONFIG.snake_color[1] = Color::BLUE;
                        }
                    }
                }
                "darkblue" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::DARKBLUE {
                            CONFIG.snake_color[1] = Color::DARKBLUE;
                        }
                    }
                }
                "green" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::GREEN {
                            CONFIG.snake_color[1] = Color::GREEN;
                        }
                    }
                }
                "darkgreen" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::DARKGREEN {
                            CONFIG.snake_color[1] = Color::DARKGREEN;
                        }
                    }
                }
                "gold" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::GOLD {
                            CONFIG.snake_color[1] = Color::GOLD;
                        }
                    }
                }
                "orange" => {
                    unsafe {
                        if CONFIG.snake_color[1] != Color::ORANGE {
                            CONFIG.snake_color[1] = Color::ORANGE;
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn body_clr_section() {
        let body_clr = Input::new(20,70,90,40,"");
        let mut body_btn = Button::new(120,70,90,40,"BODY CLR");
        body_btn.set_callback(move |_| {
            match body_clr.value().to_lowercase().as_str() {
                "pink" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::PINK {
                            CONFIG.snake_color[0] = Color::PINK;
                        } 
                    }
                }
                "purple" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::PURPLE {
                            CONFIG.snake_color[0] = Color::PURPLE;
                        }
                    }
                }
                "darkpurple" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::DARKPURPLE {
                            CONFIG.snake_color[0] = Color::DARKPURPLE;
                        }
                    }
                }
                "yellow" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::YELLOW {
                            CONFIG.snake_color[0] = Color::YELLOW;
                        }
                    }
                }
                "cyan" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::SKYBLUE {
                            CONFIG.snake_color[0] = Color::SKYBLUE;
                        }
                    }
                }
                "blue" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::BLUE {
                            CONFIG.snake_color[0] = Color::BLUE;
                        }
                    }
                }
                "darkblue" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::DARKBLUE {
                            CONFIG.snake_color[0] = Color::DARKBLUE;
                        }
                    }
                }
                "green" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::GREEN {
                            CONFIG.snake_color[0] = Color::GREEN;
                        }
                    }
                }
                "darkgreen" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::DARKGREEN {
                            CONFIG.snake_color[0] = Color::DARKGREEN;
                        }
                    }
                }
                "orange" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::ORANGE {
                            CONFIG.snake_color[0] = Color::ORANGE;
                        }
                    }
                }
                "gold" => {
                    unsafe {
                        if CONFIG.snake_color[0] != Color::GOLD {
                            CONFIG.snake_color[0] = Color::GOLD;
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn cell_control() {
        let cell_size = IntInput::new(220,20,60,40,"");
        let mut size_btn = Button::new(290,20,90,40,"CELLSIZE");
        size_btn.set_callback(move |_| {
            unsafe {
                if !GAME_RUNNING {
                    if cell_size.value().parse::<i32>().unwrap() > 0 {
                        if CONFIG.cell_info[0] != cell_size.value().parse::<i32>().unwrap() {
                            CONFIG.cell_info[0] = cell_size.value().parse::<i32>().unwrap();
                        }
                    }
                }
            }
        });

        let cell_count = IntInput::new(220,70,60,40,"");
        let mut count_btn = Button::new(290,70,90,40,"AMOUNT");
        count_btn.set_callback(move |_| {
            unsafe {
                if !GAME_RUNNING {
                    if cell_count.value().parse::<i32>().unwrap() > 0 {
                        if CONFIG.cell_info[1] != cell_count.value().parse::<i32>().unwrap() {
                            CONFIG.cell_info[1] = cell_count.value().parse::<i32>().unwrap();
                        }
                    }
                }
            }
        });
    }

    fn core_control(root:app::App) {
        let mut run_button = Button::new(20,130,90,40,"RUN");
        run_button.set_callback(move |_| {
            unsafe {
                if !GAME_RUNNING {
                    std::thread::spawn(|| {
                        GAME_RUNNING = true;
                        run_game();
                        GAME_RUNNING = false;
                    });
                }
            }
        });

        let mut default_btn = Button::new(120,130,90,40,"DEFAULT");
        default_btn.set_callback(move |_| {
            unsafe {
                if CONFIG != Configure::default() {
                    CONFIG = Configure::default();
                }
            }
        });

        let mut connected_btn = Button::new(220,130,60,40,"SMALL");
        connected_btn.set_callback(move |_| {
            unsafe {
                if !GAME_RUNNING {
                    if DRAW_BODY_CONNECTED {
                        DRAW_BODY_CONNECTED = false;
                    } else {
                        DRAW_BODY_CONNECTED = true;
                    }
                }
            }
        });

        let mut close_btn = Button::new(290,130,90,40,"CLOSE");
        close_btn.set_callback(move |_| {
            root.quit();
        });
    }

    fn run_start() {
        let root = app::App::default().with_scheme(app::Scheme::Plastic);
        let mut win = Window::new(150,150,400,200,"SnakeLauncher");
        Self::set_win_icon(&mut win);

        Self::head_clr_section();
        Self::body_clr_section();
        Self::cell_control();
        Self::core_control(root);

        win.end(); win.show();
        root.run().unwrap();

    }

    fn set_win_icon(win:&mut Window) {
        use std::path::Path;
        let home_dir = homedir::get_my_home().unwrap().unwrap();
        let game_dir = Path::new(&home_dir)
            .join(r"AppData\LocalLow")
            .join(r"Klachnkov\SnakeGame-rs V2");
        let icon_path = Path::new(&game_dir).join("icon.png");

        win.set_icon(Some(fltk::image::PngImage::load(icon_path).unwrap()));
    }
}

pub fn run_app() {
    Gui::run_start();
}
