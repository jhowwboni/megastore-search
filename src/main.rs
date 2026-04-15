mod models {
    pub mod produto;
}

mod utils {
    pub mod indexacao;
}

mod services {
    pub mod busca;
}

use models::produto::Produto;
use utils::indexacao::Indexador;
use services::busca::*;

fn main() {
    let mut index = Indexador::new();

    index.adicionar(Produto {
        nome: "Notebook".to_string(),
        categoria: "Eletronicos".to_string(),
        preco: 3500.0,
    });

    index.adicionar(Produto {
        nome: "Camisa".to_string(),
        categoria: "Roupas".to_string(),
        preco: 99.9,
    });

    // Busca por nome
    if let Some(produto) = buscar_por_nome(&index, "Notebook") {
        println!("Produto encontrado: {:?}", produto);
    }

    // Busca por categoria
    let lista = buscar_por_categoria(&index, "Roupas");
    println!("Produtos na categoria: {:?}", lista);
}