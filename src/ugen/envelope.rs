use crate::ugen::{UGen, ValueAt };

pub struct AR {
    attack: f64,
    release: f64,
    curve: f64,
}

pub trait Envelope: ValueAt {
    fn duration(&self) -> f64;
}

impl dyn Envelope {
    pub fn ar(attack: f64, release: f64, curve: f64) -> UGen<AR> {
        UGen {
            parameters: AR { attack, release, curve }
        }
    }
}

impl Envelope for UGen<AR> {
    fn duration(&self) -> f64 {
        self.parameters.attack + self.parameters.release
    }
}

impl ValueAt for AR {
    fn value_at(&self, clock: f64) -> f64 {
        if clock <= self.attack {
            let x = clock / self.attack;
            if self.curve >= 0. { x.powf(self.curve + 1.) } else { x.powf(-1. / (self.curve - 1.)) }
        } else if clock <= self.attack + self.release {
            let x = (clock - self.attack) / self.release;
            if self.curve >= 0. { 1. - x.powf(self.curve + 1.) } else { 1. - x.powf(-1. / (self.curve - 1.)) }
        } else {
            0.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ugen::{envelope, ValueAt};

    #[test]
    fn ar_envelope() {
        let envelope = Envelope::ar(1.0, 1.0, 0.);
        assert_eq!(0.0, envelope.value_at(0.0));
        assert_eq!(0.5, envelope.value_at(0.5));
        assert_eq!(1.0, envelope.value_at(1.0));
        assert_eq!(0.5, envelope.value_at(1.5));
        assert_eq!(0.0, envelope.value_at(2.0));
    }

}