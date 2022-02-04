//! Low level client to interact with the chain. For upstream usage, other than constructing a
//! [Client], you likely want to look at the [window](crate::window) module.

use crate::events::TfchainEvent;
pub use crate::types::Hash;
use crate::types::{AccountData, AccountInfo, BlockNumber, Contract, Farm, Node, Twin};
use runtime::Block;
pub use sp_core::crypto::AccountId32;
use std::sync::mpsc;
use std::sync::Arc;
use substrate_api_client::{
    compose_extrinsic, Api, ApiClientError, UncheckedExtrinsicV4, XtStatus,
};

pub use sp_core::crypto::Pair;
pub use substrate_api_client::sp_runtime::MultiSignature;

const BLOCK_TIME_SECONDS: i64 = 6;

pub type ApiResult<T> = Result<T, ApiClientError>;

#[derive(Clone)]
pub struct SharedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    inner: Arc<Client<P>>,
}

impl<P> SharedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(client: Client<P>) -> Self {
        Self {
            inner: Arc::new(client),
        }
    }
}

impl<P> std::ops::Deref for SharedClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    type Target = Client<P>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Client<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    inner: RawClient<P>,
}

impl<P> Client<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(url: String, signer: Option<P>) -> Client<P> {
        let mut api = Api::new(url).unwrap();
        if let Some(signer) = signer {
            api = api.set_signer(signer);
        }
        Client {
            inner: RawClient { api },
        }
    }

    pub fn create_twin(&self, ip: &str) -> ApiResult<Option<Hash>> {
        let mut res = self.inner.create_twin(ip);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.create_twin(ip);
        }

        res
    }

    pub fn get_twin_by_id(&self, id: u32) -> ApiResult<Twin> {
        let mut res = self.inner.get_twin_by_id(id);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_twin_by_id(id);
        }

        res
    }

    pub fn create_farm(&self, name: &str) -> ApiResult<Option<Hash>> {
        let mut res = self.inner.create_farm(name);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.create_farm(name);
        }

        res
    }

    pub fn get_farm_by_id(&self, id: u32, block: Option<Hash>) -> ApiResult<Option<Farm>> {
        let mut res = self.inner.get_farm_by_id(id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_farm_by_id(id, block);
        }

        res
    }

    pub fn get_farm_id_by_name(&self, name: &str) -> ApiResult<u32> {
        let mut res = self.inner.get_farm_id_by_name(name);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_farm_id_by_name(name);
        }

        res
    }

    pub fn farm_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        let mut res = self.inner.farm_count(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.farm_count(block);
        }

        res
    }

    pub fn get_account_free_balance(&self, account: &AccountId32) -> ApiResult<AccountData> {
        let mut res = self.inner.get_account_free_balance(account);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_account_free_balance(account);
        }

        res
    }

    pub fn get_node_by_id(&self, node_id: u32, block: Option<Hash>) -> ApiResult<Option<Node>> {
        let mut res = self.inner.get_node_by_id(node_id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_node_by_id(node_id, block);
        }

        res
    }

    pub fn node_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        let mut res = self.inner.node_count(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.node_count(block);
        }

        res
    }

    pub fn get_contract_by_id(
        &self,
        contract_id: u64,
        block: Option<Hash>,
    ) -> ApiResult<Option<Contract>> {
        let mut res = self.inner.get_contract_by_id(contract_id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_contract_by_id(contract_id, block);
        }

        res
    }

    pub fn contract_count(&self, block: Option<Hash>) -> ApiResult<u64> {
        let mut res = self.inner.contract_count(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.contract_count(block);
        }

        res
    }

    pub fn get_farm_payout_address(
        &self,
        farm_id: u32,
        block: Option<Hash>,
    ) -> ApiResult<Option<String>> {
        let mut res = self.inner.get_farm_payout_address(farm_id, block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_farm_payout_address(farm_id, block);
        }

        res
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> ApiResult<Option<Block>> {
        let mut res = self.inner.get_block_by_hash(block_hash);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_block_by_hash(block_hash);
        }

        res
    }

    pub fn get_block_events(&self, block: Option<Hash>) -> ApiResult<Vec<TfchainEvent>> {
        let mut res = self.inner.get_block_events(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_block_events(block);
        }

        res
    }

    pub fn block_timestamp(&self, block: Option<Hash>) -> ApiResult<i64> {
        let mut res = self.inner.block_timestamp(block);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.block_timestamp(block);
        }

        res
    }

    pub fn get_hash_at_height(&self, height: BlockNumber) -> ApiResult<Option<Hash>> {
        let mut res = self.inner.get_hash_at_height(height);
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.get_hash_at_height(height);
        }

        res
    }

    pub fn finalized_block_headers(&self) -> ApiResult<FinalizedHeadSubscription> {
        // TODO: what if subscription breaks
        let mut res = self.inner.finalized_block_headers();
        for _ in 0..5 {
            match res {
                Err(ApiClientError::Disconnected(_)) => {}
                x => return x,
            }
            res = self.inner.finalized_block_headers();
        }

        res
    }

    // Get the height just past the timestamp. i.e. `block_x_time | ts | block_x+1_time` returns
    // block x+1
    pub fn height_at_timestamp(&self, ts: i64) -> ApiResult<BlockNumber> {
        // TODO: clean these unwraps, this assumes block 1 always exists (which is the case for
        // now).
        // SAFETY: sanity check that ts is smaller than the last height.
        let latest_ts = self.block_timestamp(None)? / 1000;
        if latest_ts < ts {
            panic!(
                "can't fetch block for future timestamp {} vs latest {}",
                ts, latest_ts
            );
        }
        let mut height = 1;
        let mut last_height = 1;
        loop {
            let hash = match self.get_hash_at_height(height)? {
                Some(hash) => hash,
                // In case the network stalled we might be on a future block, try to fix that
                None => {
                    // Don't override last_height. That way we will incrementally approach
                    // last_height as we go
                    height = (height + last_height) / 2;
                    continue;
                }
            };
            // timestmap is in milliseconds
            let block_time = self.block_timestamp(Some(hash))? / 1000;
            let time_delta = ts - block_time;
            let block_delta = time_delta / BLOCK_TIME_SECONDS;
            if block_delta == 0 {
                if time_delta >= 0 {
                    // the timestamp is slightly before this block, so return the this block;
                    return Ok((height + 1) as u32);
                } else {
                    // the timestamp is slightly past this block, so return the next block;
                    return Ok(height as u32);
                }
            }
            // check that the delta is in range
            if (height as i64 + block_delta) < 0 {
                panic!(
                    "negative height search (height {} delta {})",
                    height, block_delta
                );
            }

            // adjust height
            last_height = height;
            // we can't just cast block_delta to u32 here, as that would misbehave in case delta is
            // negative
            height = (height as i64 + block_delta) as u32;
        }
    }
}

