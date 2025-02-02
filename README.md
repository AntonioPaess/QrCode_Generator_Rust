# Gerador de QR Code em Rust

Este projeto foi criado para facilitar a geração de QR Codes diretamente no terminal. A motivação surgiu a partir de um pedido de uma pessoa que precisava de uma forma simples e rápida de criar QR Codes.

## 📌 Funcionalidades
- Permite inserir um texto ou link para gerar um QR Code.
- Exibe o QR Code diretamente no terminal usando caracteres Unicode.
- Simples e rápido, sem necessidade de interfaces gráficas.

## 🚀 Como Executar
### Pré-requisitos
- [Rust](https://www.rust-lang.org/) instalado no seu sistema.

### Passos para rodar o projeto
1. Clone este repositório:
   ```sh
   git clone https://github.com/seu-usuario/seu-repositorio.git
   cd seu-repositorio
2. Compile e execute o programa:
   ```sh
   cargo run
3. Insira o dado que deseja transformar em QR Code e confirme com y.

## 📦 Dependências
Este projeto utiliza a crate [qrcode](https://crates.io/crates/qrcode) para gerar os QR Codes. Ela pode ser instalada manualmente com:
    
    cargo add qrcode

## 🔧 Melhorias Futuras
- Permitir salvar o QR Code como imagem.
- Criar uma versão com interface gráfica.
- suporte para diferentes formatos de saída.

## 📜 Licença

Este projeto é de código aberto e pode ser usado livremente.
