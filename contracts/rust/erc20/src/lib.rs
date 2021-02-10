// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

use erc20::*;
use schema::*;
use wasmlib::*;

mod erc20;
mod schema;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_func(FUNC_APPROVE, func_approve_thunk);
    exports.add_func(FUNC_INIT, func_init_thunk);
    exports.add_func(FUNC_TRANSFER, func_transfer_thunk);
    exports.add_func(FUNC_TRANSFER_FROM, func_transfer_from_thunk);
    exports.add_view(VIEW_ALLOWANCE, view_allowance_thunk);
    exports.add_view(VIEW_BALANCE_OF, view_balance_of_thunk);
    exports.add_view(VIEW_TOTAL_SUPPLY, view_total_supply_thunk);
}

//@formatter:off
pub struct FuncApproveParams {
    pub amount:     ScImmutableInt,     // allowance value for delegated account
    pub delegation: ScImmutableAgentId, // delegated account
}
//@formatter:on

fn func_approve_thunk(ctx: &ScFuncContext) {
    let p = ctx.params();
    let params = FuncApproveParams {
        amount: p.get_int(PARAM_AMOUNT),
        delegation: p.get_agent_id(PARAM_DELEGATION),
    };
    ctx.require(params.amount.exists(), "missing mandatory amount");
    ctx.require(params.delegation.exists(), "missing mandatory delegation");
    func_approve(ctx, &params);
}

//@formatter:off
pub struct FuncInitParams {
    pub creator: ScImmutableAgentId, // creator/owner of the initial supply
    pub supply:  ScImmutableInt,     // initial token supply
}
//@formatter:on

fn func_init_thunk(ctx: &ScFuncContext) {
    let p = ctx.params();
    let params = FuncInitParams {
        creator: p.get_agent_id(PARAM_CREATOR),
        supply: p.get_int(PARAM_SUPPLY),
    };
    ctx.require(params.creator.exists(), "missing mandatory creator");
    ctx.require(params.supply.exists(), "missing mandatory supply");
    func_init(ctx, &params);
}

//@formatter:off
pub struct FuncTransferParams {
    pub account: ScImmutableAgentId, // target account
    pub amount:  ScImmutableInt,     // amount of tokens to transfer
}
//@formatter:on

fn func_transfer_thunk(ctx: &ScFuncContext) {
    let p = ctx.params();
    let params = FuncTransferParams {
        account: p.get_agent_id(PARAM_ACCOUNT),
        amount: p.get_int(PARAM_AMOUNT),
    };
    ctx.require(params.account.exists(), "missing mandatory account");
    ctx.require(params.amount.exists(), "missing mandatory amount");
    func_transfer(ctx, &params);
}

//@formatter:off
pub struct FuncTransferFromParams {
    pub account:   ScImmutableAgentId, // sender account
    pub amount:    ScImmutableInt,     // amount of tokens to transfer
    pub recipient: ScImmutableAgentId, // recipient account
}
//@formatter:on

fn func_transfer_from_thunk(ctx: &ScFuncContext) {
    let p = ctx.params();
    let params = FuncTransferFromParams {
        account: p.get_agent_id(PARAM_ACCOUNT),
        amount: p.get_int(PARAM_AMOUNT),
        recipient: p.get_agent_id(PARAM_RECIPIENT),
    };
    ctx.require(params.account.exists(), "missing mandatory account");
    ctx.require(params.amount.exists(), "missing mandatory amount");
    ctx.require(params.recipient.exists(), "missing mandatory recipient");
    func_transfer_from(ctx, &params);
}

//@formatter:off
pub struct ViewAllowanceParams {
    pub account:    ScImmutableAgentId, // sender account
    pub delegation: ScImmutableAgentId, // delegated account
}
//@formatter:on

fn view_allowance_thunk(ctx: &ScViewContext) {
    let p = ctx.params();
    let params = ViewAllowanceParams {
        account: p.get_agent_id(PARAM_ACCOUNT),
        delegation: p.get_agent_id(PARAM_DELEGATION),
    };
    ctx.require(params.account.exists(), "missing mandatory account");
    ctx.require(params.delegation.exists(), "missing mandatory delegation");
    view_allowance(ctx, &params);
}

pub struct ViewBalanceOfParams {
    pub account: ScImmutableAgentId, // sender account
}

fn view_balance_of_thunk(ctx: &ScViewContext) {
    let p = ctx.params();
    let params = ViewBalanceOfParams {
        account: p.get_agent_id(PARAM_ACCOUNT),
    };
    ctx.require(params.account.exists(), "missing mandatory account");
    view_balance_of(ctx, &params);
}

pub struct ViewTotalSupplyParams {}

fn view_total_supply_thunk(ctx: &ScViewContext) {
    let params = ViewTotalSupplyParams {};
    view_total_supply(ctx, &params);
}