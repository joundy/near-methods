use reqwest::header;
use serde_json::{json, Value};
use wasmparser::{Parser, Payload, ExternalKind};
use std::env;

#[derive(Debug)]
pub enum ErrorType {
    FailedToSend,
    FailedToParseResponse,
    FailedToParseJson,
    CodeNotFound,
    ErrorDecodeWasmCode,
    ErrorIterAst,
    ErrorParseExportSection
}

#[derive(Debug)]
pub enum NetwrokId {
    Mainnet,
    Testnet
}

pub async fn get_contract_code(contract_id: &String, network_id: NetwrokId) -> Result<String, ErrorType>{
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let request_body = json!(
        {
            "jsonrpc": "2.0",
            "id": "dontcare",
            "method": "query",
            "params": {
                "request_type": "view_code",
                "finality": "final",
                "account_id": contract_id,
            }
        }
    )
    .to_string();

    let rpc_url = match network_id {
        NetwrokId::Mainnet => "https://rpc.mainnet.near.org",
        NetwrokId::Testnet => "https://rpc.testnet.near.org"
    };

    let send_request = reqwest::Client::new()
    .post(rpc_url)
    .headers(headers)
    .body(request_body)
    .send()
    .await;

    let response_value: Value = match send_request {
        Ok(response) => {
            let response_body = response.text().await;
            match response_body {
                Ok(body) => {
                    let response_json = serde_json::from_str(&body);
                    match response_json {
                        Ok(value) => value,
                        Err(_) => return Err(ErrorType::FailedToParseJson),
                    }
                },
                Err(_) => return Err(ErrorType::FailedToParseResponse),
            }
        },
        Err(_) => return Err(ErrorType::FailedToSend)
    };

    let response_data = &response_value["result"]["code_base64"];
    match response_data {
        Value::String(code) => return Ok(code.to_string()),
        _ => return Err(ErrorType::CodeNotFound)
    }
}

pub fn parse_wasm_to_methods(code: &String) -> Result<Vec<String>, ErrorType>{
    let mut methods = vec![];

    let wasm_code = match base64::decode(&code) {
        Ok(code) => code,
        Err(_) => return Err(ErrorType::ErrorDecodeWasmCode)
    };
    let ast = Parser::new(0).parse_all(&wasm_code);

    for payload in ast {
        let payload = match payload {
            Ok(payload) => payload,
            Err(_) => return Err(ErrorType::ErrorIterAst)
        };

        match payload {
            Payload::ExportSection(es) => {
                for export in es {
                    let export = match export {
                        Ok(export) => export,
                        Err(_) => return Err(ErrorType::ErrorParseExportSection)
                    };
                    if export.kind != ExternalKind::Func{
                        continue;
                    }

                    methods.push(export.name.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(methods)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        panic!("errors.network_id & contract_id is required: example: near_methods mainnet x.paras.id");
    }

    let network_id = match args[1].as_str() {
        "testnet" => NetwrokId::Testnet,
        "mainnet" => NetwrokId::Mainnet,
        _ => panic!("errors.network_id is not valid (mainnet|testnet)")
    };
    let contract_id = &args[2];

    let methods = get_contract_code(&contract_id, network_id)
    .await
    .and_then(|w| parse_wasm_to_methods(&w));

    match methods {
        Err(e) => {
            match e {
                ErrorType::FailedToSend => println!("errors.decoding wasm code"),
                ErrorType::FailedToParseResponse => println!("errors.failed to parse response from rpc"),
                ErrorType::FailedToParseJson => println!("errors.failed to parsee response to json"),
                ErrorType::CodeNotFound => println!("errors.code not found from {}", contract_id),
                ErrorType::ErrorDecodeWasmCode => println!("errors.decoding wasm to ast"),
                ErrorType::ErrorIterAst => println!("errors.iterating ast"),
                ErrorType::ErrorParseExportSection => println!("errors.parse exports section"),
            }
        },
        Ok(m) => {
            for method in m {
                println!("{}", method);
            }
        } 
    }
}
