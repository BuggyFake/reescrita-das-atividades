use std::collections::VecDeque;

fn main() {
    println!("--- Exercício 16: Fila de Tarefas com Prioridade de Frente ---");
    let mut tarefas = VecDeque::new();

    // Complexidade de Tempo: O(1) - Inserções e remoções lógicas nas extremidades do Deque.
    // Complexidade de Espaço: O(n) - Alocação proporcional ao número de strings armazenadas.
    tarefas.push_back("Tarefa comum A".to_string()); 
    tarefas.push_back("Tarefa comum B".to_string()); 
    tarefas.push_front("URGENTE: Resolver bug de produção".to_string()); 

    while let Some(t) = tarefas.pop_front() {
        println!("Processando: {}", t);
    }
}