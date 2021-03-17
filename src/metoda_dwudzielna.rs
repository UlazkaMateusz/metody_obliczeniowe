use crate::ekstremum::Ekstremum;

fn metoda_dwudzielna_internal<F, CMP>(f: F, mut a: f64, mut b: f64, epsilon: f64, cmp: CMP) -> f64
where
    F: Fn(f64) -> f64,
    CMP: Fn(f64, f64) -> bool,
{
    let mut xsr = (a + b) / 2.0;
    let mut l = b - a;
    loop {
        let x1 = a + l / 4.0;
        let x2 = b - l / 4.0;

        if cmp(f(x1), f(xsr)) {
            b = xsr;
            xsr = x1;
        } else {
            if cmp(f(x2), f(xsr)) {
                a = xsr;
                xsr = x2;
            } else {
                a = x1;
                b = x2;
            }
        }

        l = b - a;
        if l <= epsilon {
            return xsr;
        }
    }
}

pub fn metoda_dwudzielna<F>(f: F, a: f64, b: f64, epsilon: f64, ekstremum: Ekstremum) -> f64
where
    F: Fn(f64) -> f64,
{
    let cmp = match ekstremum {
        Ekstremum::Min => |x: f64, y: f64| x < y,
        Ekstremum::Max => |x: f64, y: f64| x > y,
    };

    metoda_dwudzielna_internal(f, a, b, epsilon, cmp)
}
