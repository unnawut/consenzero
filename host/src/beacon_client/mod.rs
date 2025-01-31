use bytes::Bytes;

use ream_consensus::deneb::beacon_state::BeaconState;
use reqwest;
use reqwest::header::ACCEPT;
use serde::{Deserialize, Serialize};
use serde_json;

pub const RESPONSE_JSON: &str = "application/json";
pub const RESPONSE_SSZ: &str = "application/octet-stream";

pub struct BeaconNodeHttpClient {
    client: reqwest::Client,
    rpc_url: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BeaconStateResponse {
    version: String,
    execution_optimistic: bool,
    finalized: bool,
    data: BeaconState,
}

impl BeaconNodeHttpClient {
    pub fn new(rpc_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            rpc_url,
        }
    }

    // GET /eth/v2/debug/beacon/states/{state_id}
    pub async fn get_beacon_state_ssz(&self, slot: u64) -> Result<Bytes, String> {
        let resp = self
            .client
            .get(self.rpc_url.clone() + "/eth/v2/debug/beacon/states/" + &slot.to_string())
            .header(ACCEPT, RESPONSE_SSZ)
            .send()
            .await
            .unwrap();

        let resp_bytes = resp.bytes().await.unwrap();

        Ok(resp_bytes)
    }

    // GET /eth/v2/debug/beacon/states/{state_id}
    pub async fn get_beacon_state(&self, slot: u64) -> Result<BeaconStateResponse, reqwest::Error> {
        let resp = self
            .client
            .get(self.rpc_url.clone() + "/eth/v2/debug/beacon/states/" + &slot.to_string())
            .header(ACCEPT, RESPONSE_JSON)
            .send()
            .await
            .expect("Could not retrieve state");

        // TODO: Conversion from json to struct does not work yet as serde does not know how to
        // deserialize into BeaconState properly
        resp.json::<BeaconStateResponse>().await
    }

    // GET /eth/v1/beacon/headers/head
    pub async fn get_beacon_head_slot(&self) -> Result<u64, String> {
        Ok(
            self.get_beacon_head_block_header().await?["data"]["header"]["message"]["slot"]
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
        )
    }

    // Get /eth/v1/beacon/headers/head
    pub async fn get_beacon_head_block_header(&self) -> Result<serde_json::Value, String> {
        let resp = self
            .client
            .get(self.rpc_url.clone() + "/eth/v1/beacon/headers/head")
            .header(ACCEPT, RESPONSE_JSON)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let block_header: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(block_header)
    }

    // pub async fn get_beacon_block(
    //     &self,
    //     slot: u64,
    // ) -> Result<Option<SignedBeaconBlock<MainnetEthSpec, BlindedPayload<MainnetEthSpec>>>, String>
    // {
    //     let block = self
    //         .client
    //         .get_beacon_blinded_blocks::<MainnetEthSpec>(BlockId::Slot(slot.into()))
    //         .await
    //         .map_err(|e| format!("Failed to get beacon block: {:?}", e))?;

    //     let block = match block {
    //         Some(block) => block,
    //         None => return Ok(None),
    //     };

    //     println!("block: {:?}", block.data.state_root());

    //     Ok(Some(block.data))
    // }
}
