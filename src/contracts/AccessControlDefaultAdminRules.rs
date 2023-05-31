pub use access_control_default_admin_rules::*;
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
pub mod access_control_default_admin_rules {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"event\",\"name\":\"DefaultAdminDelayChangeCanceled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint48\",\"name\":\"newDelay\",\"type\":\"uint48\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint48\",\"name\":\"effectSchedule\",\"type\":\"uint48\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DefaultAdminDelayChangeScheduled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"DefaultAdminTransferCanceled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint48\",\"name\":\"acceptSchedule\",\"type\":\"uint48\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DefaultAdminTransferScheduled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"previousAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"newAdminRole\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleAdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleGranted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RoleRevoked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DEFAULT_ADMIN_ROLE\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptDefaultAdminTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"beginDefaultAdminTransfer\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelDefaultAdminTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint48\",\"name\":\"newDelay\",\"type\":\"uint48\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeDefaultAdminDelay\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultAdminDelay\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultAdminDelayIncreaseWait\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoleAdmin\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"grantRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingDefaultAdmin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"schedule\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingDefaultAdminDelay\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"newDelay\",\"type\":\"uint48\",\"components\":[]},{\"internalType\":\"uint48\",\"name\":\"schedule\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"role\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revokeRole\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rollbackDefaultAdminDelay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ACCESSCONTROLDEFAULTADMINRULES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct AccessControlDefaultAdminRules<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AccessControlDefaultAdminRules<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AccessControlDefaultAdminRules<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AccessControlDefaultAdminRules<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AccessControlDefaultAdminRules<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(AccessControlDefaultAdminRules))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AccessControlDefaultAdminRules<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ACCESSCONTROLDEFAULTADMINRULES_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function
        pub fn default_admin_role(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptDefaultAdminTransfer` (0xcefc1429) function
        pub fn accept_default_admin_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 252, 20, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beginDefaultAdminTransfer` (0x634e93da) function
        pub fn begin_default_admin_transfer(
            &self,
            new_admin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 78, 147, 218], new_admin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelDefaultAdminTransfer` (0xd602b9fd) function
        pub fn cancel_default_admin_transfer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 2, 185, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeDefaultAdminDelay` (0x649a5ec7) function
        pub fn change_default_admin_delay(
            &self,
            new_delay: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 154, 94, 199], new_delay)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdmin` (0x84ef8ffc) function
        pub fn default_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([132, 239, 143, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelay` (0xcc8463c8) function
        pub fn default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([204, 132, 99, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultAdminDelayIncreaseWait` (0x022d63fb) function
        pub fn default_admin_delay_increase_wait(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([2, 45, 99, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoleAdmin` (0x248a9ca3) function
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `grantRole` (0x2f2ff15d) function
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasRole` (0x91d14854) function
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingDefaultAdmin` (0xcf6eefb7) function
        pub fn pending_default_admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u64),
        > {
            self.0
                .method_hash([207, 110, 239, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingDefaultAdminDelay` (0xa1eda53c) function
        pub fn pending_default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64)> {
            self.0
                .method_hash([161, 237, 165, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceRole` (0x36568abe) function
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeRole` (0xd547741f) function
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollbackDefaultAdminDelay` (0x0aa6220b) function
        pub fn rollback_default_admin_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 166, 34, 11], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DefaultAdminDelayChangeCanceled` event
        pub fn default_admin_delay_change_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminDelayChangeScheduled` event
        pub fn default_admin_delay_change_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminDelayChangeScheduledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminTransferCanceled` event
        pub fn default_admin_transfer_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DefaultAdminTransferScheduled` event
        pub fn default_admin_transfer_scheduled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DefaultAdminTransferScheduledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleAdminChanged` event
        pub fn role_admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleAdminChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleGranted` event
        pub fn role_granted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleGrantedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoleRevoked` event
        pub fn role_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RoleRevokedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccessControlDefaultAdminRulesEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AccessControlDefaultAdminRules<M> {
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
        name = "DefaultAdminDelayChangeCanceled",
        abi = "DefaultAdminDelayChangeCanceled()"
    )]
    pub struct DefaultAdminDelayChangeCanceledFilter;
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
        name = "DefaultAdminDelayChangeScheduled",
        abi = "DefaultAdminDelayChangeScheduled(uint48,uint48)"
    )]
    pub struct DefaultAdminDelayChangeScheduledFilter {
        pub new_delay: u64,
        pub effect_schedule: u64,
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
        name = "DefaultAdminTransferCanceled",
        abi = "DefaultAdminTransferCanceled()"
    )]
    pub struct DefaultAdminTransferCanceledFilter;
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
        name = "DefaultAdminTransferScheduled",
        abi = "DefaultAdminTransferScheduled(address,uint48)"
    )]
    pub struct DefaultAdminTransferScheduledFilter {
        #[ethevent(indexed)]
        pub new_admin: ::ethers::core::types::Address,
        pub accept_schedule: u64,
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
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
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
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
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
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccessControlDefaultAdminRulesEvents {
        DefaultAdminDelayChangeCanceledFilter(DefaultAdminDelayChangeCanceledFilter),
        DefaultAdminDelayChangeScheduledFilter(DefaultAdminDelayChangeScheduledFilter),
        DefaultAdminTransferCanceledFilter(DefaultAdminTransferCanceledFilter),
        DefaultAdminTransferScheduledFilter(DefaultAdminTransferScheduledFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AccessControlDefaultAdminRulesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DefaultAdminDelayChangeCanceledFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::DefaultAdminDelayChangeCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = DefaultAdminDelayChangeScheduledFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::DefaultAdminDelayChangeScheduledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DefaultAdminTransferCanceledFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::DefaultAdminTransferCanceledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DefaultAdminTransferScheduledFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::DefaultAdminTransferScheduledFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::RoleAdminChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::RoleGrantedFilter(decoded),
                );
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(
                    AccessControlDefaultAdminRulesEvents::RoleRevokedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AccessControlDefaultAdminRulesEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminDelayChangeCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminDelayChangeScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminTransferCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdminTransferScheduledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleAdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoleGrantedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleRevokedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeCanceledFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminDelayChangeCanceledFilter) -> Self {
            Self::DefaultAdminDelayChangeCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayChangeScheduledFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminDelayChangeScheduledFilter) -> Self {
            Self::DefaultAdminDelayChangeScheduledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferCanceledFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminTransferCanceledFilter) -> Self {
            Self::DefaultAdminTransferCanceledFilter(value)
        }
    }
    impl ::core::convert::From<DefaultAdminTransferScheduledFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: DefaultAdminTransferScheduledFilter) -> Self {
            Self::DefaultAdminTransferScheduledFilter(value)
        }
    }
    impl ::core::convert::From<RoleAdminChangedFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: RoleAdminChangedFilter) -> Self {
            Self::RoleAdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<RoleGrantedFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: RoleGrantedFilter) -> Self {
            Self::RoleGrantedFilter(value)
        }
    }
    impl ::core::convert::From<RoleRevokedFilter>
    for AccessControlDefaultAdminRulesEvents {
        fn from(value: RoleRevokedFilter) -> Self {
            Self::RoleRevokedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    ///Container type for all input parameters for the `acceptDefaultAdminTransfer` function with signature `acceptDefaultAdminTransfer()` and selector `0xcefc1429`
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
    #[ethcall(name = "acceptDefaultAdminTransfer", abi = "acceptDefaultAdminTransfer()")]
    pub struct AcceptDefaultAdminTransferCall;
    ///Container type for all input parameters for the `beginDefaultAdminTransfer` function with signature `beginDefaultAdminTransfer(address)` and selector `0x634e93da`
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
        name = "beginDefaultAdminTransfer",
        abi = "beginDefaultAdminTransfer(address)"
    )]
    pub struct BeginDefaultAdminTransferCall {
        pub new_admin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `cancelDefaultAdminTransfer` function with signature `cancelDefaultAdminTransfer()` and selector `0xd602b9fd`
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
    #[ethcall(name = "cancelDefaultAdminTransfer", abi = "cancelDefaultAdminTransfer()")]
    pub struct CancelDefaultAdminTransferCall;
    ///Container type for all input parameters for the `changeDefaultAdminDelay` function with signature `changeDefaultAdminDelay(uint48)` and selector `0x649a5ec7`
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
    #[ethcall(name = "changeDefaultAdminDelay", abi = "changeDefaultAdminDelay(uint48)")]
    pub struct ChangeDefaultAdminDelayCall {
        pub new_delay: u64,
    }
    ///Container type for all input parameters for the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
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
    #[ethcall(name = "defaultAdmin", abi = "defaultAdmin()")]
    pub struct DefaultAdminCall;
    ///Container type for all input parameters for the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
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
    #[ethcall(name = "defaultAdminDelay", abi = "defaultAdminDelay()")]
    pub struct DefaultAdminDelayCall;
    ///Container type for all input parameters for the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
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
        name = "defaultAdminDelayIncreaseWait",
        abi = "defaultAdminDelayIncreaseWait()"
    )]
    pub struct DefaultAdminDelayIncreaseWaitCall;
    ///Container type for all input parameters for the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    ///Container type for all input parameters for the `grantRole` function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`
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
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
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
    #[ethcall(name = "pendingDefaultAdmin", abi = "pendingDefaultAdmin()")]
    pub struct PendingDefaultAdminCall;
    ///Container type for all input parameters for the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
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
    #[ethcall(name = "pendingDefaultAdminDelay", abi = "pendingDefaultAdminDelay()")]
    pub struct PendingDefaultAdminDelayCall;
    ///Container type for all input parameters for the `renounceRole` function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`
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
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `revokeRole` function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`
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
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rollbackDefaultAdminDelay` function with signature `rollbackDefaultAdminDelay()` and selector `0x0aa6220b`
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
    #[ethcall(name = "rollbackDefaultAdminDelay", abi = "rollbackDefaultAdminDelay()")]
    pub struct RollbackDefaultAdminDelayCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AccessControlDefaultAdminRulesCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        AcceptDefaultAdminTransfer(AcceptDefaultAdminTransferCall),
        BeginDefaultAdminTransfer(BeginDefaultAdminTransferCall),
        CancelDefaultAdminTransfer(CancelDefaultAdminTransferCall),
        ChangeDefaultAdminDelay(ChangeDefaultAdminDelayCall),
        DefaultAdmin(DefaultAdminCall),
        DefaultAdminDelay(DefaultAdminDelayCall),
        DefaultAdminDelayIncreaseWait(DefaultAdminDelayIncreaseWaitCall),
        GetRoleAdmin(GetRoleAdminCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        Owner(OwnerCall),
        PendingDefaultAdmin(PendingDefaultAdminCall),
        PendingDefaultAdminDelay(PendingDefaultAdminDelayCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        RollbackDefaultAdminDelay(RollbackDefaultAdminDelayCall),
        SupportsInterface(SupportsInterfaceCall),
    }
    impl ::ethers::core::abi::AbiDecode for AccessControlDefaultAdminRulesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DefaultAdminRoleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminRole(decoded));
            }
            if let Ok(decoded)
                = <AcceptDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AcceptDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded)
                = <BeginDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BeginDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded)
                = <CancelDefaultAdminTransferCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelDefaultAdminTransfer(decoded));
            }
            if let Ok(decoded)
                = <ChangeDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ChangeDefaultAdminDelay(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultAdmin(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminDelay(decoded));
            }
            if let Ok(decoded)
                = <DefaultAdminDelayIncreaseWaitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DefaultAdminDelayIncreaseWait(decoded));
            }
            if let Ok(decoded)
                = <GetRoleAdminCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoleAdmin(decoded));
            }
            if let Ok(decoded)
                = <GrantRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GrantRole(decoded));
            }
            if let Ok(decoded)
                = <HasRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasRole(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PendingDefaultAdminCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PendingDefaultAdmin(decoded));
            }
            if let Ok(decoded)
                = <PendingDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PendingDefaultAdminDelay(decoded));
            }
            if let Ok(decoded)
                = <RenounceRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RenounceRole(decoded));
            }
            if let Ok(decoded)
                = <RevokeRoleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevokeRole(decoded));
            }
            if let Ok(decoded)
                = <RollbackDefaultAdminDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RollbackDefaultAdminDelay(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AccessControlDefaultAdminRulesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultAdminRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AcceptDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeginDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelDefaultAdminTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoleAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GrantRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasRole(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingDefaultAdmin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeRole(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RollbackDefaultAdminDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AccessControlDefaultAdminRulesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultAdminRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeginDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelDefaultAdminTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultAdminDelayIncreaseWait(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoleAdmin(element) => ::core::fmt::Display::fmt(element, f),
                Self::GrantRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingDefaultAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PendingDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeRole(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollbackDefaultAdminDelay(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultAdminRoleCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminRoleCall) -> Self {
            Self::DefaultAdminRole(value)
        }
    }
    impl ::core::convert::From<AcceptDefaultAdminTransferCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: AcceptDefaultAdminTransferCall) -> Self {
            Self::AcceptDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<BeginDefaultAdminTransferCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: BeginDefaultAdminTransferCall) -> Self {
            Self::BeginDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<CancelDefaultAdminTransferCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: CancelDefaultAdminTransferCall) -> Self {
            Self::CancelDefaultAdminTransfer(value)
        }
    }
    impl ::core::convert::From<ChangeDefaultAdminDelayCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: ChangeDefaultAdminDelayCall) -> Self {
            Self::ChangeDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminCall) -> Self {
            Self::DefaultAdmin(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminDelayCall) -> Self {
            Self::DefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<DefaultAdminDelayIncreaseWaitCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: DefaultAdminDelayIncreaseWaitCall) -> Self {
            Self::DefaultAdminDelayIncreaseWait(value)
        }
    }
    impl ::core::convert::From<GetRoleAdminCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: GetRoleAdminCall) -> Self {
            Self::GetRoleAdmin(value)
        }
    }
    impl ::core::convert::From<GrantRoleCall> for AccessControlDefaultAdminRulesCalls {
        fn from(value: GrantRoleCall) -> Self {
            Self::GrantRole(value)
        }
    }
    impl ::core::convert::From<HasRoleCall> for AccessControlDefaultAdminRulesCalls {
        fn from(value: HasRoleCall) -> Self {
            Self::HasRole(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AccessControlDefaultAdminRulesCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: PendingDefaultAdminCall) -> Self {
            Self::PendingDefaultAdmin(value)
        }
    }
    impl ::core::convert::From<PendingDefaultAdminDelayCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: PendingDefaultAdminDelayCall) -> Self {
            Self::PendingDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<RenounceRoleCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: RenounceRoleCall) -> Self {
            Self::RenounceRole(value)
        }
    }
    impl ::core::convert::From<RevokeRoleCall> for AccessControlDefaultAdminRulesCalls {
        fn from(value: RevokeRoleCall) -> Self {
            Self::RevokeRole(value)
        }
    }
    impl ::core::convert::From<RollbackDefaultAdminDelayCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: RollbackDefaultAdminDelayCall) -> Self {
            Self::RollbackDefaultAdminDelay(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall>
    for AccessControlDefaultAdminRulesCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_ADMIN_ROLE` function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`
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
    pub struct DefaultAdminRoleReturn(pub [u8; 32]);
    ///Container type for all return fields from the `defaultAdmin` function with signature `defaultAdmin()` and selector `0x84ef8ffc`
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
    pub struct DefaultAdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `defaultAdminDelay` function with signature `defaultAdminDelay()` and selector `0xcc8463c8`
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
    pub struct DefaultAdminDelayReturn(pub u64);
    ///Container type for all return fields from the `defaultAdminDelayIncreaseWait` function with signature `defaultAdminDelayIncreaseWait()` and selector `0x022d63fb`
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
    pub struct DefaultAdminDelayIncreaseWaitReturn(pub u64);
    ///Container type for all return fields from the `getRoleAdmin` function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`
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
    pub struct GetRoleAdminReturn(pub [u8; 32]);
    ///Container type for all return fields from the `hasRole` function with signature `hasRole(bytes32,address)` and selector `0x91d14854`
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
    pub struct HasRoleReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingDefaultAdmin` function with signature `pendingDefaultAdmin()` and selector `0xcf6eefb7`
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
    pub struct PendingDefaultAdminReturn {
        pub new_admin: ::ethers::core::types::Address,
        pub schedule: u64,
    }
    ///Container type for all return fields from the `pendingDefaultAdminDelay` function with signature `pendingDefaultAdminDelay()` and selector `0xa1eda53c`
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
    pub struct PendingDefaultAdminDelayReturn {
        pub new_delay: u64,
        pub schedule: u64,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
}
