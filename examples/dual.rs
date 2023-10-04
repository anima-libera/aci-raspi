use std::{io::Read, net::TcpListener};

const SERVER: &str = "127.0.0.1:8090";

fn main() {
    let stream = TcpListener::bind(SERVER).expect("failed to bind");

    let mut client = stream.accept().expect("could not accept client").0;

    const LENGTH: usize = 44100;
    let mut acc = Vec::with_capacity(LENGTH);
    loop {
        let mut sample_buf = [0u8; 4];
        client.read_exact(&mut sample_buf).expect("failed to read");
        let sample = f32::from_le_bytes(sample_buf) * 10.0;
        acc.push(sample.abs());

        if acc.len() == LENGTH {
            println!("Avg: {}", acc.iter().sum::<f32>() / acc.len() as f32);
            acc.clear();
        }
    }
}
