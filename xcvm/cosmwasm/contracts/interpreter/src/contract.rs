#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
<<<<<<< HEAD
    entry_point, to_binary, wasm_execute, DepsMut, Env, MessageInfo, QueryRequest, Response,
    StdError, WasmQuery,
=======
	to_binary, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, Response, StdError, WasmQuery,
>>>>>>> 6f03544a0f (feat(cosmwasm): router contract)
};
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, XCVMProgram};
use crate::state::{Config, CONFIG};
use cw20::{BalanceResponse, Cw20Contract, Cw20ExecuteMsg, Cw20QueryMsg};
use num::Zero;
use xcvm_asset_registry::msg::{GetAssetContractResponse, QueryMsg as AssetRegistryQueryMsg};
use xcvm_core::{Funds, Instruction, NetworkId};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
<<<<<<< HEAD
    let registry_address = deps.api.addr_validate(&msg.registry_address)?;

    let config = Config { registry_address };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
=======
	let registry_address = deps.api.addr_validate(&msg.registry_address)?;
	let config = Config { registry_address };
	CONFIG.save(deps.storage, &config)?;

	Ok(Response::new().set_data(to_binary(&(msg.network_id.0, msg.user_id))?))
>>>>>>> 6f03544a0f (feat(cosmwasm): router contract)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Execute { program } => interpret_program(deps, env, info, program),
    }
}

pub fn interpret_program(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    program: XCVMProgram,
) -> Result<Response, ContractError> {
    let mut response = Response::new();

    for instruction in program.instructions {
        response = match instruction {
            Instruction::Call { encoded } => interpret_call(encoded, response),
            Instruction::Spawn {
                network,
                salt,
                assets,
                program,
            } => interpret_spawn(network, salt, assets, program, response),
            Instruction::Transfer { to, assets } => {
                interpret_transfer(&mut deps, to, assets, response)
            }
        }?;
    }
    Ok(response)
}

pub fn interpret_call(encoded: Vec<u8>, response: Response) -> Result<Response, ContractError> {
    #[derive(Deserialize)]
    struct Payload {
        address: String,
        msg: String,
    }

    let payload: Payload =
        serde_json_wasm::from_slice(&encoded).map_err(|_| ContractError::InvalidCallPayload)?;
    let msg = wasm_execute(payload.address, &payload.msg, vec![])?;

    Ok(response.add_message(msg))
}

pub fn interpret_spawn(
	network: NetworkId,
	salt: Vec<u8>,
	assets: Funds,
	program: XCVMProgram,
	response: Response,
) -> Result<Response, ContractError> {
	#[derive(Serialize)]
	struct SpawnEvent {
		network: NetworkId,
		salt: Vec<u8>,
		assets: Funds,
		program: XCVMProgram,
	}

    let data = SpawnEvent {
        network,
        salt,
        assets,
        program,
    };

    Ok(response.add_attribute(
        "spawn",
        serde_json_wasm::to_string(&data).map_err(|_| ContractError::DataSerializationError)?,
    ))
}

pub fn interpret_transfer(
    deps: &mut DepsMut,
    to: String,
    assets: Funds,
    mut response: Response,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let registry_addr = config.registry_address.into_string();

    for (asset_id, amount) in assets.0 {
        let query_msg = AssetRegistryQueryMsg::GetAssetContract(asset_id.into());

        let cw20_address: GetAssetContractResponse = deps.querier.query(
            &WasmQuery::Smart {
                contract_addr: registry_addr.clone(),
                msg: to_binary(&query_msg)?,
            }
            .into(),
        )?;
        let contract = Cw20Contract(cw20_address.addr.clone());

		if amount.is_zero() {
			return Err(ContractError::ZeroTransferAmount)
		}

		let transfer_amount = {
			if amount.slope.0 == 0 {
				amount.intercept.0
			} else {
				let query_msg = Cw20QueryMsg::Balance { address: to.clone() };
				let response: BalanceResponse =
					deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
						contract_addr: cw20_address.addr.clone().into_string(),
						msg: to_binary(&query_msg)?,
					}))?;
				amount.apply(response.balance.into())
			}
		};

        response = response.add_message(contract.call(Cw20ExecuteMsg::Transfer {
            recipient: to.clone(),
            amount: transfer_amount.into(),
        })?);
    }

	Ok(response)
}

