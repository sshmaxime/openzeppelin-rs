pub use ierc777::*;
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
pub mod ierc777 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AuthorizedOperator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Burned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Minted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RevokedOperator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Sent\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"authorizeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultOperators\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"granularity\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenHolder\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOperatorFor\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"operatorBurn\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"operatorSend\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeOperator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"send\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC777_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IERC777<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC777<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC777<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC777<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC777<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC777)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IERC777<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IERC777_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `authorizeOperator` (0x959b8c3f) function
        pub fn authorize_operator(
            &self,
            operator: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 155, 140, 63], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0xfe9d9303) function
        pub fn burn(
            &self,
            amount: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 157, 147, 3], (amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultOperators` (0x06e48538) function
        pub fn default_operators(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers_core::types::Address>,
        > {
            self.0
                .method_hash([6, 228, 133, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `granularity` (0x556f0dc7) function
        pub fn granularity(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([85, 111, 13, 199], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperatorFor` (0xd95b6371) function
        pub fn is_operator_for(
            &self,
            operator: ::ethers_core::types::Address,
            token_holder: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 91, 99, 113], (operator, token_holder))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorBurn` (0xfc673c4f) function
        pub fn operator_burn(
            &self,
            account: ::ethers_core::types::Address,
            amount: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            operator_data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 103, 60, 79], (account, amount, data, operator_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSend` (0x62ad1b83) function
        pub fn operator_send(
            &self,
            sender: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            amount: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            operator_data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [98, 173, 27, 131],
                    (sender, recipient, amount, data, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeOperator` (0xfad8b32a) function
        pub fn revoke_operator(
            &self,
            operator: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 216, 179, 42], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `send` (0x9bd9bbc6) function
        pub fn send(
            &self,
            recipient: ::ethers_core::types::Address,
            amount: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 217, 187, 198], (recipient, amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AuthorizedOperator` event
        pub fn authorized_operator_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthorizedOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burned` event
        pub fn burned_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, BurnedFilter> {
            self.0.event()
        }
        ///Gets the contract's `Minted` event
        pub fn minted_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MintedFilter> {
            self.0.event()
        }
        ///Gets the contract's `RevokedOperator` event
        pub fn revoked_operator_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RevokedOperatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Sent` event
        pub fn sent_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, SentFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IERC777Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IERC777<M> {
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
    #[ethevent(name = "AuthorizedOperator", abi = "AuthorizedOperator(address,address)")]
    pub struct AuthorizedOperatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers_core::types::Address,
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
    #[ethevent(name = "Burned", abi = "Burned(address,address,uint256,bytes,bytes)")]
    pub struct BurnedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub operator_data: ::ethers_core::types::Bytes,
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
    #[ethevent(name = "Minted", abi = "Minted(address,address,uint256,bytes,bytes)")]
    pub struct MintedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub operator_data: ::ethers_core::types::Bytes,
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
    #[ethevent(name = "RevokedOperator", abi = "RevokedOperator(address,address)")]
    pub struct RevokedOperatorFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub token_holder: ::ethers_core::types::Address,
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
    #[ethevent(name = "Sent", abi = "Sent(address,address,address,uint256,bytes,bytes)")]
    pub struct SentFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers_core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub operator_data: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC777Events {
        AuthorizedOperatorFilter(AuthorizedOperatorFilter),
        BurnedFilter(BurnedFilter),
        MintedFilter(MintedFilter),
        RevokedOperatorFilter(RevokedOperatorFilter),
        SentFilter(SentFilter),
    }
    impl ::ethers_contract::EthLogDecode for IERC777Events {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = AuthorizedOperatorFilter::decode_log(log) {
                return Ok(IERC777Events::AuthorizedOperatorFilter(decoded));
            }
            if let Ok(decoded) = BurnedFilter::decode_log(log) {
                return Ok(IERC777Events::BurnedFilter(decoded));
            }
            if let Ok(decoded) = MintedFilter::decode_log(log) {
                return Ok(IERC777Events::MintedFilter(decoded));
            }
            if let Ok(decoded) = RevokedOperatorFilter::decode_log(log) {
                return Ok(IERC777Events::RevokedOperatorFilter(decoded));
            }
            if let Ok(decoded) = SentFilter::decode_log(log) {
                return Ok(IERC777Events::SentFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IERC777Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuthorizedOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BurnedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokedOperatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SentFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AuthorizedOperatorFilter> for IERC777Events {
        fn from(value: AuthorizedOperatorFilter) -> Self {
            Self::AuthorizedOperatorFilter(value)
        }
    }
    impl ::core::convert::From<BurnedFilter> for IERC777Events {
        fn from(value: BurnedFilter) -> Self {
            Self::BurnedFilter(value)
        }
    }
    impl ::core::convert::From<MintedFilter> for IERC777Events {
        fn from(value: MintedFilter) -> Self {
            Self::MintedFilter(value)
        }
    }
    impl ::core::convert::From<RevokedOperatorFilter> for IERC777Events {
        fn from(value: RevokedOperatorFilter) -> Self {
            Self::RevokedOperatorFilter(value)
        }
    }
    impl ::core::convert::From<SentFilter> for IERC777Events {
        fn from(value: SentFilter) -> Self {
            Self::SentFilter(value)
        }
    }
    ///Container type for all input parameters for the `authorizeOperator` function with signature `authorizeOperator(address)` and selector `0x959b8c3f`
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
    #[ethcall(name = "authorizeOperator", abi = "authorizeOperator(address)")]
    pub struct AuthorizeOperatorCall {
        pub operator: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256,bytes)` and selector `0xfe9d9303`
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
    #[ethcall(name = "burn", abi = "burn(uint256,bytes)")]
    pub struct BurnCall {
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `defaultOperators` function with signature `defaultOperators()` and selector `0x06e48538`
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
    #[ethcall(name = "defaultOperators", abi = "defaultOperators()")]
    pub struct DefaultOperatorsCall;
    ///Container type for all input parameters for the `granularity` function with signature `granularity()` and selector `0x556f0dc7`
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
    #[ethcall(name = "granularity", abi = "granularity()")]
    pub struct GranularityCall;
    ///Container type for all input parameters for the `isOperatorFor` function with signature `isOperatorFor(address,address)` and selector `0xd95b6371`
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
    #[ethcall(name = "isOperatorFor", abi = "isOperatorFor(address,address)")]
    pub struct IsOperatorForCall {
        pub operator: ::ethers_core::types::Address,
        pub token_holder: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `operatorBurn` function with signature `operatorBurn(address,uint256,bytes,bytes)` and selector `0xfc673c4f`
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
    #[ethcall(name = "operatorBurn", abi = "operatorBurn(address,uint256,bytes,bytes)")]
    pub struct OperatorBurnCall {
        pub account: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub operator_data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `operatorSend` function with signature `operatorSend(address,address,uint256,bytes,bytes)` and selector `0x62ad1b83`
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
        name = "operatorSend",
        abi = "operatorSend(address,address,uint256,bytes,bytes)"
    )]
    pub struct OperatorSendCall {
        pub sender: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub operator_data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `revokeOperator` function with signature `revokeOperator(address)` and selector `0xfad8b32a`
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
    #[ethcall(name = "revokeOperator", abi = "revokeOperator(address)")]
    pub struct RevokeOperatorCall {
        pub operator: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `send` function with signature `send(address,uint256,bytes)` and selector `0x9bd9bbc6`
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
    #[ethcall(name = "send", abi = "send(address,uint256,bytes)")]
    pub struct SendCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC777Calls {
        AuthorizeOperator(AuthorizeOperatorCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        DefaultOperators(DefaultOperatorsCall),
        Granularity(GranularityCall),
        IsOperatorFor(IsOperatorForCall),
        Name(NameCall),
        OperatorBurn(OperatorBurnCall),
        OperatorSend(OperatorSendCall),
        RevokeOperator(RevokeOperatorCall),
        Send(SendCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
    }
    impl ::ethers_core::abi::AbiDecode for IERC777Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AuthorizeOperatorCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AuthorizeOperator(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BurnCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <DefaultOperatorsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultOperators(decoded));
            }
            if let Ok(decoded)
                = <GranularityCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Granularity(decoded));
            }
            if let Ok(decoded)
                = <IsOperatorForCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsOperatorFor(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OperatorBurnCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OperatorBurn(decoded));
            }
            if let Ok(decoded)
                = <OperatorSendCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OperatorSend(decoded));
            }
            if let Ok(decoded)
                = <RevokeOperatorCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeOperator(decoded));
            }
            if let Ok(decoded)
                = <SendCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Send(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IERC777Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AuthorizeOperator(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DefaultOperators(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Granularity(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsOperatorFor(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OperatorBurn(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSend(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevokeOperator(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Send(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC777Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuthorizeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::Granularity(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperatorFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorBurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSend(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Send(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AuthorizeOperatorCall> for IERC777Calls {
        fn from(value: AuthorizeOperatorCall) -> Self {
            Self::AuthorizeOperator(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for IERC777Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for IERC777Calls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DefaultOperatorsCall> for IERC777Calls {
        fn from(value: DefaultOperatorsCall) -> Self {
            Self::DefaultOperators(value)
        }
    }
    impl ::core::convert::From<GranularityCall> for IERC777Calls {
        fn from(value: GranularityCall) -> Self {
            Self::Granularity(value)
        }
    }
    impl ::core::convert::From<IsOperatorForCall> for IERC777Calls {
        fn from(value: IsOperatorForCall) -> Self {
            Self::IsOperatorFor(value)
        }
    }
    impl ::core::convert::From<NameCall> for IERC777Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OperatorBurnCall> for IERC777Calls {
        fn from(value: OperatorBurnCall) -> Self {
            Self::OperatorBurn(value)
        }
    }
    impl ::core::convert::From<OperatorSendCall> for IERC777Calls {
        fn from(value: OperatorSendCall) -> Self {
            Self::OperatorSend(value)
        }
    }
    impl ::core::convert::From<RevokeOperatorCall> for IERC777Calls {
        fn from(value: RevokeOperatorCall) -> Self {
            Self::RevokeOperator(value)
        }
    }
    impl ::core::convert::From<SendCall> for IERC777Calls {
        fn from(value: SendCall) -> Self {
            Self::Send(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for IERC777Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for IERC777Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `defaultOperators` function with signature `defaultOperators()` and selector `0x06e48538`
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
    pub struct DefaultOperatorsReturn(
        pub ::std::vec::Vec<::ethers_core::types::Address>,
    );
    ///Container type for all return fields from the `granularity` function with signature `granularity()` and selector `0x556f0dc7`
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
    pub struct GranularityReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `isOperatorFor` function with signature `isOperatorFor(address,address)` and selector `0xd95b6371`
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
    pub struct IsOperatorForReturn(pub bool);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers_core::types::U256);
}
