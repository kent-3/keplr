pub use crate::keplr_sys;
use crate::keplr_sys::*;
use js_sys::{Error, JsString};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

/// An Amino encoded message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AminoMsg {
    pub r#type: String,
    pub value: Vec<u8>,
}

/// Response after signing with Amino.
#[derive(Debug, Serialize, Deserialize)]
pub struct AminoSignResponse {
    /// The sign_doc that was signed.
    ///
    /// This may be different from the input sign_doc when the signer modifies it as part of the signing process.
    pub signed: StdSignDoc,
    pub signature: StdSignature,
}

/// Standard signature.
#[derive(Debug, Serialize, Deserialize)]
pub struct StdSignature {
    #[serde(alias = "pubKey")]
    pub pub_key: Pubkey,
    pub signature: String,
}

/// Public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pubkey {
    /// Possible types include:
    /// - "tendermint/PubKeySecp256k1"
    /// - "tendermint/PubKeyEd25519"
    /// - "tendermint/PubKeySr25519
    pub r#type: String,
    /// Base64 encoded String
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StdSignDoc {
    pub chain_id: String,
    pub account_number: String,
    pub sequence: String,
    pub fee: StdFee,
    pub msgs: Vec<AminoMsg>,
    pub memo: String,
}

/// Standard fee.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StdFee {
    pub amount: Vec<Coin>,
    pub gas: String,
    pub granter: Option<String>,
}

pub type Coin = String;

use base64::prelude::{Engine as _, BASE64_STANDARD};
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeyInfo {
    pub name: String,
    pub algo: String,
    pub pub_key: Vec<u8>,
    pub address: Vec<u8>,
    pub bech32_address: String,
}

impl std::fmt::Debug for KeyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyInfo")
            .field("name", &self.name)
            .field("algo", &self.algo)
            .field("pub_key", &BASE64_STANDARD.encode(&self.pub_key)) // Convert pub_key to base64
            .field("address", &BASE64_STANDARD.encode(&self.address)) // Convert address to base64
            .field("bech32_address", &self.bech32_address)
            .finish()
    }
}

pub struct Keplr {}

// TODO: return Errors instead of Strings?
impl Keplr {
    pub fn debug() {
        console::log_1(&KEPLR.clone())
    }

    // TODO: use a Vec of chain_ids to enable multiple chains at once
    pub async fn enable(chain_id: &str) -> Result<(), String> {
        enable(chain_id).await.map_err(|js_value| {
            let error = Error::from(js_value)
                .message()
                .as_string()
                .unwrap_or("unknown error".to_string());
            error
        })
    }

    pub async fn get_key(chain_id: &str) -> Result<KeyInfo, String> {
        get_key(chain_id)
            .await
            .and_then(|key| Ok(serde_wasm_bindgen::from_value::<KeyInfo>(key)?))
            .map_err(|js_value| {
                let error = Error::from(js_value)
                    .message()
                    .as_string()
                    .unwrap_or("unknown error".to_string());
                error
            })
    }

    pub async fn get_account(chain_id: &str) -> Account {
        let signer = get_offline_signer_only_amino(chain_id);
        let accounts = signer.get_accounts().await;
        let accounts = js_sys::Array::from(&accounts);
        let account = accounts.get(0);

        let account: Account = serde_wasm_bindgen::from_value(account)
            .expect("There was a problem with deserializing the 'Account'");

        account
    }

    // pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner {
    //     get_offline_signer(chain_id)
    // }
    //
    // pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSigner {
    //     get_offline_signer_only_amino(chain_id)
    // }
    //
    // pub fn get_enigma_utils(chain_id: &str) -> EnigmaUtils {
    //     EnigmaUtils::new(get_enigma_utils(chain_id))
    // }

    pub async fn get_secret_20_viewing_key(chain_id: &str, contract_address: &str) -> String {
        JsString::from(get_secret_20_viewing_key(chain_id, contract_address).await).into()
    }

    pub fn disable(chain_id: &str) {
        disable(chain_id)
    }

    pub fn disable_all_chains() {
        disable_origin()
    }
}

impl KeplrOfflineSigner {
    pub async fn sign(&self) -> Result<JsValue, JsValue> {
        todo!()
    }
}

#[derive(Deserialize, Clone)]
pub struct Account {
    pub address: String,
    pub algo: String,
    pub pubkey: Vec<u8>,
}

impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("address", &self.address)
            .field("algo", &self.algo)
            .field("pubkey", &BASE64_STANDARD.encode(&self.pubkey)) // Convert pubkey to base64
            .finish()
    }
}

pub mod suggest_chain_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SuggestingChainInfo {
        pub chain_id: String,
        pub chain_name: String,
        pub rpc: String,
        pub rest: String,
        pub bip44: Bip44,
        pub bech32_config: Bech32Config,
        pub currencies: Vec<Currency>,
        pub fee_currencies: Vec<FeeCurrency>,
        pub stake_currency: Currency,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Bip44 {
        pub coin_type: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Bech32Config {
        pub bech32_prefix_acc_addr: String,
        pub bech32_prefix_acc_pub: String,
        pub bech32_prefix_val_addr: String,
        pub bech32_prefix_val_pub: String,
        pub bech32_prefix_cons_addr: String,
        pub bech32_prefix_cons_pub: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Currency {
        pub coin_denom: String,
        pub coin_minimal_denom: String,
        pub coin_decimals: u8,
        pub coin_gecko_id: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct FeeCurrency {
        pub coin_denom: String,
        pub coin_minimal_denom: String,
        pub coin_decimals: u8,
        pub coin_gecko_id: String,
        pub gas_price_step: GasPriceStep,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GasPriceStep {
        pub low: f64,
        pub average: f64,
        pub high: f64,
    }
}
