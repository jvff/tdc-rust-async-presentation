# Apresentação sobre Programação Assíncrona em Rust

Este repositório contém os slides apresentados no The Developer's Conference
2018 em São Paulo.

Os slides estão em forma de web-site. Tanto o servidor quanto o código que roda
no browser foram escritos usando Rust. O conteúdo da apresentação está neste
repositório, enquanto a maior parte do código do site foi movida para uma
`crate` separada, que chama [presentrs].

[presentrs]: https://github.com/jvff/presentrs

## Execução dos slides

O jeito mais fácil para rodar os slides é através de uma imagem Docker. Na pasta
`docker` é possível rodar os comandos:

docker build -t tdc-async-rust .
docker run --rm -it -p 8080:8080 tdc-async-rust

Com isso, um container rodando o site será criado, e então é possível abrir o
site acessando o endereço do container no endereço
[http://localhost:8080/index.html].

[http://localhost:8080/index.html]: http://localhost:8080/index.html
