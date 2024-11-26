use std::path::PathBuf;
use stream_handler::StreamHandler;

mod stream_handler;
mod byte_parser;
mod wav;

#[derive(PartialEq)]
enum State {
    WaitingForFile,
    Paused,
    Playing,
}

pub struct AudioPlayer {
    state: State,
    stream_handler: Option<StreamHandler>,
    current_file_name: Option<String>,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            state: State::WaitingForFile,
            stream_handler: None,
            current_file_name: None,
        }
    }

    pub fn play_file(&mut self, path: PathBuf) {
        //Should we start by stopping any file already playing?
        //Or let it keep playing until the new one is ready?
        //Going to go with the second option for now
        
        self.current_file_name = match path.file_stem() {
            Some(os_str) => match os_str.to_os_string().into_string() {
                Ok(file_name) => Some(file_name),
                Err(_) => None,
            },
            None => None,
        };

        let file_bytes = std::fs::read(&path)
            .expect("Should be able to read file");
            //This expect might need to have proper error handling incase of invalid file paths
            //RFD might handle them being correct though, I will look into it later?

        let stream_handler = match path.extension().unwrap().to_str().unwrap() { //These unwraps are bad
            "wav" | "wave" => wav::stream_from_wav_file(&file_bytes),
            _ => panic!("Unsupported file extension"),
        };

        stream_handler.play();
        self.stream_handler = Some(stream_handler);
        self.state = State::Playing;
    }

    pub fn track_name(&self) -> &Option<String> {
        return &self.current_file_name;
    }

    pub fn toggle_playing(&mut self) {
        match self.state {
            State::Playing => {
                self.state = State::Paused;
                if let Some(stream_handler) = &self.stream_handler {
                    stream_handler.pause();
                }
            },
            State::Paused => {
                if let Some(stream_handler) = &self.stream_handler {
                    stream_handler.play();
                }
                else {
                    panic!("Something went wrong playing stream")
                }
                self.state = State::Playing;
            },
            _ => {},
        }
    }

    pub fn pause_or_play_button_text(&self) -> &str {
        match self.state {
            State::Playing => "Pause",
            _ => "Play",
        }
    }

    pub fn is_playing(&self) -> bool {
        return self.state == State::Playing;
    }

    pub fn restart(&self) {
        if let Some(stream_handler) = &self.stream_handler {
            stream_handler.restart();
        }
    }

    pub fn progress(&self) -> f32 {
        if let Some(stream_handler) = &self.stream_handler {
            stream_handler.progress()
        }
        else {
            0.0
        }
    }
    
    // pub fn stop(&self) {
        //Close the stream or something?
        //I don't actually know how to do that
        //Then swap state to inactive or something?
    // }
}