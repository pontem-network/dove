address 0x2 {
    module Record {
        use 0x1::Signer;

        resource struct T {
            age: u8
        }

        public fun get_record(addr: address): T acquires T {
            move_from<T>(addr)
        }

        public fun create(age: u8): T {
            T { age }
        }

        public fun save(account: &signer, record: T) {
            move_to<T>(account, record);
        }

        public fun create_record(account: &signer, age: u8) {
            let rec = T { age };
            move_to<T>(account, rec);
        }

        public fun increment_record(account: &signer) acquires T {
            let existing_rec = move_from<T>(Signer::address_of(account));
            existing_rec.age = existing_rec.age + 1;
            move_to<T>(account, existing_rec);
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