use std::collections::VecDeque;

struct Processo { id: u32, tempo_restante: i32 }

fn main() {
    println!("--- Exercício 20: Mini-projeto Round Robin ---");
    let quantum = 3;
    let mut fila = VecDeque::new();
    
    fila.push_back(Processo { id: 1, tempo_restante: 7 });
    fila.push_back(Processo { id: 2, tempo_restante: 4 });

    let mut tempo_global = 0;

    // Complexidade de Tempo: O(t) - Loop proporcional à soma t de tempo total dos processos.
    // Complexidade de Espaço: O(n) - Fila mantém n nós de processos ativos.
    while let Some(mut proc) = fila.pop_front() {
        if proc.tempo_restante > quantum {
            tempo_global += quantum;
            proc.tempo_restante -= quantum;
            println!("P{} executou por {}ms. Restam {}ms.", proc.id, quantum, proc.tempo_restante);
            fila.push_back(proc);
        } else {
            tempo_global += proc.tempo_restante;
            println!("P{} FINALIZADO no tempo total de {}ms.", proc.id, tempo_global);
        }
    }
}