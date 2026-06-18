// Exercício 2 — Somar Lista
// Complexidade: O(n)

pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total: i32 = 0;
    for &elemento in lista { // Desestrutura a referência para obter o valor i32 diretamente
        total += elemento;
    }
    total
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];
    let resultado = somar_lista(&numeros);
    println!("A soma dos elementos da lista é: {}", resultado);
}

#[test]
fn test_somar_lista() {
    assert_eq!(somar_lista(&[10, -2, 3]), 11);
    assert_eq!(somar_lista(&[]), 0);
}