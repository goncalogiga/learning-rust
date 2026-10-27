// Exercices of page 59.


fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / (9.0 / 5.0)
}


fn nth_fibo(n: i32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    
    for _ in 0..n {
        (a, b) = (b, a + b)
    }

    return a
}


fn main() {
    println!("Exercice 1 test : {}.", fahrenheit_to_celsius(70.0));
    println!("Exercice 2 test : {}.", nth_fibo(70));
}
