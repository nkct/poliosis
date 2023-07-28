use std::path::{PathBuf};
use std::{fs::File, error::Error};
use std::io::BufReader;

use rodio::{Decoder, Sink, OutputStream};

struct AudioPlayer {
    sink: Sink,
}
impl AudioPlayer {
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.pause();
        AudioPlayer { sink }
    }

    fn add(self, path: &str) -> Result<Self, Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        self.sink.append(source);

        Ok(self)
    }

    fn play(self) {
        self.sink.play();
        self.sink.sleep_until_end();
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_audioplayer() {
        AudioPlayer::new()
            .add("sound/alarm.wav").unwrap()
            .play();
    }
}