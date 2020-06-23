address 0x2 {
    module Record {
        use 0x1::Signer;

        resource struct T {
            age: u8,
        }

        public fun get_record(addr: address): T acquires T {
            move_from<T>(addr)
        }

        public fun create(age: u8): T {
            T { age }
        }

        public fun save(record: T) {
            move_to_sender<T>(record);
        }

        public fun destroy_record(s: &signer) acquires T {
            let T { age: _ } = get_record(Signer::address_of(s));
        }

        public fun with_doubled_age(s: &signer): T acquires T {
            let record: T;
            record = get_record(Signer::address_of(s));
            record.age = record.age * 2;
            record
        }
    }
}