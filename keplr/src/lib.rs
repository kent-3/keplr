mod error;
pub use error::Error;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::{
    console,
    js_sys::{self, JsString},
    wasm_bindgen,
};

pub struct Keplr {}

impl Keplr {
    pub fn debug() {
        keplr_sys::KEPLR.with(console::log_1)
    }

    pub fn is_available() -> bool {
        web_sys::window()
            .and_then(|window| {
                js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("keplr")).ok()
            })
            .map_or(false, |keplr| !keplr.is_undefined() && !keplr.is_null())
    }

    pub async fn ping() -> Result<(), Error> {
        keplr_sys::ping().await.map_err(Into::into)
    }

    pub async fn enable(chain_ids: Vec<String>) -> Result<(), Error> {
        keplr_sys::enable(chain_ids).await.map_err(Into::into)
    }

    pub async fn get_key(chain_id: &str) -> Result<Key, Error> {
        keplr_sys::get_key(chain_id)
            .await
            .and_then(|key| Ok(serde_wasm_bindgen::from_value::<Key>(key)?))
            .map_err(Into::into)
    }

    pub async fn get_accounts(&self, chain_id: &str) -> Result<Vec<AccountData>, Error> {
        keplr_sys::get_offline_signer(chain_id)
            .get_accounts()
            .await
            .map_err(Into::into)
            .map(|val| js_sys::Array::from(&val))
            .and_then(|accounts| {
                accounts
                    .iter()
                    .map(|account| serde_wasm_bindgen::from_value(account).map_err(Into::into))
                    .collect::<Result<Vec<AccountData>, Error>>()
            })
    }

    pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner {
        keplr_sys::get_offline_signer(chain_id).into()
    }

    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSignerOnlyAmino {
        keplr_sys::get_offline_signer_only_amino(chain_id).into()
    }

    // TODO: does it make sense to have an "auto" method?
    // pub async fn get_offline_signer_auto(
    //     chain_id: &str,
    // ) -> Result<Box<dyn Signer>, Error> {
    //     let key = Self::get_key(chain_id).await?;
    //     let signer: Box<dyn Signer> = match key.is_nano_ledger {
    //         true => Box::new(Self::get_offline_signer_only_amino(chain_id)),
    //         false => Box::new(Self::get_offline_signer(chain_id)),
    //     };
    //     Ok(signer)
    // }

    pub async fn suggest_token(
        chain_id: &str,
        contract_address: &str,
        viewing_key: Option<&str>,
    ) -> Result<(), Error> {
        keplr_sys::suggest_token(chain_id, contract_address, viewing_key)
            .await
            .map_err(Into::into)
    }

    pub async fn get_secret_20_viewing_key(
        chain_id: &str,
        contract_address: &str,
    ) -> Result<String, Error> {
        keplr_sys::get_secret_20_viewing_key(chain_id, contract_address)
            .await
            .map(|foo| JsString::from(foo).into())
            .map_err(Into::into)
    }

    pub fn disable(chain_id: &str) {
        keplr_sys::disable(chain_id)
    }

    pub fn disable_origin() {
        keplr_sys::disable_origin()
    }
}

#[derive(Clone)]
pub struct KeplrOfflineSigner {
    inner: Rc<keplr_sys::KeplrOfflineSigner>,
}

impl From<keplr_sys::KeplrOfflineSigner> for KeplrOfflineSigner {
    fn from(value: keplr_sys::KeplrOfflineSigner) -> Self {
        Self {
            inner: Rc::new(value),
        }
    }
}

impl KeplrOfflineSigner {
    pub fn chain_id(&self) -> String {
        self.inner
            .chain_id()
            .as_string()
            .expect("chain_id field is missing!")
    }

    pub async fn get_accounts(&self) -> Result<Vec<AccountData>, Error> {
        self.inner
            .get_accounts()
            .await
            .map_err(|_| Error::KeplrUnavailable)
            .map(|val| js_sys::Array::from(&val))
            .and_then(|accounts| {
                accounts
                    .iter()
                    .map(|account| serde_wasm_bindgen::from_value(account).map_err(Into::into))
                    .collect::<Result<Vec<AccountData>, Error>>()
            })
    }

    // NOTE: anything related to signing should be out of scope for this crate?

