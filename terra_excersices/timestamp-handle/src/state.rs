use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};


// #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
// pub struct BalanceAndTime {
//     pub balance: Uint128,
//     /// cap is how many more tokens can be issued by the minter
//     pub timestamp: String,
// }



// pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const TimeStamp: Map<&Addr, String> = Map::new("TimeStamp");