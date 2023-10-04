use std::{io::Write, net::TcpStream, time::Duration};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};

const SERVER: &str = "127.0.0.1:8090";

fn main() {
    let mut stream = TcpStream::connect(SERVER).expect("server is not there");

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no microphone available");
    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .find(|c| c.sample_format() == SampleFormat::F32)
        .expect("no supported config?!")
        .with_max_sample_rate();

    dbg!(&supported_config);

    let mut buffer = Vec::new();

    let audio_stream = device
        .build_input_stream(
            &supported_config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                buffer.clear();
                buffer.extend(data.iter().copied().flat_map(f32::to_le_bytes));
                stream.write_all(&buffer).expect("network is gone");
            },
            move |err| {
                panic!("shud {:?}", err);
            },
            None,
        )
        .expect("failed to build audio stream for some reason");

    audio_stream.play().expect("stream failed");

    loop {
        std::thread::sleep(Duration::from_secs(u64::MAX));
    }
}
