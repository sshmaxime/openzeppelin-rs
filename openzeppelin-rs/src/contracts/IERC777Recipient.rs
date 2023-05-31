pub use ierc777_recipient::*;
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
pub mod ierc777_recipient {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"userData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"operatorData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tokensReceived\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IERC777RECIPIENT_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IERC777Recipient<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC777Recipient<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC777Recipient<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC777Recipient<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC777Recipient<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IERC777Recipient)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IERC777Recipient<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IERC777RECIPIENT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `tokensReceived` (0x0023de29) function
        pub fn tokens_received(
            &self,
            operator: ::ethers_core::types::Address,
            from: ::ethers_core::types::Address,
            to: ::ethers_core::types::Address,
            amount: ::ethers_core::types::U256,
            user_data: ::ethers_core::types::Bytes,
            operator_data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 35, 222, 41],
                    (operator, from, to, amount, user_data, operator_data),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IERC777Recipient<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `tokensReceived` function with signature `tokensReceived(address,address,address,uint256,bytes,bytes)` and selector `0x0023de29`
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
        name = "tokensReceived",
        abi = "tokensReceived(address,address,address,uint256,bytes,bytes)"
    )]
    pub struct TokensReceivedCall {
        pub operator: ::ethers_core::types::Address,
        pub from: ::ethers_core::types::Address,
        pub to: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub user_data: ::ethers_core::types::Bytes,
        pub operator_data: ::ethers_core::types::Bytes,
    }
}
