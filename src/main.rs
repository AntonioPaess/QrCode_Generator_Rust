// Importando library qrcode.
use qrcode::QrCode;

// Importando library image.
use image::Luma;

// Importando library para receber entrada do usuário.
use std::fs;

use eframe::egui;

struct QrCodeApp {
    input_text: String,
    salvar_imagem: bool,
}

impl QrCodeApp {
    fn new() -> Self {
        Self {
            input_text: String::new(),
            salvar_imagem: false,
        }
    }
}

impl eframe::App for QrCodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("QrCode Generator in Rust");
            ui.vertical(|ui| {
                ui.label("Texto para QR Code:");
                ui.text_edit_singleline(&mut self.input_text);
                ui.checkbox(&mut self.salvar_imagem,"Deseja salvar o qrCode em png?")
            });

            if ui.button("Gerar QR Code").clicked() {
                // Aqui implementaremos a geração do QR code
            }
        });
    }
}

fn dir_create(string: &str) -> std::io::Result<()> {
    fs::create_dir_all(string)?;
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "QR Code Generator",
        options,
        Box::new(|_cc| Ok(Box::new(QrCodeApp::new())))  // Corrigido aqui
    )
}