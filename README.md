# Repositório do projeto de Rust para o workshop de CI/CD do glua

Este repositório contêm o código do projeto de demonstração de uma codebase rust
com integração e delivery contínuo através das github actions e dos releases.

## Instruções de desenvolvimento

Para desenvolver utiliza-se os comandos normais:

```bash
$ cargo run
```

E para dar build ao programa final:

```bash
$ cargo build --release
```

## Workflows

A pasta `.github/workflows` contêm os workflows finais que correm no github.

Os workflows além de correrem no push, também correm quando uma nova tag é
gerada, criando automaticamente um novo issue.
