pub type Attack = f64;
pub type Release = f64;
pub type Curve = f64;

pub enum Envelope {
    AR(Attack, Release, Curve)
}

impl Envelope {

    pub(crate) fn value_at(&self, clock: f64) -> f64 {
        match self {
            Self::AR(attack, release, curve) => Envelope::ar(clock, *attack, *release, *curve)
        }
    }

    fn ar(clock: f64, attack: Attack, release: Release, curve: Curve) -> f64 {
        if clock <= attack {
            let x = clock / attack;
            if curve >= 0. { x.powf(curve + 1.) } else { x.powf(-1. / (curve - 1.)) }
        } else if clock <= attack + release {
            let x = (clock - attack) / release;
            if curve >= 0. { 1. - x.powf(curve + 1.) } else { 1. - x.powf(-1. / (curve - 1.)) }
        } else {
            0.
        }
    }

    pub fn duration(&self) -> f64 {
        match self {
            Self::AR(attack, release, curve) => attack + release
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Envelope;

    #[test]
    fn ar_envelope() {
        let envelope = Envelope::AR(1.0, 1.0, 0.);
        assert_eq!(0.0, envelope.value_at(0.0));
        assert_eq!(0.5, envelope.value_at(0.5));
        assert_eq!(1.0, envelope.value_at(1.0));
        assert_eq!(0.5, envelope.value_at(1.5));
        assert_eq!(0.0, envelope.value_at(2.0));
    }

}