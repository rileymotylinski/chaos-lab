//x_0
// \delta_0
// orbiting to see how far from eachother (?)
// \delta_n = seperation of orbit from x_0 and orbit from x_0 + \delta_0
// if \delta_n = \delta_0 *e^(n*\lambda) => \lambda is called the lyapunov exponent and \lambda > 0, then signature of chaos

// computation
// \delta_n = f^n(x_0 + \delta_0) - f^n(x_0)
// => \lambda \approx = (1/n)ln(\delta_n / \delta_0) = (1/n) * ln(f^n(x_0 + \delta_0) - f^n(x_0) / \delta_0)
// looks like a derivative!
// lim \delta_0 -> 0
// \lambda = (1/n)ln(f^n(x_0)')
// also, (f^n)(x_0) = \Pi f'(x_i)


// finally,
// \lambda = lim n -> inf \Sigma ln(f'(x_i))
// average of every point in space
// average of seperation/contraction across space
// negative -> compression
// positive -> expansion
// converges *VERY* slowly (10k - 100k datapoints)


// what is x_i? - I think just the point at i

// total = 0
// for i in range(n):
//     total += ln(f'(x_i))

/// returns lypapunov exponent given a functions derivative and n number of iterations
/// most useful for the logistic map
pub fn lyapunov<F>(n:i64,f: F) -> f64
where F:Fn(f64, &[f64]) -> Vec<f64>{
    let mut total = 0.0;
    let state = vec![0.0];

    for i in 0..n {
        let result = f(i as f64, &state)[0].abs().ln();
        total += result;
    }

    total / (n as f64)

}