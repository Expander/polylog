use num::complex::Complex;

/// Provides the fourth order polylogarithm function `li4()` of a
/// number of type `T`.
pub trait Li4<T> {
    fn li4(&self) -> T;
}

impl Li4<Complex<f64>> for Complex<f64> {
    /// Returns the fourth order polylogarithm of a complex number of type
    /// `Complex<f64>`.
    ///
    /// # Example:
    /// ```
    /// extern crate num;
    /// extern crate polylog;
    /// use num::complex::Complex;
    /// use polylog::Li4;
    ///
    /// fn main() {
    ///     let z = Complex::new(1.0, 1.0);
    ///     println!("Li4({}) = {}", z, z.li4());
    /// }
    /// ```
    fn li4(&self) -> Complex<f64> {
        let pi  = 3.141592653589793;
        let pi2 = pi*pi;
        let z4  = 1.082323233711138;
        let bf  = [
            1., -7./16.,
            1.165123456790123e-01, -1.982060185185185e-02,
            1.927932098765432e-03, -3.105709876543209e-05,
           -1.562400911485783e-05,  8.485123546773206e-07,
            2.290961660318971e-07, -2.183261421852691e-08,
           -3.882824879172015e-09,  5.446292103220332e-10,
            6.960805210682725e-11, -1.337573768644521e-11,
           -1.278485268526657e-12,  3.260562858024892e-13,
            2.364757116861825e-14, -7.923135122031161e-15,
        ];

        if self.im == 0.0 {
            if self.re == 0.0 {
                return Complex::new(0., 0.);
            }
            if self.re == 1.0 {
                return Complex::new(z4, 0.);
            }
            if self.re == -1.0 {
                return Complex::new(-7./8.*z4, 0.);
            }
        }

        let nz  = self.norm_sqr();
        let pz  = self.arg();
        let lnz = 0.5*nz.ln();

        if lnz*lnz + pz*pz < 1. { // |log(z)| < 1
            let u  = Complex::new(lnz, pz);
            let u2 = u*u;
            let c1 = 1.202056903159594; // zeta(3)
            let c2 = 0.8224670334241132;
            let c3 = (11.0/6.0 - (-u).cln())/6.0;
            let c4 = -1.0/48.0;

            let cs = [
                -6.944444444444444e-04, 1.653439153439153e-06,
                -1.093544413650234e-08, 1.043837849393405e-10,
                -1.216594230062244e-12, 1.613000652835010e-14,
                -2.342881045287934e-16
            ];

            return z4 + u2 * (c2 + u2 * c4) +
                u * (
                    c1 +
                    u2 * (c3 +
                    u2 * (cs[0] +
                    u2 * (cs[1] +
                    u2 * (cs[2] +
                    u2 * (cs[3] +
                    u2 * (cs[4] +
                    u2 * (cs[5] +
                    u2 * (cs[6]))))))))
                );
        }

        let (u, rest, sgn) = if nz <= 1.0 {
            (-(1. - self).cln(), Complex::new(0.,0.), 1.)
        } else { // nz > 1.0
            let pi4  = pi2*pi2;
            let arg = if pz > 0.0 { pz - pi } else { pz + pi };
            let lmz = Complex::new(lnz, arg); // (-self).cln()
            let lmz2 = lmz*lmz;
            (-(1. - 1./self).cln(), 1./360.*(-7.*pi4 + lmz2*(-30.*pi2 - 15.*lmz2)), -1.)
        };

        rest + sgn * (
            u * (bf[0] +
            u * (bf[1] +
            u * (bf[2] +
            u * (bf[3] +
            u * (bf[4] +
            u * (bf[5] +
            u * (bf[6] +
            u * (bf[7] +
            u * (bf[8] +
            u * (bf[9] +
            u * (bf[10] +
            u * (bf[11] +
            u * (bf[12] +
            u * (bf[13] +
            u * (bf[14] +
            u * (bf[15] +
            u * (bf[16] +
            u * (bf[17]))))))))))))))))))
        )
    }
}

trait CLn<T> {
    fn cln(&self) -> T;
}

impl CLn<Complex<f64>> for Complex<f64> {
    fn cln(&self) -> Complex<f64> {
        let z = Complex::new(
            if self.re == 0. { 0. } else { self.re },
            if self.im == 0. { 0. } else { self.im },
        );
        Complex::new(0.5*z.norm_sqr().ln(), z.arg())
    }
}
