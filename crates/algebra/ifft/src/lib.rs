pub fn fft(ar: &mut [i32], ai: &mut [i32]) {
    assert_eq!(ar.len(), ai.len());
    assert!(ar.len().is_power_of_two());
}

fn ilift(mut xr: i32, mut xi: i32, c: f64, s: f64) -> (i32, i32) {
    if s == 0.0 {
        return (xr, xi);
    }

    if s > c {
        if s > -c {
            xr = -xr;
            xr -= ((xi as f64 * (s - 1.0)) / c) as i32;
            xi -= (xr as f64 * c) as i32;
            xr -= ((xi as f64 * (s - 1.0)) / c) as i32;
            let t = xr;
            xr = xi;
            xi = t;
        } else {
            xr = -xr;
            xr -= ((xi as f64 * (-c - 1.0)) / s) as i32;
            xi -= (xr as f64 * s) as i32;
            xr -= ((xi as f64 * (-c - 1.0)) / s) as i32;
            xi = -xi;
        }
    } else {
        if s < -c {
            let t = xr;
            xr = -xi;
            xi = t;
            xr -= ((xi as f64 * (-s - 1.0)) / c) as i32;
            xi -= (xr as f64 * c) as i32;
            xr -= ((xi as f64 * (-s - 1.0)) / c) as i32;
        } else {
            xr -= ((xi as f64 * (c - 1.0)) / s) as i32;
            xi -= (xr as f64 * s) as i32;
            xr -= ((xi as f64 * (c - 1.0)) / s) as i32;
        }
    }

    (xr, xi)
}

fn ifft(n: usize, ar: &mut [i32], ai: &mut [i32]) {
    for m in (2..=n).rev().step_by(2) {
        let theta = 2.0 * PI / m as f64;
        let mq = m >> 2;
        for i in 0..mq {
            let s1 = theta.sin() * i as f64;
            let c1 = theta.cos() * i as f64;
            let s3 = (theta * 3.0).sin() * i as f64;
            let c3 = (theta * 3.0).cos() * i as f64;
            for j in (i..n).step_by(m as usize) {
                let j1 = j + mq;
                let j2 = j1 + mq;
                let j3 = j2 + mq;

                let (ar1, ai1) = ilift(ar[j1] - ar[j3], ai[j1] - ai[j3], c1, s1);
                let (ar3, ai3) = ilift(ar[j3] - ar[j1], ai[j3] - ai[j1], c3, s3);
                let ar2 = ar[j2] << 1;
                let ai2 = ai[j2] << 1;

                ar[j2] = ar[j] - ar2;
                ai[j2] = ai[j] - ai2;
                ar[j] += ar2;
                ai[j] += ai2;

                ar[j1] = ar[j] - ar1;
                ai[j1] = ai[j] - ai1;
                ar[j] += ar1;
                ai[j] += ai1;

                ar[j3] = ar[j2] - ar3;
                ai[j3] = ai[j2] - ai3;
                ar[j2] += ar3;
                ai[j2] += ai3;
            }
        }
    }

    let mut j = 1;
    for i in 1..(n - 1) {
        if i < j {
            let tmp_ar = ar[j - 1];
            let tmp_ai = ai[j - 1];
            ar[j - 1] = ar[i - 1];
            ai[j - 1] = ai[i - 1];
            ar[i - 1] = tmp_ar;
            ai[i - 1] = tmp_ai;
        }
        let mut k = n >> 1;
        while k < j {
            j -= k;
            k >>= 1;
        }
        j += k;
    }

    for i in 0..n as usize {
        ar[i] /= n as i32;
        ai[i] /= n as i32;
    }
}

fn irfft(n: usize, ar: &mut [i32], ai: &mut [i32]) {
    ifft(n, ar, ai);

    for i in 0..n as usize {
        ai[i] = -ai[i];
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ifft() {
        let mut ar = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut ai = vec![0, 0, 0, 0, 0, 0, 0, 0];
        super::ifft(8, &mut ar, &mut ai);
        assert_eq!(ar, &[36, -5, -4, -5, -4, -3, -4, -3]);
        assert_eq!(ai, &[0, 10, 4, 1, 0, -2, -4, -9])
    }
}
