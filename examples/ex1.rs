use bytes::BytesMut;
use core::future::Future;
use tokio::net::TcpStream;
use tokio_stream::{self as stream, Stream, StreamExt};
use tokio_util::codec::{Decoder, Encoder, Framed};

type RpcMessage = i32;

#[derive(Default)]
struct Ex1Decoder {}
impl Decoder for Ex1Decoder {
    type Item = RpcMessage;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(1i32))
    }
}
struct Ex1Encoder {}
impl Encoder<RpcMessage> for Ex1Encoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: i32, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

async fn game<S>(mut stream: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: Stream<Item = Result<RpcMessage, std::io::Error>> + Unpin,
{
    loop {
        match stream.next().await {
            Some(m) => {
                // let i:u32 = m;
            }
            None => {}
        }
    }
}

fn main() {
    async fn game_(stream: Framed<TcpStream, Ex1Decoder>) {
        game(stream).await.unwrap_or_else(|err| {
            println!("{}", err);
        });
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(ragtime::start_tcp(game_));
}
