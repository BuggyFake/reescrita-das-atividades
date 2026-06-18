struct FilaCircular {
    buffer: Vec<Option<String>>,
    capacidade: usize,
    frente: usize,
    fim: usize,
    tamanho: usize,
}

impl FilaCircular {
    fn new(cap: usize) -> Self {
        Self { buffer: vec![None; cap], capacidade: cap, frente: 0, fim: 0, tamanho: 0 }
    }

    // Complexidade de Tempo: O(1) - Inserção direta por índices e aritmética modular sem loops.
    // Complexidade de Espaço: O(c) - Vetor de tamanho fixo inicializado de acordo com a capacidade c.
    fn push(&mut self, msg: String) {
        if self.tamanho == self.capacidade {
            self.frente = (self.frente + 1) % self.capacidade;
            self.tamanho -= 1;
        }
        self.buffer[self.fim] = Some(msg);
        self.fim = (self.fim + 1) % self.capacidade;
        self.tamanho += 1;
    }
}

fn main() {
    println!("--- Exercício 12: Buffer de Mensagens ---");
    let mut buffer = FilaCircular::new(3);
    buffer.push("Msg 1".to_string());
    buffer.push("Msg 2".to_string());
    buffer.push("Msg 3".to_string());
    buffer.push("Msg 4".to_string()); 

    println!("Mensagem na frente do buffer: {:?}", buffer.buffer[buffer.frente].as_ref().unwrap());
}