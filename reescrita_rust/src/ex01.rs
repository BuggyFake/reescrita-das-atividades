// Exercício 1 — Verificar Primeiro
// Complexidade: O(1)

pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    if lista.is_empty() {
        None
    } else {
        Some(lista[0]) // Retorna o primeiro elemento encapsulado em Some
    }
}

fn main() {
    let lista_cheia = vec![10, 20, 30];
    let lista_vazia: Vec<i32> = vec![];

    println!("Lista cheia: {:?}", verificar_primeiro(&lista_cheia));
    println!("Lista vazia: {:?}", verificar_primeiro(&lista_vazia));
}

#[test]
fn test_verificar_primeiro() {
    assert_eq!(verificar_primeiro(&[5, 10]), Some(5));
    assert_eq!(verificar_primeiro(&[]), None);
}