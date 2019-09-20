type Attack = f64;
type Release = f64;
type Curve = f64;

pub enum Envelope {
    AR(Attack, Release, Curve)
}

impl Envelope {
    pub(crate) fn value_at(&self, clock: f64) -> f64 {
        match self {
            Self::AR(attack, release, curve) => {
                if clock <= *attack {
                    let x = (clock / attack);
                    if *curve >= 0. { x.powf(*curve + 1.) }
                    else { x.powf(-1. / (*curve - 1.)) }
                } else if clock <= attack + release {
                    (attack + release) - (clock / release)
                } else {
                    0.
                }
            }
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

/*
if curve >= 0 x^(a+1)
else x^(-1/(a-1))
*/