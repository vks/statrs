use distribution::{Continuous, Distribution, Gamma, Univariate, WeakRngDistribution};
use rand::distributions::{IndependentSample, Sample};
use rand::Rng;
use statistics::*;
use Result;

/// Implements the [Erlang](https://en.wikipedia.org/wiki/Erlang_distribution)
/// distribution
/// which is a special case of the
/// [Gamma](https://en.wikipedia.org/wiki/Gamma_distribution)
/// distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::{Erlang, Continuous};
/// use statrs::statistics::Mean;
/// use statrs::prec;
///
/// let n = Erlang::new(3, 1.0).unwrap();
/// assert_eq!(n.mean(), 3.0);
/// assert!(prec::almost_eq(n.pdf(2.0), 0.270670566473225383788, 1e-15));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Erlang {
    g: Gamma,
}

impl Erlang {
    /// Constructs a new erlang distribution with a shape (k)
    /// of `shape` and a rate (λ) of `rate`
    ///
    /// # Errors
    ///
    /// Returns an error if `shape` or `rate` are `NaN`.
    /// Also returns an error if `shape == 0` or `rate <= 0.0`
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Erlang;
    ///
    /// let mut result = Erlang::new(3, 1.0);
    /// assert!(result.is_ok());
    ///
    /// result = Erlang::new(0, 0.0);
    /// assert!(result.is_err());
    /// ```
    pub fn new(shape: u64, rate: f64) -> Result<Erlang> {
        Gamma::new(shape as f64, rate).map(|g| Erlang { g: g })
    }

    /// Returns the shape (k) of the erlang distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Erlang;
    ///
    /// let n = Erlang::new(3, 1.0).unwrap();
    /// assert_eq!(n.shape(), 3);
    /// ```
    pub fn shape(&self) -> u64 {
        self.g.shape() as u64
    }

    /// Returns the rate (λ) of the erlang distribution
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Erlang;
    ///
    /// let n = Erlang::new(3, 1.0).unwrap();
    /// assert_eq!(n.rate(), 1.0);
    /// ```
    pub fn rate(&self) -> f64 {
        self.g.rate()
    }
}

