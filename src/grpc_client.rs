pub mod services {
    tonic::include_proto!("services");
}

use services::{payment_service_client::PaymentServiceClient, PaymentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Menghubungkan client ke server
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    // Menyusun request
    let request = tonic::Request::new(PaymentRequest {
        user_id: "user_2306245075".to_string(),
        amount: 250.0,
    });

    // Memanggil fungsi di server
    let response = client.process_payment(request).await?;

    println!("RESPON DARI SERVER: {:?}", response.into_inner());

    Ok(())
}