pub struct RawClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub api: Api<P>,
}

impl<P> RawClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn new(url: String, signer: P) -> RawClient<P> {
        let api = Api::new(url).unwrap().set_signer(signer);
        RawClient { api }
    }

    pub fn create_twin(&self, ip: &str) -> ApiResult<Option<Hash>> {
        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(self.api.clone(), "TfgridModule", "create_twin", ip);
        self.api.send_extrinsic(xt.hex_encode(), XtStatus::Ready)
    }

    pub fn get_twin_by_id(&self, id: u32) -> ApiResult<Twin> {
        let twin: Twin = self
            .api
            .get_storage_map("TfgridModule", "Twins", id, None)
            .unwrap()
            .or_else(|| Some(Twin::default()))
            .unwrap();

        Ok(twin)
    }

    pub fn create_farm(&self, name: &str) -> ApiResult<Option<Hash>> {
        let xt: UncheckedExtrinsicV4<_> =
            compose_extrinsic!(self.api.clone(), "TfgridModule", "create_farm", name);
        self.api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock)
    }

    pub fn get_farm_by_id(&self, id: u32, block: Option<Hash>) -> ApiResult<Option<Farm>> {
        self.api.get_storage_map("TfgridModule", "Farms", id, block)
    }

    pub fn get_farm_id_by_name(&self, name: &str) -> ApiResult<u32> {
        let farm_id: u32 = self
            .api
            .get_storage_map("TfgridModule", "FarmIdByName", name, None)
            .unwrap()
            .or_else(|| Some(0))
            .unwrap();

        Ok(farm_id)
    }

    pub fn farm_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        // Safety: farmID is initialized in genesis so this value is always set.
        self.api
            .get_storage_value("TfgridModule", "FarmID", block)
            .map(|i| i.unwrap())
    }

    pub fn get_account_free_balance(&self, account: &AccountId32) -> ApiResult<AccountData> {
        let info: AccountInfo = self
            .api
            .get_storage_map("System", "Account", account, None)?
            .or_else(|| Some(AccountInfo::default()))
            .unwrap();

        Ok(info.data)
    }

    pub fn get_node_by_id(&self, node_id: u32, block: Option<Hash>) -> ApiResult<Option<Node>> {
        self.api
            .get_storage_map("TfgridModule", "Nodes", node_id, block)
    }

    pub fn node_count(&self, block: Option<Hash>) -> ApiResult<u32> {
        // Safety: nodeID is initialized in genesis so this value is always set.
        self.api
            .get_storage_value("TfgridModule", "NodeID", block)
            .map(|i| i.unwrap())
    }

    pub fn get_contract_by_id(
        &self,
        contract_id: u64,
        block: Option<Hash>,
    ) -> ApiResult<Option<Contract>> {
        self.api
            .get_storage_map("SmartContractModule", "Contracts", contract_id, block)
    }

    pub fn contract_count(&self, block: Option<Hash>) -> ApiResult<u64> {
        // Safety: contractID is initialized in genesis so this value is always set.
        self.api
            .get_storage_value("SmartContractModule", "ContractID", block)
            .map(|i| i.unwrap_or(0))
    }

    pub fn get_farm_payout_address(
        &self,
        farm_id: u32,
        block: Option<Hash>,
    ) -> ApiResult<Option<String>> {
        self.api.get_storage_map(
            "TfgridModule",
            "FarmPayoutV2AddressByFarmID",
            farm_id,
            block,
        )
    }

    pub fn get_block_by_hash(&self, block_hash: &str) -> ApiResult<Option<Block>> {
        // TODO: Very happy path
        let mut raw_hash = [0; 32];
        hex::decode_to_slice(&block_hash[2..], &mut raw_hash).unwrap();
        let hash = Hash::from(raw_hash);
        self.api.get_block(Some(hash))
    }

    pub fn get_block_events(&self, block: Option<Hash>) -> ApiResult<Vec<TfchainEvent>> {
        let events: Vec<system::EventRecord<runtime::Event, Hash>> = self
            .api
            .get_storage_value("System", "Events", block)?
            .unwrap();

        Ok(events
            .into_iter()
            .map(|e| TfchainEvent::from(e.event))
            .collect())
    }

    pub fn block_timestamp(&self, block: Option<Hash>) -> ApiResult<i64> {
        Ok(self
            .api
            .get_storage_value("Timestamp", "Now", block)?
            .unwrap())
    }

    pub fn get_hash_at_height(&self, height: BlockNumber) -> ApiResult<Option<Hash>> {
        let req = substrate_api_client::rpc::json_req::chain_get_block_hash(Some(height));
        let resp = self.api.get_request(req.to_string())?;
        match resp {
            None => Ok(None),
            Some(hash_str) => {
                let mut raw_hash = [0; 32];
                // TODO: this could be improved
                hex::decode_to_slice(&hash_str[3..67], &mut raw_hash).unwrap();
                Ok(Some(Hash::from(raw_hash)))
            }
        }
    }

    pub fn finalized_block_headers(&self) -> ApiResult<FinalizedHeadSubscription> {
        let (heads_in, heads_out) = mpsc::channel();
        self.api.subscribe_finalized_heads(heads_in)?;

        Ok(FinalizedHeadSubscription { stream: heads_out })
    }
}

pub struct FinalizedHeadSubscription {
    stream: mpsc::Receiver<String>,
}

impl Iterator for FinalizedHeadSubscription {
    type Item = runtime::Header;

    fn next(&mut self) -> Option<Self::Item> {
        let header_str = self.stream.recv().unwrap();
        Some(serde_json::from_str(&header_str).unwrap())
    }
}
