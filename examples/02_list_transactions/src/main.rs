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
