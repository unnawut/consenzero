use risc0_zkvm::guest::env;
mod beacon_chain;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let output: u32 = beacon_chain::state_transition::test_passing_slot(input);

    // write public output to the journal
    env::commit(&output);
}
