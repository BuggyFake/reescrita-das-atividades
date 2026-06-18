struct StackMin {
    principal: Vec<i32>,
    minimos: Vec<i32>,
}

impl StackMin {
    // Complexidade de Tempo: O(1) - Push e comparação direta com o topo.
    fn push(&mut self, val: i32) {
        self.principal.push(val);
        if self.minimos.is_empty() || val <= *self.minimos.last().unwrap() {
            self.minimos.push(val);
        }
    }

    // Complexidade de Tempo: O(1) - Pop no topo de ambas as listas internas.
    fn pop(&mut self) -> Option<i32> {
        let val = self.principal.pop();
        if let Some(v) = val {
            if v == *self.minimos.last().unwrap() {
                self.minimos.pop();
            }
        }
        val
    }

    // Complexidade de Tempo: O(1) - Retorno imediato do último elemento de mínimos.
    // Complexidade de Espaço: O(n) - Uso de uma segunda pilha auxiliar para rastreio.
    fn min(&self) -> Option<i32> {
        self.minimos.last().copied() 
    }
}

fn main() {
    println!("--- Exercício 9: Pilha com Mínimo O(1) ---");
    let mut s = StackMin { principal: vec![], minimos: vec![] };
    s.push(10);
    s.push(4);
    s.push(8);
    println!("Mínimo atual: {:?}", s.min().unwrap());
    s.pop();
    s.pop();
    println!("Mínimo após remoções: {:?}", s.min().unwrap());
}