pub use i_delayed_message_provider::*;
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
pub mod i_delayed_message_provider {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"messageNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"InboxMessageDelivered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"messageNum\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"InboxMessageDeliveredFromOrigin\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static IDELAYEDMESSAGEPROVIDER_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct IDelayedMessageProvider<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IDelayedMessageProvider<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IDelayedMessageProvider<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IDelayedMessageProvider<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IDelayedMessageProvider<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IDelayedMessageProvider))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IDelayedMessageProvider<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IDELAYEDMESSAGEPROVIDER_ABI.clone(),
                    client,
                ),
            )
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
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IDelayedMessageProviderEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IDelayedMessageProvider<M> {
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
    pub enum IDelayedMessageProviderEvents {
        InboxMessageDeliveredFilter(InboxMessageDeliveredFilter),
        InboxMessageDeliveredFromOriginFilter(InboxMessageDeliveredFromOriginFilter),
    }
    impl ::ethers_contract::EthLogDecode for IDelayedMessageProviderEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = InboxMessageDeliveredFilter::decode_log(log) {
                return Ok(
                    IDelayedMessageProviderEvents::InboxMessageDeliveredFilter(decoded),
                );
            }
            if let Ok(decoded) = InboxMessageDeliveredFromOriginFilter::decode_log(log) {
                return Ok(
                    IDelayedMessageProviderEvents::InboxMessageDeliveredFromOriginFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IDelayedMessageProviderEvents {
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
    impl ::core::convert::From<InboxMessageDeliveredFilter>
    for IDelayedMessageProviderEvents {
        fn from(value: InboxMessageDeliveredFilter) -> Self {
            Self::InboxMessageDeliveredFilter(value)
        }
    }
    impl ::core::convert::From<InboxMessageDeliveredFromOriginFilter>
    for IDelayedMessageProviderEvents {
        fn from(value: InboxMessageDeliveredFromOriginFilter) -> Self {
            Self::InboxMessageDeliveredFromOriginFilter(value)
        }
    }
}
