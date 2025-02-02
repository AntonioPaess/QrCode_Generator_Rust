// Importando library qrcode.
use qrcode::QrCode;
use qrcode::render::unicode;

// Importando library image.
use image::Luma;

// Importando library para receber entrada do usuário.
use std::io;


fn main(){
    println!("Escreva o dado que deseja transformar em Qrcode\n=>");
    
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");

    let entrada = entrada.trim();

    println!("Você deseja transformar esse dado em QrCode? (y/n) ");

    let mut confirmacao = String::new();
    io::stdin().read_line(&mut confirmacao).expect("Falha ao ler a confirmação");

    // Gerando o Qrcode do usuario em Unicode.
    if confirmacao.trim().to_lowercase() == "y" {
        let code = QrCode::new(entrada).unwrap();
        let qr_code = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

        println!("{}", qr_code);
    }
    else {
        println!("Criação do QrCode cancelada");
    }

    println!("Você deseja salvar seu QrCode? (y/n)");
    
    let mut confirmacao2 = String::new();
    io::stdin().read_line(&mut confirmacao2).expect("Falha ao ler a confirmação");

    // Gerando o Qrcode do usuario em png.
    if confirmacao2.trim().to_lowercase() == "y"{
        let code = QrCode::new(entrada).unwrap();
        let image = code.render::<Luma<u8>>().build();
        image.save("qrcode_images/qrcode.png").expect("Erro ao salvar a imagem");
        
        println!("QrCode salvo como 'qrcode.png' em 'qrcode_images'");
    }
    else{
        println!("Qrcode Deletado com sucesso.");
    }



    




}