// Exercício 7 — Fibonacci Recursivo
// Complexidade: O(2ⁿ)

pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n; // Caso base (âncora da recursão)
    }
    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}

fn main() {
    let n = 10;
    let resultado = fibonacci_recursivo(n);
    println!("O {}-ésimo número de Fibonacci é: {}", n, resultado);
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci_recursivo(0), 0);
    assert_eq!(fibonacci_recursivo(1), 1);
    assert_eq!(fibonacci_recursivo(5), 5);
}