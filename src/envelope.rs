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

#[cfg(test)]
mod tests {
    use super::Envelope;

    #[test]
    fn ar_envelope() {
        let envelope = Envelope::new(1.0, 1.0);
        assert_eq!(0.0, envelope.apply(0.0, 1.));
        assert_eq!(0.5, envelope.apply(0.5, 1.));
        assert_eq!(1.0, envelope.apply(1.0, 1.));
        assert_eq!(0.5, envelope.apply(1.5, 1.));
        assert_eq!(0.0, envelope.apply(2.0, 1.));
    }

}