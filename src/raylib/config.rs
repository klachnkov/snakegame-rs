use serde::{Deserialize, Serialize};
use raylib::prelude::Color;
use raylib::texture::Image;
use std::{ fs,io::{Read, Write},path::Path, };

#[derive(PartialEq)]
pub struct Configure {
    pub cell_info:[i32;2],
    pub snake_color:[Color;2]
}
impl Configure {
    pub const fn default() -> Self {
        let config = Self {
            cell_info:[20,20],
            snake_color:[Color::ORANGE,Color::GOLD]
        }; return config;
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    highscore: i32,
}

pub fn load_icon() -> Image {
    // NOTE using std::env::var causes the game to not work
    //let home_dir = std::env::var("HOME").expect("No APP_DATA directory");
    let home_dir = homedir::get_my_home().unwrap().unwrap();
    let game_dir = Path::new(&home_dir)
        .join(r"AppData\LocalLow")
        .join(r"Klachnkov\SnakeGame-rs V2");
    let icon_path = Path::new(&game_dir).join("icon.png");

    let icon = Image::load_image(icon_path.to_str().unwrap());
    return icon.unwrap();
}

pub fn read_highscore() -> i32 {
    // NOTE using std::env::var causes the game to not work
    //let home_dir = std::env::var("HOME").expect("No APP_DATA directory");
    let home_dir = homedir::get_my_home().unwrap().unwrap();
    let game_dir = Path::new(&home_dir)
        .join(r"AppData\LocalLow")
        .join(r"Klachnkov\SnakeGame-rs V2");
    let file_path = Path::new(&game_dir).join("data.json");

    let mut file = fs::File::open(file_path).unwrap();
    let mut file_content = String::new();
    let _ = file.read_to_string(&mut file_content);

    let data: Data = serde_json::from_str(&file_content).unwrap();
    return data.highscore;
}

pub fn update_highscore(highscore_value: i32) {
    // NOTE using std::env::var causes the game to not work
    //let home_dir = std::env::var("HOME").expect("No APP_DATA directory");
    let home_dir = homedir::get_my_home().unwrap().unwrap();
    let game_dir = Path::new(&home_dir)
        .join(r"AppData\LocalLow")
        .join(r"Klachnkov\SnakeGame-rs V2");
    let file_path = Path::new(&game_dir).join("data.json");

    let mut file = fs::File::options().write(true).open(&file_path).unwrap();

    let data = Data {
        highscore: highscore_value,
    };
    let data_json = serde_json::to_string(&data).unwrap();

    let _ = file.write_all(data_json.as_bytes());
}

pub fn init_folder() {
    // NOTE using std::env::var causes the game to not work
    //let home_dir = std::env::var("HOME").expect("No APP_DATA directory");
    let home_dir = homedir::get_my_home().unwrap().unwrap();
    let game_dir = Path::new(&home_dir)
        .join(r"AppData\LocalLow")
        .join(r"Klachnkov\SnakeGame-rs V2");
    if !game_dir.exists() {
        fs::create_dir_all(&game_dir).unwrap();

        let file_path = Path::new(&game_dir).join("data.json");
        match fs::metadata(&file_path) {
            Ok(_) => {}
            Err(_) => {
                fs::File::create(&file_path).unwrap();
                let mut file = fs::File::options().write(true).open(&file_path).unwrap();

                let data = Data { highscore: 0 };
                let data_json = serde_json::to_string(&data).unwrap();

                let _ = file.write_all(data_json.as_bytes());
            }
        }

        let icon_path = Path::new(&game_dir).join("icon.png");
        match fs::metadata(&icon_path) {
            Ok(_) => {}
            Err(_) => {
                // NOTE: idk if this stays forever "https://i.imgur.com/ybrniGv.png"
                let url = "https://github.com/kosatkanull/snakegame-rs/raw/master/assets/icon.png";
                let request = ureq::get(url).call().unwrap();

                let _ = fs::File::create(&icon_path);
                let mut file = fs::File::options().write(true).open(&icon_path).unwrap();

                let len: usize = request.header("Content-Length").unwrap().parse().unwrap();

                let mut bytes = Vec::with_capacity(len);
                request.into_reader().read_to_end(&mut bytes).unwrap();

                let _ = file.write_all(&bytes);
            }
        }
    }
}

