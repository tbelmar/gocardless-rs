use secrecy::{ExposeSecret, SecretString};
use serde_json::json;

use crate::model::*;

const URL_CREATE_TOKEN: &str = "https://bankaccountdata.gocardless.com/api/v2/token/new/";
const URL_GET_INSTITUTIONS: &str =
    "https://bankaccountdata.gocardless.com/api/v2/institutions/?country=gb"; // TODO: make country a variable
const URL_CREATE_END_USER_AGREEMENT: &str =
    "https://bankaccountdata.gocardless.com/api/v2/agreements/enduser/";
const URL_REQUISITIONS: &str = "https://bankaccountdata.gocardless.com/api/v2/requisitions/";

pub struct Client {
    req_client: reqwest::Client,
    secret_id: SecretString,
    secret_key: SecretString,
    created_token: Option<CreateTokenResponse>,
}

impl Client {
    pub async fn new(
        secret_id: impl Into<SecretString>,
        secret_key: impl Into<SecretString>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let req_client = reqwest::Client::new();

        let mut c = Client {
            req_client,
            secret_id: secret_id.into(),
            secret_key: secret_key.into(),
            created_token: None,
        };

        let created_token = c.create_token().await?;
        c.created_token = Some(created_token);

        Ok(c)
    }

    pub async fn create_token(&self) -> Result<CreateTokenResponse, Box<dyn std::error::Error>> {
        let response: CreateTokenResponse = self
            .req_client
            .post(URL_CREATE_TOKEN)
            .body(
                json!({
                    "secret_id": self.secret_id.expose_secret(),
                    "secret_key": self.secret_key.expose_secret(),
                })
                .to_string(),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_institutions(&self) -> Result<Vec<Institution>, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: Vec<Institution> = self
            .req_client
            .get(URL_GET_INSTITUTIONS)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn create_end_user_agreement(
        &self,
        institution_id: &str,
    ) -> Result<EndUserAgreement, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: EndUserAgreement = self
            .req_client
            .post(URL_CREATE_END_USER_AGREEMENT)
            .body(
                json!({
                    "institution_id": institution_id,
                    "max_historical_days": "180",
                    "access_valid_for_days": "30",
                    "access_scope": [
                        "balances",
                        "details",
                        "transactions"
                    ]
                })
                .to_string(),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn list_requisitions(
        &self,
    ) -> Result<ListRequisitionsResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: ListRequisitionsResponse = self
            .req_client
            .get(URL_REQUISITIONS)
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn create_requisition(
        &self,
        redirect: &str,
        institution_id: &str,
        agreement_id: &str,
        reference: &str,
    ) -> Result<Requisition, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: Requisition = self
            .req_client
            .post(URL_REQUISITIONS)
            .body(
                json!({
                    "redirect": redirect,
                    "institution_id": institution_id,
                    "reference": reference,
                    "agreement": agreement_id,
                    "user_language": "EN" // TODO: configurable
                })
                .to_string(),
            )
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn list_transactions(
        &self,
        account_id: &str,
    ) -> Result<ListTransactionsResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: ListTransactionsResponse = self
            .req_client
            .get(format!(
                "https://bankaccountdata.gocardless.com/api/v2/accounts/{}/transactions",
                account_id
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn list_balances(
        &self,
        account_id: &str,
    ) -> Result<ListBalancesResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: ListBalancesResponse = self
            .req_client
            .get(format!(
                "https://bankaccountdata.gocardless.com/api/v2/accounts/{}/balances",
                account_id
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub async fn get_account_details(
        &self,
        account_id: &str,
    ) -> Result<AccountDetailsResponse, Box<dyn std::error::Error>> {
        let access_token = self.created_token.clone().unwrap().access;

        let response: AccountDetailsResponse = self
            .req_client
            .get(format!(
                "https://bankaccountdata.gocardless.com/api/v2/accounts/{}/details",
                account_id
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
