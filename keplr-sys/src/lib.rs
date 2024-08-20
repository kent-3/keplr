use js_sys::wasm_bindgen::prelude::*;
use js_sys::{JsString, Uint8Array};

#[wasm_bindgen(js_namespace = ["window", "keplr"])]
extern "C" {
    #[wasm_bindgen(thread_local, js_namespace = window, js_name = keplr)]
    pub static KEPLR: JsValue;

    pub type KeplrOfflineSigner;
    pub type KeplrOfflineSignerOnlyAmino;
    pub type EnigmaUtils;

    #[wasm_bindgen(js_name = ping, catch)]
    pub async fn ping() -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = enable, catch)]
    pub async fn enable(chain_ids: Vec<String>) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = disable)]
    pub fn disable(chain_id: &str);

    #[wasm_bindgen(js_name = disableOrigin)]
    /// Disable all chains for this origin (website).
    pub fn disable_origin();

    #[wasm_bindgen(js_name = experimentalSuggestChain, catch)]
    pub async fn suggest_chain(chainInfo: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = getChainInfoWithoutEndpoints)]
    pub async fn get_chain_info(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getKey, catch)]
    pub async fn get_key(chain_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getOfflineSigner)]
    pub fn get_offline_signer(chain_id: &str) -> KeplrOfflineSigner;

    #[wasm_bindgen(js_name = getOfflineSignerOnlyAmino)]
    pub fn get_offline_signer_only_amino(chain_id: &str) -> KeplrOfflineSignerOnlyAmino;

    #[wasm_bindgen(js_name = getOfflineSignerAuto)]
    pub async fn get_offline_signer_auto(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaUtils)]
    pub fn get_enigma_utils(chain_id: &str) -> EnigmaUtils;

    #[wasm_bindgen(js_name = sendTx, catch)]
    pub async fn sendTx(chainId: &str, tx: &[u8], mode: &str) -> Result<JsValue, JsValue>;

    // KeplrOfflineSigner methods

    #[wasm_bindgen(method, js_name = chainId, getter)]
    pub fn chain_id(this: &KeplrOfflineSigner) -> JsValue;

    #[wasm_bindgen(method, js_name = getAccounts, catch)]
    pub async fn get_accounts(this: &KeplrOfflineSigner) -> Result<JsValue, JsValue>;

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

    // KeplrOfflineSignerOnlyAmino methods

    #[wasm_bindgen(method, js_name = chainId, getter)]
    pub fn chain_id(this: &KeplrOfflineSignerOnlyAmino) -> JsValue;

    #[wasm_bindgen(method, js_name = getAccounts, catch)]
    pub async fn get_accounts(this: &KeplrOfflineSignerOnlyAmino) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = signAmino)]
    pub async fn sign_amino(
        this: &KeplrOfflineSignerOnlyAmino,
        signerAddress: JsString,
        signDoc: JsValue, // StdSignDoc
    ) -> JsValue; // AminoSignResponse

    // EnigmaUtils methods (all of these return Uint8Array)

    #[wasm_bindgen(method, js_name = chainId, getter)]
    pub fn chain_id(this: &EnigmaUtils) -> JsValue;

    #[wasm_bindgen(method, js_name = decrypt)]
    pub async fn decrypt(this: &EnigmaUtils, ciphertext: Uint8Array, nonce: Uint8Array) -> JsValue;

    #[wasm_bindgen(method, js_name = encrypt)]
    pub async fn encrypt(this: &EnigmaUtils, contract_code_hash: JsString, msg: JsValue)
        -> JsValue;

    #[wasm_bindgen(method, js_name = getPubkey)]
    pub async fn get_pubkey(this: &EnigmaUtils) -> JsValue;

    #[wasm_bindgen(method, js_name = getTxEncryptionKey)]
    pub async fn get_tx_encryption_key(this: &EnigmaUtils, nonce: Uint8Array) -> JsValue;

    // Enigma functions (all of these return Uint8Array)
    // NOTE: these seem to be equivalent to the EnigmaUtils methods, but may be more convenient

    #[wasm_bindgen(js_name = enigmaEncrypt)]
    pub async fn enigma_encrypt(chain_id: &str, code_hash: &str, msg: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = enigmaDecrypt)]
    pub async fn enigma_decrypt(chain_id: &str, ciphertext: &[u8], nonce: &[u8]) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaPubKey)]
    pub async fn get_enigma_pub_key(chain_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = getEnigmaTxEncryptionKey)]
    pub async fn get_enigma_tx_encryption_key(chain_id: &str, nonce: &[u8]) -> JsValue;

    // other Secret functions

    #[wasm_bindgen(js_name = suggestToken, catch)]
    pub async fn suggest_token(
        chainId: &str,
        contract_address: &str,
        viewing_key: Option<&str>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = getSecret20ViewingKey, catch)]
    pub async fn get_secret_20_viewing_key(
        chain_id: &str,
        contract_address: &str,
    ) -> Result<JsValue, JsValue>;
}

// getOfflineSigner(
//    chainId: string,
//    signOptions?: KeplrSignOptions
//  ): OfflineAminoSigner & OfflineDirectSigner {
//    return new CosmJSOfflineSigner(chainId, this, signOptions);
//  }
//
//  getOfflineSignerOnlyAmino(
//    chainId: string,
//    signOptions?: KeplrSignOptions
//  ): OfflineAminoSigner {
//    return new CosmJSOfflineSignerOnlyAmino(chainId, this, signOptions);
//  }
//
//  async getOfflineSignerAuto(
//    chainId: string,
//    signOptions?: KeplrSignOptions
//  ): Promise<OfflineAminoSigner | OfflineDirectSigner> {
//    const key = await this.getKey(chainId);
//    if (key.isNanoLedger) {
//      return new CosmJSOfflineSignerOnlyAmino(chainId, this, signOptions);
//    }
//    return new CosmJSOfflineSigner(chainId, this, signOptions);
//  }
