address 0x1 {

/// Account is the access point for assets flow. It holds withdraw-deposit handlers
/// for generic currency <Token>. It also stores log of sent and received events
/// for every account.
module Account {

    use 0x1::Dfinance;
    use 0x1::Signer;
    use 0x1::Event;

    const ERR_ZERO_DEPOSIT: u64 = 7;

    /// holds account data, currently, only events
    resource struct T {}

    resource struct Balance<Token> {
        coin: Dfinance::T<Token>
    }

    /// Message for sent events
    struct SentPaymentEvent {
        amount: u128,
        denom: vector<u8>,
        payee: address,
        metadata: vector<u8>,
    }

    /// Message for received events
    struct ReceivedPaymentEvent {
        amount: u128,
        denom: vector<u8>,
        payer: address,
        metadata: vector<u8>,
    }

    /// Init wallet for measurable currency, hence accept <Token> currency
    public fun accept<Token>(account: &signer) {
        move_to<Balance<Token>>(account, Balance { coin: Dfinance::zero<Token>() })
    }

    public fun has_balance<Token>(payee: address): bool {
        exists<Balance<Token>>(payee)
    }

    public fun has_account(payee: address): bool {
        exists<T>(payee)
    }

    public fun balance<Token>(account: &signer): u128 acquires Balance {
        balance_for<Token>(Signer::address_of(account))
    }

    public fun balance_for<Token>(addr: address): u128 acquires Balance {
        Dfinance::value(&borrow_global<Balance<Token>>(addr).coin)
    }

    public fun deposit_to_sender<Token>(
        account: &signer,
        to_deposit: Dfinance::T<Token>
    ) acquires Balance {
        deposit<Token>(
            account,
            Signer::address_of(account),
            to_deposit
        )
    }

    public fun deposit<Token>(
        account: &signer,
        payee: address,
        to_deposit: Dfinance::T<Token>
    ) acquires Balance {
        deposit_with_metadata<Token>(
            account,
            payee,
            to_deposit,
            b""
        )
    }

    public fun deposit_with_metadata<Token>(
        account: &signer,
        payee: address,
        to_deposit: Dfinance::T<Token>,
        metadata: vector<u8>
    ) acquires Balance {
        deposit_with_sender_and_metadata<Token>(
            account,
            payee,
            to_deposit,
            metadata
        )
    }

    public fun pay_from_sender<Token>(
        account: &signer,
        payee: address,
        amount: u128
    ) acquires Balance {
        pay_from_sender_with_metadata<Token>(
            account, payee, amount, b""
        )
    }

    public fun pay_from_sender_with_metadata<Token>(
        account: &signer,
        payee: address,
        amount: u128,
        metadata: vector<u8>
    )
    acquires Balance {
        deposit_with_metadata<Token>(
            account,
            payee,
            withdraw_from_sender<Token>(account, amount),
            metadata
        )
    }

    fun deposit_with_sender_and_metadata<Token>(
        sender: &signer,
        payee: address,
        to_deposit: Dfinance::T<Token>,
        metadata: vector<u8>
    ) acquires Balance {
        let amount = Dfinance::value(&to_deposit);
        assert(amount > 0, ERR_ZERO_DEPOSIT);

        let denom = Dfinance::denom<Token>();

        // add event as sent into account
        Event::emit<SentPaymentEvent>(
            sender,
            SentPaymentEvent {
                amount, // u64 can be copied
                payee,
                denom: copy denom,
                metadata: copy metadata
            },
        );

        // there's no way to improve this place as payee is not sender :(
        if (!has_balance<Token>(payee)) {
            create_balance<Token>(payee);
        };

        if (!has_account(payee)) {
            create_account(payee);
        };

        let payee_balance = borrow_global_mut<Balance<Token>>(payee);

        // send money to payee
        Dfinance::deposit(&mut payee_balance.coin, to_deposit);
        // update payee's account with new event
        Event::emit<ReceivedPaymentEvent>(
            sender,
            ReceivedPaymentEvent {
                amount,
                denom,
                metadata,
                payer: Signer::address_of(sender)
            }
        )
    }

    public fun withdraw_from_sender<Token>(
        account: &signer,
        amount: u128
    ): Dfinance::T<Token> acquires Balance {
        let balance = borrow_global_mut<Balance<Token>>(Signer::address_of(account));

        withdraw_from_balance<Token>(balance, amount)
    }

    fun withdraw_from_balance<Token>(balance: &mut Balance<Token>, amount: u128): Dfinance::T<Token> {
        Dfinance::withdraw(&mut balance.coin, amount)
    }

    fun create_balance<Token>(addr: address) {
        let sig = create_signer(addr);

        move_to<Balance<Token>>(&sig, Balance {
            coin: Dfinance::zero<Token>()
        });

        destroy_signer(sig);
    }

    /// keep this function, we may use T in the future
    fun create_account(addr: address) {
        let sig = create_signer(addr);

        move_to<T>(&sig, T { });

        destroy_signer(sig);
    }

    native fun create_signer(addr: address): signer;
    native fun destroy_signer(sig: signer);
}
}
