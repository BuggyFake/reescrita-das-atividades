use std::collections::VecDeque;

fn main() {
    println!("--- Exercício 15: Janela Deslizante Máxima ---");
    let arr = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut resultado = Vec::new();

    // Complexidade de Tempo: O(n) amortizado - Cada índice entra e sai do deque no máximo uma vez.
    // Complexidade de Espaço: O(k) - Tamanho do Deque limitado pelo tamanho k da janela corrente.
    for i in 0..arr.len() {
        if !deque.is_empty() && *deque.front().unwrap() <= i - k { deque.pop_front(); }
        while !deque.is_empty() && arr[*deque.back().unwrap()] <= arr[i] { deque.pop_back(); }
        
        deque.push_back(i);
        if i >= k - 1 { resultado.push(arr[*deque.front().unwrap()]); }
    }
    println!("Vetor original: {:?}\nMáximos (janela={}): {:?}", arr, k, resultado);
}