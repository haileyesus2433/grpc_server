use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, transport::Server};

pub mod streaming {
    tonic::include_proto!("streaming");
}

use streaming::streaming_server::{Streaming, StreamingServer};
use streaming::{Square, Start};

#[derive(Debug, Default)]
pub struct StreamingService {}

#[tonic::async_trait]
impl Streaming for StreamingService {
    type SquaresStream = ReceiverStream<Result<Square, Status>>;
    async fn squares<'life0>(
        &'life0 self,
        request: Request<Start>,
    ) -> Result<Response<Self::SquaresStream>, Status> {
        println!("Got a request {:#?}", request);
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            for i in 0..request.into_inner().n {
                let square = Square { n: i * i };
                tx.send(Ok(square)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5051".parse().unwrap();
    println!("Square server running on: {}", addr);
    let streamer = StreamingService {};
    let svc = StreamingServer::new(streamer);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}

/*
pub mod grpc_server {
    tonic::include_proto!("hello");
}

use grpc_server::greeter_server::{Greeter, GreeterServer};
use grpc_server::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got Request : {:?}", request);
        let reply = grpc_server::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}

*/
