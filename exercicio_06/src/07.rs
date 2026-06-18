struct Editor {
    texto: String,
    desfazer_pilha: Vec<String>,
}

impl Editor {
    // Complexidade de Tempo: O(1) amortizado - Cópia e inserção do estado na pilha.
    fn digitar(&mut self, novo: &str) {
        self.desfazer_pilha.push(self.texto.clone()); 
        self.texto.push_str(novo);
    }

    // Complexidade de Tempo: O(1) - Substituição de estado removendo o topo da pilha.
    // Complexidade de Espaço: O(n) - Histórico retém estados gerados por comandos de digitação.
    fn desfazer(&mut self) {
        if let Some(anterior) = self.desfazer_pilha.pop() { 
            self.texto = anterior;
        }
    }
}

fn main() {
    println!("--- Exercício 7: Desfazer/Refazer ---");
    let mut ed = Editor { texto: String::new(), desfazer_pilha: vec![] };
    ed.digitar("Estrutura ");
    ed.digitar("de Dados");
    println!("Texto: '{}'", ed.texto);
    ed.desfazer();
    println!("Após Desfazer: '{}'", ed.texto);
}