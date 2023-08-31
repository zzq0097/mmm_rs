
use std::fs::File;
use std::io::{BufReader, Cursor};
use log::info;
use ncm_api::MusicApi;

use reqwest::StatusCode;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use crate::model::music::{Music, MusicType};

pub struct MusicPlayer {
    playlist: Vec<Music>,
    state: PlayerState,
    play_mode: PlayMode,
    sink: Sink,
    output_stream: OutputStream,
    output_stream_handle: OutputStreamHandle
}

pub enum PlayerState {
    Playing,
    Paused,
    Stopped,
}

pub enum PlayMode {
    SingleLoop,
    ListLoop,
    Random
}

impl MusicPlayer {

    pub fn new() -> Self {
        let (output_stream, output_stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&output_stream_handle).unwrap();
        Self {
            playlist: vec![],
            state: PlayerState::Stopped,
            play_mode: PlayMode::ListLoop,
            sink,
            output_stream,
            output_stream_handle
        }
    }

    pub fn play(&mut self) {
        match self.state {
            PlayerState::Playing => {
                return;
            }
            PlayerState::Paused => {
                return;
            }
            PlayerState::Stopped => {

            }
        }

        if self.playlist.is_empty() {
            info!("playlist is empty")

        } else {
            let music = &self.playlist[0];
            match music.music_type {
                MusicType::WYY => {
                    let music_api = MusicApi::new(10);
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let vec = rt.block_on(music_api.songs_url(&[music.wid], "320000")).unwrap();
                    let song_url = vec.get(0).unwrap().to_owned();
                    println!("url: {}", song_url.url);
                    let resp = reqwest::blocking::get(song_url.url).unwrap();
                    if resp.status() == StatusCode::OK {
                        let bytes = resp.bytes().unwrap();
                        let cursor = Cursor::new(bytes);
                        let source = Decoder::new(cursor).unwrap();
                        self.output_stream_handle.play_raw(source.convert_samples()).unwrap();
                        self.state = PlayerState::Playing;
                        self.sink.sleep_until_end()
                    } else {
                        info!("status code: {}", resp.status())
                    }
                }
                MusicType::QQ => {

                }
                MusicType::LOCAL => {
                    let file = File::open(music.file_path.to_owned()).expect("Failed to open audio file");
                    let source = Decoder::new(BufReader::new(file)).expect("Failed to create audio decoder");
                    self.output_stream_handle.play_raw(source.convert_samples()).unwrap();
                    self.state = PlayerState::Playing;
                    self.sink.sleep_until_end()
                }
            }
        }
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.state = PlayerState::Stopped
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.state = PlayerState::Paused
    }

    pub fn next(&mut self) {
        todo!()
    }

    pub fn previous(&mut self) {
        todo!()
    }

    pub fn get_playlist() {
        todo!()
    }

    pub fn set_playlist() {
        todo!()
    }

    pub fn add_to_playlist(&mut self, music: Music) {
        self.playlist.push(music)
    }

    pub fn set_volume() {
        todo!()
    }
}