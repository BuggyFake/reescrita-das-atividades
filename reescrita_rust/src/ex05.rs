// Exercício 5 — Imprimir Pares e Pares
// Complexidade: O(n²)

pub fn imprimir_pares_e_pares(lista: &[i32]) {
    println!("--- Bloco 1: Elementos Individuais ---");
    for &x in lista {
        println!("{}", x);
    }

    println!("--- Bloco 2: Combinação de Pares ---");
    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}

fn main() {
    let dados = vec![1, 2, 3];
    imprimir_pares_e_pares(&dados);
}

#[test]
fn test_execucao_fluxo() {
    // Como a função apenas imprime dados, este teste garante que ela roda sem pânico
    imprimir_pares_e_pares(&[1, 2]);
}