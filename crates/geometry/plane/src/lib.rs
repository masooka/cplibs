/// Calculates the area of a polygon using the shoelace formula.
///
/// # Examples
///
/// ```
/// # use plane::shoelace_formula;
/// let vertices = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
/// let area = shoelace_formula(&vertices);
/// assert_eq!(area, 1.0);
/// ```
pub fn shoelace_formula(vertices: &[(f64, f64)]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }

    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len(); // Next index, wraps around to 0 at the end
        sum1 += vertices[i].0 * vertices[j].1;
        sum2 += vertices[i].1 * vertices[j].0;
    }

    ((sum1 - sum2).abs()) / 2.0
}
