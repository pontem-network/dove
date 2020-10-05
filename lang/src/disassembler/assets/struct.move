module M {
    struct T {g: u64}

    struct Vec<G> {g: vector<G>, t: vector<T> }

    resource struct R { f: u64, g: u64, }

    native struct BarNative<K, V>;

    resource struct Pool<AssetType: copyable> {
        t: AssetType,
    }

    resource struct Pool1<AssetType: resource> {
        t: AssetType,
    }

    struct Bar<K: copyable, V> {
        key: K,
        value: V,
    }

    struct G {t: T}

    resource struct B { t: 0x00000000000000000000000000000001::Base::Test }
}