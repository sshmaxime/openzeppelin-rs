pub use i_compound_timelock::*;
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
pub mod i_compound_timelock {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CancelTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExecuteTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newDelay\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewDelay\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPendingAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"NewPendingAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"QueueTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"GRACE_PERIOD\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAXIMUM_DELAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MINIMUM_DELAY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTransaction\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"signature\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queueTransaction\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"queuedTransactions\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDelay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingAdmin\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICOMPOUNDTIMELOCK_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ICompoundTimelock<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICompoundTimelock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICompoundTimelock<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICompoundTimelock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICompoundTimelock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ICompoundTimelock)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> ICompoundTimelock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ICOMPOUNDTIMELOCK_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `GRACE_PERIOD` (0xc1a287e2) function
        pub fn grace_period(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([193, 162, 135, 226], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAXIMUM_DELAY` (0x7d645fab) function
        pub fn maximum_delay(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([125, 100, 95, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINIMUM_DELAY` (0xb1b43ae5) function
        pub fn minimum_delay(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([177, 180, 58, 229], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptAdmin` (0x0e18b681) function
        pub fn accept_admin(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 24, 182, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelTransaction` (0x591fcdfe) function
        pub fn cancel_transaction(
            &self,
            target: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            signature: ::std::string::String,
            data: ::ethers_core::types::Bytes,
            eta: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 31, 205, 254], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delay` (0x6a42b8f8) function
        pub fn delay(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([106, 66, 184, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeTransaction` (0x0825f38f) function
        pub fn execute_transaction(
            &self,
            target: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            signature: ::std::string::String,
            data: ::ethers_core::types::Bytes,
            eta: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Bytes> {
            self.0
                .method_hash([8, 37, 243, 143], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingAdmin` (0x26782247) function
        pub fn pending_admin(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([38, 120, 34, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueTransaction` (0x3a66f901) function
        pub fn queue_transaction(
            &self,
            target: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            signature: ::std::string::String,
            data: ::ethers_core::types::Bytes,
            eta: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([58, 102, 249, 1], (target, value, signature, data, eta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queuedTransactions` (0xf2b06537) function
        pub fn queued_transactions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([242, 176, 101, 55], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDelay` (0xe177246e) function
        pub fn set_delay(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 119, 36, 110], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPendingAdmin` (0x4dd18bf5) function
        pub fn set_pending_admin(
            &self,
            p0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 209, 139, 245], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CancelTransaction` event
        pub fn cancel_transaction_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CancelTransactionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExecuteTransaction` event
        pub fn execute_transaction_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecuteTransactionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewAdmin` event
        pub fn new_admin_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, NewAdminFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewDelay` event
        pub fn new_delay_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, NewDelayFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewPendingAdmin` event
        pub fn new_pending_admin_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPendingAdminFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QueueTransaction` event
        pub fn queue_transaction_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QueueTransactionFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ICompoundTimelockEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for ICompoundTimelock<M> {
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
        name = "CancelTransaction",
        abi = "CancelTransaction(bytes32,address,uint256,string,bytes,uint256)"
    )]
    pub struct CancelTransactionFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub signature: ::std::string::String,
        pub data: ::ethers_core::types::Bytes,
        pub eta: ::ethers_core::types::U256,
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
        name = "ExecuteTransaction",
        abi = "ExecuteTransaction(bytes32,address,uint256,string,bytes,uint256)"
    )]
    pub struct ExecuteTransactionFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub signature: ::std::string::String,
        pub data: ::ethers_core::types::Bytes,
        pub eta: ::ethers_core::types::U256,
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
    #[ethevent(name = "NewAdmin", abi = "NewAdmin(address)")]
    pub struct NewAdminFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers_core::types::Address,
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
    #[ethevent(name = "NewDelay", abi = "NewDelay(uint256)")]
    pub struct NewDelayFilter {
        #[ethevent(indexed)]
        pub new_delay: ::ethers_core::types::U256,
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
    #[ethevent(name = "NewPendingAdmin", abi = "NewPendingAdmin(address)")]
    pub struct NewPendingAdminFilter {
        #[ethevent(indexed)]
        pub new_pending_admin: ::ethers_core::types::Address,
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
        name = "QueueTransaction",
        abi = "QueueTransaction(bytes32,address,uint256,string,bytes,uint256)"
    )]
    pub struct QueueTransactionFilter {
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub signature: ::std::string::String,
        pub data: ::ethers_core::types::Bytes,
        pub eta: ::ethers_core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ICompoundTimelockEvents {
        CancelTransactionFilter(CancelTransactionFilter),
        ExecuteTransactionFilter(ExecuteTransactionFilter),
        NewAdminFilter(NewAdminFilter),
        NewDelayFilter(NewDelayFilter),
        NewPendingAdminFilter(NewPendingAdminFilter),
        QueueTransactionFilter(QueueTransactionFilter),
    }
    impl ::ethers_contract::EthLogDecode for ICompoundTimelockEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = CancelTransactionFilter::decode_log(log) {
                return Ok(ICompoundTimelockEvents::CancelTransactionFilter(decoded));
            }
            if let Ok(decoded) = ExecuteTransactionFilter::decode_log(log) {
                return Ok(ICompoundTimelockEvents::ExecuteTransactionFilter(decoded));
            }
            if let Ok(decoded) = NewAdminFilter::decode_log(log) {
                return Ok(ICompoundTimelockEvents::NewAdminFilter(decoded));
            }
            if let Ok(decoded) = NewDelayFilter::decode_log(log) {
                return Ok(ICompoundTimelockEvents::NewDelayFilter(decoded));
            }
            if let Ok(decoded) = NewPendingAdminFilter::decode_log(log) {
                return Ok(ICompoundTimelockEvents::NewPendingAdminFilter(decoded));
            }
            if let Ok(decoded) = QueueTransactionFilter::decode_log(log) {
                return Ok(ICompoundTimelockEvents::QueueTransactionFilter(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ICompoundTimelockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CancelTransactionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecuteTransactionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewAdminFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewDelayFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPendingAdminFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QueueTransactionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CancelTransactionFilter> for ICompoundTimelockEvents {
        fn from(value: CancelTransactionFilter) -> Self {
            Self::CancelTransactionFilter(value)
        }
    }
    impl ::core::convert::From<ExecuteTransactionFilter> for ICompoundTimelockEvents {
        fn from(value: ExecuteTransactionFilter) -> Self {
            Self::ExecuteTransactionFilter(value)
        }
    }
    impl ::core::convert::From<NewAdminFilter> for ICompoundTimelockEvents {
        fn from(value: NewAdminFilter) -> Self {
            Self::NewAdminFilter(value)
        }
    }
    impl ::core::convert::From<NewDelayFilter> for ICompoundTimelockEvents {
        fn from(value: NewDelayFilter) -> Self {
            Self::NewDelayFilter(value)
        }
    }
    impl ::core::convert::From<NewPendingAdminFilter> for ICompoundTimelockEvents {
        fn from(value: NewPendingAdminFilter) -> Self {
            Self::NewPendingAdminFilter(value)
        }
    }
    impl ::core::convert::From<QueueTransactionFilter> for ICompoundTimelockEvents {
        fn from(value: QueueTransactionFilter) -> Self {
            Self::QueueTransactionFilter(value)
        }
    }
    ///Container type for all input parameters for the `GRACE_PERIOD` function with signature `GRACE_PERIOD()` and selector `0xc1a287e2`
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
    #[ethcall(name = "GRACE_PERIOD", abi = "GRACE_PERIOD()")]
    pub struct GracePeriodCall;
    ///Container type for all input parameters for the `MAXIMUM_DELAY` function with signature `MAXIMUM_DELAY()` and selector `0x7d645fab`
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
    #[ethcall(name = "MAXIMUM_DELAY", abi = "MAXIMUM_DELAY()")]
    pub struct MaximumDelayCall;
    ///Container type for all input parameters for the `MINIMUM_DELAY` function with signature `MINIMUM_DELAY()` and selector `0xb1b43ae5`
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
    #[ethcall(name = "MINIMUM_DELAY", abi = "MINIMUM_DELAY()")]
    pub struct MinimumDelayCall;
    ///Container type for all input parameters for the `acceptAdmin` function with signature `acceptAdmin()` and selector `0x0e18b681`
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
    #[ethcall(name = "acceptAdmin", abi = "acceptAdmin()")]
    pub struct AcceptAdminCall;
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `cancelTransaction` function with signature `cancelTransaction(address,uint256,string,bytes,uint256)` and selector `0x591fcdfe`
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
        name = "cancelTransaction",
        abi = "cancelTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct CancelTransactionCall {
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub signature: ::std::string::String,
        pub data: ::ethers_core::types::Bytes,
        pub eta: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `delay` function with signature `delay()` and selector `0x6a42b8f8`
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
    #[ethcall(name = "delay", abi = "delay()")]
    pub struct DelayCall;
    ///Container type for all input parameters for the `executeTransaction` function with signature `executeTransaction(address,uint256,string,bytes,uint256)` and selector `0x0825f38f`
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
        name = "executeTransaction",
        abi = "executeTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct ExecuteTransactionCall {
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub signature: ::std::string::String,
        pub data: ::ethers_core::types::Bytes,
        pub eta: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `pendingAdmin` function with signature `pendingAdmin()` and selector `0x26782247`
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
    #[ethcall(name = "pendingAdmin", abi = "pendingAdmin()")]
    pub struct PendingAdminCall;
    ///Container type for all input parameters for the `queueTransaction` function with signature `queueTransaction(address,uint256,string,bytes,uint256)` and selector `0x3a66f901`
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
        name = "queueTransaction",
        abi = "queueTransaction(address,uint256,string,bytes,uint256)"
    )]
    pub struct QueueTransactionCall {
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub signature: ::std::string::String,
        pub data: ::ethers_core::types::Bytes,
        pub eta: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `queuedTransactions` function with signature `queuedTransactions(bytes32)` and selector `0xf2b06537`
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
    #[ethcall(name = "queuedTransactions", abi = "queuedTransactions(bytes32)")]
    pub struct QueuedTransactionsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `setDelay` function with signature `setDelay(uint256)` and selector `0xe177246e`
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
    #[ethcall(name = "setDelay", abi = "setDelay(uint256)")]
    pub struct SetDelayCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `setPendingAdmin` function with signature `setPendingAdmin(address)` and selector `0x4dd18bf5`
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
    #[ethcall(name = "setPendingAdmin", abi = "setPendingAdmin(address)")]
    pub struct SetPendingAdminCall(pub ::ethers_core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ICompoundTimelockCalls {
        GracePeriod(GracePeriodCall),
        MaximumDelay(MaximumDelayCall),
        MinimumDelay(MinimumDelayCall),
        AcceptAdmin(AcceptAdminCall),
        Admin(AdminCall),
        CancelTransaction(CancelTransactionCall),
        Delay(DelayCall),
        ExecuteTransaction(ExecuteTransactionCall),
        PendingAdmin(PendingAdminCall),
        QueueTransaction(QueueTransactionCall),
        QueuedTransactions(QueuedTransactionsCall),
        SetDelay(SetDelayCall),
        SetPendingAdmin(SetPendingAdminCall),
    }
    impl ::ethers_core::abi::AbiDecode for ICompoundTimelockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GracePeriodCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GracePeriod(decoded));
            }
            if let Ok(decoded)
                = <MaximumDelayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaximumDelay(decoded));
            }
            if let Ok(decoded)
                = <MinimumDelayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinimumDelay(decoded));
            }
            if let Ok(decoded)
                = <AcceptAdminCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AcceptAdmin(decoded));
            }
            if let Ok(decoded)
                = <AdminCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded)
                = <CancelTransactionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelTransaction(decoded));
            }
            if let Ok(decoded)
                = <DelayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delay(decoded));
            }
            if let Ok(decoded)
                = <ExecuteTransactionCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteTransaction(decoded));
            }
            if let Ok(decoded)
                = <PendingAdminCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PendingAdmin(decoded));
            }
            if let Ok(decoded)
                = <QueueTransactionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueueTransaction(decoded));
            }
            if let Ok(decoded)
                = <QueuedTransactionsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::QueuedTransactions(decoded));
            }
            if let Ok(decoded)
                = <SetDelayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDelay(decoded));
            }
            if let Ok(decoded)
                = <SetPendingAdminCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPendingAdmin(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for ICompoundTimelockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GracePeriod(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::MaximumDelay(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::MinimumDelay(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::AcceptAdmin(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Admin(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CancelTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Delay(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecuteTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PendingAdmin(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::QueueTransaction(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::QueuedTransactions(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SetDelay(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SetPendingAdmin(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ICompoundTimelockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GracePeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximumDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delay(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteTransaction(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueueTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueuedTransactions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPendingAdmin(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GracePeriodCall> for ICompoundTimelockCalls {
        fn from(value: GracePeriodCall) -> Self {
            Self::GracePeriod(value)
        }
    }
    impl ::core::convert::From<MaximumDelayCall> for ICompoundTimelockCalls {
        fn from(value: MaximumDelayCall) -> Self {
            Self::MaximumDelay(value)
        }
    }
    impl ::core::convert::From<MinimumDelayCall> for ICompoundTimelockCalls {
        fn from(value: MinimumDelayCall) -> Self {
            Self::MinimumDelay(value)
        }
    }
    impl ::core::convert::From<AcceptAdminCall> for ICompoundTimelockCalls {
        fn from(value: AcceptAdminCall) -> Self {
            Self::AcceptAdmin(value)
        }
    }
    impl ::core::convert::From<AdminCall> for ICompoundTimelockCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<CancelTransactionCall> for ICompoundTimelockCalls {
        fn from(value: CancelTransactionCall) -> Self {
            Self::CancelTransaction(value)
        }
    }
    impl ::core::convert::From<DelayCall> for ICompoundTimelockCalls {
        fn from(value: DelayCall) -> Self {
            Self::Delay(value)
        }
    }
    impl ::core::convert::From<ExecuteTransactionCall> for ICompoundTimelockCalls {
        fn from(value: ExecuteTransactionCall) -> Self {
            Self::ExecuteTransaction(value)
        }
    }
    impl ::core::convert::From<PendingAdminCall> for ICompoundTimelockCalls {
        fn from(value: PendingAdminCall) -> Self {
            Self::PendingAdmin(value)
        }
    }
    impl ::core::convert::From<QueueTransactionCall> for ICompoundTimelockCalls {
        fn from(value: QueueTransactionCall) -> Self {
            Self::QueueTransaction(value)
        }
    }
    impl ::core::convert::From<QueuedTransactionsCall> for ICompoundTimelockCalls {
        fn from(value: QueuedTransactionsCall) -> Self {
            Self::QueuedTransactions(value)
        }
    }
    impl ::core::convert::From<SetDelayCall> for ICompoundTimelockCalls {
        fn from(value: SetDelayCall) -> Self {
            Self::SetDelay(value)
        }
    }
    impl ::core::convert::From<SetPendingAdminCall> for ICompoundTimelockCalls {
        fn from(value: SetPendingAdminCall) -> Self {
            Self::SetPendingAdmin(value)
        }
    }
    ///Container type for all return fields from the `GRACE_PERIOD` function with signature `GRACE_PERIOD()` and selector `0xc1a287e2`
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
    pub struct GracePeriodReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `MAXIMUM_DELAY` function with signature `MAXIMUM_DELAY()` and selector `0x7d645fab`
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
    pub struct MaximumDelayReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `MINIMUM_DELAY` function with signature `MINIMUM_DELAY()` and selector `0xb1b43ae5`
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
    pub struct MinimumDelayReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `delay` function with signature `delay()` and selector `0x6a42b8f8`
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
    pub struct DelayReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `executeTransaction` function with signature `executeTransaction(address,uint256,string,bytes,uint256)` and selector `0x0825f38f`
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
    pub struct ExecuteTransactionReturn(pub ::ethers_core::types::Bytes);
    ///Container type for all return fields from the `pendingAdmin` function with signature `pendingAdmin()` and selector `0x26782247`
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
    pub struct PendingAdminReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `queueTransaction` function with signature `queueTransaction(address,uint256,string,bytes,uint256)` and selector `0x3a66f901`
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
    pub struct QueueTransactionReturn(pub [u8; 32]);
    ///Container type for all return fields from the `queuedTransactions` function with signature `queuedTransactions(bytes32)` and selector `0xf2b06537`
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
    pub struct QueuedTransactionsReturn(pub bool);
}
