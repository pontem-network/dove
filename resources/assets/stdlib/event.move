address 0x1 {

module Event {

    // Native procedure that writes to the actual event stream in Event store
    // This will replace the "native" portion of EmitEvent bytecode
    native public fun emit<T: copyable>(account: &signer, msg: T);
}
}
