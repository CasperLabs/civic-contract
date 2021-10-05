# KYC Contract

This repository contains the general KYC Contract written based on Casper NFT Protocol - [CEP47](https://github.com/casper-ecosystem/casper-nft-cep47)
The KYC contract was designed to provide a mechanism on issuing KYC verification on accounts interacting with the Auction mechanics (see [Auction contract]()) based on KYC / Identity Verification standards operated by Civic https://www.civic.com/  

## Contract Data Model
| Property | Object | CLType | Description |
| --- | --- | --- | --- |
| admins | Named key | Dict(PublicKey, ()) | Admins that grant/revoke gatekeepers |
| gatekeepers | Named key | Dict(PublicKey, ()) | Gatekeepers that mint/burn/update a KYC token |

## Endpoints
The KYC contract derives default endpoints of CEP47 standard and have some additional endpoints.
They can be grouped into following topics:

### Metadata
| Name | CLType | Description |
| --- | --- | --- |
| name | String | Global name of the contract |
| symbol | String | Global symbol of the contract |
| meta | Dict(String, String) | Global metadata of the contract |
| total_supply | U256 | Total amount of tokens generated |
| balance_of | U256 | Amount of tokens that a user owns |
| owner_of | PublicKey | Key of the token owner |
| get_token_by_index | String | Id of the indexed token that a user owns |
| token_meta | Dict(String, String) | Metadata of each token |
| is_kyc_proved | Bool | Whether an account is kyc'd or not |

### Token Control
| Name | Description |
| --- | --- |
| mint | Mint a new token to the provided account (Only gatekeepers) |
| burn | Burn an existing token from the provided account (Only gatekeepers/admins) |
| transfer_from | Transfer a token from a user to another one. (Only admins) |
| update_token_meta | Update metadata of an existing token. (Only gatekeepers/admins) |

### Access Management
| Name | Description |
| --- | --- |
| grant_gatekeeper | Grant the gatekeeper role to the provided account. (Only admins) |
| revoke_gatekeeper | Revoke the gatekeeper role from the provided account. (Only admins) |
| grant_admin | Grant the admin role to the provided account. (Only admins) |
| revoke_admin | Revoke the admin role from the provided account. (Only admins) |

## Install
Make sure the `wasm32-unknown-unknown` target is installed.
```
make prepare
```

### Build the Contract
```
make build-contracts
```

### Test
```
make test
```
