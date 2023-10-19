use core::fmt;
use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        *self = *self * rhs;
    }
}

pub fn fft(a: &mut [Complex], invert: bool) {
    let n = a.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let ang = 2.0 * std::f64::consts::PI / len as f64 * if invert { -1.0 } else { 1.0 };
        let wlen = Complex {
            re: ang.cos(),
            im: ang.sin(),
        };
        for i in (0..n).step_by(len) {
            let mut w = Complex { re: 1.0, im: 0.0 };
            for j in 0..len / 2 {
                let u = a[i + j];
                let v = a[i + j + len / 2] * w;
                a[i + j] = u + v;
                a[i + j + len / 2] = u - v;
                w *= wlen;
            }
        }
        len <<= 1;
    }

    if invert {
        for c in a.iter_mut() {
            c.re /= n as f64;
        }
    }
}

pub fn multiply_polynomials(a: &[u32], b: &[u32]) -> Vec<u32> {
    let max_len = a.len() + b.len() - 1;
    let mut a = a
        .iter()
        .map(|&x| Complex::new(f64::from(x), 0.0))
        .collect::<Vec<_>>();
    let mut b = b
        .iter()
        .map(|&x| Complex::new(f64::from(x), 0.0))
        .collect::<Vec<_>>();

    let n = max_len.next_power_of_two();
    a.resize(n, Complex::new(0.0, 0.0));
    b.resize(n, Complex::new(0.0, 0.0));

    fft(&mut a, false);
    fft(&mut b, false);
    a.iter_mut().zip(b.iter()).for_each(|(x, &y)| *x *= y);
    fft(&mut a, true);

    let mut c = vec![0; n];
    for i in 0..n {
        c[i] = (a[i].re + 0.5) as u32;
    }
    c.truncate(max_len);
    c
}

#[cfg(test)]
mod tests {
    #[test]
    fn multiply_polynomials() {
        use super::multiply_polynomials;

        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let c = multiply_polynomials(&a, &b);
        assert_eq!(c, vec![4, 13, 28, 27, 18]);

        let a = vec![1, 2];
        let b = vec![3, 4];
        let c = multiply_polynomials(&a, &b);
        assert_eq!(c, vec![3, 10, 8]);

        let a = vec![1];
        let b = vec![3, 4];
        let c = multiply_polynomials(&a, &b);
        assert_eq!(c, vec![3, 4]);
    }
}
