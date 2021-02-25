use core::future::Future;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Decoder, Framed};

pub async fn start_tcp<D, F>(
    f: fn(Framed<TcpStream, D>) -> F,
) -> Result<(), Box<dyn std::error::Error>>
where
    D: Decoder + Default + Send + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let framed = Framed::new(socket, D::default());
            f(framed).await
        });
    }
}
