pub trait Context {
    fn l(&self) -> usize;
    fn r(&self) -> usize;
    fn answer(&self) -> usize;
    fn extend_l(&mut self);
    fn extend_r(&mut self);
    fn shrink_l(&mut self);
    fn shrink_r(&mut self);
}

struct Query {
    l: usize,
    r: usize,
    idx: usize,
    ord: usize,
}

#[inline]
fn hilbert_order(x: usize, y: usize, pow: usize, rotate: usize) -> usize {
    if pow == 0 {
        return 0;
    }
    let hpow = 1 << (pow - 1);
    let mut seg = if x < hpow {
        if y < hpow {
            0
        } else {
            3
        }
    } else if y < hpow {
        1
    } else {
        2
    };
    seg = (seg + rotate) & 3;
    let rotate_delta = [3, 0, 0, 1];
    let nx = x & (x ^ hpow);
    let ny = y & (y ^ hpow);
    let nrot = (rotate + rotate_delta[seg]) & 3;
    let sub_square_size = 1 << (2 * pow - 2);
    let ans = seg * sub_square_size;
    let add = hilbert_order(nx, ny, pow - 1, nrot);
    ans + if seg == 1 || seg == 2 {
        add
    } else {
        sub_square_size - add - 1
    }
}

/// Applies Mo's algorithm to the given queries. `B` is the block size, and `L`
/// is the log of the maximum value of the queries.
pub fn apply<C: Context, const B: usize, const L: usize>(
    queries: &[(usize, usize)],
    ctx: &mut C,
) -> Vec<usize> {
    let mut queries = queries
        .iter()
        .enumerate()
        .map(|(idx, &(l, r))| Query {
            l,
            r,
            idx,
            ord: hilbert_order(l, r, L, 0),
        })
        .collect::<Vec<_>>();
    queries.sort_unstable_by_key(|q| q.ord);
    let mut ans = vec![0; queries.len()];
    for q in queries {
        while ctx.l() > q.l {
            ctx.extend_l();
        }
        while ctx.r() < q.r {
            ctx.extend_r();
        }
        while ctx.l() < q.l {
            ctx.shrink_l();
        }
        while ctx.r() > q.r {
            ctx.shrink_r();
        }
        ans[q.idx] = ctx.answer();
    }
    ans
}
