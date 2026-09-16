use macroquad::audio::{self, PlaySoundParams, Sound};

/// Sound effects embedded at compile time so the game runs standalone
/// without shipping a loose `assets/audio` folder next to the binary.
pub struct Sfx {
    door_correct: Sound,
    door_wrong: Sound,
    victory: Sound,
    click: Sound,
}

impl Sfx {
    pub async fn load() -> Self {
        Self {
            door_correct: audio::load_sound_from_bytes(include_bytes!(
                "../assets/audio/door_correct.wav"
            ))
            .await
            .expect("embedded door_correct.wav is a valid wav"),
            door_wrong: audio::load_sound_from_bytes(include_bytes!(
                "../assets/audio/door_wrong.wav"
            ))
            .await
            .expect("embedded door_wrong.wav is a valid wav"),
            victory: audio::load_sound_from_bytes(include_bytes!("../assets/audio/victory.wav"))
                .await
                .expect("embedded victory.wav is a valid wav"),
            click: audio::load_sound_from_bytes(include_bytes!("../assets/audio/click.wav"))
                .await
                .expect("embedded click.wav is a valid wav"),
        }
    }

    pub fn play_door_correct(&self, volume: f32) {
        play_once(&self.door_correct, volume);
    }

    pub fn play_door_wrong(&self, volume: f32) {
        play_once(&self.door_wrong, volume);
    }

    pub fn play_victory(&self, volume: f32) {
        play_once(&self.victory, volume);
    }

    pub fn play_click(&self, volume: f32) {
        play_once(&self.click, volume);
    }
}

fn play_once(sound: &Sound, volume: f32) {
    audio::play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume: volume.clamp(0.0, 1.0),
        },
    );
}
