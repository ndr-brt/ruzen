use crate::ugen::UGen;

pub fn ar(attack: f64, release: f64, curve: f64) -> UGen {
    UGen {
        duration: attack + release,
        value_at: Box::new(move |clock: f64| {
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
    use crate::ugen::envelope;

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