module CovidTracker {
    use 0x0::Vector;
    use 0x0::Transaction;
	struct NewsReport {
		news_source_id: u64,
		infected_count: u64,
	}
	resource struct CovidSituation {
		country_id: u8,
		reports: vector<NewsReport>
	}
	public fun how_many(country: u8): u64 acquires CovidSituation {
        let case = borrow_global<CovidSituation>(Transaction::sender());
        let len  = Vector::length(&case.reports);
        let sum  = 0u64;
        let i    = 0;
        while (i < len) {
            sum = sum + Vector::borrow(&case.reports, i).infected_count;
        };
        sum
	}
}