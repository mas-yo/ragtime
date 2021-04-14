use bytes::BytesMut;
use core::future::Future;
use tokio::net::TcpStream;
use tokio_stream::{self as stream, Stream, StreamExt};
use tokio_util::codec::{Decoder, Encoder, Framed};
use std::fmt;
use futures::sink::{Sink,SinkExt};
use core::pin::Pin;

#[derive(Debug)]
struct Ex1Error {
    message: String,
}
impl fmt::Display for Ex1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for Ex1Error {}
impl From<std::io::Error> for Ex1Error {
    fn from(e: std::io::Error) -> Self {
        Ex1Error { message: "io error".to_string() }
    }
}
// type RpcMessage = i32;
enum RpcMessage {
    C2SLogin(C2SLoginParam),
    S2CLoginSuccess(S2CLoginSuccessParam),
    C2SEnter(C2SEnterParam),
}
struct C2SLoginParam {
    token: String,
}
struct S2CLoginSuccessParam {
    room_id: u32,
    connect_url: String,
}
struct C2SEnterParam {
    token: String,
}

#[derive(Default)]
struct Ex1Protocol {}
impl Decoder for Ex1Protocol {
    type Item = RpcMessage;
    type Error = Ex1Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if &src[0..5] == b"login" {
            src.clear();
            Ok(Some(RpcMessage::C2SLogin(C2SLoginParam{token:"abc".to_string()})))
        }
        else {
            src.clear();
            Ok(None)
        }
    }
}
impl Encoder<RpcMessage> for Ex1Protocol {
    type Error = Ex1Error;

    fn encode(&mut self, item: RpcMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

async fn login<S>(mut stream: S, param: C2SLoginParam) -> Result<(), Box<dyn std::error::Error>>
where
    S: Stream<Item = Result<RpcMessage, Ex1Error>> + Sink<RpcMessage> + Unpin,
{
    // load user data, choose room
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    stream.send(RpcMessage::S2CLoginSuccess(S2CLoginSuccessParam{room_id:1, connect_url:"127.0.0.1".to_string()})).await;

    Ok(())
}

async fn game<S>(mut stream: S) -> Result<(), Box<dyn std::error::Error>>
where
    S: Stream<Item = Result<RpcMessage, Ex1Error>> + Sink<RpcMessage> + Unpin,
{
    loop {
        let msg = stream.next().await;
        match msg {
            Some(Ok(RpcMessage::C2SLogin(param))) => {
                println!("login {}", param.token);
                let p = login(Pin::new(&mut stream), param).await?;
            },
            Some(Ok(_)) => {
                return Err(Box::new(Ex1Error{message:"invalid command".to_string()}));
            }
            Some(Err(e)) => {

            },
            None => {
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    async fn game_(stream: Framed<TcpStream, Ex1Protocol>) {
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
