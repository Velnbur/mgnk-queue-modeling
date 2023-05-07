const PRECISION: f64 = 1000f64;

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
                let x = rng.gen::<f64>();
                (1f64 - (-λ * x).exp()) * PRECISION
            }
            ConsumingDistribution::Degenerate { μ } => (1f64 / μ) * PRECISION,
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
}

impl ProducingDistribution {
    pub(crate) fn sample(&self, rng: &mut impl rand::Rng) -> f64 {
        match self {
            ProducingDistribution::Exponential { λ } => {
                let x = rng.gen::<f64>();
                (λ * (-λ * x).exp()) * PRECISION
            }
        }
    }
}
