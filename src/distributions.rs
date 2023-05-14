use rand_distr::{Distribution, Exp};

///! The type that defines what type of distribution for generating time of
///! consuming for each [`Request`] will be used.
#[derive(Debug)]
pub enum ConsumingDistribution {
    ///! Time of consuming is defined by exponential distribution.
    Exponential {
        ///! The parameter of \(G(x) = 1 - e^{-\lambda x}\) distribution.
        λ: f64,
    },
    ///! Time of consuming is defined by constant.
    Degenerate {
        ///! The parameter of \(G(x) = \frac{1}{\mu} = const\) distribution.
        μ: f64,
    },
}

impl ConsumingDistribution {
    pub(crate) fn sample(&self, rng: &mut impl rand::Rng) -> f64 {
        match self {
            ConsumingDistribution::Exponential { λ } => {
                let exp = Exp::new(*λ).unwrap();
                exp.sample(rng)
            }
            ConsumingDistribution::Degenerate { μ } => 1.0 / μ,
        }
    }
}

///! The type that defines what type of distribution for generating time of new
///! [`Request`] will be used.
#[derive(Debug)]
pub enum ProducingDistribution {
    ///! Time of producing is defined by exponential distribution.
    Exponential {
        ///! The parameter of \(G(x) = \lambda * e^{-\lambda x}\) distribution.
        λ: f64,
    },
    ///! Time of producing is defined by constant. Used for testing.
    Degenerate {
        ///! The next sample is fixed value
        value: u64,
    },
}

impl ProducingDistribution {
    pub(crate) fn sample(&self, rng: &mut impl rand::Rng) -> f64 {
        match self {
            Self::Exponential { λ } => {
                let exp = rand_distr::Exp::new(*λ).unwrap();
                exp.sample(rng)
            }
            Self::Degenerate { value } => *value as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_distribution() {
        let mut rng = rand::thread_rng();
        let λ = 100.0;

        let dstr = ConsumingDistribution::Exponential { λ };
        let samples_number = 1000;

        let sum = (0..samples_number)
            .map(|_| dstr.sample(&mut rng))
            .fold(0.0, |acc, x| acc + x);

        let avg = sum / samples_number as f64;
        let expected = 1.0 / λ;

        assert!(
            (avg - expected).abs() < 1.0,
            "average should be nearly equal to expected, avg = {}, expected = {}",
            avg,
            expected
        );
    }
}
