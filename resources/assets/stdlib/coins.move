address 0x1 {

/// Enum-like module to make generic type-matching possible, every coin which is
/// officially supported by blockchain (or peg-zone specifically) is added here.
/// Ideally this module should be auto-generated and rarely updated via consensus
module Coins {
    struct ETH has store, key {}
    struct BTC has store, key {}
    struct USDT has store, key {}

    struct Price<Curr1: key + store, Curr2: key + store> has key, store {
        value: u128
    }

    public fun get_price<Curr1: key + store, Curr2: key + store>(): u128 acquires Price {
        borrow_global<Price<Curr1, Curr2>>(0x1).value
    }

    public fun increment_price<Curr1: key + store, Curr2: key + store>(inc: u128) acquires Price {
        let price = borrow_global_mut<Price<Curr1, Curr2>>(0x1);
        price.value = price.value + inc;
    }

    public fun has_price<Curr1: key + store, Curr2: key + store>(): bool {
        exists<Price<Curr1, Curr2>>(0x1)
    }
}
}
