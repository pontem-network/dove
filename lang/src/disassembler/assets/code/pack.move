module M {
    struct R {
        f: u64,
        g : bool
    }

    struct R1<T: resource> {
        t: T,
    }

    public fun t(): R {
      let _r: R;
      R {f: 0, g: true}
    }

    public fun t1(): R {
       R {f: 0, g: true}
    }
    // todo
    //public fun t2(): R {
    //   let r = R {f: 0, g: true};
    //   r
    //}

    public fun t3<T: resource>(t: T): R1<T> {
        R1<T> {t: t}
    }
}