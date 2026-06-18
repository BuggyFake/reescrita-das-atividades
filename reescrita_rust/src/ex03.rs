// Exercício 3 — Busca Binária
// Complexidade: O(log n)

pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize; // Conversão segura para indexação do slice

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }
    None
}

fn main() {
    let lista_ordenada = vec![10, 20, 30, 40, 50];
    let alvo = 40;

    match busca_binaria(&lista_ordenada, alvo) {
        Some(index) => println!("Elemento {} encontrado no índice {}", alvo, index),
        None => println!("Elemento {} não foi encontrado.", alvo),
    }
}

#[test]
fn test_busca_binaria() {
    let dados = [1, 3, 5, 7, 9];
    assert_eq!(busca_binaria(&dados, 7), Some(3));
    assert_eq!(busca_binaria(&dados, 2), None);
}