use crate::keplr_sys::*;
pub use crate::keplr_sys::{suggest_chain, KeplrOfflineSigner};
use js_sys::{JsString, Uint8Array};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
#[derive(Serialize, Deserialize, Clone)]
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

// TODO: implement signer traits for KeplrOfflineSigner
// TODO: implement encryption utils trait for EnigmaUtils

pub struct Keplr {}

impl Keplr {
    pub fn debug() {
        console::log_1(&KEPLR.clone())
    }

    pub async fn enable(chain_id: &str) -> Result<(), JsValue> {
        enable(chain_id).await
    }

    pub async fn get_key(chain_id: &str) -> KeyInfo {
        let key = get_key(chain_id).await;
        let key_info: KeyInfo = from_value(key).unwrap();
        key_info
    }

    pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner {
        get_offline_signer(chain_id)
    }

    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSigner {
        get_offline_signer_only_amino(chain_id)
    }

    pub fn get_enigma_utils(chain_id: &str) -> EnigmaUtils {
        EnigmaUtils::new(get_enigma_utils(chain_id))
    }

    pub async fn get_secret_20_viewing_key(chain_id: &str, contract_address: &str) -> JsString {
        get_secret_20_viewing_key(chain_id, contract_address)
            .await
            .into()
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

// TODO: this part belongs not in this crate

pub struct EnigmaUtils {
    pub inner: JsEnigmaUtils,
}

impl EnigmaUtils {
    pub fn new(enigma_utils: JsEnigmaUtils) -> Self {
        Self {
            inner: enigma_utils,
        }
    }

    /// Decrypt using Keplr.
    pub async fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<Uint8Array, JsValue> {
        let nonce = Uint8Array::new(&to_value(nonce)?);
        let ciphertext = Uint8Array::new(&to_value(ciphertext)?);
        let plaintext = self.inner.decrypt(ciphertext, nonce).await;
        let plaintext = Uint8Array::from(plaintext);

        // TODO: maybe use this for debug print
        // let decoder = web_sys::TextDecoder::new_with_label("utf-8")?;
        // console::log_1(
        //     &decoder
        //         .decode_with_buffer_source(&plaintext.clone().into())?
        //         .into(),
        // );

        Ok(plaintext.into())
    }

    /// Encrypt using Keplr.
    pub async fn encrypt<M: serde::Serialize>(
        &self,
        contract_code_hash: String,
        msg: &M,
    ) -> Result<Uint8Array, JsValue> {
        let contract_code_hash = JsString::from_str(&contract_code_hash).unwrap();
        let msg = to_value(msg)?;
        let result = self.inner.encrypt(contract_code_hash, msg).await;
        Ok(result.into())
    }

    pub async fn get_pubkey(&self) -> Result<Uint8Array, JsValue> {
        let pubkey = self.inner.get_pubkey().await;
        Ok(pubkey.into())
    }

    pub async fn get_tx_encryption_key(&self, nonce: &[u8; 32]) -> Result<Uint8Array, JsValue> {
        let nonce = Uint8Array::from(to_value(nonce)?);
        let key = self.inner.get_tx_encryption_key(nonce).await;
        Ok(key.into())
    }
}

// // TODO: this part belongs not in this crate
//
// use async_trait::async_trait;
//
// #[async_trait(?Send)]
// pub trait Enigma {
//     async fn encrypt<M: ::serde::Serialize>(
//         &self,
//         contract_code_hash: &str,
//         msg: &M,
//     ) -> Result<Uint8Array, JsValue>;
//
//     async fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 32]) -> Result<Uint8Array, JsValue>;
// }
//
// // NOTE:
// // Encryption and decryption can be more efficient if we get the tx_encryption_key once at the start, then
// // reuse it for each individual encryption/decryption call.
// // But then the WASM binary size increases if we bring in all the crypto stuff...
// // But I think we need that stuff anyway for the default implementation...
//
// #[async_trait(?Send)]
// impl Enigma for EnigmaUtils {
//     async fn encrypt<M: ::serde::Serialize>(
//         &self,
//         code_hash: &str,
//         msg: &M,
//     ) -> Result<Uint8Array, JsValue> {
//         let code_hash = JsString::from_str(code_hash).unwrap();
//         let msg = serde_wasm_bindgen::to_value(msg)?;
//
//         self.encrypt_js(code_hash, msg.into()).await
//     }
//
//     async fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 32]) -> Result<Uint8Array, JsValue> {
//         self.decrypt_js(ciphertext.into(), nonce.as_slice().into())
//             .await
//     }
// }
