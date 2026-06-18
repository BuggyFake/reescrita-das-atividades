use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    println!("--- Exercício 17: Comparação de Desempenho ---");
    let n = 10000;

    // Complexidade de Tempo: O(n²) - Remover do index 0 exige shift linear de memória.
    // Complexidade de Espaço: O(n) - Alocação estrita de itens na lista.
    let mut v_ingenua = Vec::new();
    let start = Instant::now();
    for i in 0..n { v_ingenua.push(i); }
    while !v_ingenua.is_empty() { v_ingenua.remove(0); }
    println!("Tempo Vec Ingênua: {:?}", start.elapsed());

    // Complexidade de Tempo: O(n) - Remoções no início de Deques rodam de forma constante O(1).
    // Complexidade de Espaço: O(n) - Alocação dinâmica de elementos.
    let mut v_deque = VecDeque::new();
    let start = Instant::now();
    for i in 0..n { v_deque.push_back(i); }
    while !v_deque.is_empty() { v_deque.pop_front(); }
    println!("Tempo VecDeque: {:?}", start.elapsed());
}