// Exercício 4 — Pares com Soma
// Complexidade: O(n²)

pub fn pares_com_soma(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let n = lista.len();
    let mut pares_encontrados = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("Par encontrado: {} + {} = {}", lista[i], lista[j], alvo);
                pares_encontrados.push((lista[i], lista[j]));
            }
        }
    }
    pares_encontrados
}

fn main() {
    let numeros = vec![2, 4, 3, 5, 7, 8, 1];
    let alvo = 9;
    println!("Buscando pares que somam {}:", alvo);
    pares_com_soma(&numeros, alvo);
}

#[test]
fn test_pares_com_soma() {
    let dados = [3, 1, 4, 2];
    let resultado = pares_com_soma(&dados, 5);
    assert!(resultado.contains(&(3, 2)) || resultado.contains(&(1, 4)));
}