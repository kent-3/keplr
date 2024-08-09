use js_sys::{JsString, Uint8Array};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

// TODO: implement signer traits for KeplrOfflineSigner
// TODO: implement encryption utils trait for EnigmaUtils

#[wasm_bindgen(js_namespace = ["window", "keplr"])]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = keplr)]
    pub static KEPLR: JsValue;

    pub type KeplrOfflineSigner;
    pub type JsEnigmaUtils;

    #[wasm_bindgen(catch)]
    pub async fn enable(chain_id: &str) -> Result<(), JsValue>;

    #[wasm_bindgen]
    pub fn disable(chain_id: &str);

    #[wasm_bindgen(js_name = disable)]
    /// Disable all chains for this origin (website).
    pub fn disable_origin();

    #[wasm_bindgen(catch, js_name = experimentalSuggestChain)]
    pub async fn suggest_chain(chainInfo: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = getChainInfoWithoutEndpoints)]
    pub async fn get_chain_info(chain_id: &str) -> JsValue;

    #[wasm_bindgen(catch, js_name = getKey)]
    pub async fn get_key(chain_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getOfflineSigner)]
    pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getOfflineSignerAuto)]
    pub fn get_offline_signer_auto(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getOfflineSignerOnlyAmino)]
    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getEnigmaUtils)]
    pub fn get_enigma_utils(chain_id: &str) -> JsEnigmaUtils;

    #[wasm_bindgen(catch, js_name = sendTx)]
    pub async fn sendTx(chainId: &str, tx: &[u8], mode: &str) -> Result<JsValue, JsValue>;

    // KeplrOfflineSigner methods

    #[wasm_bindgen(method, getter, js_name = chainId)]
    pub fn chain_id(this: &KeplrOfflineSigner) -> JsValue;

    #[wasm_bindgen(method, js_name = getAccounts)]
    pub async fn get_accounts(this: &KeplrOfflineSigner) -> JsValue;

    #[wasm_bindgen(method, js_name = signAmino)]
    pub async fn sign_amino(
        this: &KeplrOfflineSigner,
        signerAddress: JsString,
        signDoc: JsValue, // StdSignDoc
    ) -> JsValue; // AminoSignResponse

    #[wasm_bindgen(method, js_name = signDirect)]
    pub async fn sign_direct(
        this: &KeplrOfflineSigner,
        signerAddress: JsString,
        signDoc: JsValue, // SignDoc
    ) -> JsValue; // DirectSignResponse

    // EnigmaUtils methods (all of these return Uint8Array)

    #[wasm_bindgen(method, getter, js_name = chainId)]
    pub fn chain_id(this: &JsEnigmaUtils) -> JsValue;

    #[wasm_bindgen(method, js_name = decrypt)]
    pub async fn decrypt(
        this: &JsEnigmaUtils,
        ciphertext: Uint8Array,
        nonce: Uint8Array,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = encrypt)]
    pub async fn encrypt(
        this: &JsEnigmaUtils,
        contract_code_hash: JsString,
        msg: JsValue,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = getPubkey)]
    pub async fn get_pubkey(this: &JsEnigmaUtils) -> JsValue;

    #[wasm_bindgen(method, js_name = getTxEncryptionKey)]
    pub async fn get_tx_encryption_key(this: &JsEnigmaUtils, nonce: Uint8Array) -> JsValue;

    // Enigma functions (all of these return Uint8Array)

    #[wasm_bindgen(js_name = enigmaEncrypt)]
    pub async fn enigma_encrypt(chain_id: &str, code_hash: &str, msg: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = enigmaDecrypt)]
    pub async fn enigma_decrypt(chain_id: &str, ciphertext: &[u8], nonce: &[u8]) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaPubKey)]
    pub async fn get_enigma_pub_key(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaTxEncryptionKey)]
    pub async fn get_enigma_tx_encryption_key(chain_id: &str, nonce: &[u8]) -> JsValue;

    // other Secret functions

    #[wasm_bindgen(catch, js_name = suggestToken)]
    pub async fn suggest_token(chainId: &str, contract_address: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = getSecret20ViewingKey)]
    pub async fn get_secret_20_viewing_key(chain_id: &str, contract_address: &str) -> JsValue;
}

// TODO: this part belongs not in this crate?

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
        let nonce = Uint8Array::new(&serde_wasm_bindgen::to_value(nonce)?);
        let ciphertext = Uint8Array::new(&serde_wasm_bindgen::to_value(ciphertext)?);
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
        let msg = serde_wasm_bindgen::to_value(msg)?;
        let result = self.inner.encrypt(contract_code_hash, msg).await;
        Ok(result.into())
    }

    pub async fn get_pubkey(&self) -> Result<Uint8Array, JsValue> {
        let pubkey = self.inner.get_pubkey().await;
        Ok(pubkey.into())
    }

    pub async fn get_tx_encryption_key(&self, nonce: &[u8; 32]) -> Result<Uint8Array, JsValue> {
        let nonce = Uint8Array::from(serde_wasm_bindgen::to_value(nonce)?);
        let key = self.inner.get_tx_encryption_key(nonce).await;
        Ok(key.into())
    }
}

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
