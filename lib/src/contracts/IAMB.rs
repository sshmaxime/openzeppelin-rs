pub use iamb::*;
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
pub mod iamb {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"executor\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"messageId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"status\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AffirmationCompleted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"executor\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"messageId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"status\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RelayedMessage\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"messageId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"encodedData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UserRequestForAffirmation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"messageId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes\",\"name\":\"encodedData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UserRequestForSignature\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"destinationChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_messageId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failedMessageDataHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_messageId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failedMessageReceiver\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_messageId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"failedMessageSender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxGasPerTx\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_messageId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"messageCallStatus\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"messageId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"messageSender\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"messageSourceChainId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_contract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_gas\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requireToConfirmMessage\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_contract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_gas\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requireToPassMessage\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sourceChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"transactionHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IAMB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IAMB<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAMB<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAMB<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAMB<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAMB<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IAMB)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAMB<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IAMB_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `destinationChainId` (0xb0750611) function
        pub fn destination_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 117, 6, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessageDataHash` (0xe37c3289) function
        pub fn failed_message_data_hash(
            &self,
            message_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([227, 124, 50, 137], message_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessageReceiver` (0x3f9a8e7e) function
        pub fn failed_message_receiver(
            &self,
            message_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 154, 142, 126], message_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failedMessageSender` (0x4a610b04) function
        pub fn failed_message_sender(
            &self,
            message_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([74, 97, 11, 4], message_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxGasPerTx` (0xe5789d03) function
        pub fn max_gas_per_tx(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 120, 157, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messageCallStatus` (0xcb08a10c) function
        pub fn message_call_status(
            &self,
            message_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([203, 8, 161, 12], message_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messageId` (0x669f618b) function
        pub fn message_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([102, 159, 97, 139], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messageSender` (0xd67bdd25) function
        pub fn message_sender(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([214, 123, 221, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `messageSourceChainId` (0x9e307dff) function
        pub fn message_source_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([158, 48, 125, 255], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requireToConfirmMessage` (0x94643f71) function
        pub fn require_to_confirm_message(
            &self,
            contract: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            gas: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([148, 100, 63, 113], (contract, data, gas))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requireToPassMessage` (0xdc8601b3) function
        pub fn require_to_pass_message(
            &self,
            contract: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            gas: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([220, 134, 1, 179], (contract, data, gas))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sourceChainId` (0x1544298e) function
        pub fn source_chain_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 68, 41, 142], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transactionHash` (0x0ac1c313) function
        pub fn transaction_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([10, 193, 195, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AffirmationCompleted` event
        pub fn affirmation_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AffirmationCompletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayedMessage` event
        pub fn relayed_message_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayedMessageFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UserRequestForAffirmation` event
        pub fn user_request_for_affirmation_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserRequestForAffirmationFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `UserRequestForSignature` event
        pub fn user_request_for_signature_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UserRequestForSignatureFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IAMBEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IAMB<M> {
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
        name = "AffirmationCompleted",
        abi = "AffirmationCompleted(address,address,bytes32,bool)"
    )]
    pub struct AffirmationCompletedFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub executor: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub status: bool,
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
        name = "RelayedMessage",
        abi = "RelayedMessage(address,address,bytes32,bool)"
    )]
    pub struct RelayedMessageFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub executor: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub status: bool,
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
        name = "UserRequestForAffirmation",
        abi = "UserRequestForAffirmation(bytes32,bytes)"
    )]
    pub struct UserRequestForAffirmationFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub encoded_data: ::ethers::core::types::Bytes,
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
        name = "UserRequestForSignature",
        abi = "UserRequestForSignature(bytes32,bytes)"
    )]
    pub struct UserRequestForSignatureFilter {
        #[ethevent(indexed)]
        pub message_id: [u8; 32],
        pub encoded_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IAMBEvents {
        AffirmationCompletedFilter(AffirmationCompletedFilter),
        RelayedMessageFilter(RelayedMessageFilter),
        UserRequestForAffirmationFilter(UserRequestForAffirmationFilter),
        UserRequestForSignatureFilter(UserRequestForSignatureFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAMBEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AffirmationCompletedFilter::decode_log(log) {
                return Ok(IAMBEvents::AffirmationCompletedFilter(decoded));
            }
            if let Ok(decoded) = RelayedMessageFilter::decode_log(log) {
                return Ok(IAMBEvents::RelayedMessageFilter(decoded));
            }
            if let Ok(decoded) = UserRequestForAffirmationFilter::decode_log(log) {
                return Ok(IAMBEvents::UserRequestForAffirmationFilter(decoded));
            }
            if let Ok(decoded) = UserRequestForSignatureFilter::decode_log(log) {
                return Ok(IAMBEvents::UserRequestForSignatureFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAMBEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AffirmationCompletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayedMessageFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserRequestForAffirmationFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UserRequestForSignatureFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AffirmationCompletedFilter> for IAMBEvents {
        fn from(value: AffirmationCompletedFilter) -> Self {
            Self::AffirmationCompletedFilter(value)
        }
    }
    impl ::core::convert::From<RelayedMessageFilter> for IAMBEvents {
        fn from(value: RelayedMessageFilter) -> Self {
            Self::RelayedMessageFilter(value)
        }
    }
    impl ::core::convert::From<UserRequestForAffirmationFilter> for IAMBEvents {
        fn from(value: UserRequestForAffirmationFilter) -> Self {
            Self::UserRequestForAffirmationFilter(value)
        }
    }
    impl ::core::convert::From<UserRequestForSignatureFilter> for IAMBEvents {
        fn from(value: UserRequestForSignatureFilter) -> Self {
            Self::UserRequestForSignatureFilter(value)
        }
    }
    ///Container type for all input parameters for the `destinationChainId` function with signature `destinationChainId()` and selector `0xb0750611`
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
    #[ethcall(name = "destinationChainId", abi = "destinationChainId()")]
    pub struct DestinationChainIdCall;
    ///Container type for all input parameters for the `failedMessageDataHash` function with signature `failedMessageDataHash(bytes32)` and selector `0xe37c3289`
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
    #[ethcall(name = "failedMessageDataHash", abi = "failedMessageDataHash(bytes32)")]
    pub struct FailedMessageDataHashCall {
        pub message_id: [u8; 32],
    }
    ///Container type for all input parameters for the `failedMessageReceiver` function with signature `failedMessageReceiver(bytes32)` and selector `0x3f9a8e7e`
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
    #[ethcall(name = "failedMessageReceiver", abi = "failedMessageReceiver(bytes32)")]
    pub struct FailedMessageReceiverCall {
        pub message_id: [u8; 32],
    }
    ///Container type for all input parameters for the `failedMessageSender` function with signature `failedMessageSender(bytes32)` and selector `0x4a610b04`
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
    #[ethcall(name = "failedMessageSender", abi = "failedMessageSender(bytes32)")]
    pub struct FailedMessageSenderCall {
        pub message_id: [u8; 32],
    }
    ///Container type for all input parameters for the `maxGasPerTx` function with signature `maxGasPerTx()` and selector `0xe5789d03`
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
    #[ethcall(name = "maxGasPerTx", abi = "maxGasPerTx()")]
    pub struct MaxGasPerTxCall;
    ///Container type for all input parameters for the `messageCallStatus` function with signature `messageCallStatus(bytes32)` and selector `0xcb08a10c`
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
    #[ethcall(name = "messageCallStatus", abi = "messageCallStatus(bytes32)")]
    pub struct MessageCallStatusCall {
        pub message_id: [u8; 32],
    }
    ///Container type for all input parameters for the `messageId` function with signature `messageId()` and selector `0x669f618b`
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
    #[ethcall(name = "messageId", abi = "messageId()")]
    pub struct MessageIdCall;
    ///Container type for all input parameters for the `messageSender` function with signature `messageSender()` and selector `0xd67bdd25`
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
    #[ethcall(name = "messageSender", abi = "messageSender()")]
    pub struct MessageSenderCall;
    ///Container type for all input parameters for the `messageSourceChainId` function with signature `messageSourceChainId()` and selector `0x9e307dff`
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
    #[ethcall(name = "messageSourceChainId", abi = "messageSourceChainId()")]
    pub struct MessageSourceChainIdCall;
    ///Container type for all input parameters for the `requireToConfirmMessage` function with signature `requireToConfirmMessage(address,bytes,uint256)` and selector `0x94643f71`
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
        name = "requireToConfirmMessage",
        abi = "requireToConfirmMessage(address,bytes,uint256)"
    )]
    pub struct RequireToConfirmMessageCall {
        pub contract: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub gas: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requireToPassMessage` function with signature `requireToPassMessage(address,bytes,uint256)` and selector `0xdc8601b3`
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
        name = "requireToPassMessage",
        abi = "requireToPassMessage(address,bytes,uint256)"
    )]
    pub struct RequireToPassMessageCall {
        pub contract: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub gas: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sourceChainId` function with signature `sourceChainId()` and selector `0x1544298e`
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
    #[ethcall(name = "sourceChainId", abi = "sourceChainId()")]
    pub struct SourceChainIdCall;
    ///Container type for all input parameters for the `transactionHash` function with signature `transactionHash()` and selector `0x0ac1c313`
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
    #[ethcall(name = "transactionHash", abi = "transactionHash()")]
    pub struct TransactionHashCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IAMBCalls {
        DestinationChainId(DestinationChainIdCall),
        FailedMessageDataHash(FailedMessageDataHashCall),
        FailedMessageReceiver(FailedMessageReceiverCall),
        FailedMessageSender(FailedMessageSenderCall),
        MaxGasPerTx(MaxGasPerTxCall),
        MessageCallStatus(MessageCallStatusCall),
        MessageId(MessageIdCall),
        MessageSender(MessageSenderCall),
        MessageSourceChainId(MessageSourceChainIdCall),
        RequireToConfirmMessage(RequireToConfirmMessageCall),
        RequireToPassMessage(RequireToPassMessageCall),
        SourceChainId(SourceChainIdCall),
        TransactionHash(TransactionHashCall),
    }
    impl ::ethers::core::abi::AbiDecode for IAMBCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DestinationChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DestinationChainId(decoded));
            }
            if let Ok(decoded)
                = <FailedMessageDataHashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FailedMessageDataHash(decoded));
            }
            if let Ok(decoded)
                = <FailedMessageReceiverCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FailedMessageReceiver(decoded));
            }
            if let Ok(decoded)
                = <FailedMessageSenderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FailedMessageSender(decoded));
            }
            if let Ok(decoded)
                = <MaxGasPerTxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxGasPerTx(decoded));
            }
            if let Ok(decoded)
                = <MessageCallStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MessageCallStatus(decoded));
            }
            if let Ok(decoded)
                = <MessageIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MessageId(decoded));
            }
            if let Ok(decoded)
                = <MessageSenderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MessageSender(decoded));
            }
            if let Ok(decoded)
                = <MessageSourceChainIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MessageSourceChainId(decoded));
            }
            if let Ok(decoded)
                = <RequireToConfirmMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RequireToConfirmMessage(decoded));
            }
            if let Ok(decoded)
                = <RequireToPassMessageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RequireToPassMessage(decoded));
            }
            if let Ok(decoded)
                = <SourceChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SourceChainId(decoded));
            }
            if let Ok(decoded)
                = <TransactionHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransactionHash(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAMBCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DestinationChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMessageDataHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMessageReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedMessageSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxGasPerTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MessageCallStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MessageId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MessageSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MessageSourceChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequireToConfirmMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequireToPassMessage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SourceChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransactionHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAMBCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DestinationChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedMessageDataHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedMessageReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedMessageSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxGasPerTx(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageCallStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageId(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::MessageSourceChainId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequireToConfirmMessage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequireToPassMessage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SourceChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactionHash(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DestinationChainIdCall> for IAMBCalls {
        fn from(value: DestinationChainIdCall) -> Self {
            Self::DestinationChainId(value)
        }
    }
    impl ::core::convert::From<FailedMessageDataHashCall> for IAMBCalls {
        fn from(value: FailedMessageDataHashCall) -> Self {
            Self::FailedMessageDataHash(value)
        }
    }
    impl ::core::convert::From<FailedMessageReceiverCall> for IAMBCalls {
        fn from(value: FailedMessageReceiverCall) -> Self {
            Self::FailedMessageReceiver(value)
        }
    }
    impl ::core::convert::From<FailedMessageSenderCall> for IAMBCalls {
        fn from(value: FailedMessageSenderCall) -> Self {
            Self::FailedMessageSender(value)
        }
    }
    impl ::core::convert::From<MaxGasPerTxCall> for IAMBCalls {
        fn from(value: MaxGasPerTxCall) -> Self {
            Self::MaxGasPerTx(value)
        }
    }
    impl ::core::convert::From<MessageCallStatusCall> for IAMBCalls {
        fn from(value: MessageCallStatusCall) -> Self {
            Self::MessageCallStatus(value)
        }
    }
    impl ::core::convert::From<MessageIdCall> for IAMBCalls {
        fn from(value: MessageIdCall) -> Self {
            Self::MessageId(value)
        }
    }
    impl ::core::convert::From<MessageSenderCall> for IAMBCalls {
        fn from(value: MessageSenderCall) -> Self {
            Self::MessageSender(value)
        }
    }
    impl ::core::convert::From<MessageSourceChainIdCall> for IAMBCalls {
        fn from(value: MessageSourceChainIdCall) -> Self {
            Self::MessageSourceChainId(value)
        }
    }
    impl ::core::convert::From<RequireToConfirmMessageCall> for IAMBCalls {
        fn from(value: RequireToConfirmMessageCall) -> Self {
            Self::RequireToConfirmMessage(value)
        }
    }
    impl ::core::convert::From<RequireToPassMessageCall> for IAMBCalls {
        fn from(value: RequireToPassMessageCall) -> Self {
            Self::RequireToPassMessage(value)
        }
    }
    impl ::core::convert::From<SourceChainIdCall> for IAMBCalls {
        fn from(value: SourceChainIdCall) -> Self {
            Self::SourceChainId(value)
        }
    }
    impl ::core::convert::From<TransactionHashCall> for IAMBCalls {
        fn from(value: TransactionHashCall) -> Self {
            Self::TransactionHash(value)
        }
    }
    ///Container type for all return fields from the `destinationChainId` function with signature `destinationChainId()` and selector `0xb0750611`
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
    pub struct DestinationChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `failedMessageDataHash` function with signature `failedMessageDataHash(bytes32)` and selector `0xe37c3289`
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
    pub struct FailedMessageDataHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `failedMessageReceiver` function with signature `failedMessageReceiver(bytes32)` and selector `0x3f9a8e7e`
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
    pub struct FailedMessageReceiverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `failedMessageSender` function with signature `failedMessageSender(bytes32)` and selector `0x4a610b04`
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
    pub struct FailedMessageSenderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `maxGasPerTx` function with signature `maxGasPerTx()` and selector `0xe5789d03`
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
    pub struct MaxGasPerTxReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `messageCallStatus` function with signature `messageCallStatus(bytes32)` and selector `0xcb08a10c`
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
    pub struct MessageCallStatusReturn(pub bool);
    ///Container type for all return fields from the `messageId` function with signature `messageId()` and selector `0x669f618b`
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
    pub struct MessageIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `messageSender` function with signature `messageSender()` and selector `0xd67bdd25`
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
    pub struct MessageSenderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `messageSourceChainId` function with signature `messageSourceChainId()` and selector `0x9e307dff`
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
    pub struct MessageSourceChainIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `requireToConfirmMessage` function with signature `requireToConfirmMessage(address,bytes,uint256)` and selector `0x94643f71`
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
    pub struct RequireToConfirmMessageReturn(pub [u8; 32]);
    ///Container type for all return fields from the `requireToPassMessage` function with signature `requireToPassMessage(address,bytes,uint256)` and selector `0xdc8601b3`
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
    pub struct RequireToPassMessageReturn(pub [u8; 32]);
    ///Container type for all return fields from the `sourceChainId` function with signature `sourceChainId()` and selector `0x1544298e`
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
    pub struct SourceChainIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transactionHash` function with signature `transactionHash()` and selector `0x0ac1c313`
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
    pub struct TransactionHashReturn(pub [u8; 32]);
}
