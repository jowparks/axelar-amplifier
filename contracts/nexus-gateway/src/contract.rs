#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::nexus;
use crate::state::{Config, GatewayStore, Store};

mod execute;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, axelar_wasm_std::ContractError> {
    let nexus = deps.api.addr_validate(&msg.nexus)?;
    let router = deps.api.addr_validate(&msg.router)?;

    GatewayStore::new(deps.storage)
        .save_config(Config { nexus, router })
        .expect("config must be saved");

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<nexus::Message>, axelar_wasm_std::ContractError> {
    let store = GatewayStore::new(deps.storage);
    let config = store.load_config().expect("config must be loaded");
    let contract = Contract { store, config };

    let res = match msg {
        ExecuteMsg::VerifyMessages(_) => unimplemented!(),
        ExecuteMsg::RouteMessages(msgs) => contract.route_messages(info.sender, msgs)?,
    };

    Ok(res)
}

struct Contract<S>
where
    S: Store,
{
    store: S,
    config: Config,
}