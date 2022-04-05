use std::convert::AsRef;
use std::time::Duration;
use rodio::{Decoder, source::Source};

#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub enum Status {
    Playing(::std::time::Instant, ::std::time::Duration),
    Stopped(::std::time::Duration),
}


impl Status {
    // Time elapsed
    pub fn elapsed(self) -> ::std::time::Duration {
        match self {
            Status::Stopped(d) => d,
            Status::Playing(start, extra) => start.elapsed() + extra,
        }
    }
    // stop
    pub fn stop(&mut self) {
        *self = match *self {
            Status::Stopped(_) => *self,
            Status::Playing(start, extra) => Status::Stopped(start.elapsed() + extra),
        };
    }
    // resume track
    pub fn resume(&mut self) {
        *self = match *self {
            Status::Playing(_, _) => *self,
            Status::Stopped(duration) => Status::Playing(::std::time::Instant::now(), duration),
        };
    }
    #[allow(unused)]
    pub fn is_stopped(self) -> bool {
        match self {
            Status::Stopped(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Track {
    /// Duration of the song
    pub duration: Duration,
    /// File path to the song
    pub file: String,
    /// Elapsed time of song playing or Start time
    pub status: Status,
}

impl Track {
    /// Returns the `Duration` of the song
    #[allow(unused)]
    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn elapsed(&self) -> Duration {
        self.status.elapsed()
    }
    /// Pause the song
    pub fn stop(&mut self) {
        self.status.stop()
    }
    /// Resume the song
    pub fn resume(&mut self) {
        self.status.resume()
    }
    /// Check if the song is stopped/paused
    #[allow(unused)]
    pub fn is_stopped(&self) -> bool {
        self.status.is_stopped()
    }
    /// Returns the path of the song
    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn load(file: String) -> Result<Self, failure::Error > {
        let f = std::fs::File::open(&file).unwrap();
        let source = Decoder::new(std::io::BufReader::new(f)).unwrap();
        let duration = match source.total_duration() {
            Some(d) => d,
            None => mp3_duration::from_path(&file).unwrap(),
        };
            // Ok(d) => d,
            // Err(_) => std::time::Duration::new(100, 0)
        // };
        Ok(Self {
            duration,
            file,
            status: Status::Stopped(::std::time::Duration::from_nanos(0)),
        })
    }
}

impl AsRef<String> for Track {
    fn as_ref(&self) -> &String {
        &self.file
    }
}
