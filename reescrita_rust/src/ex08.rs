// Exercício 8 — Ordenação Bolha (Bubble Sort)
// Complexidade: O(n²)

pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1); // Mutação segura utilizando API nativa de Slices do Rust
            }
        }
    }
}

fn main() {
    let mut vetor = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Vetor original: {:?}", vetor);
    
    ordenacao_bolha(&mut vetor);
    println!("Vetor ordenado: {:?}", vetor);
}

#[test]
fn test_ordenacao_bolha() {
    let mut arr = [4, 1, 3, 2];
    ordenacao_bolha(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4]);
}