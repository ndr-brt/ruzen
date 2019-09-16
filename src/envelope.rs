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
            value = 1. - (elapsed / (self.attack + self.release))
        }
        sound * value
    }

    pub fn is_valid(&self) -> bool {
        self.attack != 0. && self.release != 0.
    }
}