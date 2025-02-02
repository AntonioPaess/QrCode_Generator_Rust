# Gerador de QR Code em Rust

Este projeto foi criado para facilitar a geração de QR Codes diretamente no terminal. A motivação surgiu a partir de um pedido de uma pessoa que precisava de uma forma simples e rápida de criar QR Codes.

## 📌 Funcionalidades
- Permite inserir um texto ou link para gerar um QR Code.
- Exibe o QR Code diretamente no terminal usando caracteres Unicode.
- Simples e rápido, sem necessidade de interfaces gráficas.
- Novo: Permite salvar o QR Code gerado como uma imagem PNG.

## 🚀 Como Executar
### Pré-requisitos
- [Rust](https://www.rust-lang.org/) instalado no seu sistema.

### Passos para rodar o projeto
1. Clone este repositório:
   ```sh
   git clone https://github.com/seu-usuario/seu-repositorio.git
   cd seu-repositorio
2. Crie a pasta onde a imagem será salva::
   ```sh
   mkdir -p qrcode_images
3. Compile e execute o programa:
   ```sh
   cargo run
4. Insira o dado que deseja transformar em QR Code e confirme com y.

5. Escolha se deseja salvar o QR Code como uma imagem PNG.
   - Se confirmar (y), o arquivo será salvo em qrcode_images/qrcode.png.
   - Caso contrário, o QR Code será apenas exibido no terminal.

## 📦 Dependências
Este projeto utiliza as seguintes crates:
 - qrcode para gerar os QR Codes.
 - Novo: image para salvar os QR Codes como imagens PNG.

 Para instalar as dependências manualmente, use:  
   
      cargo add qrcode image


## 🔧 Melhorias Futuras
- Permitir salvar o QR Code como imagem. ✅
- Criar uma versão com interface gráfica.
- suporte para diferentes formatos de saída.
- Personalização do QR Code (cores, tamanhos, etc.).

## 📜 Licença

Este projeto é de código aberto e pode ser usado livremente.
