use megastore_search::utils::indexacao::Indexador;
use megastore_search::models::produto::Produto;
use megastore_search::services::busca::*;

#[test]
fn teste_busca_nome() {
    let mut index = Indexador::new();

    index.adicionar(Produto {
        nome: "TV".to_string(),
        categoria: "Eletronicos".to_string(),
        preco: 2000.0,
    });

    let resultado = buscar_por_nome(&index, "TV");

    assert!(resultado.is_some());
}
