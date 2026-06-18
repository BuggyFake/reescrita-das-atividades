fn main() {
    println!("--- Exercício 3: Remoção Condicional ---");
    let mut numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let mut i = 0;
    // Complexidade de Tempo: O(n²) - Laço percorre o vetor e remove() causa deslocamento de memória O(n).
    // Complexidade de Espaço: O(1) - Modificação realizada in-place no próprio vetor.
    while i < numeros.len() {
        if numeros[i] % 2 == 0 {
            numeros.remove(i);
        } else {
            i += 1;
        }
    }
    println!("Apenas números ímpares: {:?}", numeros);
}