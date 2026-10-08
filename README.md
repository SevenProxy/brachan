# brachan

Imageboard/chan simples, desenvolvido em Rust, com foco em simplicidade, performance e facilidade de manutenção.

## Objetivo

Plataforma no estilo imageboard com:

- Criação de boards
- Criação de threads
- Respostas em threads
- Upload de imagens
- Paginação e limite de bump
- Moderação e exclusão de posts
- Proteção contra spam e abuso

O projeto começa simples e evolui conforme necessário. Sem overengineering.

## Stack

**Backend**

- Rust
- [Axum](https://github.com/tokio-rs/axum) — framework HTTP
- Tokio — async runtime
- SQLx — acesso ao banco de dados
- PostgreSQL — banco de dados

**Frontend**

- HTML, CSS e JavaScript vanilla
- Renderização server-side (SSR)
- Templates via Askama ou Maud

**Infraestrutura**

- Docker e Docker Compose (desenvolvimento)
- Storage local inicialmente, S3 no futuro

## Estrutura

```
zahard/
├── src/
│   ├── main.rs        # bootstrap do servidor
│   ├── routes/        # registro de rotas
│   └── infra/         # controllers e acesso externo
│       └── controlller/
├── Cargo.toml
└── Cargo.lock
```

Fluxo:

```
HTTP Request → Axum → Handler → Service → DB / Storage
```

Handlers cuidam do HTTP. Regras de negócio ficam nos services.

## Rotas

```
GET  /                     # Home
GET  /:board               # Board
GET  /:board/thread/:id    # Thread

POST /:board/thread        # Criar thread
POST /:board/thread/:id    # Criar resposta

POST /mod/delete/:id       # Deletar post
```

A estrutura pode mudar conforme o projeto evolui.

## Desenvolvimento

```bash
# Rodar
cargo run

# Testes
cargo test

# Formatação
cargo fmt

# Linter
cargo clippy

# Build
cargo build --release
```

Todas as rotas acima são executadas dentro de `zahard/`.

A CI roda `cargo fmt --check`, `cargo clippy --deny warnings`, `cargo test` e `cargo doc` a cada push.

## Banco de dados

PostgreSQL é a fonte principal de dados. Alterações de schema usam migrations em `migrations/`. Nunca alterar o banco em produção sem a migration correspondente.

## Segurança

Uploads são tratados como conteúdo não confiável. Conforme necessário:

- Rate limiting e controle de flood
- Limite de tamanho e validação de MIME type
- Sanitização/escape de conteúdo
- Hash de IP para mecanismos de moderação
- Validação de todos os inputs
- Headers de segurança e CSRF

## Princípios

- Código simples, sem abstrações prematuras
- Soluções idiomáticas de Rust
- Sem dependências sem necessidade
- SSR antes de framework frontend
- Toda funcionalidade nova considera segurança e abuso

## Licença

MIT
