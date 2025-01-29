use risc0_zkvm::guest::env;
use ream_consensus::deneb::beacon_state::BeaconState;

mod beacon_chain;

fn main() {
    // TODO: Implement your guest code here

    // Read the input
    let pre_state: BeaconState = env::read();

    // Try transition the state
    let output: BeaconState = beacon_chain::state_transition::state_transition(&pre_state);

    // Write public output to the journal
    env::commit(&output);
}
