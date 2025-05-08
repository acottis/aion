use crate::error::{Error, Result};

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Message {
    pub user_id: Option<String>,
    pub user_center: Option<u16>,
    pub login_name: Option<String>,
    pub status: Option<String>,
    pub user_name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Connect {
    conn_type: u16,
    app_index: u8,
    build: u16,
    product_type: u16,
    address: std::net::IpAddr,
    process: u16,
    epoch: u16,
    program: u16,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_rand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_algorithm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    premaster_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authn_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_provider_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    net_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    game_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_account: Option<String>,
}

impl Request {
    pub fn encryption_key(&self) -> &Option<String> {
        &self.encryption_key
    }
    pub fn premaster_secret(&self) -> &Option<String> {
        &self.premaster_secret
    }
    pub fn authn_token(&self) -> &Option<String> {
        &self.authn_token
    }
    pub fn client_rand(&self) -> &Option<String> {
        &self.client_rand
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Body {
    Connect(Connect),
    Request(Request),
    Message(Message),
    Empty,
}

impl Body {
    pub fn deserialise(body: &str) -> Result<Self> {
        let message =
            quick_xml::de::from_str(body).map_err(Error::InvalidPacketBody)?;
        Ok(message)
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Reply {
    // Encryption Related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_rand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_signature: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<u16>,
    /// Account Creation Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    /// These are they same value, not sure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_market_code: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_center: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_login_name_validated: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_center: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_mask: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_credential_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authn_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_account: Option<GameAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<ExternalAccounts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    // Currency?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_info_list: Option<BalanceInfoList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_info_list_count: Option<usize>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceInfoList {
    pub balance_info: Vec<BalanceInfo>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceInfo {
    pub currency_id: u16,
    pub currency_group_id: u16,
    pub balance: u64,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalAccounts {
    pub external_account: Vec<ExternalAccount>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub struct ExternalAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_provider_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_associated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dissociable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_account_apps: Option<ExternalAccountApps>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalAccountApps {
    pub external_account_app: Vec<ExternalAccountApp>,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalAccountApp {
    pub app_group_code: String,
    pub external_user_app_key: String,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameAccount {
    pub alias: String,
    pub created: String,
    pub game_account_id: String,
}
