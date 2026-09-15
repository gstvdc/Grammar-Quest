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

    pub fn play_door_correct(&self) {
        play_once(&self.door_correct);
    }

    pub fn play_door_wrong(&self) {
        play_once(&self.door_wrong);
    }

    pub fn play_victory(&self) {
        play_once(&self.victory);
    }

    pub fn play_click(&self) {
        play_once(&self.click);
    }
}

fn play_once(sound: &Sound) {
    audio::play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );
}
