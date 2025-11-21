use {
    crate::sample_buffer::SampleBuffer,
    rodio::OutputStreamBuilder,
    std::net::{Ipv4Addr, SocketAddrV4},
    tokio::{
        io::{AsyncReadExt, BufReader},
        net::{TcpListener, TcpStream},
        sync::mpsc::{Receiver, Sender, channel},
    },
};

mod sample_buffer;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 6767))
        .await
        .unwrap();

    let (sender, receiver) = channel::<SampleBuffer>(1024);

    tokio::spawn(handle_audio(receiver));

    while let Ok((stream, _)) = listener.accept().await {
        let sender = sender.clone();
        tokio::spawn(handle_client(stream, sender));
    }
}

async fn handle_audio(mut receiver: Receiver<SampleBuffer>) {
    let device = OutputStreamBuilder::open_default_stream().unwrap();

    while let Some(sample_buffer) = receiver.recv().await {
        device.mixer().add(sample_buffer);
    }
}

async fn handle_client(stream: TcpStream, buffer_sender: Sender<SampleBuffer>) {
    let mut reader = BufReader::new(stream);

    let Ok(sample_count) = reader.read_u16().await else {
        return;
    };

    let mut samples = Vec::with_capacity(sample_count as usize);

    for _ in 0..sample_count {
        let Ok(sample) = reader.read_f32().await else {
            return;
        };

        samples.push(sample);
    }

    let sample_buffer = SampleBuffer::from(samples);
    buffer_sender.send(sample_buffer).await.ok();
}
