pub use ierc1820_registry::*;
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
pub mod ierc1820_registry {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"interfaceHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"implementer\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"InterfaceImplementerSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newManager\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ManagerChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_interfaceHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInterfaceImplementer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getManager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementsERC165Interface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"implementsERC165InterfaceNoCache\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"interfaceName\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"interfaceHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_interfaceHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setInterfaceImplementer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"newManager\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setManager\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateERC165Cache\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1820REGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IERC1820Registry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1820Registry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1820Registry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1820Registry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1820Registry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1820Registry)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC1820Registry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC1820REGISTRY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getInterfaceImplementer` (0xaabbb8ca) function
        pub fn get_interface_implementer(
            &self,
            account: ::ethers::core::types::Address,
            interface_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([170, 187, 184, 202], (account, interface_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getManager` (0x3d584063) function
        pub fn get_manager(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 88, 64, 99], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementsERC165Interface` (0xf712f3e8) function
        pub fn implements_erc165_interface(
            &self,
            account: ::ethers::core::types::Address,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([247, 18, 243, 232], (account, interface_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementsERC165InterfaceNoCache` (0xb7056765) function
        pub fn implements_erc165_interface_no_cache(
            &self,
            account: ::ethers::core::types::Address,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 5, 103, 101], (account, interface_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `interfaceHash` (0x65ba36c1) function
        pub fn interface_hash(
            &self,
            interface_name: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([101, 186, 54, 193], interface_name)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setInterfaceImplementer` (0x29965a1d) function
        pub fn set_interface_implementer(
            &self,
            account: ::ethers::core::types::Address,
            interface_hash: [u8; 32],
            implementer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 150, 90, 29], (account, interface_hash, implementer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setManager` (0x5df8122f) function
        pub fn set_manager(
            &self,
            account: ::ethers::core::types::Address,
            new_manager: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 248, 18, 47], (account, new_manager))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateERC165Cache` (0xa41e7d51) function
        pub fn update_erc165_cache(
            &self,
            account: ::ethers::core::types::Address,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 30, 125, 81], (account, interface_id))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `InterfaceImplementerSet` event
        pub fn interface_implementer_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InterfaceImplementerSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ManagerChanged` event
        pub fn manager_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ManagerChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IERC1820RegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC1820Registry<M> {
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
        name = "InterfaceImplementerSet",
        abi = "InterfaceImplementerSet(address,bytes32,address)"
    )]
    pub struct InterfaceImplementerSetFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub interface_hash: [u8; 32],
        #[ethevent(indexed)]
        pub implementer: ::ethers::core::types::Address,
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
    #[ethevent(name = "ManagerChanged", abi = "ManagerChanged(address,address)")]
    pub struct ManagerChangedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_manager: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1820RegistryEvents {
        InterfaceImplementerSetFilter(InterfaceImplementerSetFilter),
        ManagerChangedFilter(ManagerChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IERC1820RegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InterfaceImplementerSetFilter::decode_log(log) {
                return Ok(
                    IERC1820RegistryEvents::InterfaceImplementerSetFilter(decoded),
                );
            }
            if let Ok(decoded) = ManagerChangedFilter::decode_log(log) {
                return Ok(IERC1820RegistryEvents::ManagerChangedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IERC1820RegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InterfaceImplementerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ManagerChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InterfaceImplementerSetFilter>
    for IERC1820RegistryEvents {
        fn from(value: InterfaceImplementerSetFilter) -> Self {
            Self::InterfaceImplementerSetFilter(value)
        }
    }
    impl ::core::convert::From<ManagerChangedFilter> for IERC1820RegistryEvents {
        fn from(value: ManagerChangedFilter) -> Self {
            Self::ManagerChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `getInterfaceImplementer` function with signature `getInterfaceImplementer(address,bytes32)` and selector `0xaabbb8ca`
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
        name = "getInterfaceImplementer",
        abi = "getInterfaceImplementer(address,bytes32)"
    )]
    pub struct GetInterfaceImplementerCall {
        pub account: ::ethers::core::types::Address,
        pub interface_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getManager` function with signature `getManager(address)` and selector `0x3d584063`
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
    #[ethcall(name = "getManager", abi = "getManager(address)")]
    pub struct GetManagerCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `implementsERC165Interface` function with signature `implementsERC165Interface(address,bytes4)` and selector `0xf712f3e8`
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
        name = "implementsERC165Interface",
        abi = "implementsERC165Interface(address,bytes4)"
    )]
    pub struct ImplementsERC165InterfaceCall {
        pub account: ::ethers::core::types::Address,
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `implementsERC165InterfaceNoCache` function with signature `implementsERC165InterfaceNoCache(address,bytes4)` and selector `0xb7056765`
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
        name = "implementsERC165InterfaceNoCache",
        abi = "implementsERC165InterfaceNoCache(address,bytes4)"
    )]
    pub struct ImplementsERC165InterfaceNoCacheCall {
        pub account: ::ethers::core::types::Address,
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `interfaceHash` function with signature `interfaceHash(string)` and selector `0x65ba36c1`
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
    #[ethcall(name = "interfaceHash", abi = "interfaceHash(string)")]
    pub struct InterfaceHashCall {
        pub interface_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `setInterfaceImplementer` function with signature `setInterfaceImplementer(address,bytes32,address)` and selector `0x29965a1d`
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
        name = "setInterfaceImplementer",
        abi = "setInterfaceImplementer(address,bytes32,address)"
    )]
    pub struct SetInterfaceImplementerCall {
        pub account: ::ethers::core::types::Address,
        pub interface_hash: [u8; 32],
        pub implementer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setManager` function with signature `setManager(address,address)` and selector `0x5df8122f`
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
    #[ethcall(name = "setManager", abi = "setManager(address,address)")]
    pub struct SetManagerCall {
        pub account: ::ethers::core::types::Address,
        pub new_manager: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateERC165Cache` function with signature `updateERC165Cache(address,bytes4)` and selector `0xa41e7d51`
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
    #[ethcall(name = "updateERC165Cache", abi = "updateERC165Cache(address,bytes4)")]
    pub struct UpdateERC165CacheCall {
        pub account: ::ethers::core::types::Address,
        pub interface_id: [u8; 4],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IERC1820RegistryCalls {
        GetInterfaceImplementer(GetInterfaceImplementerCall),
        GetManager(GetManagerCall),
        ImplementsERC165Interface(ImplementsERC165InterfaceCall),
        ImplementsERC165InterfaceNoCache(ImplementsERC165InterfaceNoCacheCall),
        InterfaceHash(InterfaceHashCall),
        SetInterfaceImplementer(SetInterfaceImplementerCall),
        SetManager(SetManagerCall),
        UpdateERC165Cache(UpdateERC165CacheCall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC1820RegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetInterfaceImplementerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetInterfaceImplementer(decoded));
            }
            if let Ok(decoded)
                = <GetManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetManager(decoded));
            }
            if let Ok(decoded)
                = <ImplementsERC165InterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ImplementsERC165Interface(decoded));
            }
            if let Ok(decoded)
                = <ImplementsERC165InterfaceNoCacheCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ImplementsERC165InterfaceNoCache(decoded));
            }
            if let Ok(decoded)
                = <InterfaceHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InterfaceHash(decoded));
            }
            if let Ok(decoded)
                = <SetInterfaceImplementerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetInterfaceImplementer(decoded));
            }
            if let Ok(decoded)
                = <SetManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetManager(decoded));
            }
            if let Ok(decoded)
                = <UpdateERC165CacheCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UpdateERC165Cache(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC1820RegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetInterfaceImplementer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ImplementsERC165Interface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ImplementsERC165InterfaceNoCache(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InterfaceHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetInterfaceImplementer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateERC165Cache(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC1820RegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetInterfaceImplementer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::ImplementsERC165Interface(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ImplementsERC165InterfaceNoCache(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InterfaceHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetInterfaceImplementer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateERC165Cache(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetInterfaceImplementerCall> for IERC1820RegistryCalls {
        fn from(value: GetInterfaceImplementerCall) -> Self {
            Self::GetInterfaceImplementer(value)
        }
    }
    impl ::core::convert::From<GetManagerCall> for IERC1820RegistryCalls {
        fn from(value: GetManagerCall) -> Self {
            Self::GetManager(value)
        }
    }
    impl ::core::convert::From<ImplementsERC165InterfaceCall> for IERC1820RegistryCalls {
        fn from(value: ImplementsERC165InterfaceCall) -> Self {
            Self::ImplementsERC165Interface(value)
        }
    }
    impl ::core::convert::From<ImplementsERC165InterfaceNoCacheCall>
    for IERC1820RegistryCalls {
        fn from(value: ImplementsERC165InterfaceNoCacheCall) -> Self {
            Self::ImplementsERC165InterfaceNoCache(value)
        }
    }
    impl ::core::convert::From<InterfaceHashCall> for IERC1820RegistryCalls {
        fn from(value: InterfaceHashCall) -> Self {
            Self::InterfaceHash(value)
        }
    }
    impl ::core::convert::From<SetInterfaceImplementerCall> for IERC1820RegistryCalls {
        fn from(value: SetInterfaceImplementerCall) -> Self {
            Self::SetInterfaceImplementer(value)
        }
    }
    impl ::core::convert::From<SetManagerCall> for IERC1820RegistryCalls {
        fn from(value: SetManagerCall) -> Self {
            Self::SetManager(value)
        }
    }
    impl ::core::convert::From<UpdateERC165CacheCall> for IERC1820RegistryCalls {
        fn from(value: UpdateERC165CacheCall) -> Self {
            Self::UpdateERC165Cache(value)
        }
    }
    ///Container type for all return fields from the `getInterfaceImplementer` function with signature `getInterfaceImplementer(address,bytes32)` and selector `0xaabbb8ca`
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
    pub struct GetInterfaceImplementerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getManager` function with signature `getManager(address)` and selector `0x3d584063`
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
    pub struct GetManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `implementsERC165Interface` function with signature `implementsERC165Interface(address,bytes4)` and selector `0xf712f3e8`
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
    pub struct ImplementsERC165InterfaceReturn(pub bool);
    ///Container type for all return fields from the `implementsERC165InterfaceNoCache` function with signature `implementsERC165InterfaceNoCache(address,bytes4)` and selector `0xb7056765`
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
    pub struct ImplementsERC165InterfaceNoCacheReturn(pub bool);
    ///Container type for all return fields from the `interfaceHash` function with signature `interfaceHash(string)` and selector `0x65ba36c1`
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
    pub struct InterfaceHashReturn(pub [u8; 32]);
}
