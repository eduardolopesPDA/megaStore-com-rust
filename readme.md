# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📝 Descrição do Projeto
Este projeto consiste na implementação de um sistema de busca de alta performance para o catálogo de produtos da "MegaStore". O desafio principal era substituir sistemas de busca tradicionais lentos por uma solução escalável e eficiente.

A solução utiliza a linguagem **Rust** e estruturas de dados de **Tabelas Hash (HashMaps)** para garantir que as buscas por nome e categoria ocorram em tempo constante, independentemente do volume de dados, resolvendo o problema de latência e imprecisão da plataforma original.

## 🚀 Funcionalidades
- **Indexação Eficiente:** Uso de múltiplos HashMaps para busca instantânea por ID e Nome.
- **Agrupamento por Categoria:** Sistema que permite listar todos os produtos de um setor específico.
- **Busca Case-Insensitive:** O sistema normaliza as entradas, permitindo que o usuário encontre produtos mesmo que digite com letras maiúsculas ou minúsculas diferentes do cadastro.
- **Gerenciamento de Stock:** Cada produto possui controle de quantidade, permitindo a visualização da disponibilidade em tempo real.

## 🛠️ Tecnologias Utilizadas
- **Linguagem:** [Rust](https://www.rust-lang.org/) (Edição 2021)
- **Gerenciador de Pacotes:** Cargo
- **Estruturas de Dados:** `std::collections::HashMap` para indexação e `std::vec::Vec` para coleções de dados.

## 📂 Estrutura do Repositório
- `src/main.rs`: Contém o código-fonte, incluindo a lógica das estruturas `Produto` e `Catalogo`.
- `Cargo.toml`: Arquivo de configuração e dependências do projeto Rust.

## ⚙️ Como Executar o Projeto

### Pré-requisitos
- Ter o Rust e o Cargo instalados. Se não tiver, instale através de [rustup.rs](https://rustup.rs/).

### Passo a Passo
1. Clone o repositório:
   digite no terminal:
   git clone <link-do-seu-repositorio>

  2. Entre na pasta do projeto:
  digite no terminal:
  cd "megaStore com rust"

  3. Execute o sistema:
     cargo run

