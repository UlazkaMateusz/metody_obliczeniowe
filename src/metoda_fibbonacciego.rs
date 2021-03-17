use crate::ekstremum::Ekstremum;

fn fib(n: i32) -> f64 {
    let mut fpp = 1.0;
    let mut fp = 1.0;

    for _ in 0..(n as i32 - 1) {
        let f = fpp + fp;
        fpp = fp;
        fp = f;
    }

    fp
}

fn find_max_n(a: f64, b: f64, epsilon: f64) -> i32 {
    for n in 0.. {
        if ((b - a) / fib(n) as f64) < 2.0 * epsilon {
            return n - 1;
        }
    }
    unreachable!();
}

fn metoda_fibbonacciego_internal<F, CMP>(
    f: F,
    mut a: f64,
    mut b: f64,
    epsilon: f64,
    cmp: CMP,
) -> f64
where
    F: Fn(f64) -> f64,
    CMP: Fn(f64, f64) -> bool,
{
    let mut n = find_max_n(a, b, epsilon);
    let mut x1 = b - fib(n - 1) / fib(n) * (b - a);
    let mut x2 = a + fib(n - 1) / fib(n) * (b - a);

    loop {
        if cmp(f(x1), f(x2)) {
            b = x2;
            x2 = x1;
            n = n - 1;
            x1 = b - fib(n - 1) / fib(n) * (b - a);
        } else {
            a = x1;
            x1 = x2;
            n = n - 1;
            x2 = a + fib(n - 1) / fib(n) * (b - a);
        }

        if (x2 - x1).abs() < epsilon || n == 1 {
            return (a + b) / 2.0;
        }
    }
}

pub fn metoda_fibbonacciego<F>(f: F, a: f64, b: f64, epsilon: f64, ekstremum: Ekstremum) -> f64
where
    F: Fn(f64) -> f64,
{
    let cmp = match ekstremum {
        Ekstremum::Min => |x: f64, y: f64| x < y,
        Ekstremum::Max => |x: f64, y: f64| x > y,
    };

    metoda_fibbonacciego_internal(f, a, b, epsilon, cmp)
}
