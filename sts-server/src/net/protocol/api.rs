use base64::prelude::{Engine, BASE64_STANDARD};
use krypt::sts::{aes256_cbc_decrypt, generate_rc4_keys, Rc4};
use std::{fmt::Display, str::from_utf8};

use crate::{
    error::{Error, Result},
    net::{protocol::body::Body, Session},
};

use super::{
    body::{
        self, BalanceInfo, BalanceInfoList, ExternalAccount,
        ExternalAccountApp, ExternalAccountApps, ExternalAccounts, Reply,
    },
    Request, Response, ResponseHeader,
};

// This is static
const EXTERNAL_ACCOUNT_KEY: &str = "1bb702a4-02f9-4b9d-b44d-15ffb2e84568";
// This is static
const ACCESS_MASK: &str = "134217727";
// This is static
const SESSION: &str = "4BEEADEB-5C6B-4191-BC4A-0B517750E202";
// this is static
const LOCATION_ID: &str = "4BEEADEB-5C6B-4191-BC4A-0B517750E202";
// This is static
const GAME_ACCOUNT_ID: &str = "40028854";

const USER_STATUS: u16 = 1;
const CREATED: &str = "2023-04-25T10:47:54Z";
const CREATED2: &str = "2023-04-25T12:47:54.820+02:00";
const USER_CENTER: u16 = 22;
const USER_NAME: &str = "Nickname";
const LOGIN_NAME: &str = "0187b806-ec9a-72f8-9a6f-d8bbf29ae667@gameforge.com";
const IS_LOGIN_NAME_VALIDATED: u16 = 0;
const AUTH_PROVIDER_CODE: &str = "gameforge";
const EXTERNAL_ACCOUNT_NAME: &str = "Nickname";
const USER_CREDENTIAL_CODE: &str = "gameforge";
const APP_GROUP_CODE: &str = "aiongfc";

// this is static
const USER_ID: &str = "9E054E89-C0BD-4792-9A22-33116D9EABB7";
// THIS CHANGES - PROBABLY THE ACTUAL AUTH
const TOKEN: &str = "00DF51EC-591D-46BF-8859-1C8FEC95FC09";
// USER_ID + TOKEN then base64?
const AUTHN_TOKEN: &str = "OUUwNTRFODktQzBCRC00NzkyLTlBMjItMzMxMTZEOUVBQkI3OjY3NDA2NzQ4LTI0NTUtNEQxMy1CRUVFLTFGNjc2NDQxRjRFQwA=";

#[derive(Debug, PartialEq, Eq)]
pub enum Api {
    Auth(Auth),
    Sts(Sts),
    GameAccount(GameAccount),
    VirtualCurrency(VirtualCurrency),
    AccountV1(AccountV1),
    Presence(Presence),
}

impl Api {
    /// Compare the incomming method against implemented replies
    pub fn handle(
        &self,
        session: &mut Session,
        request: &Request,
        seq: Option<u16>,
    ) -> Option<Response> {
        match self {
            Api::Auth(method) => method.handle(session, request, seq),
            Api::Sts(method) => method.handle(seq),
            Api::GameAccount(method) => method.handle(seq),
            Api::AccountV1(method) => method.handle(seq),
            Api::VirtualCurrency(method) => method.handle(seq),
            Api::Presence(_) => unimplemented!(),
        }
    }
}

impl Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Api::Auth(method) => write!(f, "/Auth/{method}"),
            Api::Sts(method) => write!(f, "/Sts/{method}"),
            Api::GameAccount(method) => write!(f, "/GameAccount/{method}"),
            Api::AccountV1(method) => write!(f, "/Account/{method}"),
            Api::Presence(method) => write!(f, "/Presence/{method}"),
            Api::VirtualCurrency(method) => {
                write!(f, "/VirtualCurrency/{method}")
            }
        }
    }
}

