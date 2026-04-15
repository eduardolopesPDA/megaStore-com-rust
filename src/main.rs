use std::collections::HashMap;

//  definindo o Modelo do banco de dados
#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
    marca: String,
    preco: f64,
    quantidade_estoque: u32,
}

//  estrutura do Catálogo 
struct Catalogo {
    // Acesso direto por ID
    produtos: HashMap<u32, Produto>,
    
    // indice para busca rápida por Nome (Chave: Nome em minúsculo)
    indice_por_nome: HashMap<String, u32>,
    
    // indice para busca por Categoria (Chave: Categoria em minúsculo)
    indice_por_categoria: HashMap<String, Vec<u32>>,
}

impl Catalogo {
    fn new() -> Self {
        Catalogo {
            produtos: HashMap::new(),
            indice_por_nome: HashMap::new(),
            indice_por_categoria: HashMap::new(),
        }
    }

    // Adiciona o produto e atualiza todos os "atalhos" (indices)
    fn adicionar_produto(&mut self, produto: Produto) {
        let id = produto.id;
        let nome_key = produto.nome.to_lowercase();
        let cat_key = produto.categoria.to_lowercase();

        // Insere no armazenamento principal
        self.produtos.insert(id, produto);

        // Atualiza indice de nomes
        self.indice_por_nome.insert(nome_key, id);

        // Atualiza indice de categorias (adiciona o ID à lista daquela categoria)
        self.indice_por_categoria
            .entry(cat_key)
            .or_insert(Vec::new())
            .push(id);
    }

    // busca O(1) por nome
    fn buscar_por_nome(&self, nome: &str) -> Option<&Produto> {
        let busca = nome.to_lowercase();
        self.indice_por_nome.get(&busca).and_then(|id| self.produtos.get(id))
    }

    // busca por categoria retornando uma lista de referências
    fn buscar_por_categoria(&self, categoria: &str) -> Vec<&Produto> {
        let busca = categoria.to_lowercase();
        let mut resultados = Vec::new();

        if let Some(ids) = self.indice_por_categoria.get(&busca) {
            for id in ids {
                if let Some(p) = self.produtos.get(id) {
                    resultados.push(p);
                }
            }
        }
        resultados
    }
}

// função adicionar os itens ao sistema
fn inicializar_dados(catalogo: &mut Catalogo) {
    let itens = vec![
        // Eletrônicos
        (1, "Smartphone X1", "Eletronicos", "TechCo", 2500.0, 10),
        (2, "Auscultadores Noise-Free", "Eletronicos", "SoundMax", 800.0, 5),
        (3, "Portatil Workstation", "Eletronicos", "FastChip", 5500.0, 3),
        (4, "Smartwatch Fit", "Eletronicos", "HealthTrack", 1200.0, 15),
        (5, "Monitor 4K 27\"", "Eletronicos", "VisionPro", 2200.0, 7),

        // Guitarras
        (6, "Guitarra Stratocaster Classica", "Guitarras", "Fender", 4500.0, 2),
        (7, "Guitarra Les Paul Standard", "Guitarras", "Gibson", 6000.0, 1),
        (8, "Guitarra Semi-Acustica", "Guitarras", "Ibanez", 3200.0, 4),
        (9, "Guitarra Telecaster Pro", "Guitarras", "Fender", 4800.0, 3),
        (10, "Guitarra de 7 Cordas Metal", "Guitarras", "ESP", 5200.0, 2),

        // Móveis
        (11, "Secretaria de Carvalho", "Moveis", "WoodArt", 1500.0, 6),
        (12, "Cadeira Ergonomica Pro", "Moveis", "ComfortSeat", 1800.0, 10),
        (13, "Estante de Livros Minimalista", "Moveis", "DecorHome", 900.0, 8),
        (14, "Mesa de Centro Industrial", "Moveis", "LoftStyle", 750.0, 5),
        (15, "Sofa 3 Lugares Veludo", "Moveis", "SoftLiving", 3500.0, 4),
    ];

    for (id, nome, cat, marca, preco, qtd) in itens {
        catalogo.adicionar_produto(Produto {
            id,
            nome: nome.to_string(),
            categoria: cat.to_string(),
            marca: marca.to_string(),
            preco,
            quantidade_estoque: qtd,
        });
    }
}

// função principal
fn main() {
    let mut megastore = Catalogo::new();
    inicializar_dados(&mut megastore);

    println!("BEM VINDO(A) A MEGASTORE ");

    //busca por Nome
    println!("\n> Teste de busca por nome ('Smartphone X1'):");
    if let Some(p) = megastore.buscar_por_nome("Smartphone X1") {
        println!("  Resultado: {} | Preço: R${:.2} | Estoque: {}", p.nome, p.preco, p.quantidade_estoque);
    }

    //busca por Categoria 
    println!("\n> Teste de busca por categoria ('GUITARRAS'):");
    let guitarras = megastore.buscar_por_categoria("GUITARRAS");
    for g in guitarras {
        println!("  - {:<30} | Marca: {:<10} | R${:>8.2}", g.nome, g.marca, g.preco);
    }


    println!("Busca finalizada com sucesso!");
}