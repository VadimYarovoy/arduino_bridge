use std::io::{self, Write};
use serial::prelude::*;
use std::io::Read;

fn main() {
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600; 

    let mut port = serial::open(port_name).expect("Failed to open port");
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::BaudRate::from_speed(baud_rate)).unwrap();
        Ok(())
    }).expect("Failed to configure port");

    loop {
        let mut input = String::new();

        print!("Введите значение для отправки на Arduino: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            println!("Выход из программы.");
            break;
        }

        port.write(input.as_bytes()).expect("Failed to send data");
        println!("Данные успешно отправлены на Arduino!");

        let mut buffer = [0; 128];
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Ответ от Arduino: {}", response);
            }
            Err(e) => eprintln!("Failed to read data: {:?}", e),
        }
    }
}
