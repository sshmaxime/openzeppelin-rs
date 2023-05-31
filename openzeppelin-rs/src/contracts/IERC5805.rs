pub use ierc5805::*;
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
pub mod ierc5805 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"fromDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"toDelegate\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DelegateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegate\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"previousBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DelegateVotesChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CLOCK_MODE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"clock\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"delegatee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expiry\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"delegateBySig\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delegates\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timepoint\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPastTotalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timepoint\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPastVotes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVotes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC5805_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IERC5805<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC5805<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC5805<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC5805<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC5805<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC5805)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IERC5805<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IERC5805_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `CLOCK_MODE` (0x4bf5d7e9) function
        pub fn clock_mode(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 245, 215, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clock` (0x91ddadf4) function
        pub fn clock(&self) -> ::ethers_contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([145, 221, 173, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegate` (0x5c19a95c) function
        pub fn delegate(
            &self,
            delegatee: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 25, 169, 92], delegatee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateBySig` (0xc3cda520) function
        pub fn delegate_by_sig(
            &self,
            delegatee: ::ethers_core::types::Address,
            nonce: ::ethers_core::types::U256,
            expiry: ::ethers_core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 205, 165, 32], (delegatee, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegates` (0x587cde1e) function
        pub fn delegates(
            &self,
            account: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([88, 124, 222, 30], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPastTotalSupply` (0x8e539e8c) function
        pub fn get_past_total_supply(
            &self,
            timepoint: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([142, 83, 158, 140], timepoint)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPastVotes` (0x3a46b1a8) function
        pub fn get_past_votes(
            &self,
            account: ::ethers_core::types::Address,
            timepoint: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([58, 70, 177, 168], (account, timepoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotes` (0x9ab24eb0) function
        pub fn get_votes(
            &self,
            account: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([154, 178, 78, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DelegateChanged` event
        pub fn delegate_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegateChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DelegateVotesChanged` event
        pub fn delegate_votes_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegateVotesChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IERC5805Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IERC5805<M> {
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
        name = "DelegateChanged",
        abi = "DelegateChanged(address,address,address)"
    )]
    pub struct DelegateChangedFilter {
        #[ethevent(indexed)]
        pub delegator: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub from_delegate: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to_delegate: ::ethers_core::types::Address,
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
        name = "DelegateVotesChanged",
        abi = "DelegateVotesChanged(address,uint256,uint256)"
    )]
    pub struct DelegateVotesChangedFilter {
        #[ethevent(indexed)]
        pub delegate: ::ethers_core::types::Address,
        pub previous_balance: ::ethers_core::types::U256,
        pub new_balance: ::ethers_core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC5805Events {
        DelegateChangedFilter(DelegateChangedFilter),
        DelegateVotesChangedFilter(DelegateVotesChangedFilter),
    }
    impl ::ethers_contract::EthLogDecode for IERC5805Events {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = DelegateChangedFilter::decode_log(log) {
                return Ok(IERC5805Events::DelegateChangedFilter(decoded));
            }
            if let Ok(decoded) = DelegateVotesChangedFilter::decode_log(log) {
                return Ok(IERC5805Events::DelegateVotesChangedFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IERC5805Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelegateChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegateVotesChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DelegateChangedFilter> for IERC5805Events {
        fn from(value: DelegateChangedFilter) -> Self {
            Self::DelegateChangedFilter(value)
        }
    }
    impl ::core::convert::From<DelegateVotesChangedFilter> for IERC5805Events {
        fn from(value: DelegateVotesChangedFilter) -> Self {
            Self::DelegateVotesChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
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
    #[ethcall(name = "CLOCK_MODE", abi = "CLOCK_MODE()")]
    pub struct ClockModeCall;
    ///Container type for all input parameters for the `clock` function with signature `clock()` and selector `0x91ddadf4`
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
    #[ethcall(name = "clock", abi = "clock()")]
    pub struct ClockCall;
    ///Container type for all input parameters for the `delegate` function with signature `delegate(address)` and selector `0x5c19a95c`
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
    #[ethcall(name = "delegate", abi = "delegate(address)")]
    pub struct DelegateCall {
        pub delegatee: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `delegateBySig` function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`
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
        name = "delegateBySig",
        abi = "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DelegateBySigCall {
        pub delegatee: ::ethers_core::types::Address,
        pub nonce: ::ethers_core::types::U256,
        pub expiry: ::ethers_core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `delegates` function with signature `delegates(address)` and selector `0x587cde1e`
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
    #[ethcall(name = "delegates", abi = "delegates(address)")]
    pub struct DelegatesCall {
        pub account: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `getPastTotalSupply` function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`
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
    #[ethcall(name = "getPastTotalSupply", abi = "getPastTotalSupply(uint256)")]
    pub struct GetPastTotalSupplyCall {
        pub timepoint: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `getPastVotes` function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`
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
    #[ethcall(name = "getPastVotes", abi = "getPastVotes(address,uint256)")]
    pub struct GetPastVotesCall {
        pub account: ::ethers_core::types::Address,
        pub timepoint: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `getVotes` function with signature `getVotes(address)` and selector `0x9ab24eb0`
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
    #[ethcall(name = "getVotes", abi = "getVotes(address)")]
    pub struct GetVotesCall {
        pub account: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC5805Calls {
        ClockMode(ClockModeCall),
        Clock(ClockCall),
        Delegate(DelegateCall),
        DelegateBySig(DelegateBySigCall),
        Delegates(DelegatesCall),
        GetPastTotalSupply(GetPastTotalSupplyCall),
        GetPastVotes(GetPastVotesCall),
        GetVotes(GetVotesCall),
    }
    impl ::ethers_core::abi::AbiDecode for IERC5805Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ClockModeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClockMode(decoded));
            }
            if let Ok(decoded)
                = <ClockCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Clock(decoded));
            }
            if let Ok(decoded)
                = <DelegateCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegate(decoded));
            }
            if let Ok(decoded)
                = <DelegateBySigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelegateBySig(decoded));
            }
            if let Ok(decoded)
                = <DelegatesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegates(decoded));
            }
            if let Ok(decoded)
                = <GetPastTotalSupplyCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetPastTotalSupply(decoded));
            }
            if let Ok(decoded)
                = <GetPastVotesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPastVotes(decoded));
            }
            if let Ok(decoded)
                = <GetVotesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVotes(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IERC5805Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClockMode(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Clock(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Delegate(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DelegateBySig(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Delegates(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetPastTotalSupply(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetPastVotes(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetVotes(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IERC5805Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClockMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::Clock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateBySig(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegates(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPastTotalSupply(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPastVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClockModeCall> for IERC5805Calls {
        fn from(value: ClockModeCall) -> Self {
            Self::ClockMode(value)
        }
    }
    impl ::core::convert::From<ClockCall> for IERC5805Calls {
        fn from(value: ClockCall) -> Self {
            Self::Clock(value)
        }
    }
    impl ::core::convert::From<DelegateCall> for IERC5805Calls {
        fn from(value: DelegateCall) -> Self {
            Self::Delegate(value)
        }
    }
    impl ::core::convert::From<DelegateBySigCall> for IERC5805Calls {
        fn from(value: DelegateBySigCall) -> Self {
            Self::DelegateBySig(value)
        }
    }
    impl ::core::convert::From<DelegatesCall> for IERC5805Calls {
        fn from(value: DelegatesCall) -> Self {
            Self::Delegates(value)
        }
    }
    impl ::core::convert::From<GetPastTotalSupplyCall> for IERC5805Calls {
        fn from(value: GetPastTotalSupplyCall) -> Self {
            Self::GetPastTotalSupply(value)
        }
    }
    impl ::core::convert::From<GetPastVotesCall> for IERC5805Calls {
        fn from(value: GetPastVotesCall) -> Self {
            Self::GetPastVotes(value)
        }
    }
    impl ::core::convert::From<GetVotesCall> for IERC5805Calls {
        fn from(value: GetVotesCall) -> Self {
            Self::GetVotes(value)
        }
    }
    ///Container type for all return fields from the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
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
    pub struct ClockModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `clock` function with signature `clock()` and selector `0x91ddadf4`
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
    pub struct ClockReturn(pub u64);
    ///Container type for all return fields from the `delegates` function with signature `delegates(address)` and selector `0x587cde1e`
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
    pub struct DelegatesReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `getPastTotalSupply` function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`
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
    pub struct GetPastTotalSupplyReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getPastVotes` function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`
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
    pub struct GetPastVotesReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getVotes` function with signature `getVotes(address)` and selector `0x9ab24eb0`
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
    pub struct GetVotesReturn(pub ::ethers_core::types::U256);
}
