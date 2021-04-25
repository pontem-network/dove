module M {
    struct T {g: u64}

    struct Vec<G> {g: vector<G>, t: vector<T> }

    struct R has key, store { f: u64, g: u64, }

    native struct BarNative<K, V>;

    struct Pool<AssetType> has key, store{
        t: AssetType,
    }

    struct Pool1<AssetType: key + store> has key, store {
        t: AssetType,
    }

    struct Bar<K: drop + copy, V> {
        key: K,
        value: V,
    }

    struct G {t: T}

    struct B has key, store { t: 0x00000000000000000000000000000001::Base::Test }
}