use crate::utils::indexacao::Indexador;
use crate::models::produto::Produto;

pub fn buscar_por_nome(index: &Indexador, nome: &str) -> Option<&Produto> {
    index.produtos.get(nome)
}

pub fn buscar_por_categoria(index: &Indexador, categoria: &str) -> Vec<&Produto> {
    index
        .produtos
        .values()
        .filter(|p| p.categoria == categoria)
        .collect()
}