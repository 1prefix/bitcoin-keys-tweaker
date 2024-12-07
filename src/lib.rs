mod input;
mod result;

use base58::{FromBase58, ToBase58};
use input::InputComponent;
use libsecp256k1::SecretKey;
use result::ResultComponent;
use sha2::{Digest, Sha256};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let secret_key_a = use_state(|| None);
    let secret_key_b = use_state(|| None);
    let button_is_disabled = use_state(|| true);
    let result_wif = use_state(|| None);

    let on_secret_key_a_change = {
        let secret_key_clone = secret_key_a.clone();
        Callback::from(move |secret_key: Option<String>| {
            secret_key_clone.set(secret_key);
        })
    };

    let on_secret_key_b_change = {
        let secret_key_clone = secret_key_b.clone();
        Callback::from(move |secret_key: Option<String>| {
            secret_key_clone.set(secret_key);
        })
    };

    {
        let secret_key_a_clone = secret_key_a.clone();
        let secret_key_b_clone = secret_key_b.clone();
        let button_is_disabled_clone = button_is_disabled.clone();

        use_effect_with([secret_key_a.clone(), secret_key_b.clone()], move |_| {
            let both_keys_are_valid =
                (*secret_key_a_clone).clone().is_some() && (*secret_key_b_clone).clone().is_some();
            let _ = button_is_disabled_clone.set(!both_keys_are_valid);
        })
    }

    {
        let secret_key_a_clone = secret_key_a.clone();
        let secret_key_b_clone = secret_key_b.clone();
        let button_is_disabled_clone = button_is_disabled.clone();
        let result_wif_clone = result_wif.clone();

        use_effect_with(
            button_is_disabled.clone(),
            move |_| match (*button_is_disabled_clone).clone() {
                true => {
                    let _ = result_wif_clone.set(None);
                }
                false => {
                    let secter_key_a = match (*secret_key_a_clone).clone() {
                        Some(secret_key) => parse_secret_key(secret_key),
                        None => None,
                    };

                    let secter_key_b = match (*secret_key_b_clone).clone() {
                        Some(secret_key) => parse_secret_key(secret_key),
                        None => None,
                    };

                    if let Some(secret_key_a) = secter_key_a {
                        if let Some(secret_key_b) = secter_key_b {
                            let mut result_key = secret_key_a;
                            let _ = result_key.tweak_add_assign(&secret_key_b);
                            let result_wif = secret_key_to_wif(&result_key);

                            let _ = result_wif_clone.set(Some(result_wif));
                        }
                    }
                }
            },
        )
    }

    html! {
        <div id="wrapper">
            <nav>
                <a href="https://github.com/1prefix/bitcoin-keys-tweaker" target="_blank">{"Github Repo"}</a>
            </nav>

            <h1>{"Bitcoin Keys Tweaker"}</h1>

            <p>{"Please enter valid WIF (Wallet Import Format) secret keys."}</p>

            <InputComponent label={String::from("Your Secret Key")} on_secret_key_change={on_secret_key_a_change} />
            <InputComponent label={String::from("Calculated Secret Key")} on_secret_key_change={on_secret_key_b_change} />

            {match (*result_wif).clone() {
                Some(wif) => html!(<ResultComponent secret_key={wif} />),
                None => html!()
            }}
        </div>
    }
}

pub fn parse_secret_key(secret_key: String) -> Option<SecretKey> {
    // Decode from Base58
    let decoded = secret_key.from_base58().ok()?;

    // Take 32 bytes of the Secret Key
    let secret_key_bytes: Vec<u8> = decoded
        .iter()
        .skip(1)
        .take(32)
        .map(|e| e.to_owned())
        .collect();

    // Parse decoded bytes into SecretKey
    SecretKey::parse_slice(secret_key_bytes.as_slice()).ok()
}

fn secret_key_to_wif(secret_key: &SecretKey) -> String {
    let mut payload = Vec::new();

    // Add the prefix (0x80 for mainnet, 0xef for testnet)
    payload.push(0x80);

    // Add the private key bytes
    payload.extend_from_slice(&secret_key.serialize());

    // Add the compression flag if compressed
    payload.push(0x01);

    // Calculate the checksum (first 4 bytes of double SHA-256)
    let checksum = {
        let hash1 = Sha256::digest(&payload);
        let hash2 = Sha256::digest(hash1);
        hash2[..4].to_vec()
    };

    // Append the checksum to the payload
    payload.extend_from_slice(&checksum);

    // Encode the result in Base58
    payload.to_base58()
}
