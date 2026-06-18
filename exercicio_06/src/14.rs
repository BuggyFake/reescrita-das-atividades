use std::collections::VecDeque;

fn main() {
    println!("--- Exercício 14: Palíndromo com Deque ---");
    let frase = "A man a plan a canal Panama";
    let mut deque = VecDeque::new();

    for c in frase.to_lowercase().chars() {
        if c.is_alphanumeric() { deque.push_back(c); }
    }

    let mut eh_palindromo = true;
    // Complexidade de Tempo: O(n) - Remoção em O(1) de ambas as pontas simultaneamente.
    // Complexidade de Espaço: O(n) - Armazenamento de caracteres válidos filtrados no Deque.
    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            eh_palindromo = false;
            break;
        }
    }
    println!("A frase '{}' é um palíndromo? {}", frase, eh_palindromo);
}