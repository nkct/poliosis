use std::{fs::File, error::Error};
use std::io::BufReader;

use rodio::source::Buffered;
use rodio::{Decoder, Sink, OutputStream, Source};

struct AudioHandler {
    sources: Vec<Buffered<Decoder<BufReader<File>>>>,
}
impl AudioHandler {
    fn new() -> Self {
        Self { 
            sources: Vec::new()
         }
    }

    fn add(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        self.sources.push(source.buffered());

        Ok(())
    }

    fn play(self) -> Result<(), Box<dyn Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        for source in self.sources {
            sink.append(source)
        }

        sink.play();
        sink.sleep_until_end();

        Ok(())
    }
}

// ----- TESTS -----
#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_audioplayer() {
        let mut audio_handler = AudioHandler::new();
        audio_handler.add("sound/alarm.wav").unwrap();
        audio_handler.play().unwrap();
    }
}