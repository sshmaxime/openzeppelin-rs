pub use i_inbox::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_inbox {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"messageNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InboxMessageDelivered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"messageNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"InboxMessageDeliveredFromOrigin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridge\",\"outputs\":[{\"internalType\":\"contract IBridge\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"dataLength\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"baseFee\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateRetryableSubmissionFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2CallValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxSubmissionCost\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"excessFeeRefundAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"callValueRefundAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"createRetryableTicket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"depositEth\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IBridge\",\"name\":\"_bridge\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_sequencerInbox\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IBridge\",\"name\":\"_bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"postUpgradeInit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendContractTransaction\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendL1FundedContractTransaction\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendL1FundedUnsignedTransaction\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"messageData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendL2Message\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"messageData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendL2MessageFromOrigin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sendUnsignedTransaction\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerInbox\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2CallValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxSubmissionCost\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"excessFeeRefundAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"callValueRefundAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"gasLimit\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxFeePerGas\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"unsafeCreateRetryableTicket\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IINBOX_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IInbox<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IInbox<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IInbox<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IInbox<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IInbox<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IInbox)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IInbox<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IINBOX_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `bridge` (0xe78cea92) function
        pub fn bridge(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([231, 140, 234, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateRetryableSubmissionFee` (0xa66b327d) function
        pub fn calculate_retryable_submission_fee(
            &self,
            data_length: ::ethers_core::types::U256,
            base_fee: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([166, 107, 50, 125], (data_length, base_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createRetryableTicket` (0x679b6ded) function
        pub fn create_retryable_ticket(
            &self,
            to: ::ethers_core::types::Address,
            l_2_call_value: ::ethers_core::types::U256,
            max_submission_cost: ::ethers_core::types::U256,
            excess_fee_refund_address: ::ethers_core::types::Address,
            call_value_refund_address: ::ethers_core::types::Address,
            gas_limit: ::ethers_core::types::U256,
            max_fee_per_gas: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [103, 155, 109, 237],
                    (
                        to,
                        l_2_call_value,
                        max_submission_cost,
                        excess_fee_refund_address,
                        call_value_refund_address,
                        gas_limit,
                        max_fee_per_gas,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositEth` (0x439370b1) function
        pub fn deposit_eth(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([67, 147, 112, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            bridge: ::ethers_core::types::Address,
            sequencer_inbox: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (bridge, sequencer_inbox))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `postUpgradeInit` (0xc474d2c5) function
        pub fn post_upgrade_init(
            &self,
            bridge: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 116, 210, 197], bridge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendContractTransaction` (0x8a631aa6) function
        pub fn send_contract_transaction(
            &self,
            gas_limit: ::ethers_core::types::U256,
            max_fee_per_gas: ::ethers_core::types::U256,
            to: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [138, 99, 26, 166],
                    (gas_limit, max_fee_per_gas, to, value, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendL1FundedContractTransaction` (0x5e916758) function
        pub fn send_l1_funded_contract_transaction(
            &self,
            gas_limit: ::ethers_core::types::U256,
            max_fee_per_gas: ::ethers_core::types::U256,
            to: ::ethers_core::types::Address,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([94, 145, 103, 88], (gas_limit, max_fee_per_gas, to, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendL1FundedUnsignedTransaction` (0x67ef3ab8) function
        pub fn send_l1_funded_unsigned_transaction(
            &self,
            gas_limit: ::ethers_core::types::U256,
            max_fee_per_gas: ::ethers_core::types::U256,
            nonce: ::ethers_core::types::U256,
            to: ::ethers_core::types::Address,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [103, 239, 58, 184],
                    (gas_limit, max_fee_per_gas, nonce, to, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendL2Message` (0xb75436bb) function
        pub fn send_l2_message(
            &self,
            message_data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([183, 84, 54, 187], message_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendL2MessageFromOrigin` (0x1fe927cf) function
        pub fn send_l2_message_from_origin(
            &self,
            message_data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([31, 233, 39, 207], message_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendUnsignedTransaction` (0x5075788b) function
        pub fn send_unsigned_transaction(
            &self,
            gas_limit: ::ethers_core::types::U256,
            max_fee_per_gas: ::ethers_core::types::U256,
            nonce: ::ethers_core::types::U256,
            to: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [80, 117, 120, 139],
                    (gas_limit, max_fee_per_gas, nonce, to, value, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sequencerInbox` (0xee35f327) function
        pub fn sequencer_inbox(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([238, 53, 243, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unsafeCreateRetryableTicket` (0x6e6e8a6a) function
        pub fn unsafe_create_retryable_ticket(
            &self,
            to: ::ethers_core::types::Address,
            l_2_call_value: ::ethers_core::types::U256,
            max_submission_cost: ::ethers_core::types::U256,
            excess_fee_refund_address: ::ethers_core::types::Address,
            call_value_refund_address: ::ethers_core::types::Address,
            gas_limit: ::ethers_core::types::U256,
            max_fee_per_gas: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [110, 110, 138, 106],
                    (
                        to,
                        l_2_call_value,
                        max_submission_cost,
                        excess_fee_refund_address,
                        call_value_refund_address,
                        gas_limit,
                        max_fee_per_gas,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `InboxMessageDelivered` event
        pub fn inbox_message_delivered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InboxMessageDeliveredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InboxMessageDeliveredFromOrigin` event
        pub fn inbox_message_delivered_from_origin_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InboxMessageDeliveredFromOriginFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IInboxEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IInbox<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "InboxMessageDelivered",
        abi = "InboxMessageDelivered(uint256,bytes)"
    )]
    pub struct InboxMessageDeliveredFilter {
        #[ethevent(indexed)]
        pub message_num: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "InboxMessageDeliveredFromOrigin",
        abi = "InboxMessageDeliveredFromOrigin(uint256)"
    )]
    pub struct InboxMessageDeliveredFromOriginFilter {
        #[ethevent(indexed)]
        pub message_num: ::ethers_core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IInboxEvents {
        InboxMessageDeliveredFilter(InboxMessageDeliveredFilter),
        InboxMessageDeliveredFromOriginFilter(InboxMessageDeliveredFromOriginFilter),
    }
    impl ::ethers_contract::EthLogDecode for IInboxEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = InboxMessageDeliveredFilter::decode_log(log) {
                return Ok(IInboxEvents::InboxMessageDeliveredFilter(decoded));
            }
            if let Ok(decoded) = InboxMessageDeliveredFromOriginFilter::decode_log(log) {
                return Ok(IInboxEvents::InboxMessageDeliveredFromOriginFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IInboxEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InboxMessageDeliveredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InboxMessageDeliveredFromOriginFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InboxMessageDeliveredFilter> for IInboxEvents {
        fn from(value: InboxMessageDeliveredFilter) -> Self {
            Self::InboxMessageDeliveredFilter(value)
        }
    }
    impl ::core::convert::From<InboxMessageDeliveredFromOriginFilter> for IInboxEvents {
        fn from(value: InboxMessageDeliveredFromOriginFilter) -> Self {
            Self::InboxMessageDeliveredFromOriginFilter(value)
        }
    }
    ///Container type for all input parameters for the `bridge` function with signature `bridge()` and selector `0xe78cea92`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bridge", abi = "bridge()")]
    pub struct BridgeCall;
    ///Container type for all input parameters for the `calculateRetryableSubmissionFee` function with signature `calculateRetryableSubmissionFee(uint256,uint256)` and selector `0xa66b327d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "calculateRetryableSubmissionFee",
        abi = "calculateRetryableSubmissionFee(uint256,uint256)"
    )]
    pub struct CalculateRetryableSubmissionFeeCall {
        pub data_length: ::ethers_core::types::U256,
        pub base_fee: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `createRetryableTicket` function with signature `createRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)` and selector `0x679b6ded`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "createRetryableTicket",
        abi = "createRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)"
    )]
    pub struct CreateRetryableTicketCall {
        pub to: ::ethers_core::types::Address,
        pub l_2_call_value: ::ethers_core::types::U256,
        pub max_submission_cost: ::ethers_core::types::U256,
        pub excess_fee_refund_address: ::ethers_core::types::Address,
        pub call_value_refund_address: ::ethers_core::types::Address,
        pub gas_limit: ::ethers_core::types::U256,
        pub max_fee_per_gas: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `depositEth` function with signature `depositEth()` and selector `0x439370b1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "depositEth", abi = "depositEth()")]
    pub struct DepositEthCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub bridge: ::ethers_core::types::Address,
        pub sequencer_inbox: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `postUpgradeInit` function with signature `postUpgradeInit(address)` and selector `0xc474d2c5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "postUpgradeInit", abi = "postUpgradeInit(address)")]
    pub struct PostUpgradeInitCall {
        pub bridge: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `sendContractTransaction` function with signature `sendContractTransaction(uint256,uint256,address,uint256,bytes)` and selector `0x8a631aa6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sendContractTransaction",
        abi = "sendContractTransaction(uint256,uint256,address,uint256,bytes)"
    )]
    pub struct SendContractTransactionCall {
        pub gas_limit: ::ethers_core::types::U256,
        pub max_fee_per_gas: ::ethers_core::types::U256,
        pub to: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendL1FundedContractTransaction` function with signature `sendL1FundedContractTransaction(uint256,uint256,address,bytes)` and selector `0x5e916758`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sendL1FundedContractTransaction",
        abi = "sendL1FundedContractTransaction(uint256,uint256,address,bytes)"
    )]
    pub struct SendL1FundedContractTransactionCall {
        pub gas_limit: ::ethers_core::types::U256,
        pub max_fee_per_gas: ::ethers_core::types::U256,
        pub to: ::ethers_core::types::Address,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendL1FundedUnsignedTransaction` function with signature `sendL1FundedUnsignedTransaction(uint256,uint256,uint256,address,bytes)` and selector `0x67ef3ab8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sendL1FundedUnsignedTransaction",
        abi = "sendL1FundedUnsignedTransaction(uint256,uint256,uint256,address,bytes)"
    )]
    pub struct SendL1FundedUnsignedTransactionCall {
        pub gas_limit: ::ethers_core::types::U256,
        pub max_fee_per_gas: ::ethers_core::types::U256,
        pub nonce: ::ethers_core::types::U256,
        pub to: ::ethers_core::types::Address,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendL2Message` function with signature `sendL2Message(bytes)` and selector `0xb75436bb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendL2Message", abi = "sendL2Message(bytes)")]
    pub struct SendL2MessageCall {
        pub message_data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendL2MessageFromOrigin` function with signature `sendL2MessageFromOrigin(bytes)` and selector `0x1fe927cf`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendL2MessageFromOrigin", abi = "sendL2MessageFromOrigin(bytes)")]
    pub struct SendL2MessageFromOriginCall {
        pub message_data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendUnsignedTransaction` function with signature `sendUnsignedTransaction(uint256,uint256,uint256,address,uint256,bytes)` and selector `0x5075788b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sendUnsignedTransaction",
        abi = "sendUnsignedTransaction(uint256,uint256,uint256,address,uint256,bytes)"
    )]
    pub struct SendUnsignedTransactionCall {
        pub gas_limit: ::ethers_core::types::U256,
        pub max_fee_per_gas: ::ethers_core::types::U256,
        pub nonce: ::ethers_core::types::U256,
        pub to: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sequencerInbox` function with signature `sequencerInbox()` and selector `0xee35f327`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sequencerInbox", abi = "sequencerInbox()")]
    pub struct SequencerInboxCall;
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `unsafeCreateRetryableTicket` function with signature `unsafeCreateRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)` and selector `0x6e6e8a6a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "unsafeCreateRetryableTicket",
        abi = "unsafeCreateRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)"
    )]
    pub struct UnsafeCreateRetryableTicketCall {
        pub to: ::ethers_core::types::Address,
        pub l_2_call_value: ::ethers_core::types::U256,
        pub max_submission_cost: ::ethers_core::types::U256,
        pub excess_fee_refund_address: ::ethers_core::types::Address,
        pub call_value_refund_address: ::ethers_core::types::Address,
        pub gas_limit: ::ethers_core::types::U256,
        pub max_fee_per_gas: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IInboxCalls {
        Bridge(BridgeCall),
        CalculateRetryableSubmissionFee(CalculateRetryableSubmissionFeeCall),
        CreateRetryableTicket(CreateRetryableTicketCall),
        DepositEth(DepositEthCall),
        Initialize(InitializeCall),
        Pause(PauseCall),
        PostUpgradeInit(PostUpgradeInitCall),
        SendContractTransaction(SendContractTransactionCall),
        SendL1FundedContractTransaction(SendL1FundedContractTransactionCall),
        SendL1FundedUnsignedTransaction(SendL1FundedUnsignedTransactionCall),
        SendL2Message(SendL2MessageCall),
        SendL2MessageFromOrigin(SendL2MessageFromOriginCall),
        SendUnsignedTransaction(SendUnsignedTransactionCall),
        SequencerInbox(SequencerInboxCall),
        Unpause(UnpauseCall),
        UnsafeCreateRetryableTicket(UnsafeCreateRetryableTicketCall),
    }
    impl ::ethers_core::abi::AbiDecode for IInboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BridgeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Bridge(decoded));
            }
            if let Ok(decoded)
                = <CalculateRetryableSubmissionFeeCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateRetryableSubmissionFee(decoded));
            }
            if let Ok(decoded)
                = <CreateRetryableTicketCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CreateRetryableTicket(decoded));
            }
            if let Ok(decoded)
                = <DepositEthCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositEth(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <PauseCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded)
                = <PostUpgradeInitCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PostUpgradeInit(decoded));
            }
            if let Ok(decoded)
                = <SendContractTransactionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SendContractTransaction(decoded));
            }
            if let Ok(decoded)
                = <SendL1FundedContractTransactionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SendL1FundedContractTransaction(decoded));
            }
            if let Ok(decoded)
                = <SendL1FundedUnsignedTransactionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SendL1FundedUnsignedTransaction(decoded));
            }
            if let Ok(decoded)
                = <SendL2MessageCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendL2Message(decoded));
            }
            if let Ok(decoded)
                = <SendL2MessageFromOriginCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SendL2MessageFromOrigin(decoded));
            }
            if let Ok(decoded)
                = <SendUnsignedTransactionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SendUnsignedTransaction(decoded));
            }
            if let Ok(decoded)
                = <SequencerInboxCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SequencerInbox(decoded));
            }
            if let Ok(decoded)
                = <UnpauseCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded)
                = <UnsafeCreateRetryableTicketCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnsafeCreateRetryableTicket(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IInboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Bridge(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CalculateRetryableSubmissionFee(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CreateRetryableTicket(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DepositEth(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Pause(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::PostUpgradeInit(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendContractTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendL1FundedContractTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendL1FundedUnsignedTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendL2Message(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendL2MessageFromOrigin(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendUnsignedTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SequencerInbox(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UnsafeCreateRetryableTicket(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IInboxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Bridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateRetryableSubmissionFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateRetryableTicket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PostUpgradeInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendContractTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendL1FundedContractTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendL1FundedUnsignedTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendL2Message(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendL2MessageFromOrigin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendUnsignedTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerInbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeCreateRetryableTicket(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BridgeCall> for IInboxCalls {
        fn from(value: BridgeCall) -> Self {
            Self::Bridge(value)
        }
    }
    impl ::core::convert::From<CalculateRetryableSubmissionFeeCall> for IInboxCalls {
        fn from(value: CalculateRetryableSubmissionFeeCall) -> Self {
            Self::CalculateRetryableSubmissionFee(value)
        }
    }
    impl ::core::convert::From<CreateRetryableTicketCall> for IInboxCalls {
        fn from(value: CreateRetryableTicketCall) -> Self {
            Self::CreateRetryableTicket(value)
        }
    }
    impl ::core::convert::From<DepositEthCall> for IInboxCalls {
        fn from(value: DepositEthCall) -> Self {
            Self::DepositEth(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IInboxCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<PauseCall> for IInboxCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PostUpgradeInitCall> for IInboxCalls {
        fn from(value: PostUpgradeInitCall) -> Self {
            Self::PostUpgradeInit(value)
        }
    }
    impl ::core::convert::From<SendContractTransactionCall> for IInboxCalls {
        fn from(value: SendContractTransactionCall) -> Self {
            Self::SendContractTransaction(value)
        }
    }
    impl ::core::convert::From<SendL1FundedContractTransactionCall> for IInboxCalls {
        fn from(value: SendL1FundedContractTransactionCall) -> Self {
            Self::SendL1FundedContractTransaction(value)
        }
    }
    impl ::core::convert::From<SendL1FundedUnsignedTransactionCall> for IInboxCalls {
        fn from(value: SendL1FundedUnsignedTransactionCall) -> Self {
            Self::SendL1FundedUnsignedTransaction(value)
        }
    }
    impl ::core::convert::From<SendL2MessageCall> for IInboxCalls {
        fn from(value: SendL2MessageCall) -> Self {
            Self::SendL2Message(value)
        }
    }
    impl ::core::convert::From<SendL2MessageFromOriginCall> for IInboxCalls {
        fn from(value: SendL2MessageFromOriginCall) -> Self {
            Self::SendL2MessageFromOrigin(value)
        }
    }
    impl ::core::convert::From<SendUnsignedTransactionCall> for IInboxCalls {
        fn from(value: SendUnsignedTransactionCall) -> Self {
            Self::SendUnsignedTransaction(value)
        }
    }
    impl ::core::convert::From<SequencerInboxCall> for IInboxCalls {
        fn from(value: SequencerInboxCall) -> Self {
            Self::SequencerInbox(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for IInboxCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UnsafeCreateRetryableTicketCall> for IInboxCalls {
        fn from(value: UnsafeCreateRetryableTicketCall) -> Self {
            Self::UnsafeCreateRetryableTicket(value)
        }
    }
    ///Container type for all return fields from the `bridge` function with signature `bridge()` and selector `0xe78cea92`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BridgeReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `calculateRetryableSubmissionFee` function with signature `calculateRetryableSubmissionFee(uint256,uint256)` and selector `0xa66b327d`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateRetryableSubmissionFeeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `createRetryableTicket` function with signature `createRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)` and selector `0x679b6ded`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateRetryableTicketReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `depositEth` function with signature `depositEth()` and selector `0x439370b1`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DepositEthReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sendContractTransaction` function with signature `sendContractTransaction(uint256,uint256,address,uint256,bytes)` and selector `0x8a631aa6`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SendContractTransactionReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sendL1FundedContractTransaction` function with signature `sendL1FundedContractTransaction(uint256,uint256,address,bytes)` and selector `0x5e916758`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SendL1FundedContractTransactionReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sendL1FundedUnsignedTransaction` function with signature `sendL1FundedUnsignedTransaction(uint256,uint256,uint256,address,bytes)` and selector `0x67ef3ab8`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SendL1FundedUnsignedTransactionReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sendL2Message` function with signature `sendL2Message(bytes)` and selector `0xb75436bb`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SendL2MessageReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sendL2MessageFromOrigin` function with signature `sendL2MessageFromOrigin(bytes)` and selector `0x1fe927cf`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SendL2MessageFromOriginReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sendUnsignedTransaction` function with signature `sendUnsignedTransaction(uint256,uint256,uint256,address,uint256,bytes)` and selector `0x5075788b`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SendUnsignedTransactionReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sequencerInbox` function with signature `sequencerInbox()` and selector `0xee35f327`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SequencerInboxReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `unsafeCreateRetryableTicket` function with signature `unsafeCreateRetryableTicket(address,uint256,uint256,address,address,uint256,uint256,bytes)` and selector `0x6e6e8a6a`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UnsafeCreateRetryableTicketReturn(pub ::ethers_core::types::U256);
}
