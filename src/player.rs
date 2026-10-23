use std::{fs::File, io::BufReader, path::Path};

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};

pub struct AudioPlayer {
    _device: MixerDeviceSink,
    player: Player,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(device.mixer());

        Ok(Self {
            _device: device,
            player,
        })
    }

    pub fn play(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        self.player.stop();
        self.player.append(source);
        self.player.play();

        Ok(())
    }

    pub fn toggle_pause(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    pub fn stop(&self) {
        self.player.stop();
    }
}
