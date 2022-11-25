
# Near Methods
‘Near methods’ is a simple tool to get all of the methods from the near contract, this can be achieved by parsing near wasm from RPC to AST.

### Install
```
cargo install --git https://github.com/joundy/near-methods.git
```
### How to use
```
near-methods mainnet|testnet x.paras.near
```
### Example
```
➜  near-methods git:(master) near-methods mainnet x.paras.near
new_default_meta
new
migrate
set_transaction_fee
calculate_new_market_data_transaction_fee
calculate_current_transaction_fee
get_transaction_fee
get_market_data_transaction_fee
set_treasury
nft_create_series
nft_buy
nft_mint
nft_mint_and_approve
nft_decrease_series_copies
nft_set_series_price
nft_burn
nft_get_series_single
nft_get_series_format
nft_get_series_price
nft_get_series
nft_supply_for_series
nft_tokens_by_series
nft_token
nft_transfer
nft_transfer_call
nft_total_supply
nft_tokens
nft_supply_for_owner
nft_tokens_for_owner
nft_payout
nft_transfer_payout
get_owner
nft_approve
nft_revoke
nft_revoke_all
nft_is_approved
nft_metadata
nft_resolve_transfer

```

