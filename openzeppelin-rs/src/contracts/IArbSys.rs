pub use i_arb_sys::*;
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
pub mod i_arb_sys {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"uniqueId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"batchNumber\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"indexInBatch\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"arbBlockNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"ethBlockNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"callvalue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"L2ToL1Transaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"hash\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"position\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"arbBlockNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"ethBlockNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"callvalue\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"L2ToL1Tx\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"reserved\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"position\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SendMerkleUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"arbBlockNum\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"arbBlockHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"arbBlockNumber\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"arbChainID\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"arbOSVersion\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getStorageGasAvailable\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isTopLevelCall\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"unused\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"mapL1SenderContractAddressToL2Alias\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"myCallersAddressWithoutAliasing\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sendMerkleTreeState\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"size\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"root\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"partials\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"sendTxToL1\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"wasMyCallersAddressAliased\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"destination\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"withdrawEth\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IARBSYS_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IArbSys<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IArbSys<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IArbSys<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IArbSys<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IArbSys<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IArbSys)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IArbSys<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IARBSYS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `arbBlockHash` (0x2b407a82) function
        pub fn arb_block_hash(
            &self,
            arb_block_num: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([43, 64, 122, 130], arb_block_num)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbBlockNumber` (0xa3b1b31d) function
        pub fn arb_block_number(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([163, 177, 179, 29], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbChainID` (0xd127f54a) function
        pub fn arb_chain_id(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([209, 39, 245, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbOSVersion` (0x051038f2) function
        pub fn arb_os_version(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([5, 16, 56, 242], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStorageGasAvailable` (0xa94597ff) function
        pub fn get_storage_gas_available(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([169, 69, 151, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTopLevelCall` (0x08bd624c) function
        pub fn is_top_level_call(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([8, 189, 98, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mapL1SenderContractAddressToL2Alias` (0x4dbbd506) function
        pub fn map_l1_sender_contract_address_to_l2_alias(
            &self,
            sender: ::ethers_core::types::Address,
            unused: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([77, 187, 213, 6], (sender, unused))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `myCallersAddressWithoutAliasing` (0xd74523b3) function
        pub fn my_callers_address_without_aliasing(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([215, 69, 35, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendMerkleTreeState` (0x7aeecd2a) function
        pub fn send_merkle_tree_state(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (::ethers_core::types::U256, [u8; 32], ::std::vec::Vec<[u8; 32]>),
        > {
            self.0
                .method_hash([122, 238, 205, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendTxToL1` (0x928c169a) function
        pub fn send_tx_to_l1(
            &self,
            destination: ::ethers_core::types::Address,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([146, 140, 22, 154], (destination, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wasMyCallersAddressAliased` (0x175a260b) function
        pub fn was_my_callers_address_aliased(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 90, 38, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawEth` (0x25e16063) function
        pub fn withdraw_eth(
            &self,
            destination: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([37, 225, 96, 99], destination)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `L2ToL1Transaction` event
        pub fn l2_to_l1_transaction_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            L2ToL1TransactionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `L2ToL1Tx` event
        pub fn l2_to_l1_tx_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, L2ToL1TxFilter> {
            self.0.event()
        }
        ///Gets the contract's `SendMerkleUpdate` event
        pub fn send_merkle_update_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SendMerkleUpdateFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IArbSysEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IArbSys<M> {
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
        name = "L2ToL1Transaction",
        abi = "L2ToL1Transaction(address,address,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bytes)"
    )]
    pub struct L2ToL1TransactionFilter {
        pub caller: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub destination: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub unique_id: ::ethers_core::types::U256,
        #[ethevent(indexed)]
        pub batch_number: ::ethers_core::types::U256,
        pub index_in_batch: ::ethers_core::types::U256,
        pub arb_block_num: ::ethers_core::types::U256,
        pub eth_block_num: ::ethers_core::types::U256,
        pub timestamp: ::ethers_core::types::U256,
        pub callvalue: ::ethers_core::types::U256,
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
        name = "L2ToL1Tx",
        abi = "L2ToL1Tx(address,address,uint256,uint256,uint256,uint256,uint256,uint256,bytes)"
    )]
    pub struct L2ToL1TxFilter {
        pub caller: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub destination: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub hash: ::ethers_core::types::U256,
        #[ethevent(indexed)]
        pub position: ::ethers_core::types::U256,
        pub arb_block_num: ::ethers_core::types::U256,
        pub eth_block_num: ::ethers_core::types::U256,
        pub timestamp: ::ethers_core::types::U256,
        pub callvalue: ::ethers_core::types::U256,
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
        name = "SendMerkleUpdate",
        abi = "SendMerkleUpdate(uint256,bytes32,uint256)"
    )]
    pub struct SendMerkleUpdateFilter {
        #[ethevent(indexed)]
        pub reserved: ::ethers_core::types::U256,
        #[ethevent(indexed)]
        pub hash: [u8; 32],
        #[ethevent(indexed)]
        pub position: ::ethers_core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IArbSysEvents {
        L2ToL1TransactionFilter(L2ToL1TransactionFilter),
        L2ToL1TxFilter(L2ToL1TxFilter),
        SendMerkleUpdateFilter(SendMerkleUpdateFilter),
    }
    impl ::ethers_contract::EthLogDecode for IArbSysEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = L2ToL1TransactionFilter::decode_log(log) {
                return Ok(IArbSysEvents::L2ToL1TransactionFilter(decoded));
            }
            if let Ok(decoded) = L2ToL1TxFilter::decode_log(log) {
                return Ok(IArbSysEvents::L2ToL1TxFilter(decoded));
            }
            if let Ok(decoded) = SendMerkleUpdateFilter::decode_log(log) {
                return Ok(IArbSysEvents::SendMerkleUpdateFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IArbSysEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::L2ToL1TransactionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::L2ToL1TxFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendMerkleUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<L2ToL1TransactionFilter> for IArbSysEvents {
        fn from(value: L2ToL1TransactionFilter) -> Self {
            Self::L2ToL1TransactionFilter(value)
        }
    }
    impl ::core::convert::From<L2ToL1TxFilter> for IArbSysEvents {
        fn from(value: L2ToL1TxFilter) -> Self {
            Self::L2ToL1TxFilter(value)
        }
    }
    impl ::core::convert::From<SendMerkleUpdateFilter> for IArbSysEvents {
        fn from(value: SendMerkleUpdateFilter) -> Self {
            Self::SendMerkleUpdateFilter(value)
        }
    }
    ///Container type for all input parameters for the `arbBlockHash` function with signature `arbBlockHash(uint256)` and selector `0x2b407a82`
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
    #[ethcall(name = "arbBlockHash", abi = "arbBlockHash(uint256)")]
    pub struct ArbBlockHashCall {
        pub arb_block_num: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `arbBlockNumber` function with signature `arbBlockNumber()` and selector `0xa3b1b31d`
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
    #[ethcall(name = "arbBlockNumber", abi = "arbBlockNumber()")]
    pub struct ArbBlockNumberCall;
    ///Container type for all input parameters for the `arbChainID` function with signature `arbChainID()` and selector `0xd127f54a`
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
    #[ethcall(name = "arbChainID", abi = "arbChainID()")]
    pub struct ArbChainIDCall;
    ///Container type for all input parameters for the `arbOSVersion` function with signature `arbOSVersion()` and selector `0x051038f2`
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
    #[ethcall(name = "arbOSVersion", abi = "arbOSVersion()")]
    pub struct ArbOSVersionCall;
    ///Container type for all input parameters for the `getStorageGasAvailable` function with signature `getStorageGasAvailable()` and selector `0xa94597ff`
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
    #[ethcall(name = "getStorageGasAvailable", abi = "getStorageGasAvailable()")]
    pub struct GetStorageGasAvailableCall;
    ///Container type for all input parameters for the `isTopLevelCall` function with signature `isTopLevelCall()` and selector `0x08bd624c`
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
    #[ethcall(name = "isTopLevelCall", abi = "isTopLevelCall()")]
    pub struct IsTopLevelCallCall;
    ///Container type for all input parameters for the `mapL1SenderContractAddressToL2Alias` function with signature `mapL1SenderContractAddressToL2Alias(address,address)` and selector `0x4dbbd506`
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
        name = "mapL1SenderContractAddressToL2Alias",
        abi = "mapL1SenderContractAddressToL2Alias(address,address)"
    )]
    pub struct MapL1SenderContractAddressToL2AliasCall {
        pub sender: ::ethers_core::types::Address,
        pub unused: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `myCallersAddressWithoutAliasing` function with signature `myCallersAddressWithoutAliasing()` and selector `0xd74523b3`
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
        name = "myCallersAddressWithoutAliasing",
        abi = "myCallersAddressWithoutAliasing()"
    )]
    pub struct MyCallersAddressWithoutAliasingCall;
    ///Container type for all input parameters for the `sendMerkleTreeState` function with signature `sendMerkleTreeState()` and selector `0x7aeecd2a`
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
    #[ethcall(name = "sendMerkleTreeState", abi = "sendMerkleTreeState()")]
    pub struct SendMerkleTreeStateCall;
    ///Container type for all input parameters for the `sendTxToL1` function with signature `sendTxToL1(address,bytes)` and selector `0x928c169a`
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
    #[ethcall(name = "sendTxToL1", abi = "sendTxToL1(address,bytes)")]
    pub struct SendTxToL1Call {
        pub destination: ::ethers_core::types::Address,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `wasMyCallersAddressAliased` function with signature `wasMyCallersAddressAliased()` and selector `0x175a260b`
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
    #[ethcall(name = "wasMyCallersAddressAliased", abi = "wasMyCallersAddressAliased()")]
    pub struct WasMyCallersAddressAliasedCall;
    ///Container type for all input parameters for the `withdrawEth` function with signature `withdrawEth(address)` and selector `0x25e16063`
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
    #[ethcall(name = "withdrawEth", abi = "withdrawEth(address)")]
    pub struct WithdrawEthCall {
        pub destination: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IArbSysCalls {
        ArbBlockHash(ArbBlockHashCall),
        ArbBlockNumber(ArbBlockNumberCall),
        ArbChainID(ArbChainIDCall),
        ArbOSVersion(ArbOSVersionCall),
        GetStorageGasAvailable(GetStorageGasAvailableCall),
        IsTopLevelCall(IsTopLevelCallCall),
        MapL1SenderContractAddressToL2Alias(MapL1SenderContractAddressToL2AliasCall),
        MyCallersAddressWithoutAliasing(MyCallersAddressWithoutAliasingCall),
        SendMerkleTreeState(SendMerkleTreeStateCall),
        SendTxToL1(SendTxToL1Call),
        WasMyCallersAddressAliased(WasMyCallersAddressAliasedCall),
        WithdrawEth(WithdrawEthCall),
    }
    impl ::ethers_core::abi::AbiDecode for IArbSysCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ArbBlockHashCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ArbBlockHash(decoded));
            }
            if let Ok(decoded)
                = <ArbBlockNumberCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ArbBlockNumber(decoded));
            }
            if let Ok(decoded)
                = <ArbChainIDCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ArbChainID(decoded));
            }
            if let Ok(decoded)
                = <ArbOSVersionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ArbOSVersion(decoded));
            }
            if let Ok(decoded)
                = <GetStorageGasAvailableCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetStorageGasAvailable(decoded));
            }
            if let Ok(decoded)
                = <IsTopLevelCallCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTopLevelCall(decoded));
            }
            if let Ok(decoded)
                = <MapL1SenderContractAddressToL2AliasCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MapL1SenderContractAddressToL2Alias(decoded));
            }
            if let Ok(decoded)
                = <MyCallersAddressWithoutAliasingCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MyCallersAddressWithoutAliasing(decoded));
            }
            if let Ok(decoded)
                = <SendMerkleTreeStateCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SendMerkleTreeState(decoded));
            }
            if let Ok(decoded)
                = <SendTxToL1Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendTxToL1(decoded));
            }
            if let Ok(decoded)
                = <WasMyCallersAddressAliasedCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WasMyCallersAddressAliased(decoded));
            }
            if let Ok(decoded)
                = <WithdrawEthCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawEth(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IArbSysCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ArbBlockHash(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ArbBlockNumber(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ArbChainID(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ArbOSVersion(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetStorageGasAvailable(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsTopLevelCall(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::MapL1SenderContractAddressToL2Alias(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::MyCallersAddressWithoutAliasing(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendMerkleTreeState(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SendTxToL1(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::WasMyCallersAddressAliased(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawEth(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IArbSysCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ArbBlockHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbChainID(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbOSVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStorageGasAvailable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsTopLevelCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::MapL1SenderContractAddressToL2Alias(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MyCallersAddressWithoutAliasing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendMerkleTreeState(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendTxToL1(element) => ::core::fmt::Display::fmt(element, f),
                Self::WasMyCallersAddressAliased(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawEth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ArbBlockHashCall> for IArbSysCalls {
        fn from(value: ArbBlockHashCall) -> Self {
            Self::ArbBlockHash(value)
        }
    }
    impl ::core::convert::From<ArbBlockNumberCall> for IArbSysCalls {
        fn from(value: ArbBlockNumberCall) -> Self {
            Self::ArbBlockNumber(value)
        }
    }
    impl ::core::convert::From<ArbChainIDCall> for IArbSysCalls {
        fn from(value: ArbChainIDCall) -> Self {
            Self::ArbChainID(value)
        }
    }
    impl ::core::convert::From<ArbOSVersionCall> for IArbSysCalls {
        fn from(value: ArbOSVersionCall) -> Self {
            Self::ArbOSVersion(value)
        }
    }
    impl ::core::convert::From<GetStorageGasAvailableCall> for IArbSysCalls {
        fn from(value: GetStorageGasAvailableCall) -> Self {
            Self::GetStorageGasAvailable(value)
        }
    }
    impl ::core::convert::From<IsTopLevelCallCall> for IArbSysCalls {
        fn from(value: IsTopLevelCallCall) -> Self {
            Self::IsTopLevelCall(value)
        }
    }
    impl ::core::convert::From<MapL1SenderContractAddressToL2AliasCall>
    for IArbSysCalls {
        fn from(value: MapL1SenderContractAddressToL2AliasCall) -> Self {
            Self::MapL1SenderContractAddressToL2Alias(value)
        }
    }
    impl ::core::convert::From<MyCallersAddressWithoutAliasingCall> for IArbSysCalls {
        fn from(value: MyCallersAddressWithoutAliasingCall) -> Self {
            Self::MyCallersAddressWithoutAliasing(value)
        }
    }
    impl ::core::convert::From<SendMerkleTreeStateCall> for IArbSysCalls {
        fn from(value: SendMerkleTreeStateCall) -> Self {
            Self::SendMerkleTreeState(value)
        }
    }
    impl ::core::convert::From<SendTxToL1Call> for IArbSysCalls {
        fn from(value: SendTxToL1Call) -> Self {
            Self::SendTxToL1(value)
        }
    }
    impl ::core::convert::From<WasMyCallersAddressAliasedCall> for IArbSysCalls {
        fn from(value: WasMyCallersAddressAliasedCall) -> Self {
            Self::WasMyCallersAddressAliased(value)
        }
    }
    impl ::core::convert::From<WithdrawEthCall> for IArbSysCalls {
        fn from(value: WithdrawEthCall) -> Self {
            Self::WithdrawEth(value)
        }
    }
    ///Container type for all return fields from the `arbBlockHash` function with signature `arbBlockHash(uint256)` and selector `0x2b407a82`
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
    pub struct ArbBlockHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `arbBlockNumber` function with signature `arbBlockNumber()` and selector `0xa3b1b31d`
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
    pub struct ArbBlockNumberReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `arbChainID` function with signature `arbChainID()` and selector `0xd127f54a`
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
    pub struct ArbChainIDReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `arbOSVersion` function with signature `arbOSVersion()` and selector `0x051038f2`
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
    pub struct ArbOSVersionReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getStorageGasAvailable` function with signature `getStorageGasAvailable()` and selector `0xa94597ff`
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
    pub struct GetStorageGasAvailableReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `isTopLevelCall` function with signature `isTopLevelCall()` and selector `0x08bd624c`
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
    pub struct IsTopLevelCallReturn(pub bool);
    ///Container type for all return fields from the `mapL1SenderContractAddressToL2Alias` function with signature `mapL1SenderContractAddressToL2Alias(address,address)` and selector `0x4dbbd506`
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
    pub struct MapL1SenderContractAddressToL2AliasReturn(
        pub ::ethers_core::types::Address,
    );
    ///Container type for all return fields from the `myCallersAddressWithoutAliasing` function with signature `myCallersAddressWithoutAliasing()` and selector `0xd74523b3`
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
    pub struct MyCallersAddressWithoutAliasingReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `sendMerkleTreeState` function with signature `sendMerkleTreeState()` and selector `0x7aeecd2a`
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
    pub struct SendMerkleTreeStateReturn {
        pub size: ::ethers_core::types::U256,
        pub root: [u8; 32],
        pub partials: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `sendTxToL1` function with signature `sendTxToL1(address,bytes)` and selector `0x928c169a`
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
    pub struct SendTxToL1Return(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `wasMyCallersAddressAliased` function with signature `wasMyCallersAddressAliased()` and selector `0x175a260b`
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
    pub struct WasMyCallersAddressAliasedReturn(pub bool);
    ///Container type for all return fields from the `withdrawEth` function with signature `withdrawEth(address)` and selector `0x25e16063`
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
    pub struct WithdrawEthReturn(pub ::ethers_core::types::U256);
}
