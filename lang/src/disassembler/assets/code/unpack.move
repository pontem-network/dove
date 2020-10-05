module M {
    struct R {
        f: u64,
        g : bool
    }

    struct R1<T> {
        t: T
    }

    public fun t(r: R): (u64, bool) {
        let R {f:f, g:g} = r;
        (f, g)
    }

    public fun t0(r: R): u64 {
        let R {f:f, g:_} = r;
        f
    }

    public fun t1<T>(r: R1<T>): T {
        let R1<T> {t: t} = r;
        t
    }
}