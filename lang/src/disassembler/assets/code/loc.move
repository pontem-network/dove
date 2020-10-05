module M {
    resource struct R { f: u64, g: u64}

    struct R1<T> {
        t: T,
        r: u8
    }

    public fun t0(r: &R): &u64 {
        &r.f
    }

    public fun t1(r: &mut R): &u64 {
        &mut r.f
    }

    public fun t2(r: &mut R): &mut u64 {
        &mut r.f
    }

    public fun t00<T>(r: &R1<T>): &T {
        &r.t
    }

    public fun t01<T>(r: &mut R1<T>): &T {
        &mut r.t
    }

    public fun t02<T>(r: &mut R1<T>): &mut T {
        &mut r.t
    }

    fun t11() {
        let x: u64;
        let ref_x: &u64;

        x = 5;
        ref_x = &x;

        let  _ = *ref_x;
    }

    fun t11_mut() {
        let x: u64;
        let ref_x: &mut u64;

        x = 5;
        ref_x = &mut x;

        let  _ = *ref_x;
    }

    struct T {
        x: u64
    }

    struct S {
        y: u64
    }

    fun t22(cond: bool): (T, S) {
        let a = T {x: 3};
        let b = S {y: 4};
        let a_ref = &mut a;
        let b_ref = &mut b;
        let x_ref = if (cond) { &mut a_ref.x } else { &mut b_ref.y };

        if (cond) {
          *x_ref = 2;
        } else {
          *x_ref = 0;
        };

        (a, b)
    }
}