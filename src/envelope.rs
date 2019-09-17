#[derive(Clone,Copy)]
pub struct Envelope {
    attack: f64,
    release: f64
}

impl Envelope {
    pub fn new(attack: f64, release: f64) -> Envelope {
        Envelope {
            attack,
            release
        }
    }

    pub fn apply(&self, elapsed: f64, sound: f64) -> f64 {
        let mut value: f64 = 0.0;
        if elapsed <= self.attack {
            value = elapsed / self.attack;
        } else if elapsed <= self.attack + self.release {
            value = (self.attack + self.release) - (elapsed / self.release)
        }
        sound * value
    }

}

// TODO: add tests! pordios!