fn main() {
    println!("--- Exercício 1: Inversão com Vec ---");
    let mut original = vec![1, 2, 3, 4, 5];
    let mut auxiliar = Vec::new();

    // Complexidade de Tempo: O(n) - Cada elemento é removido e inserido uma vez.
    // Complexidade de Espaço: O(n) - Alocação do vetor auxiliar.
    while let Some(elemento) = original.pop() {
        auxiliar.push(elemento);
    }

    println!("Vetor invertido: {:?}", auxiliar);
}