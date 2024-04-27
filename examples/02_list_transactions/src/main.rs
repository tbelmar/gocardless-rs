/// # example-02
///
/// This package demonstrates how to interact with the GoCardless API using the unofficial GoCardless client.
///
/// # Dependencies
///
/// * `gocardless-unofficial`: This is the unofficial GoCardless client that is used to interact with the GoCardless API.
/// * `tokio`: This is used to provide an async runtime.
///
/// # Usage
///
/// To run this example, navigate to the `example-02` directory and run `cargo run`.
///
/// This will:
/// * Retrieve a list of requisitions
/// * Find a linked requisition from the list
/// * Retrieve the account ID associated with the linked requisition
/// * Retrieve the balances for the account
/// * Retrieve the details for the account
/// * Retrieve the transactions for the account
///
/// The results will be printed to the console.
///
/// Please note that you will need to replace the `GOCARDLESS_SECRET_ID` and `GOCARDLESS_SECRET_KEY` environment variables with your own GoCardless API credentials.

use gocardless_unofficial::Client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret_id = std::env::var("GOCARDLESS_SECRET_ID").unwrap();
    let secret_key = std::env::var("GOCARDLESS_SECRET_KEY").unwrap();

    let client = Client::new(secret_id, secret_key).await?;

    let requisitions = client.list_requisitions().await?;
    dbg!(&requisitions);

    let linked_requisition = requisitions.results.iter().find(|requisition| requisition.status == "LN").unwrap();
    dbg!(&linked_requisition);

    let account_id = &linked_requisition.accounts[0];
    dbg!(&account_id);

    let balances = client.list_balances(&account_id).await?;
    dbg!(&balances);

    let details = client.get_account_details(&account_id).await?;
    dbg!(&details);

    let transactions = client.list_transactions(&account_id).await?;
    dbg!(&transactions);

    Ok(())
}
