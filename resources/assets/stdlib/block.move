address 0x1 {

module Block {

    struct BlockMetadata has store, key {
        // height of the current block
        height: u64,
    }

    // Get the current block height
    public fun get_current_block_height(): u64 acquires BlockMetadata {
        borrow_global<BlockMetadata>(@0x1).height
    }
}
}
