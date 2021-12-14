use candid::{candid_method, Principal};
use ic_cdk::api;
use ic_cdk_macros::*;
use tokio::runtime::Runtime;

// The Replica returned an error: code 5, message: "Canister cav4d-eaaaa-aaaah-aaqrq-cai violated contract: "ic0_call_new" cannot be executed in init mode"
// #[init]
// #[candid_method(init)]
// async fn init(p: Principal) {
//     let _ : Result<(bool, ), _> = api::call::call(p, "set", ("from init", )).await;
// }

// The Replica returned an error: code 5, message: "Canister cav4d-eaaaa-aaaah-aaqrq-cai trapped: unreachable"
#[init]
#[candid_method(init)]
fn init(p: Principal) {
    let rt  = Runtime::new().unwrap();
    let _ : Result<(bool, ), _> = rt.block_on(api::call::call(p, "set", ("from init", )));
}

#[query]
#[candid_method(query)]
fn test() -> u64 {
    314
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
