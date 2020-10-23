address 0x1 {
module Time {
    // A singleton resource holding the current Unix time in seconds
    resource struct CurrentTimestamp {
        seconds: u64,
    }

    // Get the timestamp representing `now` in seconds.
    public fun now(): u64 acquires CurrentTimestamp {
        borrow_global<CurrentTimestamp>(0x1).seconds
    }

    // Helper function to determine if the blockchain is at genesis state.
    public fun is_genesis(): bool {
        !exists<Self::CurrentTimestamp>(0x1)
    }
}
}
