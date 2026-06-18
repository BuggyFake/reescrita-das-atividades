use std::collections::VecDeque;

struct Trabalho { nome: String, paginas: u32 }

fn main() {
    println!("--- Exercício 11: Impressora Compartilhada ---");
    let mut fila_impressao = VecDeque::new();

    // Complexidade de Tempo: O(1) - Entrada e saída do VecDeque ocorrem em tempo constante.
    // Complexidade de Espaço: O(n) - Vetor guarda a fila de objetos estruturados sob demanda.
    fila_impressao.push_back(Trabalho { nome: "Planilha_Financeira.xlsx".to_string(), paginas: 4 });
    fila_impressao.push_back(Trabalho { nome: "Foto_Grupo.jpg".to_string(), paginas: 1 });

    while let Some(job) = fila_impressao.pop_front() {
        println!("Documento '{}' ({} págs) impresso com sucesso.", job.nome, job.paginas);
    }
}