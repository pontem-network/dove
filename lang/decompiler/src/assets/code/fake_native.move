module 0x1::M {
    use 0x1::Transaction;

    struct R<T: key + store> has key, store {
        t: T
    }

    struct R1 has key, store {}

    fun build_in_functions<T: key + store>(sender: &signer, r: R<T>, r1: R1) {
        exists<R<T>>(Transaction::sender());
        exists<R1>(Transaction::sender());
        move_to<R1>(sender, r1);
        move_to<R<T>>(sender, r);
    }

    fun mf(): R1 acquires R1 {
        move_from(@0x0)
    }

    fun mfg<T: key + store>(): R<T> acquires R {
        move_from(@0x0)
    }

    fun bg1<T: key + store>() acquires R {
        let r = borrow_global<R<T>>(@0x0);
        let t = &r.t;
        supply_ref(t);
        supply_ref(t);
    }

    fun supply_ref<T>(ref: &T) {
        supply_ref(ref);
    }

    fun bg<T: key + store>() acquires R, R1 {
        borrow_global<R1>(@0x0);
        borrow_global<R<T>>(@0x0);
    }

    fun bgm<T: key + store>() acquires R, R1 {
        borrow_global_mut<R1>(@0x0);
        borrow_global_mut<R<T>>(@0x0);
    }
}