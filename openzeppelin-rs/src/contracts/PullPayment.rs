pub use pull_payment::*;
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
pub mod pull_payment {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"dest\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"payments\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"payee\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawPayments\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PULLPAYMENT_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct PullPayment<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for PullPayment<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PullPayment<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PullPayment<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PullPayment<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(PullPayment)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> PullPayment<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    PULLPAYMENT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `payments` (0xe2982c21) function
        pub fn payments(
            &self,
            dest: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([226, 152, 44, 33], dest)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawPayments` (0x31b3eb94) function
        pub fn withdraw_payments(
            &self,
            payee: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 179, 235, 148], payee)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for PullPayment<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `payments` function with signature `payments(address)` and selector `0xe2982c21`
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
    #[ethcall(name = "payments", abi = "payments(address)")]
    pub struct PaymentsCall {
        pub dest: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawPayments` function with signature `withdrawPayments(address)` and selector `0x31b3eb94`
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
    #[ethcall(name = "withdrawPayments", abi = "withdrawPayments(address)")]
    pub struct WithdrawPaymentsCall {
        pub payee: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PullPaymentCalls {
        Payments(PaymentsCall),
        WithdrawPayments(WithdrawPaymentsCall),
    }
    impl ::ethers_core::abi::AbiDecode for PullPaymentCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <PaymentsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Payments(decoded));
            }
            if let Ok(decoded)
                = <WithdrawPaymentsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawPayments(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for PullPaymentCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Payments(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::WithdrawPayments(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PullPaymentCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Payments(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawPayments(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PaymentsCall> for PullPaymentCalls {
        fn from(value: PaymentsCall) -> Self {
            Self::Payments(value)
        }
    }
    impl ::core::convert::From<WithdrawPaymentsCall> for PullPaymentCalls {
        fn from(value: WithdrawPaymentsCall) -> Self {
            Self::WithdrawPayments(value)
        }
    }
    ///Container type for all return fields from the `payments` function with signature `payments(address)` and selector `0xe2982c21`
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
    pub struct PaymentsReturn(pub ::ethers_core::types::U256);
}
