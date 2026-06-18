struct Item { tarefa: String, prioridade: i32 }

fn main() {
    println!("--- Exercício 13: Fila de Prioridade Manual ---");
    let mut fila: Vec<Item> = Vec::new();

    fila.push(Item { tarefa: "Limpar mesa".to_string(), prioridade: 1 });
    fila.push(Item { tarefa: "Salvar servidor".to_string(), prioridade: 10 });
    fila.push(Item { tarefa: "Responder e-mail".to_string(), prioridade: 3 });

    // Complexidade de Tempo: O(n²) - Busca linear do maior elemento seguida por shift de remoção O(n).
    // Complexidade de Espaço: O(n) - Armazenamento linear dos n itens no vetor.
    while !fila.is_empty() {
        let mut idx_maior = 0;
        for i in 1..fila.len() {
            if fila[i].prioridade > fila[idx_maior].prioridade { idx_maior = i; }
        }
        let resolvido = fila.remove(idx_maior);
        println!("Executando: {} (Prioridade: {})", resolvido.tarefa, resolvido.prioridade);
    }
}