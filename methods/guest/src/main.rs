use risc0_zkvm::guest::env;
use beam_types::BeamState;

mod beacon_chain;

fn main() {
    // TODO: Implement your guest code here

    // Read the input
    let pre_state: BeamState = env::read();

    // TODO: Transition the state
    let post_state = pre_state;

    // Write public output to the journal
    env::commit(&post_state);
}
