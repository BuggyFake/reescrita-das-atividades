use std::collections::VecDeque;

// Complexidade de Tempo: O(n) - Cada elemento é removido e lido exatamente uma vez.
// Complexidade de Espaço: O(m) - Vetor auxiliar retém temporariamente apenas o tamanho m do lote.
fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    while !fila.is_empty() {
        let mut lote = Vec::new();
        for _ in 0..tamanho_lote {
            if let Some(val) = fila.pop_front() { lote.push(val); }
        }
        println!("Processando lote coletado: {:?}", lote);
    }
}

fn main() {
    println!("--- Exercício 19: Fila com Iteração Controlada ---");
    let mut dados = VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    processar_em_lotes(&mut dados, 3);
}