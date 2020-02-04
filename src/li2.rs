use std;
use num::complex::Complex;

/// Provides the dilogarithm function `li2()` of a number of type `T`.
pub trait Li2<T> {
    fn li2(&self) -> T;
}

impl Li2<f64> for f64 {
    /// Returns the real dilogarithm of a real number of type `f64`.
    ///
    /// This function has been translated from the
    /// [ROOT](https://root.cern.ch/) package.  Original implementation by
    /// K.S. Kölbig into CERNLIB DILOG function C332, translated to C++ by
    /// R.Brun.
    ///
    /// Implemented as a truncated series expansion in terms of Chebyshev
    /// polynomials, see [Yudell L. Luke: Mathematical functions and their
    /// approximations, Academic Press Inc., New York 1975, p.67].
    ///
    /// # Example:
    /// ```
    /// use polylog::Li2;
    ///
    /// let z = 1.0;
    /// println!("Li2({}) = {}", z, z.li2());
    /// ```
    fn li2(&self) -> f64 {
        let pi  = 3.141592653589793;
        let pi2 = pi*pi;
        let pi3 = pi2/3.;
        let pi6 = pi2/6.;
        let coeffs = [0.42996693560813697, 0.40975987533077105,
           -0.01858843665014592, 0.00145751084062268,-0.00014304184442340,
            0.00001588415541880,-0.00000190784959387, 0.00000024195180854,
           -0.00000003193341274, 0.00000000434545063,-0.00000000060578480,
            0.00000000008612098,-0.00000000001244332, 0.00000000000182256,
           -0.00000000000027007, 0.00000000000004042,-0.00000000000000610,
            0.00000000000000093,-0.00000000000000014, 0.00000000000000002];

        if *self == 1.0 {
            pi6
        } else if *self == -1.0 {
            -pi2/12.
        } else {
            let t = -*self;
            let (y, s, a) = if t <= -2.0 {
                let b1 = (-t).ln();
                let b2 = (1.0 + 1.0/t).ln();
                (-1.0/(1.0 + t), 1.0, -pi3 + 0.5*(b1*b1 - b2*b2))
            } else if t < -1.0 {
                let a = (-t).ln();
                (-1.0 - t, -1.0, -pi6 + a*(a + (1.0 + 1.0/t).ln()))
            } else if t <= -0.5 {
                let a = (-t).ln();
                (-(1.0 + t)/t, 1.0, -pi6 + a*(-0.5*a + (1.0 + t).ln()))
            } else if t < 0.0 {
                let b1 = (1.0 + t).ln();
                (-t/(1.0 + t), -1.0, 0.5*b1*b1)
            } else if t <= 1.0 {
                (t, 1.0, 0.)
            } else {
                let b1 = t.ln();
                (1.0/t, -1.0, pi6 + 0.5*b1*b1)
            };

            let h      = y+y - 1.0;
            let alfa   = h+h;
            let mut b0 = 0.0;
            let mut b1 = 0.0;
            let mut b2 = 0.0;
            for c in coeffs.iter().rev() {
                b0 = c + alfa*b1 - b2;
                b2 = b1;
                b1 = b0;
            }
            -(s*(b0 - h*b2) + a)
        }
    }
}

impl Li2<Complex<f64>> for Complex<f64> {
    /// Returns the dilogarithm of a complex number of type
    /// `Complex<f64>`.
    ///
    /// This function has been translated from the
    /// [SPheno](https://spheno.hepforge.org/) package.
    ///
    /// # Example:
    /// ```
    /// extern crate num;
    /// extern crate polylog;
    /// use num::complex::Complex;
    /// use polylog::Li2;
    ///
    /// fn main() {
    ///     let z = Complex::new(1.0, 1.0);
    ///     println!("Li2({}) = {}", z, z.li2());
    /// }
    /// ```
    fn li2(&self) -> Complex<f64> {
        let pi = 3.141592653589793;

        // bf[1..N-1] are the even Bernoulli numbers / (2 n + 1)!
        // generated by: Table[BernoulliB[2 n]/(2 n + 1)!, {n, 1, 19}]
        let bf = [
            - 1./4.,
              1./36.,
            - 1./3600.,
              1./211680.,
            - 1./10886400.,
              1./526901760.,
            - 4.064761645144226e-11,
              8.921691020456453e-13,
            - 1.993929586072108e-14,
              4.518980029619918e-16,
        ];

        let rz = self.re;
        let iz = self.im;
        let nz = self.norm_sqr();

        // special cases
        if iz == 0. {
            if rz <= 1. {
                return Complex::new(rz.li2(), 0.0)
            } else { // rz > 1.
                return Complex::new(rz.li2(), -pi*rz.ln())
            }
        } else if nz < std::f64::EPSILON {
            return *self;
        }

        let (cy, cz, jsgn, ipi12) = if rz <= 0.5 {
            if nz > 1. {
                (-0.5 * sqr((-self).ln()), -(1. - 1. / self).ln(), -1., -2.)
            } else { // nz <= 1.
                (Complex::new(0.,0.), -(1. - self).ln(), 1., 0.)
            }
        } else { // rz > 0.5
            if nz <= 2.0*rz {
                let l = -(self).ln();
                (l * (1. - self).ln(), l, -1., 2.)
            } else { // nz > 2.0*rz
                (-0.5 * sqr((-self).ln()), -(1. - 1. / self).ln(), -1., -2.)
            }
        };

        // the dilogarithm
        let cz2 = sqr(cz);
        let sum =
            cz +
            cz2 * (bf[0] +
            cz  * (bf[1] +
            cz2 * (bf[2] +
            cz2 * (bf[3] +
            cz2 * (bf[4] +
            cz2 * (bf[5] +
            cz2 * (bf[6] +
            cz2 * (bf[7] +
            cz2 * (bf[8] +
            cz2 * (bf[9]))))))))));

        jsgn * sum + cy + ipi12 * pi * pi / 12.
    }
}

fn sqr(x: Complex<f64>) -> Complex<f64> { x*x }
