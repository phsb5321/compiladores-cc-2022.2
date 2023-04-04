# Interpretador RPN (Notação Polonesa Reversa)

Este projeto é um interpretador simples de Notação Polonesa Reversa (RPN) escrito em Rust. O interpretador processa expressões RPN de arquivos de texto (`.txt`) ou arquivos específicos (`.stk`).

## Requisitos

- Rust 1.54 ou superior
- Cargo (geralmente incluído na instalação do Rust)

## Como compilar e executar

Para compilar o projeto, navegue até o diretório do projeto e execute:

    $ cargo build --release

Isso gerará um executável no diretório `target/release`. Para executar o interpretador em um arquivo ou diretório específico, use o seguinte comando:

    $ cargo run -- -f arquivo_entrada.stk

Ou:

    $ cargo run -- -d diretorio_exemplos

## Exemplo de uso

Suponha que você tenha um arquivo chamado `input.stk` com o seguinte conteúdo:

    4 8 + 3 *

Esta é uma expressão RPN que representa a expressão matemática `(4 + 8) * 3`. Para avaliar esta expressão usando o interpretador RPN, execute:

    $ cargo run -- -f input.stk

A saída será:

    Resultado para input.stk: 36

## Contribuindo

Este projeto é de código aberto e aceita contribuições! Sinta-se à vontade para abrir uma issue ou enviar um pull request com melhorias ou correções de bugs.

## Licença

Este projeto está licenciado sob a Licença MIT. Consulte o arquivo `LICENSE` para obter detalhes.
