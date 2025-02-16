// Importando library qrcode.
use qrcode::QrCode;
use qrcode::render::unicode;

// Importando library image.
use image::Luma;

// Importando library para receber entrada do usuário.
use std::fs;

use eframe::egui;

struct QrCodeApp {
    input_text: String,
    salvar_imagem: bool,
    qr_code_text: Option<String>,  // Adicionando campo para armazenar o QR code
    status_message: Option<String>,
}

impl QrCodeApp {
    fn new() -> Self {
        Self {
            input_text: String::new(),
            salvar_imagem: false,
            qr_code_text: None,
            status_message: None,
        }
    }
}

impl eframe::App for QrCodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustQRForge");
            ui.vertical_centered(|ui| {
                ui.label("Texto para QR Code:");
                ui.text_edit_singleline(&mut self.input_text);
                
                // Checkbox único para controlar o salvamento
                ui.checkbox(&mut self.salvar_imagem, "Deseja salvar o qrCode em png?");
                
                if ui.button("Gerar QR Code").clicked() && !self.input_text.is_empty() {
                    // Gerar QR Code para exibição
                    let code = QrCode::new(&self.input_text).unwrap();
                    self.qr_code_text = Some(
                        code.render::<unicode::Dense1x2>()
                            .dark_color(unicode::Dense1x2::Light)
                            .light_color(unicode::Dense1x2::Dark)
                            .build()
                    );

                    // Salvar QR Code se solicitado
                    if self.salvar_imagem {
                        // Criar diretório se não existir
                        if let Err(e) = dir_create("qrcode_images") {
                            self.status_message = Some(format!("Erro ao criar diretório: {}", e));
                            return;
                        }

                        // Gerar e salvar imagem
                        let code = QrCode::new(&self.input_text).unwrap();
                        let image = code.render::<Luma<u8>>()
                            .quiet_zone(true)
                            .build();

                        match image.save("qrcode_images/qrcode.png") {
                            Ok(_) => self.status_message = Some("QR Code salvo com sucesso!".to_string()),
                            Err(e) => self.status_message = Some(format!("Erro ao salvar imagem: {}", e)),
                        }
                    }
                }

                // Exibir mensagem de status se houver
                if let Some(msg) = &self.status_message {
                    ui.add_space(5.0);
                    ui.label(msg);
                }
            });

            // Exibe o QR code se ele existir
            if let Some(qr_code) = &self.qr_code_text {
                ui.add_space(10.0); // Adiciona espaço entre os elementos
                ui.monospace(qr_code); // Usa fonte monoespaçada para manter o formato
            }
        });
    }
}

fn dir_create(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RustQRForge",
        options,
        Box::new(|_cc| Ok(Box::new(QrCodeApp::new())))  // Corrigido aqui
    )
}