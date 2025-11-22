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
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await.unwrap();
    let mut samples = Vec::with_capacity(buf.len() / 4);
    for chunk in buf.chunks_exact(4) {
        let arr = [chunk[0], chunk[1], chunk[2], chunk[3]];
        let sample = f32::from_be_bytes(arr);
        samples.push(sample);
    }
    buffer_sender
        .send(SampleBuffer::from(samples))
        .await
        .unwrap();
}
