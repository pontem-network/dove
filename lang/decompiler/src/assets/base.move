module 0x1::Base {
    struct Test has key, store {
        val: u64,
    }

    fun test() {
    }

    public fun code(): u64 {
        1
    }
}