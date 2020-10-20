address 0x1 {

module Oracle {
    native public fun get_price<Curr1, Curr2>(): u64;
}
}