    // async fn get_sign_mode(&self) -> Result<SignMode, Error> {
    //     Ok(SignMode::Direct)
    // }
    //
    // async fn sign_amino(
    //     &self,
    //     signer_address: &str,
    //     sign_doc: StdSignDoc,
    // ) -> Result<AminoSignResponse, Error> {
    //     todo!()
    // }
    //
    // async fn sign_permit(
    //     &self,
    //     signer_address: &str,
    //     sign_doc: StdSignDoc,
    // ) -> Result<AminoSignResponse, Error> {
    //     todo!()
    // }
    //
    // async fn sign_direct(
    //     &self,
    //     signer_address: &str,
    //     sign_doc: SignDoc,
    // ) -> Result<DirectSignResponse, Error> {
    //     todo!()
    // }
}

#[derive(Clone)]
pub struct KeplrOfflineSignerOnlyAmino {
    inner: Rc<keplr_sys::KeplrOfflineSignerOnlyAmino>,
}

impl From<keplr_sys::KeplrOfflineSignerOnlyAmino> for KeplrOfflineSignerOnlyAmino {
    fn from(value: keplr_sys::KeplrOfflineSignerOnlyAmino) -> Self {
        Self {
            inner: Rc::new(value),
        }
    }
}

impl KeplrOfflineSignerOnlyAmino {
    pub fn chain_id(&self) -> String {
        self.inner
            .chain_id()
            .as_string()
            .expect("chain_id field is missing!")
    }

    pub async fn get_accounts(&self) -> Result<Vec<AccountData>, Error> {
        self.inner
            .get_accounts()
            .await
            .map_err(|_| Error::KeplrUnavailable)
            .map(|val| js_sys::Array::from(&val))
            .and_then(|accounts| {
                accounts
                    .iter()
                    .map(|account| serde_wasm_bindgen::from_value(account).map_err(Into::into))
                    .collect::<Result<Vec<AccountData>, Error>>()
            })
    }

    // NOTE: anything related to signing should be out of scope for this crate?

    // async fn get_sign_mode(&self) -> Result<SignMode, Error> {
    //     Ok(SignMode::LegacyAminoJson)
    // }
    //
    // async fn sign_amino(
    //     &self,
    //     signer_address: &str,
    //     sign_doc: StdSignDoc,
    // ) -> Result<AminoSignResponse, Error> {
    //     todo!()
    // }
    //
    // async fn sign_permit(
    //     &self,
    //     signer_address: &str,
    //     sign_doc: StdSignDoc,
    // ) -> Result<AminoSignResponse, Error> {
    //     todo!()
    // }
    //
    // async fn sign_direct(
    //     &self,
    //     signer_address: &str,
    //     sign_doc: SignDoc,
    // ) -> Result<DirectSignResponse, Error> {
    //     unimplemented!()
    // }
}

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    /// Name of the selected key store.
    pub name: String,
    pub algo: String,
    pub pub_key: Vec<u8>,
    pub address: Vec<u8>,
    pub bech32_address: String,
    pub ethereum_hex_address: String,
    // Indicate whether the selected account is from the nano ledger.
    // Because current cosmos app in the nano ledger doesn't support the direct (proto) format msgs,
    // this can be used to select the amino or direct signer.
    pub is_nano_ledger: bool,
    pub is_keystone: bool,
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("name", &self.name)
            .field("algo", &self.algo)
            .field("pub_key", &BASE64_STANDARD.encode(&self.pub_key)) // Convert pub_key to base64
            .field("address", &BASE64_STANDARD.encode(&self.address)) // Convert address to base64
            .field("bech32_address", &self.bech32_address)
            .field("ethereum_hex_address", &self.ethereum_hex_address)
            .field("is_nano_ledger", &self.is_nano_ledger)
            .field("is_keystone", &self.is_keystone)
            .finish()
    }
}

/// Data related to an account.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountData {
    pub address: String,
    pub algo: Algo,
    pub pubkey: Vec<u8>,
}

impl std::fmt::Debug for AccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("address", &self.address)
            .field("algo", &self.algo)
            .field("pubkey", &BASE64_STANDARD.encode(&self.pubkey))
            .finish()
    }
}

/// Algorithm types used for signing.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Algo {
    Secp256k1,
    Ed25519,
    Sr25519,
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
