use gocardless_unofficial::Client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_id = std::env::var("GOCARDLESS_SECRET_ID").unwrap();
    let secret_key = std::env::var("GOCARDLESS_SECRET_KEY").unwrap();

    let client = Client::new(secret_id, secret_key).await?;

    let institutions = client.get_institutions().await?;
    dbg!(&institutions);

    let starling_bank = institutions
        .iter()
        .find(|institution| institution.name.starts_with("Starling Bank"))
        .unwrap();
    dbg!(&starling_bank);

    let end_user_agreement = client.create_end_user_agreement(&starling_bank.id).await?;
    dbg!(&end_user_agreement);
    
    let reference = nanoid::nanoid!();
    dbg!(&reference);

    let requisition = client
        .create_requisition(
            "https://www.example.org",
            &starling_bank.id,
            &end_user_agreement.id,
            &reference,
        )
        .await?;
    dbg!(requisition);

    Ok(())
}
