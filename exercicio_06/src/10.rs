use std::collections::VecDeque;

fn main() {
    println!("--- Exercício 10: Simulador de Fila de Banco ---");
    let mut fila: VecDeque<u32> = VecDeque::new();
    let tempos_chegada = vec![1, 2, 5, 8, 15]; 
    let mut tempo_atual = 0;
    let mut total_espera = 0;

    for tempo in tempos_chegada { fila.push_back(tempo); }
    let total_clientes = fila.len() as f32;

    // Complexidade de Tempo: O(n) - Cada cliente entra e sai da fila em tempo constante O(1).
    // Complexidade de Espaço: O(n) - Armazenamento dos tempos de chegada no Deque.
    while let Some(chegada) = fila.pop_front() {
        if tempo_atual < chegada { tempo_atual = chegada; }
        total_espera += tempo_atual - chegada;
        tempo_atual += 5; 
    }
    println!("Tempo médio de espera na fila: {} minutos", total_espera as f32 / total_clientes);
}