impl Sample<f64> for Erlang {
    /// Generate a random sample from a erlang
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn sample<R: Rng>(&mut self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl IndependentSample<f64> for Erlang {
    /// Generate a random independent sample from a erlang
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn ind_sample<R: Rng>(&self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl Distribution<f64> for Erlang {
    /// Generate a random sample from a erlang distribution using
    /// `r` as the source of randomness.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate statrs;
    /// use rand::StdRng;
    /// use statrs::distribution::{Erlang, Distribution};
    ///
    /// # fn main() {
    /// let mut r = rand::StdRng::new().unwrap();
    /// let n = Erlang::new(3, 1.0).unwrap();
    /// print!("{}", n.sample::<StdRng>(&mut r));
    /// # }
    /// ```
    fn sample<R: Rng>(&self, r: &mut R) -> f64 {
        self.g.sample(r)
    }
}

impl WeakRngDistribution<f64> for Erlang {}

impl Univariate<f64, f64> for Erlang {
    /// Calculates the cumulative distribution function for the erlang
    /// distribution
    /// at `x`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// γ(k, λx)  (k - 1)!
    /// ```
    ///
    /// where `k` is the shape, `λ` is the rate, and `γ` is the lower
    /// incomplete gamma function
    fn cdf(&self, x: f64) -> f64 {
        self.g.cdf(x)
    }
}

impl Min<f64> for Erlang {
    /// Returns the minimum value in the domain of the
    /// erlang distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 0
    /// ```
    fn min(&self) -> f64 {
        self.g.min()
    }
}

impl Max<f64> for Erlang {
    /// Returns the maximum value in the domain of the
    /// erlang distribution representable by a double precision
    /// float
    ///
    /// # Formula
    ///
    /// ```ignore
    /// INF
    /// ```
    fn max(&self) -> f64 {
        self.g.max()
    }
}

impl Mean<f64> for Erlang {
    /// Returns the mean of the erlang distribution
    ///
    /// # Remarks
    ///
    /// Returns `shape` if `rate == f64::INFINITY`. This behavior
    /// is borrowed from the Math.NET implementation
    ///
    /// # Formula
    ///
    /// ```ignore
    /// k / λ
    /// ```
    ///
    /// where `k` is the shape and `λ` is the rate
    fn mean(&self) -> f64 {
        self.g.mean()
    }
}

impl Variance<f64> for Erlang {
    /// Returns the variance of the erlang distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// k / λ^2
    /// ```
    ///
    /// where `α` is the shape and `λ` is the rate
    fn variance(&self) -> f64 {
        self.g.variance()
    }

    /// Returns the standard deviation of the erlang distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt(k) / λ
    /// ```
    ///
    /// where `k` is the shape and `λ` is the rate
    fn std_dev(&self) -> f64 {
        self.g.std_dev()
    }
}

impl Entropy<f64> for Erlang {
    /// Returns the entropy of the erlang distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// k - ln(λ) + ln(Γ(k)) + (1 - k) * ψ(k)
    /// ```
    ///
    /// where `k` is the shape, `λ` is the rate, `Γ` is the gamma function,
    /// and `ψ` is the digamma function
    fn entropy(&self) -> f64 {
        self.g.entropy()
    }
}

impl Skewness<f64> for Erlang {
    /// Returns the skewness of the erlang distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 2 / sqrt(k)
    /// ```
    ///
    /// where `k` is the shape
    fn skewness(&self) -> f64 {
        self.g.skewness()
    }
}

impl Mode<f64> for Erlang {
    /// Returns the mode for the erlang distribution
    ///
    /// # Remarks
    ///
    /// Returns `shape` if `rate ==f64::INFINITY`. This behavior
    /// is borrowed from the Math.NET implementation
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (k - 1) / λ
    /// ```
    ///
    /// where `k` is the shape and `λ` is the rate
    fn mode(&self) -> f64 {
        self.g.mode()
    }
}

impl Continuous<f64, f64> for Erlang {
    /// Calculates the probability density function for the erlang distribution
    /// at `x`
    ///
    /// # Remarks
    ///
    /// Returns `NAN` if any of `shape` or `rate` are `INF`
    /// or if `x` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (λ^k / Γ(k)) * x^(k - 1) * e^(-λ * x)
    /// ```
    ///
    /// where `k` is the shape, `λ` is the rate, and `Γ` is the gamma function
    fn pdf(&self, x: f64) -> f64 {
        self.g.pdf(x)
    }

    /// Calculates the log probability density function for the erlang
    /// distribution
    /// at `x`
    ///
    /// # Remarks
    ///
    /// Returns `NAN` if any of `shape` or `rate` are `INF`
    /// or if `x` is `INF`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln((λ^k / Γ(k)) * x^(k - 1) * e ^(-λ * x))
    /// ```
    ///
    /// where `k` is the shape, `λ` is the rate, and `Γ` is the gamma function
    fn ln_pdf(&self, x: f64) -> f64 {
        self.g.ln_pdf(x)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64;
    use distribution::Erlang;
    use distribution::internal::*;

    fn try_create(shape: u64, rate: f64) -> Erlang {
        let n = Erlang::new(shape, rate);
        assert!(n.is_ok());
        n.unwrap()
    }

    fn create_case(shape: u64, rate: f64) {
        let n = try_create(shape, rate);
        assert_eq!(shape, n.shape());
        assert_eq!(rate, n.rate());
    }

    fn bad_create_case(shape: u64, rate: f64) {
        let n = Erlang::new(shape, rate);
        assert!(n.is_err());
    }

    #[test]
    fn test_create() {
        create_case(1, 0.1);
        create_case(1, 1.0);
        create_case(10, 10.0);
        create_case(10, 1.0);
        create_case(10, f64::INFINITY);
    }

    #[test]
    fn test_bad_create() {
        bad_create_case(0, 1.0);
        bad_create_case(1, 0.0);
        bad_create_case(1, f64::NAN);
        bad_create_case(1, -1.0);
    }

    #[test]
    fn test_continuous() {
        test::check_continuous_distribution(&try_create(1, 2.5), 0.0, 20.0);
        test::check_continuous_distribution(&try_create(2, 1.5), 0.0, 20.0);
        test::check_continuous_distribution(&try_create(3, 0.5), 0.0, 20.0);
    }
}
