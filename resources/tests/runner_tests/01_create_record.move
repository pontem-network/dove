script {
    use 0x2::Record;

    fun main(s: &signer) {
        Record::create_record(s, 10);
    }
}

script {
    use 0x2::Record;

    fun main(s: &signer) {
        assert(Record::record_age(s) == 10, 401);
    }
}