# test_ignore

```sh

dfx canister --no-wallet --network ic call test1 get
("")
dfx canister --no-wallet --network ic call test2 get
("")

dfx canister --no-wallet --network ic call test1 call '("from test1", principal "othsc-viaaa-aaaah-aa5ta-cai")'
(true)

dfx canister --no-wallet --network ic call test1 get
("")

dfx canister --no-wallet --network ic call test2 get
("from test1")



```