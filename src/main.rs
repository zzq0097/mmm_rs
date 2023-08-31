use std::env;
use std::env::set_var;
use log::info;
use slint::{Model, ModelRc};
use crate::model::music::{Music, MusicType};
use crate::model::music_player::MusicPlayer;

pub mod model;
pub mod config;
mod service;
mod util;

slint::include_modules!();
slint::slint!(import { Player } from "ui/player.slint";);

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    set_var("RUST_LOG", "info");
    let user = env::var("wyy_user").unwrap();
    let password = env::var("wyy_password").unwrap();
    info!("start >>>");
    let mut music_player = MusicPlayer::new();
    info!("login user: {}", user);


    let playlist_id = 78645916;

    let music_api = ncm_api::MusicApi::new(10);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let login_info = rt.block_on(music_api.login(user, password)).unwrap();
    println!("{:?}", login_info);

    let play_list_detail = rt.block_on(music_api.song_list_detail(playlist_id)).unwrap();


    for song_info in play_list_detail.songs {
        music_player.add_to_playlist(Music {
            name: song_info.name,
            author: song_info.singer,
            music_type: MusicType::WYY,
            wid: song_info.id,
            qid: String::new(),
            url: song_info.song_url,
            file_path: String::new(),
        })
    }

    let ui = AppWindow::new()?;

    let pl = ui.get_playlist();
    pl.iter().for_each(|m| {
        let name = m.0;
        let author = m.1;
        println!("{}", name);
        println!("{}", author);
    });

    ui.on_play(move || {
        music_player.play()
    });

    ui.run()
}