impl TryFrom<Option<&str>> for Api {
    type Error = Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        // Remove the leading / from the Api
        let api = &value.ok_or(Error::InvalidApi)?[1..];

        let (namespace, function) =
            api.split_once('/').ok_or(Error::InvalidApi)?;

        match namespace {
            "Sts" => match function {
                "Connect" => Ok(Self::Sts(Sts::Connect)),
                "Ping" => Ok(Self::Sts(Sts::Ping)),
                function => Err(Error::ApiFunctionUnimplemented(format!(
                    "/Sts/{function}"
                ))),
            },
            "Auth" => match function {
                "TokenKeyData" => Ok(Self::Auth(Auth::TokenKeyData)),
                "LoginTokenStart" => Ok(Self::Auth(Auth::LoginKeyStart)),
                "LoginStart" => Ok(Self::Auth(Auth::LoginStart)),
                "LoginFinish" => Ok(Self::Auth(Auth::LoginFinish)),
                "RequestToken" => Ok(Self::Auth(Auth::RequestToken)),
                "RequestGameToken" => Ok(Self::Auth(Auth::RequestGameToken)),
                function => Err(Error::ApiFunctionUnimplemented(format!(
                    "/Auth/{function}"
                ))),
            },
            "GameAccount" => match function {
                "ListMyAccounts" => {
                    Ok(Self::GameAccount(GameAccount::ListMyAccounts))
                }
                function => Err(Error::ApiFunctionUnimplemented(format!(
                    "/GameAccount/{function}"
                ))),
            },
            "Account" => match function {
                "1/GetUserInfo" => Ok(Self::AccountV1(AccountV1::GetUserInfo)),
                function => Err(Error::ApiFunctionUnimplemented(format!(
                    "/Account/{function}"
                ))),
            },
            "VirtualCurrency" => match function {
                "GetBalance" => {
                    Ok(Self::VirtualCurrency(VirtualCurrency::GetBalance))
                }
                function => Err(Error::ApiFunctionUnimplemented(format!(
                    "/VirtualCurrency/{function}"
                ))),
            },
            namespace => {
                Err(Error::ApiNamespaceUnimplemented(namespace.into()))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Presence {
    UserInfo,
}

impl Display for Presence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserInfo => write!(f, "UserInfo"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AccountV1 {
    GetUserInfo,
}
impl AccountV1 {
    fn handle(&self, sequence: Option<u16>) -> Option<Response> {
        match self {
            Self::GetUserInfo => {
                let header = ResponseHeader::new(sequence);

                let external_account_apps = ExternalAccountApps {
                    external_account_app: vec![ExternalAccountApp {
                        app_group_code: APP_GROUP_CODE.into(),
                        external_user_app_key: EXTERNAL_ACCOUNT_KEY.into(),
                    }],
                };

                let external_accounts = ExternalAccounts {
                    external_account: vec![ExternalAccount {
                        auth_provider_code: Some(USER_CREDENTIAL_CODE.into()),
                        external_account_key: Some(EXTERNAL_ACCOUNT_KEY.into()),
                        external_account_name: Some(
                            EXTERNAL_ACCOUNT_NAME.into(),
                        ),
                        created: Some(CREATED2.into()),
                        manual_associated: Some(false),
                        dissociable: Some(false),
                        external_account_apps: Some(external_account_apps),
                        ..Default::default()
                    }],
                };

                let body = Reply {
                    user_id: Some(USER_ID.into()),
                    user_status: Some(USER_STATUS),
                    created: Some(CREATED.into()),
                    user_market_code: Some(USER_CENTER),
                    user_center: Some(USER_CENTER),
                    user_name: Some(USER_NAME.into()),
                    login_name: Some(LOGIN_NAME.into()),
                    is_login_name_validated: Some(IS_LOGIN_NAME_VALIDATED),
                    external_accounts: Some(external_accounts),
                    ..Default::default()
                };
                Some(Response { header, body })
            }
        }
    }
}

impl Display for AccountV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetUserInfo => write!(f, "GetUserInfo"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Auth {
    TokenKeyData,
    LoginKeyStart,
    LoginStart,
    LoginFinish,
    RequestToken,
    RequestGameToken,
}

impl Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Auth::TokenKeyData => write!(f, "TokenKeyData"),
            Auth::LoginKeyStart => write!(f, "LoginKeyStart"),
            Auth::LoginStart => write!(f, "LoginStart"),
            Auth::LoginFinish => write!(f, "LoginFinish"),
            Auth::RequestToken => write!(f, "RequestToken"),
            Auth::RequestGameToken => write!(f, "RequestGameToken"),
        }
    }
}

impl Auth {
    fn handle(
        &self,
        session: &mut Session,
        request: &Request,
        sequence: Option<u16>,
    ) -> Option<Response> {
        match self {
            Self::LoginKeyStart => {
                let request = match &request.body {
                    Body::Request(request) => request,
                    _ => todo!(),
                };

                let mut buf = [0u8; 1000];
                let len = BASE64_STANDARD
                    .decode_slice(
                        &request.client_rand().as_ref().unwrap(),
                        &mut buf,
                    )
                    .unwrap();
                session.krypt.client_rand = buf[..len].try_into().unwrap();

                let mut buf = [0u8; 1000];
                let len =
                    session.krypt.rsa.serialise_as_public_key_blob(&mut buf);
                let mut base64_pub_key_buf = [0u8; 5000];
                let len = BASE64_STANDARD
                    .encode_slice(&buf[..len], &mut base64_pub_key_buf)
                    .unwrap();
                let pub_key = from_utf8(&base64_pub_key_buf[..len]).unwrap();

                let mut buf = [0u8; 1000];
                let len = BASE64_STANDARD
                    .encode_slice(session.krypt.server_rand, &mut buf)
                    .unwrap();
                let server_rand = from_utf8(&buf[..len]).unwrap();

                let header = ResponseHeader::new(sequence);
                let body = Reply {
                    server_rand: Some(server_rand.into()),
                    server_public_key: Some(pub_key.into()),
                    ..Default::default()
                };
                Some(Response { header, body })
            }
            Self::TokenKeyData => {
                let request = match &request.body {
                    Body::Request(request) => request,
                    _ => todo!(),
                };
                println!("{request:#?}");

                let mut buf = [0u8; 5000];

                let key = request.encryption_key().as_ref().unwrap();
                let len = BASE64_STANDARD.decode_slice(&key, &mut buf).unwrap();
                let key = session.krypt.rsa.decrypt(&buf[..len]);
                let key: [u8; 32] = key.as_slice().try_into().unwrap();

                let premaster_secret_base64 =
                    request.premaster_secret().as_ref().unwrap();
                let len = BASE64_STANDARD
                    .decode_slice(&premaster_secret_base64, &mut buf)
                    .unwrap();
                assert_eq!(len, 48, "premaster secret must be 48 bytes");
                let mut premaster_secret: [u8; 48] =
                    buf[..len].try_into().unwrap();
                aes256_cbc_decrypt(&key, &mut premaster_secret).unwrap();

                let authn_token_base64 =
                    request.authn_token().as_ref().unwrap();
                let len = BASE64_STANDARD
                    .decode_slice(&authn_token_base64, &mut buf)
                    .unwrap();
                let authn_token = &mut buf[..len];
                aes256_cbc_decrypt(&key, authn_token).unwrap();
                session.authn_token =
                    std::str::from_utf8(authn_token).unwrap().into();

                let (encrypt_key, decrypt_key) = generate_rc4_keys(
                    session.krypt.server_rand,
                    session.krypt.client_rand,
                    premaster_secret,
                );
                session.krypt.rc4 = Some(Rc4::new(encrypt_key, decrypt_key));

                let header = ResponseHeader::new(sequence);
                let body = Reply::default();
                Some(Response { header, body })
            }
            Self::LoginStart => unimplemented!("LoginStart"),
            Self::LoginFinish => {
                let header = ResponseHeader::new(sequence);

                let external_accounts = ExternalAccounts {
                    external_account: vec![ExternalAccount {
                        auth_provider_code: Some(AUTH_PROVIDER_CODE.into()),
                        external_account_key: Some(EXTERNAL_ACCOUNT_KEY.into()),
                        external_account_name: Some(
                            EXTERNAL_ACCOUNT_NAME.into(),
                        ),
                        ..Default::default()
                    }],
                };

                let body = Reply {
                    user_id: Some(USER_ID.into()),
                    user_status: Some(USER_STATUS),
                    created: Some(CREATED.into()),
                    user_center: Some(USER_CENTER),
                    login_center: Some(USER_CENTER),
                    user_market_code: Some(USER_CENTER),
                    user_name: Some(USER_NAME.into()),
                    login_name: Some(LOGIN_NAME.into()),
                    is_login_name_validated: Some(IS_LOGIN_NAME_VALIDATED),
                    external_accounts: Some(external_accounts),
                    game_account_id: Some(GAME_ACCOUNT_ID.into()),
                    access_mask: Some(ACCESS_MASK.into()),
                    session: Some(SESSION.into()),
                    location_id: Some(LOCATION_ID.into()),
                    user_credential_code: Some(USER_CREDENTIAL_CODE.into()),
                    ..Default::default()
                };

                Some(Response { header, body })
            }
            Self::RequestToken => {
                let header = ResponseHeader::new(sequence);

                let body = Reply {
                    authn_token: Some(AUTHN_TOKEN.into()),
                    ..Default::default()
                };

                Some(Response { header, body })
            }
            Self::RequestGameToken => {
                let header = ResponseHeader::new(sequence);

                let body = Reply {
                    token: Some(TOKEN.into()),
                    ..Default::default()
                };

                Some(Response { header, body })
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameAccount {
    ListMyAccounts,
}
impl GameAccount {
    fn handle(&self, sequence: Option<u16>) -> Option<Response> {
        match self {
            Self::ListMyAccounts => {
                let header = ResponseHeader::new(sequence);

                let body = Reply {
                    game_account: Some(body::GameAccount {
                        alias: USER_ID.into(),
                        created: CREATED.into(),
                        game_account_id: GAME_ACCOUNT_ID.into(),
                    }),
                    ..Default::default()
                };
                Some(Response { header, body })
            }
        }
    }
}

impl Display for GameAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListMyAccounts => write!(f, "ListMyAccounts"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VirtualCurrency {
    GetBalance,
}
impl VirtualCurrency {
    fn handle(&self, sequence: Option<u16>) -> Option<Response> {
        match self {
            Self::GetBalance => {
                let header = ResponseHeader::new(sequence);

                let balance_info_list = BalanceInfoList {
                    balance_info: vec![
                        BalanceInfo {
                            currency_id: 74,
                            currency_group_id: 100,
                            balance: 0,
                        },
                        BalanceInfo {
                            currency_id: 75,
                            currency_group_id: 101,
                            balance: 0,
                        },
                    ],
                };

                let balance_info_list_count =
                    balance_info_list.balance_info.len();

                let body = Reply {
                    balance_info_list: Some(balance_info_list),
                    balance_info_list_count: Some(balance_info_list_count),
                    ..Default::default()
                };
                Some(Response { header, body })
            }
        }
    }
}

impl Display for VirtualCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GetBalance => write!(f, "GetBalance"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sts {
    Connect,
    Ping,
}

impl Display for Sts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sts::Connect => write!(f, "Connect"),
            Sts::Ping => write!(f, "Ping"),
        }
    }
}

impl Sts {
    fn handle(&self, _sequence: Option<u16>) -> Option<Response> {
        None
    }
}
