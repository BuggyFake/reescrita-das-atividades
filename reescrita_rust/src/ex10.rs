// Exercício 10 — Merge Sort
// Complexidade: O(n log n)

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(esquerda.len() + direita.len());
    let (mut i, mut j) = (0, 0);

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    // Estende o vetor resultante com os elementos que sobraram das fatias
    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);
    resultado
}

pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }
    let meio = lista.len() / 2;
    // Divide e copia os dados em novos vetores para manter a posse dos dados
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());
    
    merge(esquerda, direita)
}

fn main() {
    let vetor_desordenado = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Original: {:?}", vetor_desordenado);
    
    let vetor_ordenado = merge_sort(vetor_desordenado);
    println!("Ordenado pelo Merge Sort: {:?}", vetor_ordenado);
}

#[test]
fn test_merge_sort() {
    let lista = vec![5, -1, 0, 9, 2];
    assert_eq!(merge_sort(lista), vec![-1, 0, 2, 5, 9]);
}