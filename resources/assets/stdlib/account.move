address 0x1 {

/// Account is the access point for assets flow. It holds withdraw-deposit handlers
/// for generic currency <Token>. It also stores log of sent and received events
/// for every account.
module Account {

    use 0x1::Pontem;
    use 0x1::Signer;
    use 0x1::Event;

    const ERR_ZERO_DEPOSIT: u64 = 7;

    /// holds account data, currently, only events
    struct T has key {}

    struct Balance<Token> has key {
        coin: Pontem::T<Token>
    }

    /// Message for sent events
    struct SentPaymentEvent has copy {
        amount: u128,
        denom: vector<u8>,
        payee: address,
        metadata: vector<u8>,
    }

    /// Message for received events
    struct ReceivedPaymentEvent has copy {
        amount: u128,
        denom: vector<u8>,
        payer: address,
        metadata: vector<u8>,
    }

    /// Init wallet for measurable currency, hence accept <Token> currency
    public fun accept<Token: store>(account: &signer) {
        move_to<Balance<Token>>(account, Balance { coin: Pontem::zero<Token>() })
    }

    public fun has_balance<Token: store>(payee: address): bool {
        exists<Balance<Token>>(payee)
    }

    public fun has_account(payee: address): bool {
        exists<T>(payee)
    }

    public fun balance<Token: store>(account: &signer): u128 acquires Balance {
        balance_for<Token>(Signer::address_of(account))
    }

    public fun balance_for<Token: store>(addr: address): u128 acquires Balance {
        Pontem::value(&borrow_global<Balance<Token>>(addr).coin)
    }

    public fun deposit_to_sender<Token: store>(
        account: &signer,
        to_deposit: Pontem::T<Token>
    ) acquires Balance {
        deposit<Token>(
            account,
            Signer::address_of(account),
            to_deposit
        )
    }

    public fun deposit<Token: store>(
        account: &signer,
        payee: address,
        to_deposit: Pontem::T<Token>
    ) acquires Balance {
        deposit_with_metadata<Token>(
            account,
            payee,
            to_deposit,
            b""
        )
    }

    public fun deposit_with_metadata<Token: store>(
        account: &signer,
        payee: address,
        to_deposit: Pontem::T<Token>,
        metadata: vector<u8>
    ) acquires Balance {
        deposit_with_sender_and_metadata<Token>(
            account,
            payee,
            to_deposit,
            metadata
        )
    }

    public fun pay_from_sender<Token: store>(
        account: &signer,
        payee: address,
        amount: u128
    ) acquires Balance {
        pay_from_sender_with_metadata<Token>(
            account, payee, amount, b""
        )
    }

    public fun pay_from_sender_with_metadata<Token: store>(
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

    fun deposit_with_sender_and_metadata<Token: store>(
        sender: &signer,
        payee: address,
        to_deposit: Pontem::T<Token>,
        metadata: vector<u8>
    ) acquires Balance {
        let amount = Pontem::value(&to_deposit);
        assert(amount > 0, ERR_ZERO_DEPOSIT);

        let denom = Pontem::denom<Token>();

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
        Pontem::deposit(&mut payee_balance.coin, to_deposit);
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

    public fun withdraw_from_sender<Token: store>(
        account: &signer,
        amount: u128
    ): Pontem::T<Token> acquires Balance {
        let balance = borrow_global_mut<Balance<Token>>(Signer::address_of(account));

        withdraw_from_balance<Token>(balance, amount)
    }

    fun withdraw_from_balance<Token: store>(balance: &mut Balance<Token>, amount: u128): Pontem::T<Token> {
        Pontem::withdraw(&mut balance.coin, amount)
    }

    fun create_balance<Token: store>(addr: address) {
        let sig = create_signer(addr);

        move_to<Balance<Token>>(&sig, Balance {
            coin: Pontem::zero<Token>()
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