#[cfg(test)]
mod tests {
	use std::collections::BTreeMap;

	use crate::msg::XCVMInstruction;

	use super::*;
	use cosmwasm_std::{
		testing::{mock_dependencies, mock_env, mock_info, MockQuerier},
		wasm_execute, Addr, Attribute, ContractResult, QuerierResult,
	};
	use xcvm_core::Picasso;

	#[test]
	fn proper_instantiation() {
		let mut deps = mock_dependencies();

		let msg = InstantiateMsg {
			registry_address: "addr".to_string(),
			network_id: Picasso.into(),
			user_id: vec![],
		};
		let info = mock_info("sender", &vec![]);

		let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
		assert_eq!(0, res.messages.len());

		// Make sure that the storage is empty
		assert_eq!(
			CONFIG.load(&deps.storage).unwrap(),
			Config { registry_address: Addr::unchecked("addr") }
		);
	}

	fn wasm_querier(_: &WasmQuery) -> QuerierResult {
		Ok(ContractResult::Ok(
			to_binary(&xcvm_asset_registry::msg::GetAssetContractResponse {
				addr: Addr::unchecked("mock"),
			})
			.unwrap(),
		))
		.into()
	}

	#[test]
	fn execute_transfer() {
		let mut deps = mock_dependencies();
		let mut querier = MockQuerier::default();
		querier.update_wasm(wasm_querier);
		deps.querier = querier;

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: "addr".into(),
				network_id: Picasso.into(),
				user_id: vec![],
			},
		)
		.unwrap();

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Transfer {
				to: "asset".into(),
				assets: Funds::from([(1, 1_u128)]),
			}]
			.into(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		let contract = Cw20Contract(Addr::unchecked("mock"));
		let msg = contract
			.call(Cw20ExecuteMsg::Transfer { recipient: "asset".into(), amount: 1_u128.into() })
			.unwrap();

		assert_eq!(res.messages[0].msg, msg);
	}

	#[test]
	fn execute_call() {
		let mut deps = mock_dependencies();

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: "addr".into(),
				network_id: Picasso.into(),
				user_id: vec![],
			},
		)
		.unwrap();

		let cosmos_msg: CosmosMsg =
			wasm_execute("1234", &"hello world".to_string(), vec![]).unwrap().into();
		

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Call { encoded: serde_json_wasm::to_vec(&cosmos_msg).unwrap().into() }].into(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		assert_eq!(res.messages[0].msg, cosmos_msg);
	}

	#[test]
	fn execute_spawn() {
		let mut deps = mock_dependencies();

		let info = mock_info("sender", &vec![]);
		let _ = instantiate(
			deps.as_mut(),
			mock_env(),
			info.clone(),
			InstantiateMsg {
				registry_address: "addr".into(),
				network_id: Picasso.into(),
				user_id: vec![],
			},
		)
		.unwrap();

		let program = XCVMProgram {
			tag: vec![],
			instructions: vec![XCVMInstruction::Spawn {
				network: Picasso.into(),
				salt: vec![],
				assets: Funds(BTreeMap::new()),
				program: XCVMProgram {
					tag: vec![],
					instructions: vec![XCVMInstruction::Call { encoded: vec![] }].into(),
				},
			}]
			.into(),
		};

		let res = execute(deps.as_mut(), mock_env(), info.clone(), ExecuteMsg::Execute { program })
			.unwrap();
		assert_eq!(res.attributes[0], Attribute { key: "spawn".to_string(), value: r#"{"network":1,"salt":[],"assets":{},"program":{"tag":[],"instructions":[{"call":{"encoded":[]}}]}}"#.to_string() });
	}
>>>>>>> 2486c18e56 (fix(xcvm): assets in xcvm core)
}