use std::collections::HashMap;
use crate::models::produto::Produto;

pub struct Indexador {
    pub produtos: HashMap<String, Produto>,
}

impl Indexador {
    pub fn new() -> Self {
        Indexador {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar(&mut self, produto: Produto) {
        self.produtos.insert(produto.nome.clone(), produto);
    }
}