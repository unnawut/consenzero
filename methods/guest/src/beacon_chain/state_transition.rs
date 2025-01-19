// Beacon chain state transition function
// Ref: https://ethereum.github.io/consensus-specs/specs/phase0/beacon-chain/#beacon-chain-state-transition-function

// The post-state corresponding to a pre-state state and a signed block signed_block is defined
// as state_transition(state, signed_block). State transitions that trigger an unhandled exception
// (e.g. a failed assert or an out-of-range list access) are considered invalid. State transitions
// that cause a uint64 overflow or underflow are also considered invalid.

// def state_transition(state: BeaconState, signed_block: SignedBeaconBlock, validate_result: bool=True) -> None:
//     block = signed_block.message

//     # Process slots (including those with no blocks) since block
//     process_slots(state, block.slot)

//     # Verify signature
//     if validate_result:
//         assert verify_block_signature(state, signed_block)

//     # Process block
//     process_block(state, block)

//     # Verify state root
//     if validate_result:
//         assert block.state_root == hash_tree_root(state)

use ream_consensus::deneb::beacon_block::SignedBeaconBlock;
use ream_consensus::deneb::beacon_state::BeaconState;

pub fn _state_transition(state: &BeaconState, _signed_block: &SignedBeaconBlock) -> BeaconState{
    state.clone()
}

pub fn test_passing_slot(slot_number: u32) -> u32 {
    slot_number + 1
}