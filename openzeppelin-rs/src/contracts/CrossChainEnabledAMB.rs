pub use cross_chain_enabled_amb::*;
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
pub mod cross_chain_enabled_amb {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"bridge\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CROSSCHAINENABLEDAMB_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        160,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        64,
        81,
        96,
        188,
        56,
        3,
        128,
        96,
        188,
        131,
        57,
        129,
        1,
        96,
        64,
        129,
        144,
        82,
        96,
        42,
        145,
        96,
        58,
        86,
        91,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        22,
        96,
        128,
        82,
        96,
        104,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        75,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        96,
        97,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        128,
        81,
        96,
        63,
        96,
        125,
        96,
        0,
        57,
        96,
        0,
        80,
        80,
        96,
        63,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        211,
        66,
        114,
        86,
        242,
        204,
        57,
        102,
        180,
        50,
        127,
        12,
        221,
        237,
        22,
        243,
        30,
        162,
        255,
        106,
        25,
        215,
        117,
        214,
        101,
        172,
        0,
        41,
        182,
        82,
        1,
        59,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        13,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static CROSSCHAINENABLEDAMB_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        211,
        66,
        114,
        86,
        242,
        204,
        57,
        102,
        180,
        50,
        127,
        12,
        221,
        237,
        22,
        243,
        30,
        162,
        255,
        106,
        25,
        215,
        117,
        214,
        101,
        172,
        0,
        41,
        182,
        82,
        1,
        59,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        13,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static CROSSCHAINENABLEDAMB_DEPLOYED_BYTECODE: ::ethers_core::types::Bytes = ::ethers_core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CrossChainEnabledAMB<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CrossChainEnabledAMB<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CrossChainEnabledAMB<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CrossChainEnabledAMB<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CrossChainEnabledAMB<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CrossChainEnabledAMB))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> CrossChainEnabledAMB<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    CROSSCHAINENABLEDAMB_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers_core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers_contract::builders::ContractDeployer<M, Self>,
            ::ethers_contract::ContractError<M>,
        > {
            let factory = ::ethers_contract::ContractFactory::new(
                CROSSCHAINENABLEDAMB_ABI.clone(),
                CROSSCHAINENABLEDAMB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for CrossChainEnabledAMB<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
