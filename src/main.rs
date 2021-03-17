mod ekstremum;
mod metoda_bisekcji;
mod metoda_dwudzielna;
mod metoda_fibbonacciego;
mod metoda_siecznych;
mod metoda_stycznych;

use crate::ekstremum::Ekstremum;
use crate::metoda_bisekcji::metoda_bisekcji;
use crate::metoda_dwudzielna::metoda_dwudzielna;
use crate::metoda_fibbonacciego::metoda_fibbonacciego;
use crate::metoda_siecznych::metoda_siecznych;
use crate::metoda_stycznych::metoda_stycznych;

fn main() {
    //let a = -3.0;
    let a = 1.0;
    //let b = 5.0;
    let b = 2.0;
    //let epsilon = 0.001;
    let epsilon = 0.01;
    let f = |x: f64| x.powi(3) + 10.0 * x.powi(2) + 9.0 * x - 1.0;
    //let f_poch = |x: f64| 3.0 * x.powi(2) + 20.0 * x + 9.0;
    let f_poch = |x: f64| x * x + x - 5.0;
    //let f_poch2 = |x: f64| 6.0 * x + 20.0;
    //let f_poch3 = |_: f64| 6.0;
    let f_poch2 = |x: f64| 2.0 * x + 1.0;
    let f_poch3 = |_: f64| 2.0;

    println!(
        "Metoda dwudzielna: {}",
        metoda_dwudzielna(f, a, b, epsilon, Ekstremum::Min)
    );

    println!(
        "Metoda fibbonacciego: {}",
        metoda_fibbonacciego(f, a, b, epsilon, Ekstremum::Min)
    );

    match metoda_bisekcji(f_poch, a, b, epsilon) {
        Some(wynik) => {
            println!("Metoda bisekcji: {}", wynik);
        }
        None => {
            println!("Warunek konieczny nie został spełniony.");
        }
    }

    match metoda_stycznych(f_poch, f_poch2, f_poch3, a, b, epsilon) {
        Some(wynik) => {
            println!("Metoda stycznych: {}", wynik);
        }
        None => {
            println!("Warunek konieczny nie został spełniony.");
        }
    }

    match metoda_siecznych(f_poch, f_poch3, a, b, epsilon) {
        Some(wynik) => {
            println!("Metoda metoda siecznych: {}", wynik);
        }
        None => {
            println!("Warunek konieczny nie został spełniony.");
        }
    }
}
