pub fn metoda_bisekcji<F>(f_poch: F, mut a: f64, mut b: f64, epsilon: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    if f_poch(a) * f_poch(b) >= 0.0 {
        return None;
    }

    loop {
        let xsr = (a + b) / 2.0;
        if f_poch(xsr).abs() < epsilon {
            return Some(xsr);
        } else if f_poch(a) * f_poch(xsr) < 0.0 {
            b = xsr;
        } else {
            a = xsr;
        }
    }
}
