address 0x0 {

module Oracle {
    native public fun get_price<Curr1, Curr2>(): u64;
}
}
