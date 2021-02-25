use bytes::BytesMut;
use core::future::Future;
use tokio_stream::{self as stream, StreamExt};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Default)]
struct Ex1Decoder {}
impl Decoder for Ex1Decoder {
    type Item = i32;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(Some(1i32))
    }
}
struct Ex1Encoder {}
impl Encoder<i32> for Ex1Encoder {
    type Error = std::io::Error;

    fn encode(&mut self, item: i32, dst: &mut BytesMut) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn test<S>(mut stream: S) -> impl Future<Output = ()>
where
    S: StreamExt + Unpin,
{
    async move {
        stream.next().await;
    }
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(ragtime::start_tcp::<Ex1Decoder, _>(test));
}
