use tonic::transport::{Channel, ClientTlsConfig};
use lykke_rs::PricesRequest;
use lykke_rs::public_service_client::PublicServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = PublicServiceClient::connect("https://hft-apiv2-grpc.lykke.com:443").await?;

    let prices = client.get_prices(PricesRequest{
        asset_pair_ids: vec![String::from("BTCUSD"), String::from("ETHUSD")]
    }).await?;

    let prices = prices.into_inner();
    assert_eq!(prices.error, None);

    let prices = prices.payload;

    for price in prices {
        println!("{:?}", price)
    }

    Ok(())
}