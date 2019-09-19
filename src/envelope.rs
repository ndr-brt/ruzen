pub enum Envelope {
    AR(f64, f64)
}


impl Envelope {
    pub(crate) fn value_at(&self, clock: f64) -> f64 {
        match self {
            Self::AR(attack, release) => {
                if clock <= *attack {
                    clock / attack
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
        let envelope = Envelope::AR(1.0, 1.0);
        assert_eq!(0.0, envelope.value_at(0.0));
        assert_eq!(0.5, envelope.value_at(0.5));
        assert_eq!(1.0, envelope.value_at(1.0));
        assert_eq!(0.5, envelope.value_at(1.5));
        assert_eq!(0.0, envelope.value_at(2.0));
    }

}