use crate::keplr::{suggest_chain, suggest_chain_types::*, Keplr};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = alert)]
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    const CHAIN_ID: &str = "secret-4";
    let window = web_sys::window().expect("no global `window` exists");

    if js_sys::Reflect::has(&window, &JsValue::from_str("keplr")).unwrap() {
        // suggest().await;
        // disable(CHAIN_ID);
        let keplr = Keplr::new().enable(CHAIN_ID).await?;
        keplr.debug();

        let key_info = keplr.get_key(CHAIN_ID).await;
        console::log_1(&serde_wasm_bindgen::to_value(&key_info)?);

        let offline_signer = keplr.get_offline_signer(CHAIN_ID);

        let accounts = offline_signer.get_accounts().await;
        let accounts = js_sys::Array::from(&accounts);
        let account = accounts.get(0);
        console::log_1(&account);

        // hey it works

        // let signer_address = JsString::from_str(&key_info.bech32_address).unwrap();
        // let sign_doc = StdSignDoc {
        //     chain_id: CHAIN_ID.into(),
        //     account_number: "0".into(),
        //     sequence: "0".into(),
        //     fee: StdFee::default(),
        //     msgs: vec![],
        //     memo: "".into(),
        // };
        // let result = offline_signer
        //     .sign_amino(signer_address, serde_wasm_bindgen::to_value(&sign_doc)?)
        //     .await;

        let enigma_utils = keplr.get_enigma_utils(CHAIN_ID);

        let contract_code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";

        #[derive(Serialize)]
        pub struct SomeMsg {
            pub value: String,
        }

        let msg = SomeMsg {
            value: "hello".to_string(),
        };

        let message = enigma_utils
            .encrypt(contract_code_hash.into(), &msg)
            .await?;

        let message: Vec<u8> = serde_wasm_bindgen::from_value(message.into())?;
        let nonce = &message[0..32];
        let ciphertext = &message[64..];
        let plaintext = enigma_utils.decrypt(ciphertext, nonce).await?;
        let plaintext = plaintext.to_vec();
        let plaintext = String::from_utf8_lossy(&plaintext);
        console::log_1(&plaintext.to_string().into());
    } else {
        alert("Please install keplr extension");
    }
    Ok(())
}

pub async fn suggest() {
    let chain_info = SuggestingChainInfo {
        chain_id: "mychain-1".to_string(),
        chain_name: "my new chain".to_string(),
        rpc: "http://123.456.789.012:26657".to_string(),
        rest: "http://123.456.789.012:1317".to_string(),
        bip44: Bip44 { coin_type: 118 },
        bech32_config: Bech32Config {
            bech32_prefix_acc_addr: "cosmos".to_string(),
            bech32_prefix_acc_pub: "cosmospub".to_string(),
            bech32_prefix_val_addr: "cosmosvaloper".to_string(),
            bech32_prefix_val_pub: "cosmosvaloperpub".to_string(),
            bech32_prefix_cons_addr: "cosmosvalcons".to_string(),
            bech32_prefix_cons_pub: "cosmosvalconspub".to_string(),
        },
        currencies: vec![Currency {
            coin_denom: "ATOM".to_string(),
            coin_minimal_denom: "uatom".to_string(),
            coin_decimals: 6,
            coin_gecko_id: "cosmos".to_string(),
        }],
        fee_currencies: vec![FeeCurrency {
            coin_denom: "ATOM".to_string(),
            coin_minimal_denom: "uatom".to_string(),
            coin_decimals: 6,
            coin_gecko_id: "cosmos".to_string(),
            gas_price_step: GasPriceStep {
                low: 0.01,
                average: 0.025,
                high: 0.04,
            },
        }],
        stake_currency: Currency {
            coin_denom: "ATOM".to_string(),
            coin_minimal_denom: "uatom".to_string(),
            coin_decimals: 6,
            coin_gecko_id: "cosmos".to_string(),
        },
    };

    let chain_info_js = serde_wasm_bindgen::to_value(&chain_info).unwrap();
    let _ = suggest_chain(chain_info_js).await;
}
