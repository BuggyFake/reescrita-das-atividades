fn main() {
    println!("--- Exercício 8: Sequências de Símbolos ---");
    let expressao = "{[()]}";
    let mut pilha = Vec::new();
    let mut balanceado = true;

    // Complexidade de Tempo: O(n) - Varredura completa da string com manipulações O(1) na pilha.
    // Complexidade de Espaço: O(n) - Alocação dinâmica da pilha para delimitadores abertos.
    for c in expressao.chars() {
        match c {
            '{' | '[' | '(' => pilha.push(c),
            '}' => if pilha.pop() != Some('{') { balanceado = false; break; },
            ']' => if pilha.pop() != Some('[') { balanceado = false; break; },
            ')' => if pilha.pop() != Some('(') { balanceado = false; break; },
            _ => {}
        }
    }
    if !pilha.is_empty() { balanceado = false; }
    println!("A expressão '{}' está balanceada? {}", expressao, balanceado);
}