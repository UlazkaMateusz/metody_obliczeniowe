pub fn metoda_siecznych<F1, F3>(
    f_poch: F1,
    f_poch3: F3,
    a: f64,
    b: f64,
    epsilon: f64,
) -> Option<f64>
where
    F1: Fn(f64) -> f64,
    F3: Fn(f64) -> f64,
{
    if f_poch(a) * f_poch(b) >= 0.0 {
        return None;
    }

    // Jeżeli f' oraz f''' mają te same znaki
    let (mut xn, fxn): (_, Box<dyn Fn(f64) -> f64>) = if f_poch(a) * f_poch3(a) >= 0.0 {
        (
            b,
            Box::new(|xn| xn - f_poch(xn) / (f_poch(xn) - f_poch(a)) * (xn - a)),
        )
    } else {
        (
            a,
            Box::new(|xn| xn - f_poch(xn) / (f_poch(b) - f_poch(xn)) * (b - xn)),
        )
    };

    loop {
        let xn1 = fxn(xn);
        if f_poch(xn1).abs() < epsilon || (xn1 - xn).abs() < epsilon {
            return Some(xn1);
        }

        xn = xn1;
    }
}
