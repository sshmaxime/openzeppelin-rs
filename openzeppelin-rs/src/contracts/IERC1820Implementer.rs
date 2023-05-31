pub use ierc1820_implementer::*;
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
pub mod ierc1820_implementer {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"interfaceHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"canImplementInterfaceForAddress\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC1820IMPLEMENTER_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct IERC1820Implementer<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC1820Implementer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC1820Implementer<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC1820Implementer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC1820Implementer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC1820Implementer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IERC1820Implementer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IERC1820IMPLEMENTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `canImplementInterfaceForAddress` (0x249cb3fa) function
        pub fn can_implement_interface_for_address(
            &self,
            interface_hash: [u8; 32],
            account: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 156, 179, 250], (interface_hash, account))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IERC1820Implementer<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `canImplementInterfaceForAddress` function with signature `canImplementInterfaceForAddress(bytes32,address)` and selector `0x249cb3fa`
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
        name = "canImplementInterfaceForAddress",
        abi = "canImplementInterfaceForAddress(bytes32,address)"
    )]
    pub struct CanImplementInterfaceForAddressCall {
        pub interface_hash: [u8; 32],
        pub account: ::ethers_core::types::Address,
    }
    ///Container type for all return fields from the `canImplementInterfaceForAddress` function with signature `canImplementInterfaceForAddress(bytes32,address)` and selector `0x249cb3fa`
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
    pub struct CanImplementInterfaceForAddressReturn(pub [u8; 32]);
}
