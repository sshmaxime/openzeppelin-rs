pub use cross_chain_enabled_polygon_child::*;
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
pub mod cross_chain_enabled_polygon_child {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"NotCrossChainCall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"rootMessageSender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"processMessageFromRoot\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CROSSCHAINENABLEDPOLYGONCHILD_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct CrossChainEnabledPolygonChild<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CrossChainEnabledPolygonChild<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CrossChainEnabledPolygonChild<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CrossChainEnabledPolygonChild<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CrossChainEnabledPolygonChild<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CrossChainEnabledPolygonChild))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> CrossChainEnabledPolygonChild<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    CROSSCHAINENABLEDPOLYGONCHILD_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `processMessageFromRoot` (0x9a7c4b71) function
        pub fn process_message_from_root(
            &self,
            p0: ::ethers_core::types::U256,
            root_message_sender: ::ethers_core::types::Address,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 124, 75, 113], (p0, root_message_sender, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for CrossChainEnabledPolygonChild<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `NotCrossChainCall` with signature `NotCrossChainCall()` and selector `0x4a74df92`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotCrossChainCall", abi = "NotCrossChainCall()")]
    pub struct NotCrossChainCall;
    ///Container type for all input parameters for the `processMessageFromRoot` function with signature `processMessageFromRoot(uint256,address,bytes)` and selector `0x9a7c4b71`
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
        name = "processMessageFromRoot",
        abi = "processMessageFromRoot(uint256,address,bytes)"
    )]
    pub struct ProcessMessageFromRootCall {
        pub p0: ::ethers_core::types::U256,
        pub root_message_sender: ::ethers_core::types::Address,
        pub data: ::ethers_core::types::Bytes,
    }
}
