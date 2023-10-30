pub trait Context {
    fn l(&self) -> usize;
    fn r(&self) -> usize;
    fn answer(&self) -> usize;
    fn extend_l(&mut self);
    fn extend_r(&mut self);
    fn shrink_l(&mut self);
    fn shrink_r(&mut self);
}

pub struct Query {
    l: usize,
    r: usize,
    idx: usize,
}

pub fn apply<C: Context, const B: usize>(queries: &[(usize, usize)], ctx: &mut C) -> Vec<usize> {
    let mut queries = queries
        .iter()
        .enumerate()
        .map(|(idx, &(l, r))| Query { l, r, idx })
        .collect::<Vec<_>>();
    queries.sort_unstable_by(|a, b| {
        let block_a = a.l / B;
        let block_b = b.l / B;
        match block_a.cmp(&block_b) {
            std::cmp::Ordering::Equal => {
                if block_a % 2 == 0 {
                    b.r.cmp(&a.r)
                } else {
                    a.r.cmp(&b.r)
                }
            }
            other => other,
        }
    });
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
