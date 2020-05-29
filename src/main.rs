extern crate qrcode;
extern crate uuid;

use qrcode::render::svg;
use qrcode::QrCode;
use qrcode::{EcLevel, Version};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::stdin;
use std::io::{stdout, Write};
use uuid::Uuid;

fn main() {
    print("Enter the amount of QR codes you want to generate: ");

    create_dir_all("image/").expect("Failed to create image directory");

    for i in 0..read() {
        // Generate QR code
        let rand = Uuid::new_v4();

        println!("{}", format!("{}", rand).replace("-", ""));

        let code = QrCode::with_version(
            format!("{}", rand).replace("-", ""),
            Version::Normal(4),
            EcLevel::L,
        )
        .expect("Failed to generate qr code");
        let image = code
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        // Create file
        let mut file =
            File::create(format!("image/{}.svg", i)).expect("Failed to create image file.");
        file.write_all(image.as_bytes()).unwrap();
    }
}

fn print(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}

fn read() -> usize {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf = buf.replace("\r\n", "");

    loop {
        let amount: usize = match buf.parse() {
            Result::Ok(n) => n,
            Result::Err(_) => {
                println!("The number you entered is not usable. Please enter another number.");
                continue;
            }
        };

        return amount;
    }
}
