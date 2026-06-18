struct Navegador {
    atual: String,
    back: Vec<String>,
    forward: Vec<String>,
}

fn main() {
    println!("--- Exercício 6: Histórico de Navegação ---");
    let mut nav = Navegador {
        atual: String::from("google.com"),
        back: vec![],
        forward: vec![],
    };
    
    // Complexidade de Tempo: O(1) - Operações de push e pop no topo da pilha levam tempo constante.
    // Complexidade de Espaço: O(n) - Pilhas acumulam o histórico proporcionalmente às páginas visitadas.
    nav.back.push(nav.atual.clone());
    nav.atual = String::from("rust-lang.org");
    nav.forward.clear();
    
    if let Some(antiga) = nav.back.pop() {
        nav.forward.push(nav.atual.clone());
        nav.atual = antiga;
    }
    println!("Página atual após comando Voltar: {}", nav.atual);
}