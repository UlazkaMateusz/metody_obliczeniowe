pub fn metoda_stycznych<F1, F2, F3>(
    f_poch: F1,
    f_poch2: F2,
    f_poch3: F3,
    a: f64,
    b: f64,
    epsilon: f64,
) -> Option<f64>
where
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
    F3: Fn(f64) -> f64,
{
    if f_poch(a) * f_poch(b) >= 0.0 {
        return None;
    }

    if f_poch2(a) * f_poch2(b) < 0.0 || f_poch3(a) * f_poch3(b) < 0.0 {
        println!("Warunki zbieżności nie zostały spełnione");
    }

    // Jeżeli f' oraz f''' mają te same znaki
    let mut xn = if f_poch(a) * f_poch3(a) >= 0.0 { a } else { b };

    loop {
        let xn1 = xn - f_poch(xn) / f_poch2(xn);
        if f_poch(xn1).abs() < epsilon || (xn1 - xn).abs() < epsilon {
            return Some(xn1);
        }

        xn = xn1;
    }
}
