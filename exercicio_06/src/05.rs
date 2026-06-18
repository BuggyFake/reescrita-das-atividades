fn main() {
    println!("--- Exercício 5: Calculadora RPN ---");
    let expressao = "3 4 + 2 *";
    let mut pilha: Vec<f64> = Vec::new();

    // Complexidade de Tempo: O(n) - Processamento sequencial de cada token com inserções O(1) na pilha.
    // Complexidade de Espaço: O(n) - Armazenamento temporário de operandos na pilha.
    for token in expressao.split_whitespace() {
        match token {
            "+" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a + b);
            }
            "*" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a * b);
            }
            _ => {
                if let Ok(num) = token.parse::<f64>() {
                    pilha.push(num);
                }
            }
        }
    }
    println!("Resultado da expressão RPN: {:?}", pilha.pop());
}