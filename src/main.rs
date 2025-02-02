use qrcode::QrCode;
use qrcode::render::unicode::{self, Dense1x2};

use std::io;


fn main(){
    println!("Escreva o dado que deseja transformar em Qrcode\n=>");
    
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");

    let entrada = entrada.trim();

    println!("Você deseja transformar esse dado em QrCode? (y/n) ");

    let mut confirmacao = String::new();
    io::stdin().read_line(&mut confirmacao).expect("Falha ao ler a confirmação");

    if confirmacao.trim().to_lowercase() == "y" {
        let code = QrCode::new(entrada).unwrap();
        let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

        println!("{}", image);
    }
    else {
        println!("Criação do QrCode cancelada");
    }


    




}