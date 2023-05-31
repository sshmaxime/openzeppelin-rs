pub use i_bridge::*;
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
pub mod i_bridge {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"outbox\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BridgeCallTriggered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"inbox\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InboxToggle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"messageIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"beforeInboxAcc\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"inbox\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"kind\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"messageDataHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"baseFeeL1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MessageDelivered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"outbox\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OutboxToggle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newSequencerInbox\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SequencerInboxUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"activeOutbox\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"allowedDelayedInboxList\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"inbox\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedDelayedInboxes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"allowedOutboxList\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"outbox\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedOutboxes\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delayedInboxAccs\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delayedMessageCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"kind\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"messageDataHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"enqueueDelayedMessage\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"afterDelayedMessagesRead\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"prevMessageCount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newMessageCount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enqueueSequencerMessage\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"seqMessageIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"beforeAcc\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"delayedAcc\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"acc\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"returnData\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"rollup_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollup\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerInbox\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerInboxAccs\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerMessageCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencerReportedSubMessageCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"inbox\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDelayedInbox\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"inbox\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"enabled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOutbox\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_sequencerInbox\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSequencerInbox\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"batchPoster\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"dataHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBatchSpendingReport\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"msgNum\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IBRIDGE_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IBridge<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IBridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IBridge<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IBridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IBridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IBridge)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IBridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IBRIDGE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `activeOutbox` (0xab5d8943) function
        pub fn active_outbox(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([171, 93, 137, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowedDelayedInboxList` (0xe76f5c8d) function
        pub fn allowed_delayed_inbox_list(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([231, 111, 92, 141], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowedDelayedInboxes` (0xae60bd13) function
        pub fn allowed_delayed_inboxes(
            &self,
            inbox: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([174, 96, 189, 19], inbox)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowedOutboxList` (0x945e1147) function
        pub fn allowed_outbox_list(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([148, 94, 17, 71], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowedOutboxes` (0x413b35bd) function
        pub fn allowed_outboxes(
            &self,
            outbox: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([65, 59, 53, 189], outbox)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delayedInboxAccs` (0xd5719dc2) function
        pub fn delayed_inbox_accs(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 113, 157, 194], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delayedMessageCount` (0xeca067ad) function
        pub fn delayed_message_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([236, 160, 103, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enqueueDelayedMessage` (0x8db5993b) function
        pub fn enqueue_delayed_message(
            &self,
            kind: u8,
            sender: ::ethers_core::types::Address,
            message_data_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([141, 181, 153, 59], (kind, sender, message_data_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enqueueSequencerMessage` (0x86598a56) function
        pub fn enqueue_sequencer_message(
            &self,
            data_hash: [u8; 32],
            after_delayed_messages_read: ::ethers_core::types::U256,
            prev_message_count: ::ethers_core::types::U256,
            new_message_count: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (::ethers_core::types::U256, [u8; 32], [u8; 32], [u8; 32]),
        > {
            self.0
                .method_hash(
                    [134, 89, 138, 86],
                    (
                        data_hash,
                        after_delayed_messages_read,
                        prev_message_count,
                        new_message_count,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeCall` (0x9e5d4c49) function
        pub fn execute_call(
            &self,
            to: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (bool, ::ethers_core::types::Bytes),
        > {
            self.0
                .method_hash([158, 93, 76, 73], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            rollup: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], rollup)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollup` (0xcb23bcb5) function
        pub fn rollup(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([203, 35, 188, 181], ())
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
        ///Calls the contract's `sequencerInboxAccs` (0x16bf5579) function
        pub fn sequencer_inbox_accs(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([22, 191, 85, 121], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sequencerMessageCount` (0x0084120c) function
        pub fn sequencer_message_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([0, 132, 18, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sequencerReportedSubMessageCount` (0x5fca4a16) function
        pub fn sequencer_reported_sub_message_count(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([95, 202, 74, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDelayedInbox` (0x47fb24c5) function
        pub fn set_delayed_inbox(
            &self,
            inbox: ::ethers_core::types::Address,
            enabled: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 251, 36, 197], (inbox, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOutbox` (0xcee3d728) function
        pub fn set_outbox(
            &self,
            inbox: ::ethers_core::types::Address,
            enabled: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 227, 215, 40], (inbox, enabled))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSequencerInbox` (0x4f61f850) function
        pub fn set_sequencer_inbox(
            &self,
            sequencer_inbox: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 97, 248, 80], sequencer_inbox)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitBatchSpendingReport` (0x7a88b107) function
        pub fn submit_batch_spending_report(
            &self,
            batch_poster: ::ethers_core::types::Address,
            data_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([122, 136, 177, 7], (batch_poster, data_hash))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BridgeCallTriggered` event
        pub fn bridge_call_triggered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BridgeCallTriggeredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InboxToggle` event
        pub fn inbox_toggle_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InboxToggleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MessageDelivered` event
        pub fn message_delivered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MessageDeliveredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OutboxToggle` event
        pub fn outbox_toggle_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OutboxToggleFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SequencerInboxUpdated` event
        pub fn sequencer_inbox_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SequencerInboxUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IBridgeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IBridge<M> {
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
        name = "BridgeCallTriggered",
        abi = "BridgeCallTriggered(address,address,uint256,bytes)"
    )]
    pub struct BridgeCallTriggeredFilter {
        #[ethevent(indexed)]
        pub outbox: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
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
    #[ethevent(name = "InboxToggle", abi = "InboxToggle(address,bool)")]
    pub struct InboxToggleFilter {
        #[ethevent(indexed)]
        pub inbox: ::ethers_core::types::Address,
        pub enabled: bool,
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
        name = "MessageDelivered",
        abi = "MessageDelivered(uint256,bytes32,address,uint8,address,bytes32,uint256,uint64)"
    )]
    pub struct MessageDeliveredFilter {
        #[ethevent(indexed)]
        pub message_index: ::ethers_core::types::U256,
        #[ethevent(indexed)]
        pub before_inbox_acc: [u8; 32],
        pub inbox: ::ethers_core::types::Address,
        pub kind: u8,
        pub sender: ::ethers_core::types::Address,
        pub message_data_hash: [u8; 32],
        pub base_fee_l1: ::ethers_core::types::U256,
        pub timestamp: u64,
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
    #[ethevent(name = "OutboxToggle", abi = "OutboxToggle(address,bool)")]
    pub struct OutboxToggleFilter {
        #[ethevent(indexed)]
        pub outbox: ::ethers_core::types::Address,
        pub enabled: bool,
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
    #[ethevent(name = "SequencerInboxUpdated", abi = "SequencerInboxUpdated(address)")]
    pub struct SequencerInboxUpdatedFilter {
        pub new_sequencer_inbox: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IBridgeEvents {
        BridgeCallTriggeredFilter(BridgeCallTriggeredFilter),
        InboxToggleFilter(InboxToggleFilter),
        MessageDeliveredFilter(MessageDeliveredFilter),
        OutboxToggleFilter(OutboxToggleFilter),
        SequencerInboxUpdatedFilter(SequencerInboxUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for IBridgeEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = BridgeCallTriggeredFilter::decode_log(log) {
                return Ok(IBridgeEvents::BridgeCallTriggeredFilter(decoded));
            }
            if let Ok(decoded) = InboxToggleFilter::decode_log(log) {
                return Ok(IBridgeEvents::InboxToggleFilter(decoded));
            }
            if let Ok(decoded) = MessageDeliveredFilter::decode_log(log) {
                return Ok(IBridgeEvents::MessageDeliveredFilter(decoded));
            }
            if let Ok(decoded) = OutboxToggleFilter::decode_log(log) {
                return Ok(IBridgeEvents::OutboxToggleFilter(decoded));
            }
            if let Ok(decoded) = SequencerInboxUpdatedFilter::decode_log(log) {
                return Ok(IBridgeEvents::SequencerInboxUpdatedFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IBridgeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BridgeCallTriggeredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InboxToggleFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageDeliveredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutboxToggleFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerInboxUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BridgeCallTriggeredFilter> for IBridgeEvents {
        fn from(value: BridgeCallTriggeredFilter) -> Self {
            Self::BridgeCallTriggeredFilter(value)
        }
    }
    impl ::core::convert::From<InboxToggleFilter> for IBridgeEvents {
        fn from(value: InboxToggleFilter) -> Self {
            Self::InboxToggleFilter(value)
        }
    }
    impl ::core::convert::From<MessageDeliveredFilter> for IBridgeEvents {
        fn from(value: MessageDeliveredFilter) -> Self {
            Self::MessageDeliveredFilter(value)
        }
    }
    impl ::core::convert::From<OutboxToggleFilter> for IBridgeEvents {
        fn from(value: OutboxToggleFilter) -> Self {
            Self::OutboxToggleFilter(value)
        }
    }
    impl ::core::convert::From<SequencerInboxUpdatedFilter> for IBridgeEvents {
        fn from(value: SequencerInboxUpdatedFilter) -> Self {
            Self::SequencerInboxUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `activeOutbox` function with signature `activeOutbox()` and selector `0xab5d8943`
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
    #[ethcall(name = "activeOutbox", abi = "activeOutbox()")]
    pub struct ActiveOutboxCall;
    ///Container type for all input parameters for the `allowedDelayedInboxList` function with signature `allowedDelayedInboxList(uint256)` and selector `0xe76f5c8d`
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
        name = "allowedDelayedInboxList",
        abi = "allowedDelayedInboxList(uint256)"
    )]
    pub struct AllowedDelayedInboxListCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `allowedDelayedInboxes` function with signature `allowedDelayedInboxes(address)` and selector `0xae60bd13`
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
    #[ethcall(name = "allowedDelayedInboxes", abi = "allowedDelayedInboxes(address)")]
    pub struct AllowedDelayedInboxesCall {
        pub inbox: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `allowedOutboxList` function with signature `allowedOutboxList(uint256)` and selector `0x945e1147`
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
    #[ethcall(name = "allowedOutboxList", abi = "allowedOutboxList(uint256)")]
    pub struct AllowedOutboxListCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `allowedOutboxes` function with signature `allowedOutboxes(address)` and selector `0x413b35bd`
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
    #[ethcall(name = "allowedOutboxes", abi = "allowedOutboxes(address)")]
    pub struct AllowedOutboxesCall {
        pub outbox: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `delayedInboxAccs` function with signature `delayedInboxAccs(uint256)` and selector `0xd5719dc2`
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
    #[ethcall(name = "delayedInboxAccs", abi = "delayedInboxAccs(uint256)")]
    pub struct DelayedInboxAccsCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `delayedMessageCount` function with signature `delayedMessageCount()` and selector `0xeca067ad`
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
    #[ethcall(name = "delayedMessageCount", abi = "delayedMessageCount()")]
    pub struct DelayedMessageCountCall;
    ///Container type for all input parameters for the `enqueueDelayedMessage` function with signature `enqueueDelayedMessage(uint8,address,bytes32)` and selector `0x8db5993b`
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
        name = "enqueueDelayedMessage",
        abi = "enqueueDelayedMessage(uint8,address,bytes32)"
    )]
    pub struct EnqueueDelayedMessageCall {
        pub kind: u8,
        pub sender: ::ethers_core::types::Address,
        pub message_data_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `enqueueSequencerMessage` function with signature `enqueueSequencerMessage(bytes32,uint256,uint256,uint256)` and selector `0x86598a56`
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
        name = "enqueueSequencerMessage",
        abi = "enqueueSequencerMessage(bytes32,uint256,uint256,uint256)"
    )]
    pub struct EnqueueSequencerMessageCall {
        pub data_hash: [u8; 32],
        pub after_delayed_messages_read: ::ethers_core::types::U256,
        pub prev_message_count: ::ethers_core::types::U256,
        pub new_message_count: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `executeCall` function with signature `executeCall(address,uint256,bytes)` and selector `0x9e5d4c49`
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
    #[ethcall(name = "executeCall", abi = "executeCall(address,uint256,bytes)")]
    pub struct ExecuteCallCall {
        pub to: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub rollup: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `rollup` function with signature `rollup()` and selector `0xcb23bcb5`
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
    #[ethcall(name = "rollup", abi = "rollup()")]
    pub struct RollupCall;
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
    ///Container type for all input parameters for the `sequencerInboxAccs` function with signature `sequencerInboxAccs(uint256)` and selector `0x16bf5579`
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
    #[ethcall(name = "sequencerInboxAccs", abi = "sequencerInboxAccs(uint256)")]
    pub struct SequencerInboxAccsCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `sequencerMessageCount` function with signature `sequencerMessageCount()` and selector `0x0084120c`
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
    #[ethcall(name = "sequencerMessageCount", abi = "sequencerMessageCount()")]
    pub struct SequencerMessageCountCall;
    ///Container type for all input parameters for the `sequencerReportedSubMessageCount` function with signature `sequencerReportedSubMessageCount()` and selector `0x5fca4a16`
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
        name = "sequencerReportedSubMessageCount",
        abi = "sequencerReportedSubMessageCount()"
    )]
    pub struct SequencerReportedSubMessageCountCall;
    ///Container type for all input parameters for the `setDelayedInbox` function with signature `setDelayedInbox(address,bool)` and selector `0x47fb24c5`
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
    #[ethcall(name = "setDelayedInbox", abi = "setDelayedInbox(address,bool)")]
    pub struct SetDelayedInboxCall {
        pub inbox: ::ethers_core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setOutbox` function with signature `setOutbox(address,bool)` and selector `0xcee3d728`
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
    #[ethcall(name = "setOutbox", abi = "setOutbox(address,bool)")]
    pub struct SetOutboxCall {
        pub inbox: ::ethers_core::types::Address,
        pub enabled: bool,
    }
    ///Container type for all input parameters for the `setSequencerInbox` function with signature `setSequencerInbox(address)` and selector `0x4f61f850`
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
    #[ethcall(name = "setSequencerInbox", abi = "setSequencerInbox(address)")]
    pub struct SetSequencerInboxCall {
        pub sequencer_inbox: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `submitBatchSpendingReport` function with signature `submitBatchSpendingReport(address,bytes32)` and selector `0x7a88b107`
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
        name = "submitBatchSpendingReport",
        abi = "submitBatchSpendingReport(address,bytes32)"
    )]
    pub struct SubmitBatchSpendingReportCall {
        pub batch_poster: ::ethers_core::types::Address,
        pub data_hash: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IBridgeCalls {
        ActiveOutbox(ActiveOutboxCall),
        AllowedDelayedInboxList(AllowedDelayedInboxListCall),
        AllowedDelayedInboxes(AllowedDelayedInboxesCall),
        AllowedOutboxList(AllowedOutboxListCall),
        AllowedOutboxes(AllowedOutboxesCall),
        DelayedInboxAccs(DelayedInboxAccsCall),
        DelayedMessageCount(DelayedMessageCountCall),
        EnqueueDelayedMessage(EnqueueDelayedMessageCall),
        EnqueueSequencerMessage(EnqueueSequencerMessageCall),
        ExecuteCall(ExecuteCallCall),
        Initialize(InitializeCall),
        Rollup(RollupCall),
        SequencerInbox(SequencerInboxCall),
        SequencerInboxAccs(SequencerInboxAccsCall),
        SequencerMessageCount(SequencerMessageCountCall),
        SequencerReportedSubMessageCount(SequencerReportedSubMessageCountCall),
        SetDelayedInbox(SetDelayedInboxCall),
        SetOutbox(SetOutboxCall),
        SetSequencerInbox(SetSequencerInboxCall),
        SubmitBatchSpendingReport(SubmitBatchSpendingReportCall),
    }
    impl ::ethers_core::abi::AbiDecode for IBridgeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ActiveOutboxCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ActiveOutbox(decoded));
            }
            if let Ok(decoded)
                = <AllowedDelayedInboxListCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AllowedDelayedInboxList(decoded));
            }
            if let Ok(decoded)
                = <AllowedDelayedInboxesCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AllowedDelayedInboxes(decoded));
            }
            if let Ok(decoded)
                = <AllowedOutboxListCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AllowedOutboxList(decoded));
            }
            if let Ok(decoded)
                = <AllowedOutboxesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AllowedOutboxes(decoded));
            }
            if let Ok(decoded)
                = <DelayedInboxAccsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelayedInboxAccs(decoded));
            }
            if let Ok(decoded)
                = <DelayedMessageCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DelayedMessageCount(decoded));
            }
            if let Ok(decoded)
                = <EnqueueDelayedMessageCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EnqueueDelayedMessage(decoded));
            }
            if let Ok(decoded)
                = <EnqueueSequencerMessageCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EnqueueSequencerMessage(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCallCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteCall(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <RollupCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rollup(decoded));
            }
            if let Ok(decoded)
                = <SequencerInboxCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SequencerInbox(decoded));
            }
            if let Ok(decoded)
                = <SequencerInboxAccsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SequencerInboxAccs(decoded));
            }
            if let Ok(decoded)
                = <SequencerMessageCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SequencerMessageCount(decoded));
            }
            if let Ok(decoded)
                = <SequencerReportedSubMessageCountCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SequencerReportedSubMessageCount(decoded));
            }
            if let Ok(decoded)
                = <SetDelayedInboxCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDelayedInbox(decoded));
            }
            if let Ok(decoded)
                = <SetOutboxCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOutbox(decoded));
            }
            if let Ok(decoded)
                = <SetSequencerInboxCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetSequencerInbox(decoded));
            }
            if let Ok(decoded)
                = <SubmitBatchSpendingReportCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SubmitBatchSpendingReport(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IBridgeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ActiveOutbox(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AllowedDelayedInboxList(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AllowedDelayedInboxes(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AllowedOutboxList(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AllowedOutboxes(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DelayedInboxAccs(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::DelayedMessageCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::EnqueueDelayedMessage(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::EnqueueSequencerMessage(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Rollup(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SequencerInbox(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SequencerInboxAccs(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SequencerMessageCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SequencerReportedSubMessageCount(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetDelayedInbox(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetOutbox(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetSequencerInbox(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SubmitBatchSpendingReport(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IBridgeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ActiveOutbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowedDelayedInboxList(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedDelayedInboxes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AllowedOutboxList(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowedOutboxes(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelayedInboxAccs(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelayedMessageCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnqueueDelayedMessage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnqueueSequencerMessage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rollup(element) => ::core::fmt::Display::fmt(element, f),
                Self::SequencerInbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SequencerInboxAccs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerMessageCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SequencerReportedSubMessageCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDelayedInbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOutbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSequencerInbox(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitBatchSpendingReport(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ActiveOutboxCall> for IBridgeCalls {
        fn from(value: ActiveOutboxCall) -> Self {
            Self::ActiveOutbox(value)
        }
    }
    impl ::core::convert::From<AllowedDelayedInboxListCall> for IBridgeCalls {
        fn from(value: AllowedDelayedInboxListCall) -> Self {
            Self::AllowedDelayedInboxList(value)
        }
    }
    impl ::core::convert::From<AllowedDelayedInboxesCall> for IBridgeCalls {
        fn from(value: AllowedDelayedInboxesCall) -> Self {
            Self::AllowedDelayedInboxes(value)
        }
    }
    impl ::core::convert::From<AllowedOutboxListCall> for IBridgeCalls {
        fn from(value: AllowedOutboxListCall) -> Self {
            Self::AllowedOutboxList(value)
        }
    }
    impl ::core::convert::From<AllowedOutboxesCall> for IBridgeCalls {
        fn from(value: AllowedOutboxesCall) -> Self {
            Self::AllowedOutboxes(value)
        }
    }
    impl ::core::convert::From<DelayedInboxAccsCall> for IBridgeCalls {
        fn from(value: DelayedInboxAccsCall) -> Self {
            Self::DelayedInboxAccs(value)
        }
    }
    impl ::core::convert::From<DelayedMessageCountCall> for IBridgeCalls {
        fn from(value: DelayedMessageCountCall) -> Self {
            Self::DelayedMessageCount(value)
        }
    }
    impl ::core::convert::From<EnqueueDelayedMessageCall> for IBridgeCalls {
        fn from(value: EnqueueDelayedMessageCall) -> Self {
            Self::EnqueueDelayedMessage(value)
        }
    }
    impl ::core::convert::From<EnqueueSequencerMessageCall> for IBridgeCalls {
        fn from(value: EnqueueSequencerMessageCall) -> Self {
            Self::EnqueueSequencerMessage(value)
        }
    }
    impl ::core::convert::From<ExecuteCallCall> for IBridgeCalls {
        fn from(value: ExecuteCallCall) -> Self {
            Self::ExecuteCall(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IBridgeCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<RollupCall> for IBridgeCalls {
        fn from(value: RollupCall) -> Self {
            Self::Rollup(value)
        }
    }
    impl ::core::convert::From<SequencerInboxCall> for IBridgeCalls {
        fn from(value: SequencerInboxCall) -> Self {
            Self::SequencerInbox(value)
        }
    }
    impl ::core::convert::From<SequencerInboxAccsCall> for IBridgeCalls {
        fn from(value: SequencerInboxAccsCall) -> Self {
            Self::SequencerInboxAccs(value)
        }
    }
    impl ::core::convert::From<SequencerMessageCountCall> for IBridgeCalls {
        fn from(value: SequencerMessageCountCall) -> Self {
            Self::SequencerMessageCount(value)
        }
    }
    impl ::core::convert::From<SequencerReportedSubMessageCountCall> for IBridgeCalls {
        fn from(value: SequencerReportedSubMessageCountCall) -> Self {
            Self::SequencerReportedSubMessageCount(value)
        }
    }
    impl ::core::convert::From<SetDelayedInboxCall> for IBridgeCalls {
        fn from(value: SetDelayedInboxCall) -> Self {
            Self::SetDelayedInbox(value)
        }
    }
    impl ::core::convert::From<SetOutboxCall> for IBridgeCalls {
        fn from(value: SetOutboxCall) -> Self {
            Self::SetOutbox(value)
        }
    }
    impl ::core::convert::From<SetSequencerInboxCall> for IBridgeCalls {
        fn from(value: SetSequencerInboxCall) -> Self {
            Self::SetSequencerInbox(value)
        }
    }
    impl ::core::convert::From<SubmitBatchSpendingReportCall> for IBridgeCalls {
        fn from(value: SubmitBatchSpendingReportCall) -> Self {
            Self::SubmitBatchSpendingReport(value)
        }
    }
    ///Container type for all return fields from the `activeOutbox` function with signature `activeOutbox()` and selector `0xab5d8943`
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
    pub struct ActiveOutboxReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `allowedDelayedInboxList` function with signature `allowedDelayedInboxList(uint256)` and selector `0xe76f5c8d`
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
    pub struct AllowedDelayedInboxListReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `allowedDelayedInboxes` function with signature `allowedDelayedInboxes(address)` and selector `0xae60bd13`
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
    pub struct AllowedDelayedInboxesReturn(pub bool);
    ///Container type for all return fields from the `allowedOutboxList` function with signature `allowedOutboxList(uint256)` and selector `0x945e1147`
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
    pub struct AllowedOutboxListReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `allowedOutboxes` function with signature `allowedOutboxes(address)` and selector `0x413b35bd`
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
    pub struct AllowedOutboxesReturn(pub bool);
    ///Container type for all return fields from the `delayedInboxAccs` function with signature `delayedInboxAccs(uint256)` and selector `0xd5719dc2`
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
    pub struct DelayedInboxAccsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `delayedMessageCount` function with signature `delayedMessageCount()` and selector `0xeca067ad`
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
    pub struct DelayedMessageCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `enqueueDelayedMessage` function with signature `enqueueDelayedMessage(uint8,address,bytes32)` and selector `0x8db5993b`
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
    pub struct EnqueueDelayedMessageReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `enqueueSequencerMessage` function with signature `enqueueSequencerMessage(bytes32,uint256,uint256,uint256)` and selector `0x86598a56`
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
    pub struct EnqueueSequencerMessageReturn {
        pub seq_message_index: ::ethers_core::types::U256,
        pub before_acc: [u8; 32],
        pub delayed_acc: [u8; 32],
        pub acc: [u8; 32],
    }
    ///Container type for all return fields from the `executeCall` function with signature `executeCall(address,uint256,bytes)` and selector `0x9e5d4c49`
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
    pub struct ExecuteCallReturn {
        pub success: bool,
        pub return_data: ::ethers_core::types::Bytes,
    }
    ///Container type for all return fields from the `rollup` function with signature `rollup()` and selector `0xcb23bcb5`
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
    pub struct RollupReturn(pub ::ethers_core::types::Address);
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
    ///Container type for all return fields from the `sequencerInboxAccs` function with signature `sequencerInboxAccs(uint256)` and selector `0x16bf5579`
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
    pub struct SequencerInboxAccsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `sequencerMessageCount` function with signature `sequencerMessageCount()` and selector `0x0084120c`
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
    pub struct SequencerMessageCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `sequencerReportedSubMessageCount` function with signature `sequencerReportedSubMessageCount()` and selector `0x5fca4a16`
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
    pub struct SequencerReportedSubMessageCountReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `submitBatchSpendingReport` function with signature `submitBatchSpendingReport(address,bytes32)` and selector `0x7a88b107`
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
    pub struct SubmitBatchSpendingReportReturn {
        pub msg_num: ::ethers_core::types::U256,
    }
}
