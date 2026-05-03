use tonic::{transport::Server, Request, Response, Status};

pub mod services {
    tonic::include_proto!("services");
}

use services::{
    payment_service_server::{PaymentService, PaymentServiceServer},
    PaymentRequest, PaymentResponse,
};

#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Menerima request pembayaran: {:?}", request);

        let req = request.into_inner();

        let success = req.amount > 0.0;

        let reply = PaymentResponse { success };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();

    println!("Server gRPC berjalan di {}", addr);

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}