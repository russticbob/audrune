use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    time::Duration,
};

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};

pub struct AudioPlayer {
    _device: MixerDeviceSink,
    player: Player,
    duration: Option<Duration>,
    current_path: Option<PathBuf>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(device.mixer());

        Ok(Self {
            _device: device,
            player,
            duration: None,
            current_path: None,
        })
    }

    pub fn play(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        let duration = source.total_duration();

        self.player.stop();
        self.player.append(source);
        self.player.play();

        self.duration = duration;
        self.current_path = Some(path.to_path_buf());

        Ok(())
    }

    pub fn seek_forward(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let target = self.position().saturating_add(Duration::from_secs(10));

        let target = self
            .duration
            .map(|total| target.min(total))
            .unwrap_or(target);

        self.seek_to(target)
    }

    pub fn seek_backward(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let target = self.position().saturating_sub(Duration::from_secs(10));

        self.seek_to(target)
    }

    fn seek_to(&mut self, target: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let Some(path) = self.current_path.as_ref() else {
            return Ok(());
        };

        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        let was_paused = self.player.is_paused();

        self.player.stop();
        self.player.append(source);

        // Seek the newly loaded decoder.
        self.player.try_seek(target)?;

        if was_paused {
            self.player.pause();
        } else {
            self.player.play();
        }

        Ok(())
    }

    pub fn toggle_pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.duration = None;
        self.current_path = None;
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn set_duration(&mut self, duration: Duration) {
        if !duration.is_zero() {
            self.duration = Some(duration);
        }
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn is_finished(&self) -> bool {
        self.player.empty()
    }
}
