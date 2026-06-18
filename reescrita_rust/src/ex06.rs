// Exercício 6 — Potências de Dois
// Complexidade: O(log n)

pub fn potencias_de_dois(n: u64) {
    let mut i: u64 = 1;
    while i < n {
        println!("{}", i);
        i *= 2; // Crescimento exponencial controlado
    }
}

fn main() {
    let limite = 100;
    println!("Potências de 2 menores que {}:", limite);
    potencias_de_dois(limite);
}

#[test]
fn test_potencias_de_dois() {
    // Garante o comportamento correto para o limite inferior
    potencias_de_dois(5);
}