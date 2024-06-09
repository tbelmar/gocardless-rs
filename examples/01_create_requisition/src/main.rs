/// # example-01-create-requisition
///
/// This package demonstrates how to create a requisition using the unofficial GoCardless client.
///
/// # Dependencies
///
/// * `gocardless-unofficial`: This is the unofficial GoCardless client that is used to interact with the GoCardless API.
/// * `nanoid`: This is used to generate unique IDs.
/// * `tokio`: This is used to provide an async runtime.
///
/// # Usage
///
/// To run this example, navigate to the `example-01-create-requisition` directory and run `cargo run`.
///
/// This will create a new requisition and print the details to the console.
///
/// Please note that you will need to replace the `secret_id` and `secret_key` in the `main.rs` file with your own GoCardless API credentials.
///
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

    let end_user_agreement = client
        .create_end_user_agreement(&starling_bank.id, 180)
        .await?;
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
