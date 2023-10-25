use cosmwasm_std::coins;
use osmosis_test_tube::{Module, OraichainTestApp, Wasm};
use test_tube::cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use test_tube::cosmrs::proto::cosmos::base::v1beta1::Coin;
use test_tube::{Account, Bank};

use crate::msg::{self, ValueResp};

static CONTRACT_BYTES: &[u8] = include_bytes!("../artifacts/test-cosm-wasm-contract.wasm");

#[test]
fn test_execute() {
    let app = OraichainTestApp::default();
    let owner = &app
        .init_account(&coins(5_000_000_000_000u128, "orai"))
        .unwrap();
    let bank = Bank::new(&app);
    let wasm = Wasm::new(&app);
    let code_id = wasm
        .store_code(CONTRACT_BYTES, None, owner)
        .unwrap()
        .data
        .code_id;

    let contract_addr = wasm
        .instantiate(
            code_id,
            &msg::InitMsg { counter: 1 },
            Some(&owner.address()),
            Some("contract"),
            &[],
            owner,
        )
        .unwrap()
        .data
        .address;

    let value: ValueResp = wasm
        .query(&contract_addr, &msg::QueryMsg::Value {})
        .unwrap();
    println!("value {:?}", value.value);

    let bob = &app.init_account(&[]).unwrap();

    let res = wasm
        .execute(
            &contract_addr,
            &msg::ExecMsg::DonateOrai {
                receiver: bob.address().clone(),
            },
            &vec![Coin {
                denom: "orai".to_string(),
                amount: "1000000".to_string(),
            }],
            owner,
        )
        .unwrap();

    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: bob.address(),
            denom: "orai".to_string(),
        })
        .unwrap();

    println!("gasInfo {:?} balance {:?}", res.gas_info, balance);
}
