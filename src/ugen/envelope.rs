use crate::ugen::{UGen, ValueAt, Duration};
use std::collections::HashMap;

pub struct AR {
    attack: f64,
    release: f64,
    curve: f64,
}

impl Duration for UGen<AR> {
    fn duration(&self) -> f64 {
        self.parameters.attack + self.parameters.release
    }
}

pub fn ar(attack: f64, release: f64, curve: f64) -> UGen<AR> {
    UGen {
        duraaaa: attack + release,
        parameters: AR { attack, release, curve },
        value: Box::new(move |clock: f64| {
            if clock <= attack {
                let x = clock / attack;
                if curve >= 0. { x.powf(curve + 1.) } else { x.powf(-1. / (curve - 1.)) }
            } else if clock <= attack + release {
                let x = (clock - attack) / release;
                if curve >= 0. { 1. - x.powf(curve + 1.) } else { 1. - x.powf(-1. / (curve - 1.)) }
            } else {
                0.
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ugen::{envelope, ValueAt};

    #[test]
    fn ar_envelope() {
        let envelope = envelope::ar(1.0, 1.0, 0.);
        assert_eq!(0.0, envelope.value_at(0.0));
        assert_eq!(0.5, envelope.value_at(0.5));
        assert_eq!(1.0, envelope.value_at(1.0));
        assert_eq!(0.5, envelope.value_at(1.5));
        assert_eq!(0.0, envelope.value_at(2.0));
    }

}