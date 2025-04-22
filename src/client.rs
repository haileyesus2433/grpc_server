pub mod streaming {
    tonic::include_proto!("streaming");
}

use streaming::{Start, streaming_client::StreamingClient};

#[tokio::main]
async fn main() {
    let mut client = StreamingClient::connect("http://[::1]:5051").await.unwrap();
    for n in 1..10 {
        println!("requesting squares up to {}", n);
        let request = tonic::Request::new(Start { n });
        let mut stream = client.squares(request).await.unwrap().into_inner();
        while let Some(result) = stream.message().await.unwrap() {
            println!("RESULT= {:?}", result)
        }
    }
}

// pub mod grpc_server {
//     tonic::include_proto!("hello");
// }

// use grpc_server::HelloRequest;
// use grpc_server::greeter_client::GreeterClient;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = GreeterClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(HelloRequest {
//         name: "Haile".into(),
//     });
//     let response = client.say_hello(request).await?;

//     println!("RESPONSE={:?}", response);

//     Ok(())
// }
