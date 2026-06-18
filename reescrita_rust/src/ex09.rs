// Exercício 9 — Produto de Matrizes
// Complexidade: O(n³)

pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0i64; n]; n]; // Aloca matriz quadrada de resultado com zeros

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn main() {
    let matriz_a = vec![
        vec![1, 2],
        vec![3, 4]
    ];
    let matriz_b = vec![
        vec![2, 0],
        vec![1, 2]
    ];

    // Corrigido de &matiz_a para &matriz_a (com o "r")
    let resultado = produto_de_matrizes(&matriz_a, &matriz_b);
    println!("Resultado do Produto das Matrizes: {:?}", resultado);
}