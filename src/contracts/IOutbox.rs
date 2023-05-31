pub use i_outbox::*;
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
pub mod i_outbox {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"l2Sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"zero\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"transactionIndex\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"OutBoxTransactionExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"blockHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"outputRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SendRootUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"OUTBOX_VERSION\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridge\",\"outputs\":[{\"internalType\":\"contract IBridge\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"l2Sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2Block\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l1Block\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2Timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculateItemHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"path\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"item\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculateMerkleRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"proof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"l2Sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2Block\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l1Block\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2Timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeTransaction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"l2Sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2Block\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l1Block\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"l2Timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeTransactionSimulation\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSpent\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"l2ToL1Block\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"l2ToL1EthBlock\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"l2ToL1OutputId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"l2ToL1Sender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"l2ToL1Timestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollup\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"roots\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"spent\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"sendRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"l2BlockHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateSendRoot\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IOUTBOX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IOutbox<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IOutbox<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IOutbox<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IOutbox<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IOutbox<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IOutbox)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOutbox<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IOUTBOX_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `OUTBOX_VERSION` (0xc75184df) function
        pub fn outbox_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([199, 81, 132, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bridge` (0xe78cea92) function
        pub fn bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([231, 140, 234, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateItemHash` (0x9f0c04bf) function
        pub fn calculate_item_hash(
            &self,
            l_2_sender: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            l_2_block: ::ethers::core::types::U256,
            l_1_block: ::ethers::core::types::U256,
            l_2_timestamp: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [159, 12, 4, 191],
                    (l_2_sender, to, l_2_block, l_1_block, l_2_timestamp, value, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateMerkleRoot` (0x007436d3) function
        pub fn calculate_merkle_root(
            &self,
            proof: ::std::vec::Vec<[u8; 32]>,
            path: ::ethers::core::types::U256,
            item: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([0, 116, 54, 211], (proof, path, item))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeTransaction` (0x08635a95) function
        pub fn execute_transaction(
            &self,
            proof: ::std::vec::Vec<[u8; 32]>,
            index: ::ethers::core::types::U256,
            l_2_sender: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            l_2_block: ::ethers::core::types::U256,
            l_1_block: ::ethers::core::types::U256,
            l_2_timestamp: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [8, 99, 90, 149],
                    (
                        proof,
                        index,
                        l_2_sender,
                        to,
                        l_2_block,
                        l_1_block,
                        l_2_timestamp,
                        value,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeTransactionSimulation` (0x288e5b10) function
        pub fn execute_transaction_simulation(
            &self,
            index: ::ethers::core::types::U256,
            l_2_sender: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            l_2_block: ::ethers::core::types::U256,
            l_1_block: ::ethers::core::types::U256,
            l_2_timestamp: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [40, 142, 91, 16],
                    (
                        index,
                        l_2_sender,
                        to,
                        l_2_block,
                        l_1_block,
                        l_2_timestamp,
                        value,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSpent` (0x5a129efe) function
        pub fn is_spent(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 18, 158, 254], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2ToL1Block` (0x46547790) function
        pub fn l_2_to_l1_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 84, 119, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2ToL1EthBlock` (0x8515bc6a) function
        pub fn l_2_to_l1_eth_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 21, 188, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2ToL1OutputId` (0x72f2a8c7) function
        pub fn l_2_to_l1_output_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([114, 242, 168, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2ToL1Sender` (0x80648b02) function
        pub fn l_2_to_l1_sender(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([128, 100, 139, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `l2ToL1Timestamp` (0xb0f30537) function
        pub fn l_2_to_l1_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 243, 5, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollup` (0xcb23bcb5) function
        pub fn rollup(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([203, 35, 188, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roots` (0xae6dead7) function
        pub fn roots(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([174, 109, 234, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spent` (0xd5b5cc23) function
        pub fn spent(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([213, 181, 204, 35], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSendRoot` (0xa04cee60) function
        pub fn update_send_root(
            &self,
            send_root: [u8; 32],
            l_2_block_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 76, 238, 96], (send_root, l_2_block_hash))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OutBoxTransactionExecuted` event
        pub fn out_box_transaction_executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OutBoxTransactionExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SendRootUpdated` event
        pub fn send_root_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SendRootUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IOutboxEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IOutbox<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OutBoxTransactionExecuted",
        abi = "OutBoxTransactionExecuted(address,address,uint256,uint256)"
    )]
    pub struct OutBoxTransactionExecutedFilter {
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub l_2_sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zero: ::ethers::core::types::U256,
        pub transaction_index: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SendRootUpdated", abi = "SendRootUpdated(bytes32,bytes32)")]
    pub struct SendRootUpdatedFilter {
        #[ethevent(indexed)]
        pub block_hash: [u8; 32],
        #[ethevent(indexed)]
        pub output_root: [u8; 32],
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IOutboxEvents {
        OutBoxTransactionExecutedFilter(OutBoxTransactionExecutedFilter),
        SendRootUpdatedFilter(SendRootUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IOutboxEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OutBoxTransactionExecutedFilter::decode_log(log) {
                return Ok(IOutboxEvents::OutBoxTransactionExecutedFilter(decoded));
            }
            if let Ok(decoded) = SendRootUpdatedFilter::decode_log(log) {
                return Ok(IOutboxEvents::SendRootUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IOutboxEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OutBoxTransactionExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendRootUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OutBoxTransactionExecutedFilter> for IOutboxEvents {
        fn from(value: OutBoxTransactionExecutedFilter) -> Self {
            Self::OutBoxTransactionExecutedFilter(value)
        }
    }
    impl ::core::convert::From<SendRootUpdatedFilter> for IOutboxEvents {
        fn from(value: SendRootUpdatedFilter) -> Self {
            Self::SendRootUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `OUTBOX_VERSION` function with signature `OUTBOX_VERSION()` and selector `0xc75184df`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "OUTBOX_VERSION", abi = "OUTBOX_VERSION()")]
    pub struct OutboxVersionCall;
    ///Container type for all input parameters for the `bridge` function with signature `bridge()` and selector `0xe78cea92`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "bridge", abi = "bridge()")]
    pub struct BridgeCall;
    ///Container type for all input parameters for the `calculateItemHash` function with signature `calculateItemHash(address,address,uint256,uint256,uint256,uint256,bytes)` and selector `0x9f0c04bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "calculateItemHash",
        abi = "calculateItemHash(address,address,uint256,uint256,uint256,uint256,bytes)"
    )]
    pub struct CalculateItemHashCall {
        pub l_2_sender: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub l_2_block: ::ethers::core::types::U256,
        pub l_1_block: ::ethers::core::types::U256,
        pub l_2_timestamp: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `calculateMerkleRoot` function with signature `calculateMerkleRoot(bytes32[],uint256,bytes32)` and selector `0x007436d3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "calculateMerkleRoot",
        abi = "calculateMerkleRoot(bytes32[],uint256,bytes32)"
    )]
    pub struct CalculateMerkleRootCall {
        pub proof: ::std::vec::Vec<[u8; 32]>,
        pub path: ::ethers::core::types::U256,
        pub item: [u8; 32],
    }
    ///Container type for all input parameters for the `executeTransaction` function with signature `executeTransaction(bytes32[],uint256,address,address,uint256,uint256,uint256,uint256,bytes)` and selector `0x08635a95`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeTransaction",
        abi = "executeTransaction(bytes32[],uint256,address,address,uint256,uint256,uint256,uint256,bytes)"
    )]
    pub struct ExecuteTransactionCall {
        pub proof: ::std::vec::Vec<[u8; 32]>,
        pub index: ::ethers::core::types::U256,
        pub l_2_sender: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub l_2_block: ::ethers::core::types::U256,
        pub l_1_block: ::ethers::core::types::U256,
        pub l_2_timestamp: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeTransactionSimulation` function with signature `executeTransactionSimulation(uint256,address,address,uint256,uint256,uint256,uint256,bytes)` and selector `0x288e5b10`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "executeTransactionSimulation",
        abi = "executeTransactionSimulation(uint256,address,address,uint256,uint256,uint256,uint256,bytes)"
    )]
    pub struct ExecuteTransactionSimulationCall {
        pub index: ::ethers::core::types::U256,
        pub l_2_sender: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub l_2_block: ::ethers::core::types::U256,
        pub l_1_block: ::ethers::core::types::U256,
        pub l_2_timestamp: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isSpent` function with signature `isSpent(uint256)` and selector `0x5a129efe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isSpent", abi = "isSpent(uint256)")]
    pub struct IsSpentCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `l2ToL1Block` function with signature `l2ToL1Block()` and selector `0x46547790`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "l2ToL1Block", abi = "l2ToL1Block()")]
    pub struct L2ToL1BlockCall;
    ///Container type for all input parameters for the `l2ToL1EthBlock` function with signature `l2ToL1EthBlock()` and selector `0x8515bc6a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "l2ToL1EthBlock", abi = "l2ToL1EthBlock()")]
    pub struct L2ToL1EthBlockCall;
    ///Container type for all input parameters for the `l2ToL1OutputId` function with signature `l2ToL1OutputId()` and selector `0x72f2a8c7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "l2ToL1OutputId", abi = "l2ToL1OutputId()")]
    pub struct L2ToL1OutputIdCall;
    ///Container type for all input parameters for the `l2ToL1Sender` function with signature `l2ToL1Sender()` and selector `0x80648b02`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "l2ToL1Sender", abi = "l2ToL1Sender()")]
    pub struct L2ToL1SenderCall;
    ///Container type for all input parameters for the `l2ToL1Timestamp` function with signature `l2ToL1Timestamp()` and selector `0xb0f30537`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "l2ToL1Timestamp", abi = "l2ToL1Timestamp()")]
    pub struct L2ToL1TimestampCall;
    ///Container type for all input parameters for the `rollup` function with signature `rollup()` and selector `0xcb23bcb5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollup", abi = "rollup()")]
    pub struct RollupCall;
    ///Container type for all input parameters for the `roots` function with signature `roots(bytes32)` and selector `0xae6dead7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "roots", abi = "roots(bytes32)")]
    pub struct RootsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `spent` function with signature `spent(uint256)` and selector `0xd5b5cc23`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "spent", abi = "spent(uint256)")]
    pub struct SpentCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `updateSendRoot` function with signature `updateSendRoot(bytes32,bytes32)` and selector `0xa04cee60`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updateSendRoot", abi = "updateSendRoot(bytes32,bytes32)")]
    pub struct UpdateSendRootCall {
        pub send_root: [u8; 32],
        pub l_2_block_hash: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IOutboxCalls {
        OutboxVersion(OutboxVersionCall),
        Bridge(BridgeCall),
        CalculateItemHash(CalculateItemHashCall),
        CalculateMerkleRoot(CalculateMerkleRootCall),
        ExecuteTransaction(ExecuteTransactionCall),
        ExecuteTransactionSimulation(ExecuteTransactionSimulationCall),
        IsSpent(IsSpentCall),
        L2ToL1Block(L2ToL1BlockCall),
        L2ToL1EthBlock(L2ToL1EthBlockCall),
        L2ToL1OutputId(L2ToL1OutputIdCall),
        L2ToL1Sender(L2ToL1SenderCall),
        L2ToL1Timestamp(L2ToL1TimestampCall),
        Rollup(RollupCall),
        Roots(RootsCall),
        Spent(SpentCall),
        UpdateSendRoot(UpdateSendRootCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOutboxCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <OutboxVersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutboxVersion(decoded));
            }
            if let Ok(decoded)
                = <BridgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Bridge(decoded));
            }
            if let Ok(decoded)
                = <CalculateItemHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateItemHash(decoded));
            }
            if let Ok(decoded)
                = <CalculateMerkleRootCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CalculateMerkleRoot(decoded));
            }
            if let Ok(decoded)
                = <ExecuteTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteTransaction(decoded));
            }
            if let Ok(decoded)
                = <ExecuteTransactionSimulationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteTransactionSimulation(decoded));
            }
            if let Ok(decoded)
                = <IsSpentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSpent(decoded));
            }
            if let Ok(decoded)
                = <L2ToL1BlockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::L2ToL1Block(decoded));
            }
            if let Ok(decoded)
                = <L2ToL1EthBlockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::L2ToL1EthBlock(decoded));
            }
            if let Ok(decoded)
                = <L2ToL1OutputIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::L2ToL1OutputId(decoded));
            }
            if let Ok(decoded)
                = <L2ToL1SenderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::L2ToL1Sender(decoded));
            }
            if let Ok(decoded)
                = <L2ToL1TimestampCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::L2ToL1Timestamp(decoded));
            }
            if let Ok(decoded)
                = <RollupCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rollup(decoded));
            }
            if let Ok(decoded)
                = <RootsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Roots(decoded));
            }
            if let Ok(decoded)
                = <SpentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Spent(decoded));
            }
            if let Ok(decoded)
                = <UpdateSendRootCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateSendRoot(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOutboxCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OutboxVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bridge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateItemHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateMerkleRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteTransactionSimulation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSpent(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::L2ToL1Block(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2ToL1EthBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2ToL1OutputId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2ToL1Sender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::L2ToL1Timestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rollup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Roots(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Spent(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateSendRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IOutboxCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OutboxVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateItemHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateMerkleRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteTransactionSimulation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2ToL1Block(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2ToL1EthBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2ToL1OutputId(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2ToL1Sender(element) => ::core::fmt::Display::fmt(element, f),
                Self::L2ToL1Timestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rollup(element) => ::core::fmt::Display::fmt(element, f),
                Self::Roots(element) => ::core::fmt::Display::fmt(element, f),
                Self::Spent(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSendRoot(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OutboxVersionCall> for IOutboxCalls {
        fn from(value: OutboxVersionCall) -> Self {
            Self::OutboxVersion(value)
        }
    }
    impl ::core::convert::From<BridgeCall> for IOutboxCalls {
        fn from(value: BridgeCall) -> Self {
            Self::Bridge(value)
        }
    }
    impl ::core::convert::From<CalculateItemHashCall> for IOutboxCalls {
        fn from(value: CalculateItemHashCall) -> Self {
            Self::CalculateItemHash(value)
        }
    }
    impl ::core::convert::From<CalculateMerkleRootCall> for IOutboxCalls {
        fn from(value: CalculateMerkleRootCall) -> Self {
            Self::CalculateMerkleRoot(value)
        }
    }
    impl ::core::convert::From<ExecuteTransactionCall> for IOutboxCalls {
        fn from(value: ExecuteTransactionCall) -> Self {
            Self::ExecuteTransaction(value)
        }
    }
    impl ::core::convert::From<ExecuteTransactionSimulationCall> for IOutboxCalls {
        fn from(value: ExecuteTransactionSimulationCall) -> Self {
            Self::ExecuteTransactionSimulation(value)
        }
    }
    impl ::core::convert::From<IsSpentCall> for IOutboxCalls {
        fn from(value: IsSpentCall) -> Self {
            Self::IsSpent(value)
        }
    }
    impl ::core::convert::From<L2ToL1BlockCall> for IOutboxCalls {
        fn from(value: L2ToL1BlockCall) -> Self {
            Self::L2ToL1Block(value)
        }
    }
    impl ::core::convert::From<L2ToL1EthBlockCall> for IOutboxCalls {
        fn from(value: L2ToL1EthBlockCall) -> Self {
            Self::L2ToL1EthBlock(value)
        }
    }
    impl ::core::convert::From<L2ToL1OutputIdCall> for IOutboxCalls {
        fn from(value: L2ToL1OutputIdCall) -> Self {
            Self::L2ToL1OutputId(value)
        }
    }
    impl ::core::convert::From<L2ToL1SenderCall> for IOutboxCalls {
        fn from(value: L2ToL1SenderCall) -> Self {
            Self::L2ToL1Sender(value)
        }
    }
    impl ::core::convert::From<L2ToL1TimestampCall> for IOutboxCalls {
        fn from(value: L2ToL1TimestampCall) -> Self {
            Self::L2ToL1Timestamp(value)
        }
    }
    impl ::core::convert::From<RollupCall> for IOutboxCalls {
        fn from(value: RollupCall) -> Self {
            Self::Rollup(value)
        }
    }
    impl ::core::convert::From<RootsCall> for IOutboxCalls {
        fn from(value: RootsCall) -> Self {
            Self::Roots(value)
        }
    }
    impl ::core::convert::From<SpentCall> for IOutboxCalls {
        fn from(value: SpentCall) -> Self {
            Self::Spent(value)
        }
    }
    impl ::core::convert::From<UpdateSendRootCall> for IOutboxCalls {
        fn from(value: UpdateSendRootCall) -> Self {
            Self::UpdateSendRoot(value)
        }
    }
    ///Container type for all return fields from the `OUTBOX_VERSION` function with signature `OUTBOX_VERSION()` and selector `0xc75184df`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OutboxVersionReturn(pub u128);
    ///Container type for all return fields from the `bridge` function with signature `bridge()` and selector `0xe78cea92`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BridgeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `calculateItemHash` function with signature `calculateItemHash(address,address,uint256,uint256,uint256,uint256,bytes)` and selector `0x9f0c04bf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateItemHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateMerkleRoot` function with signature `calculateMerkleRoot(bytes32[],uint256,bytes32)` and selector `0x007436d3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CalculateMerkleRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isSpent` function with signature `isSpent(uint256)` and selector `0x5a129efe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsSpentReturn(pub bool);
    ///Container type for all return fields from the `l2ToL1Block` function with signature `l2ToL1Block()` and selector `0x46547790`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct L2ToL1BlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2ToL1EthBlock` function with signature `l2ToL1EthBlock()` and selector `0x8515bc6a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct L2ToL1EthBlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `l2ToL1OutputId` function with signature `l2ToL1OutputId()` and selector `0x72f2a8c7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct L2ToL1OutputIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `l2ToL1Sender` function with signature `l2ToL1Sender()` and selector `0x80648b02`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct L2ToL1SenderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `l2ToL1Timestamp` function with signature `l2ToL1Timestamp()` and selector `0xb0f30537`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct L2ToL1TimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rollup` function with signature `rollup()` and selector `0xcb23bcb5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RollupReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `roots` function with signature `roots(bytes32)` and selector `0xae6dead7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RootsReturn(pub [u8; 32]);
    ///Container type for all return fields from the `spent` function with signature `spent(uint256)` and selector `0xd5b5cc23`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SpentReturn(pub [u8; 32]);
}
