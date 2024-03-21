use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Response, StdResult};
use cw_storage_plus::Item;

use sylvia::contract;
use sylvia::types::{ExecCtx, InstantiateCtx, QueryCtx};

pub struct CounterContract {
    pub count: Item<'static, u64>,
}

#[cfg_attr(not(feature = "library"), sylvia::entry_points)]
#[contract]
impl CounterContract {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
        }
    }

    #[msg(instantiate)]
    fn instantiate(&self, ctx: InstantiateCtx) -> StdResult<Response> {
        self.count.save(ctx.deps.storage, &0)?;
        Ok(Response::new())
    }

    #[msg(exec)]
    fn increment(&self, ctx: ExecCtx) -> StdResult<Response> {
        self.count
            .update(ctx.deps.storage, |count| -> StdResult<u64> {
                Ok(count + 1)
            })?;
        Ok(Response::new())
    }

    #[msg(query)]
    fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
        let count = self.count.load(ctx.deps.storage)?;
        Ok(CountResponse { count })
    }
}

#[cw_serde]
pub struct CountResponse {
    pub count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    // Unit tests don't have to use a testing framework for simple things.
    //
    // For more complex tests (particularly involving cross-contract calls), you
    // may want to check out `cw-multi-test`:
    // https://github.com/CosmWasm/cw-multi-test

    #[test]
    fn init() {
        let contract = CounterContract::new();
        let mut deps = mock_dependencies();
        let ctx = InstantiateCtx::from((deps.as_mut(), mock_env(), mock_info("alice", &[])));
        contract.instantiate(ctx).unwrap();

        // We're inspecting the raw storage here, which is fine in unit tests. In
        // integration tests, you should not inspect the internal state like this,
        // but observe the external results.
        assert_eq!(0, contract.count.load(deps.as_ref().storage).unwrap());
    }

    #[test]
    fn query() {
        let contract = CounterContract::new();
        let mut deps = mock_dependencies();
        let ctx = InstantiateCtx::from((deps.as_mut(), mock_env(), mock_info("alice", &[])));
        contract.instantiate(ctx).unwrap();

        let ctx = QueryCtx::from((deps.as_ref(), mock_env()));
        let res = contract.count(ctx).unwrap();
        assert_eq!(0, res.count);
    }

    #[test]
    fn inc() {
        let contract = CounterContract::new();
        let mut deps = mock_dependencies();
        let ctx = InstantiateCtx::from((deps.as_mut(), mock_env(), mock_info("alice", &[])));
        contract.instantiate(ctx).unwrap();

        let ctx = ExecCtx::from((deps.as_mut(), mock_env(), mock_info("alice", &[])));
        contract.increment(ctx).unwrap();

        let ctx = QueryCtx::from((deps.as_ref(), mock_env()));
        let res = contract.count(ctx).unwrap();
        assert_eq!(1, res.count);
    }
}
