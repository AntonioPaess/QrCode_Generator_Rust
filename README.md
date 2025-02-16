# RustQRForge - Gerador de QR Code em Rust

![RustQRForge Logo](assets/logo.png)

Um gerador de QR Code moderno com interface gráfica, desenvolvido em Rust. Combine a eficiência do Rust com uma interface amigável para criar QR Codes facilmente.

---

## ✨ Funcionalidades

- **Interface gráfica intuitiva:** Desenvolvida com egui/eframe.
- **Geração instantânea:** Cria QR Codes a partir do texto digitado.
- **Visualização em tempo real:** Veja o QR Code assim que ele for gerado.
- **Salvar como PNG:** Opção de salvar automaticamente o QR Code em `qrcode_images/`.
- **Feedback visual:** Mensagens e indicadores integrados à interface.
- **Design moderno e centralizado.**

---

## 🚀 Como Executar

### Pré-requisitos
- [Rust](https://www.rust-lang.org/) instalado no seu sistema.
- Ambiente gráfico (X11 no Linux, nativo no macOS/Windows).


### Instalação

1. Clone o repositório:
   ```sh
   git clone https://github.com/seu-usuario/RustQRForge.git
   cd RustQRForge
   ```

2. Execute o programa:
   ```sh
   cargo run
   ```

## 💻 Como Usar

1. Digite o texto desejado na caixa de entrada
2. (Opcional) Marque a caixa "Salvar como PNG" se desejar salvar o QR Code
3. Clique em "Gerar QR Code"
4. O QR Code será exibido instantaneamente na interface
5. Se selecionado, o arquivo PNG será salvo em `qrcode_images/`

## 📦 Dependências

```toml
[dependencies]
qrcode = "0.14.1"
image = "0.25.5"
egui = "0.31"
eframe = "0.31"
```

## 🖼️ Screenshots

[Aqui você pode adicionar screenshots da sua interface]

## 🔧 Melhorias Futuras

- [x] Permitir salvar o QR Code como imagem
- [x] Criar uma versão com interface gráfica
- [ ] Suporte para diferentes formatos de saída (SVG, JPG, PDF)
- [ ] Personalização do QR Code (cores, tamanhos, etc.)
- [ ] Suporte para QR Codes PIX
- [ ] Histórico de QR Codes gerados

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Abrir issues
2. Enviar Pull Requests
3. Sugerir melhorias
4. Reportar bugs

## 📜 Licença

Este projeto é distribuído sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.

## 📱 Contato

- **Linkedin & email** - <a href="mailto:ajpaj@cesar.school">📧</a> - <a href="https://www.linkedin.com/in/ant%C3%B4niopaess/"><img src="https://upload.wikimedia.org/wikipedia/commons/c/ca/LinkedIn_logo_initials.png" width="20"></a>

---
