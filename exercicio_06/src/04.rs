fn main() {
    println!("--- Exercício 4: Mescla Ordenada ---");
    let v1 = vec![1, 3, 5, 7];
    let v2 = vec![2, 4, 6, 8, 9, 10];
    let mut resultado = Vec::with_capacity(v1.len() + v2.len());
    
    let (mut i, mut j) = (0, 0);
    // Complexidade de Tempo: O(n + m) - Varredura linear simultânea usando ponteiros independentes.
    // Complexidade de Espaço: O(n + m) - Alocação do vetor resultado para união dos elementos.
    while i < v1.len() && j < v2.len() {
        if v1[i] <= v2[j] {
            resultado.push(v1[i]);
            i += 1;
        } else {
            resultado.push(v2[j]);
            j += 1;
        }
    }
    if i < v1.len() { resultado.extend_from_slice(&v1[i..]); }
    if j < v2.len() { resultado.extend_from_slice(&v2[j..]); }

    println!("Vetor mesclado e ordenado: {:?}", resultado);
}