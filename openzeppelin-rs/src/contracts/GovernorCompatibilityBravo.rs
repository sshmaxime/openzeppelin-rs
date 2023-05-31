pub use governor_compatibility_bravo::*;
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
pub mod governor_compatibility_bravo {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"Empty\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidShortString\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"str\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"StringTooLong\",\"outputs\":[]},{\"inputs\":[],\"type\":\"event\",\"name\":\"EIP712DomainChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProposalCanceled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"string[]\",\"name\":\"signatures\",\"type\":\"string[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"voteStart\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"voteEnd\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProposalCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProposalExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProposalQueued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"voter\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"weight\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VoteCast\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"voter\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"weight\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"VoteCastWithParams\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"BALLOT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"CLOCK_MODE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"COUNTING_MODE\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"EXTENDED_BALLOT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancel\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"descriptionHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancel\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"castVote\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"castVoteBySig\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"castVoteWithReason\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"castVoteWithReasonAndParams\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"reason\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"castVoteWithReasonAndParamsBySig\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"clock\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"\",\"type\":\"uint48\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"eip712Domain\",\"outputs\":[{\"internalType\":\"bytes1\",\"name\":\"fields\",\"type\":\"bytes1\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"chainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"verifyingContract\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"extensions\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"descriptionHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"execute\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getActions\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"string[]\",\"name\":\"signatures\",\"type\":\"string[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"voter\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReceipt\",\"outputs\":[{\"internalType\":\"struct IGovernorCompatibilityBravo.Receipt\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"hasVoted\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"support\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint96\",\"name\":\"votes\",\"type\":\"uint96\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timepoint\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVotes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timepoint\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"params\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getVotesWithParams\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasVoted\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"descriptionHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"hashProposal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC721Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalDeadline\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalEta\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalProposer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalSnapshot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalThreshold\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposals\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"eta\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endBlock\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"forVotes\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"againstVotes\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"abstainVotes\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"canceled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"executed\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"propose\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"string[]\",\"name\":\"signatures\",\"type\":\"string[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"propose\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"calldatas\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"descriptionHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queue\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"queue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"timepoint\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"quorum\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"quorumVotes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"relay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"proposalId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"state\",\"outputs\":[{\"internalType\":\"enum IGovernor.ProposalState\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"timelock\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"votingDelay\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"votingPeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static GOVERNORCOMPATIBILITYBRAVO_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
    });
    pub struct GovernorCompatibilityBravo<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for GovernorCompatibilityBravo<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GovernorCompatibilityBravo<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GovernorCompatibilityBravo<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GovernorCompatibilityBravo<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(GovernorCompatibilityBravo))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> GovernorCompatibilityBravo<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    GOVERNORCOMPATIBILITYBRAVO_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `BALLOT_TYPEHASH` (0xdeaaa7cc) function
        pub fn ballot_typehash(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([222, 170, 167, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CLOCK_MODE` (0x4bf5d7e9) function
        pub fn clock_mode(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([75, 245, 215, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `COUNTING_MODE` (0xdd4e2ba5) function
        pub fn counting_mode(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([221, 78, 43, 165], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXTENDED_BALLOT_TYPEHASH` (0x2fe3e261) function
        pub fn extended_ballot_typehash(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([47, 227, 226, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancel` (0x40e58ee5) function
        pub fn cancel(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 229, 142, 229], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancel` (0x452115d6) function
        pub fn cancel_with_targets(
            &self,
            targets: ::std::vec::Vec<::ethers_core::types::Address>,
            values: ::std::vec::Vec<::ethers_core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [69, 33, 21, 214],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVote` (0x56781388) function
        pub fn cast_vote(
            &self,
            proposal_id: ::ethers_core::types::U256,
            support: u8,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([86, 120, 19, 136], (proposal_id, support))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteBySig` (0x3bccf4fd) function
        pub fn cast_vote_by_sig(
            &self,
            proposal_id: ::ethers_core::types::U256,
            support: u8,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([59, 204, 244, 253], (proposal_id, support, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReason` (0x7b3c71d3) function
        pub fn cast_vote_with_reason(
            &self,
            proposal_id: ::ethers_core::types::U256,
            support: u8,
            reason: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([123, 60, 113, 211], (proposal_id, support, reason))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReasonAndParams` (0x5f398a14) function
        pub fn cast_vote_with_reason_and_params(
            &self,
            proposal_id: ::ethers_core::types::U256,
            support: u8,
            reason: ::std::string::String,
            params: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([95, 57, 138, 20], (proposal_id, support, reason, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `castVoteWithReasonAndParamsBySig` (0x03420181) function
        pub fn cast_vote_with_reason_and_params_by_sig(
            &self,
            proposal_id: ::ethers_core::types::U256,
            support: u8,
            reason: ::std::string::String,
            params: ::ethers_core::types::Bytes,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [3, 66, 1, 129],
                    (proposal_id, support, reason, params, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clock` (0x91ddadf4) function
        pub fn clock(&self) -> ::ethers_contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([145, 221, 173, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers_core::types::U256,
                ::ethers_core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers_core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x2656227d) function
        pub fn execute_with_targets(
            &self,
            targets: ::std::vec::Vec<::ethers_core::types::Address>,
            values: ::std::vec::Vec<::ethers_core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [38, 86, 34, 125],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0xfe0d94c1) function
        pub fn execute(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 13, 148, 193], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getActions` (0x328dd982) function
        pub fn get_actions(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers_core::types::Address>,
                ::std::vec::Vec<::ethers_core::types::U256>,
                ::std::vec::Vec<::std::string::String>,
                ::std::vec::Vec<::ethers_core::types::Bytes>,
            ),
        > {
            self.0
                .method_hash([50, 141, 217, 130], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReceipt` (0xe23a9a52) function
        pub fn get_receipt(
            &self,
            proposal_id: ::ethers_core::types::U256,
            voter: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, Receipt> {
            self.0
                .method_hash([226, 58, 154, 82], (proposal_id, voter))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotes` (0xeb9019d4) function
        pub fn get_votes(
            &self,
            account: ::ethers_core::types::Address,
            timepoint: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([235, 144, 25, 212], (account, timepoint))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVotesWithParams` (0x9a802a6d) function
        pub fn get_votes_with_params(
            &self,
            account: ::ethers_core::types::Address,
            timepoint: ::ethers_core::types::U256,
            params: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([154, 128, 42, 109], (account, timepoint, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasVoted` (0x43859632) function
        pub fn has_voted(
            &self,
            proposal_id: ::ethers_core::types::U256,
            account: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([67, 133, 150, 50], (proposal_id, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashProposal` (0xc59057e4) function
        pub fn hash_proposal(
            &self,
            targets: ::std::vec::Vec<::ethers_core::types::Address>,
            values: ::std::vec::Vec<::ethers_core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [197, 144, 87, 228],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers_core::types::Address,
            p1: ::ethers_core::types::Address,
            p2: ::std::vec::Vec<::ethers_core::types::U256>,
            p3: ::std::vec::Vec<::ethers_core::types::U256>,
            p4: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers_core::types::Address,
            p1: ::ethers_core::types::Address,
            p2: ::ethers_core::types::U256,
            p3: ::ethers_core::types::U256,
            p4: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers_core::types::Address,
            p1: ::ethers_core::types::Address,
            p2: ::ethers_core::types::U256,
            p3: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalDeadline` (0xc01f9e37) function
        pub fn proposal_deadline(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([192, 31, 158, 55], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalEta` (0xab58fb8e) function
        pub fn proposal_eta(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([171, 88, 251, 142], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalProposer` (0x143489d0) function
        pub fn proposal_proposer(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([20, 52, 137, 208], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalSnapshot` (0x2d63f693) function
        pub fn proposal_snapshot(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([45, 99, 246, 147], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalThreshold` (0xb58131b0) function
        pub fn proposal_threshold(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([181, 129, 49, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposals` (0x013cf08b) function
        pub fn proposals(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                ::ethers_core::types::U256,
                ::ethers_core::types::Address,
                ::ethers_core::types::U256,
                ::ethers_core::types::U256,
                ::ethers_core::types::U256,
                ::ethers_core::types::U256,
                ::ethers_core::types::U256,
                ::ethers_core::types::U256,
                bool,
                bool,
            ),
        > {
            self.0
                .method_hash([1, 60, 240, 139], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `propose` (0x7d5e81e2) function
        pub fn propose(
            &self,
            targets: ::std::vec::Vec<::ethers_core::types::Address>,
            values: ::std::vec::Vec<::ethers_core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
            description: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [125, 94, 129, 226],
                    (targets, values, calldatas, description),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `propose` (0xda95691a) function
        pub fn propose_with_targets_and_values_and_signatures_and_calldatas(
            &self,
            targets: ::std::vec::Vec<::ethers_core::types::Address>,
            values: ::std::vec::Vec<::ethers_core::types::U256>,
            signatures: ::std::vec::Vec<::std::string::String>,
            calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
            description: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [218, 149, 105, 26],
                    (targets, values, signatures, calldatas, description),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queue` (0x160cbed7) function
        pub fn queue_with_targets(
            &self,
            targets: ::std::vec::Vec<::ethers_core::types::Address>,
            values: ::std::vec::Vec<::ethers_core::types::U256>,
            calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
            description_hash: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash(
                    [22, 12, 190, 215],
                    (targets, values, calldatas, description_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queue` (0xddf0b009) function
        pub fn queue(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 240, 176, 9], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorum` (0xf8ce560a) function
        pub fn quorum(
            &self,
            timepoint: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([248, 206, 86, 10], timepoint)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumVotes` (0x24bc1a64) function
        pub fn quorum_votes(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([36, 188, 26, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relay` (0xc28bc2fa) function
        pub fn relay(
            &self,
            target: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 139, 194, 250], (target, value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `state` (0x3e4f49e6) function
        pub fn state(
            &self,
            proposal_id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([62, 79, 73, 230], proposal_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timelock` (0xd33219b4) function
        pub fn timelock(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers_core::types::Address,
        > {
            self.0
                .method_hash([211, 50, 25, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingDelay` (0x3932abb1) function
        pub fn voting_delay(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([57, 50, 171, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votingPeriod` (0x02a251a3) function
        pub fn voting_period(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([2, 162, 81, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalCanceled` event
        pub fn proposal_canceled_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalCanceledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalCreated` event
        pub fn proposal_created_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalExecuted` event
        pub fn proposal_executed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalExecutedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposalQueued` event
        pub fn proposal_queued_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalQueuedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VoteCast` event
        pub fn vote_cast_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, VoteCastFilter> {
            self.0.event()
        }
        ///Gets the contract's `VoteCastWithParams` event
        pub fn vote_cast_with_params_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoteCastWithParamsFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GovernorCompatibilityBravoEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for GovernorCompatibilityBravo<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Empty` with signature `Empty()` and selector `0x3db2a12a`
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
    #[etherror(name = "Empty", abi = "Empty()")]
    pub struct Empty;
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
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
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
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
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GovernorCompatibilityBravoErrors {
        Empty(Empty),
        InvalidShortString(InvalidShortString),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for GovernorCompatibilityBravoErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Empty as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Empty(decoded));
            }
            if let Ok(decoded)
                = <InvalidShortString as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded)
                = <StringTooLong as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StringTooLong(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for GovernorCompatibilityBravoErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Empty(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::InvalidShortString(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::StringTooLong(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for GovernorCompatibilityBravoErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <Empty as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidShortString as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringTooLong as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GovernorCompatibilityBravoErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Empty(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidShortString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for GovernorCompatibilityBravoErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Empty> for GovernorCompatibilityBravoErrors {
        fn from(value: Empty) -> Self {
            Self::Empty(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for GovernorCompatibilityBravoErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for GovernorCompatibilityBravoErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
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
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
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
    #[ethevent(name = "ProposalCanceled", abi = "ProposalCanceled(uint256)")]
    pub struct ProposalCanceledFilter {
        pub proposal_id: ::ethers_core::types::U256,
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
        name = "ProposalCreated",
        abi = "ProposalCreated(uint256,address,address[],uint256[],string[],bytes[],uint256,uint256,string)"
    )]
    pub struct ProposalCreatedFilter {
        pub proposal_id: ::ethers_core::types::U256,
        pub proposer: ::ethers_core::types::Address,
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub signatures: ::std::vec::Vec<::std::string::String>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub vote_start: ::ethers_core::types::U256,
        pub vote_end: ::ethers_core::types::U256,
        pub description: ::std::string::String,
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
    #[ethevent(name = "ProposalExecuted", abi = "ProposalExecuted(uint256)")]
    pub struct ProposalExecutedFilter {
        pub proposal_id: ::ethers_core::types::U256,
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
    #[ethevent(name = "ProposalQueued", abi = "ProposalQueued(uint256,uint256)")]
    pub struct ProposalQueuedFilter {
        pub proposal_id: ::ethers_core::types::U256,
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
        name = "VoteCast",
        abi = "VoteCast(address,uint256,uint8,uint256,string)"
    )]
    pub struct VoteCastFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers_core::types::Address,
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
        pub weight: ::ethers_core::types::U256,
        pub reason: ::std::string::String,
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
        name = "VoteCastWithParams",
        abi = "VoteCastWithParams(address,uint256,uint8,uint256,string,bytes)"
    )]
    pub struct VoteCastWithParamsFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers_core::types::Address,
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
        pub weight: ::ethers_core::types::U256,
        pub reason: ::std::string::String,
        pub params: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GovernorCompatibilityBravoEvents {
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        ProposalCanceledFilter(ProposalCanceledFilter),
        ProposalCreatedFilter(ProposalCreatedFilter),
        ProposalExecutedFilter(ProposalExecutedFilter),
        ProposalQueuedFilter(ProposalQueuedFilter),
        VoteCastFilter(VoteCastFilter),
        VoteCastWithParamsFilter(VoteCastWithParamsFilter),
    }
    impl ::ethers_contract::EthLogDecode for GovernorCompatibilityBravoEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(
                    GovernorCompatibilityBravoEvents::Eip712DomainChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ProposalCanceledFilter::decode_log(log) {
                return Ok(
                    GovernorCompatibilityBravoEvents::ProposalCanceledFilter(decoded),
                );
            }
            if let Ok(decoded) = ProposalCreatedFilter::decode_log(log) {
                return Ok(
                    GovernorCompatibilityBravoEvents::ProposalCreatedFilter(decoded),
                );
            }
            if let Ok(decoded) = ProposalExecutedFilter::decode_log(log) {
                return Ok(
                    GovernorCompatibilityBravoEvents::ProposalExecutedFilter(decoded),
                );
            }
            if let Ok(decoded) = ProposalQueuedFilter::decode_log(log) {
                return Ok(
                    GovernorCompatibilityBravoEvents::ProposalQueuedFilter(decoded),
                );
            }
            if let Ok(decoded) = VoteCastFilter::decode_log(log) {
                return Ok(GovernorCompatibilityBravoEvents::VoteCastFilter(decoded));
            }
            if let Ok(decoded) = VoteCastWithParamsFilter::decode_log(log) {
                return Ok(
                    GovernorCompatibilityBravoEvents::VoteCastWithParamsFilter(decoded),
                );
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GovernorCompatibilityBravoEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Eip712DomainChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalExecutedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposalQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoteCastFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoteCastWithParamsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter>
    for GovernorCompatibilityBravoEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalCanceledFilter>
    for GovernorCompatibilityBravoEvents {
        fn from(value: ProposalCanceledFilter) -> Self {
            Self::ProposalCanceledFilter(value)
        }
    }
    impl ::core::convert::From<ProposalCreatedFilter>
    for GovernorCompatibilityBravoEvents {
        fn from(value: ProposalCreatedFilter) -> Self {
            Self::ProposalCreatedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalExecutedFilter>
    for GovernorCompatibilityBravoEvents {
        fn from(value: ProposalExecutedFilter) -> Self {
            Self::ProposalExecutedFilter(value)
        }
    }
    impl ::core::convert::From<ProposalQueuedFilter>
    for GovernorCompatibilityBravoEvents {
        fn from(value: ProposalQueuedFilter) -> Self {
            Self::ProposalQueuedFilter(value)
        }
    }
    impl ::core::convert::From<VoteCastFilter> for GovernorCompatibilityBravoEvents {
        fn from(value: VoteCastFilter) -> Self {
            Self::VoteCastFilter(value)
        }
    }
    impl ::core::convert::From<VoteCastWithParamsFilter>
    for GovernorCompatibilityBravoEvents {
        fn from(value: VoteCastWithParamsFilter) -> Self {
            Self::VoteCastWithParamsFilter(value)
        }
    }
    ///Container type for all input parameters for the `BALLOT_TYPEHASH` function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`
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
    #[ethcall(name = "BALLOT_TYPEHASH", abi = "BALLOT_TYPEHASH()")]
    pub struct BallotTypehashCall;
    ///Container type for all input parameters for the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
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
    #[ethcall(name = "CLOCK_MODE", abi = "CLOCK_MODE()")]
    pub struct ClockModeCall;
    ///Container type for all input parameters for the `COUNTING_MODE` function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`
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
    #[ethcall(name = "COUNTING_MODE", abi = "COUNTING_MODE()")]
    pub struct CountingModeCall;
    ///Container type for all input parameters for the `EXTENDED_BALLOT_TYPEHASH` function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`
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
    #[ethcall(name = "EXTENDED_BALLOT_TYPEHASH", abi = "EXTENDED_BALLOT_TYPEHASH()")]
    pub struct ExtendedBallotTypehashCall;
    ///Container type for all input parameters for the `cancel` function with signature `cancel(uint256)` and selector `0x40e58ee5`
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
    #[ethcall(name = "cancel", abi = "cancel(uint256)")]
    pub struct CancelCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `cancel` function with signature `cancel(address[],uint256[],bytes[],bytes32)` and selector `0x452115d6`
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
    #[ethcall(name = "cancel", abi = "cancel(address[],uint256[],bytes[],bytes32)")]
    pub struct CancelWithTargetsCall {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `castVote` function with signature `castVote(uint256,uint8)` and selector `0x56781388`
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
    #[ethcall(name = "castVote", abi = "castVote(uint256,uint8)")]
    pub struct CastVoteCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
    }
    ///Container type for all input parameters for the `castVoteBySig` function with signature `castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)` and selector `0x3bccf4fd`
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
        name = "castVoteBySig",
        abi = "castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)"
    )]
    pub struct CastVoteBySigCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `castVoteWithReason` function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`
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
        name = "castVoteWithReason",
        abi = "castVoteWithReason(uint256,uint8,string)"
    )]
    pub struct CastVoteWithReasonCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
    }
    ///Container type for all input parameters for the `castVoteWithReasonAndParams` function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`
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
        name = "castVoteWithReasonAndParams",
        abi = "castVoteWithReasonAndParams(uint256,uint8,string,bytes)"
    )]
    pub struct CastVoteWithReasonAndParamsCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
        pub params: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `castVoteWithReasonAndParamsBySig` function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)` and selector `0x03420181`
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
        name = "castVoteWithReasonAndParamsBySig",
        abi = "castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)"
    )]
    pub struct CastVoteWithReasonAndParamsBySigCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub support: u8,
        pub reason: ::std::string::String,
        pub params: ::ethers_core::types::Bytes,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `clock` function with signature `clock()` and selector `0x91ddadf4`
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
    #[ethcall(name = "clock", abi = "clock()")]
    pub struct ClockCall;
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all input parameters for the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
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
    #[ethcall(name = "execute", abi = "execute(address[],uint256[],bytes[],bytes32)")]
    pub struct ExecuteWithTargetsCall {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(uint256)` and selector `0xfe0d94c1`
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
    #[ethcall(name = "execute", abi = "execute(uint256)")]
    pub struct ExecuteCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `getActions` function with signature `getActions(uint256)` and selector `0x328dd982`
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
    #[ethcall(name = "getActions", abi = "getActions(uint256)")]
    pub struct GetActionsCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `getReceipt` function with signature `getReceipt(uint256,address)` and selector `0xe23a9a52`
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
    #[ethcall(name = "getReceipt", abi = "getReceipt(uint256,address)")]
    pub struct GetReceiptCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub voter: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `getVotes` function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`
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
    #[ethcall(name = "getVotes", abi = "getVotes(address,uint256)")]
    pub struct GetVotesCall {
        pub account: ::ethers_core::types::Address,
        pub timepoint: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `getVotesWithParams` function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`
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
        name = "getVotesWithParams",
        abi = "getVotesWithParams(address,uint256,bytes)"
    )]
    pub struct GetVotesWithParamsCall {
        pub account: ::ethers_core::types::Address,
        pub timepoint: ::ethers_core::types::U256,
        pub params: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasVoted` function with signature `hasVoted(uint256,address)` and selector `0x43859632`
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
    #[ethcall(name = "hasVoted", abi = "hasVoted(uint256,address)")]
    pub struct HasVotedCall {
        pub proposal_id: ::ethers_core::types::U256,
        pub account: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `hashProposal` function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`
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
        name = "hashProposal",
        abi = "hashProposal(address[],uint256[],bytes[],bytes32)"
    )]
    pub struct HashProposalCall {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::Address,
        pub ::std::vec::Vec<::ethers_core::types::U256>,
        pub ::std::vec::Vec<::ethers_core::types::U256>,
        pub ::ethers_core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::U256,
        pub ::ethers_core::types::U256,
        pub ::ethers_core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::U256,
        pub ::ethers_core::types::Bytes,
    );
    ///Container type for all input parameters for the `proposalDeadline` function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`
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
    #[ethcall(name = "proposalDeadline", abi = "proposalDeadline(uint256)")]
    pub struct ProposalDeadlineCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `proposalEta` function with signature `proposalEta(uint256)` and selector `0xab58fb8e`
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
    #[ethcall(name = "proposalEta", abi = "proposalEta(uint256)")]
    pub struct ProposalEtaCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `proposalProposer` function with signature `proposalProposer(uint256)` and selector `0x143489d0`
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
    #[ethcall(name = "proposalProposer", abi = "proposalProposer(uint256)")]
    pub struct ProposalProposerCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `proposalSnapshot` function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`
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
    #[ethcall(name = "proposalSnapshot", abi = "proposalSnapshot(uint256)")]
    pub struct ProposalSnapshotCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `proposalThreshold` function with signature `proposalThreshold()` and selector `0xb58131b0`
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
    #[ethcall(name = "proposalThreshold", abi = "proposalThreshold()")]
    pub struct ProposalThresholdCall;
    ///Container type for all input parameters for the `proposals` function with signature `proposals(uint256)` and selector `0x013cf08b`
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
    #[ethcall(name = "proposals", abi = "proposals(uint256)")]
    pub struct ProposalsCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `propose` function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`
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
    #[ethcall(name = "propose", abi = "propose(address[],uint256[],bytes[],string)")]
    pub struct ProposeCall {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `propose` function with signature `propose(address[],uint256[],string[],bytes[],string)` and selector `0xda95691a`
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
        name = "propose",
        abi = "propose(address[],uint256[],string[],bytes[],string)"
    )]
    pub struct ProposeWithTargetsAndValuesAndSignaturesAndCalldatasCall {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub signatures: ::std::vec::Vec<::std::string::String>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub description: ::std::string::String,
    }
    ///Container type for all input parameters for the `queue` function with signature `queue(address[],uint256[],bytes[],bytes32)` and selector `0x160cbed7`
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
    #[ethcall(name = "queue", abi = "queue(address[],uint256[],bytes[],bytes32)")]
    pub struct QueueWithTargetsCall {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub description_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `queue` function with signature `queue(uint256)` and selector `0xddf0b009`
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
    #[ethcall(name = "queue", abi = "queue(uint256)")]
    pub struct QueueCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `quorum` function with signature `quorum(uint256)` and selector `0xf8ce560a`
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
    #[ethcall(name = "quorum", abi = "quorum(uint256)")]
    pub struct QuorumCall {
        pub timepoint: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `quorumVotes` function with signature `quorumVotes()` and selector `0x24bc1a64`
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
    #[ethcall(name = "quorumVotes", abi = "quorumVotes()")]
    pub struct QuorumVotesCall;
    ///Container type for all input parameters for the `relay` function with signature `relay(address,uint256,bytes)` and selector `0xc28bc2fa`
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
    #[ethcall(name = "relay", abi = "relay(address,uint256,bytes)")]
    pub struct RelayCall {
        pub target: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `state` function with signature `state(uint256)` and selector `0x3e4f49e6`
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
    #[ethcall(name = "state", abi = "state(uint256)")]
    pub struct StateCall {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `timelock` function with signature `timelock()` and selector `0xd33219b4`
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
    #[ethcall(name = "timelock", abi = "timelock()")]
    pub struct TimelockCall;
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `votingDelay` function with signature `votingDelay()` and selector `0x3932abb1`
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
    #[ethcall(name = "votingDelay", abi = "votingDelay()")]
    pub struct VotingDelayCall;
    ///Container type for all input parameters for the `votingPeriod` function with signature `votingPeriod()` and selector `0x02a251a3`
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
    #[ethcall(name = "votingPeriod", abi = "votingPeriod()")]
    pub struct VotingPeriodCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GovernorCompatibilityBravoCalls {
        BallotTypehash(BallotTypehashCall),
        ClockMode(ClockModeCall),
        CountingMode(CountingModeCall),
        ExtendedBallotTypehash(ExtendedBallotTypehashCall),
        Cancel(CancelCall),
        CancelWithTargets(CancelWithTargetsCall),
        CastVote(CastVoteCall),
        CastVoteBySig(CastVoteBySigCall),
        CastVoteWithReason(CastVoteWithReasonCall),
        CastVoteWithReasonAndParams(CastVoteWithReasonAndParamsCall),
        CastVoteWithReasonAndParamsBySig(CastVoteWithReasonAndParamsBySigCall),
        Clock(ClockCall),
        Eip712Domain(Eip712DomainCall),
        ExecuteWithTargets(ExecuteWithTargetsCall),
        Execute(ExecuteCall),
        GetActions(GetActionsCall),
        GetReceipt(GetReceiptCall),
        GetVotes(GetVotesCall),
        GetVotesWithParams(GetVotesWithParamsCall),
        HasVoted(HasVotedCall),
        HashProposal(HashProposalCall),
        Name(NameCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        ProposalDeadline(ProposalDeadlineCall),
        ProposalEta(ProposalEtaCall),
        ProposalProposer(ProposalProposerCall),
        ProposalSnapshot(ProposalSnapshotCall),
        ProposalThreshold(ProposalThresholdCall),
        Proposals(ProposalsCall),
        Propose(ProposeCall),
        ProposeWithTargetsAndValuesAndSignaturesAndCalldatas(
            ProposeWithTargetsAndValuesAndSignaturesAndCalldatasCall,
        ),
        QueueWithTargets(QueueWithTargetsCall),
        Queue(QueueCall),
        Quorum(QuorumCall),
        QuorumVotes(QuorumVotesCall),
        Relay(RelayCall),
        State(StateCall),
        SupportsInterface(SupportsInterfaceCall),
        Timelock(TimelockCall),
        Version(VersionCall),
        VotingDelay(VotingDelayCall),
        VotingPeriod(VotingPeriodCall),
    }
    impl ::ethers_core::abi::AbiDecode for GovernorCompatibilityBravoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BallotTypehashCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BallotTypehash(decoded));
            }
            if let Ok(decoded)
                = <ClockModeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClockMode(decoded));
            }
            if let Ok(decoded)
                = <CountingModeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CountingMode(decoded));
            }
            if let Ok(decoded)
                = <ExtendedBallotTypehashCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExtendedBallotTypehash(decoded));
            }
            if let Ok(decoded)
                = <CancelCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Cancel(decoded));
            }
            if let Ok(decoded)
                = <CancelWithTargetsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CancelWithTargets(decoded));
            }
            if let Ok(decoded)
                = <CastVoteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CastVote(decoded));
            }
            if let Ok(decoded)
                = <CastVoteBySigCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CastVoteBySig(decoded));
            }
            if let Ok(decoded)
                = <CastVoteWithReasonCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CastVoteWithReason(decoded));
            }
            if let Ok(decoded)
                = <CastVoteWithReasonAndParamsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CastVoteWithReasonAndParams(decoded));
            }
            if let Ok(decoded)
                = <CastVoteWithReasonAndParamsBySigCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CastVoteWithReasonAndParamsBySig(decoded));
            }
            if let Ok(decoded)
                = <ClockCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Clock(decoded));
            }
            if let Ok(decoded)
                = <Eip712DomainCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded)
                = <ExecuteWithTargetsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ExecuteWithTargets(decoded));
            }
            if let Ok(decoded)
                = <ExecuteCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded)
                = <GetActionsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetActions(decoded));
            }
            if let Ok(decoded)
                = <GetReceiptCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReceipt(decoded));
            }
            if let Ok(decoded)
                = <GetVotesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVotes(decoded));
            }
            if let Ok(decoded)
                = <GetVotesWithParamsCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetVotesWithParams(decoded));
            }
            if let Ok(decoded)
                = <HasVotedCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasVoted(decoded));
            }
            if let Ok(decoded)
                = <HashProposalCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashProposal(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155BatchReceivedCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155ReceivedCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded)
                = <OnERC721ReceivedCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded)
                = <ProposalDeadlineCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposalDeadline(decoded));
            }
            if let Ok(decoded)
                = <ProposalEtaCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposalEta(decoded));
            }
            if let Ok(decoded)
                = <ProposalProposerCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposalProposer(decoded));
            }
            if let Ok(decoded)
                = <ProposalSnapshotCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposalSnapshot(decoded));
            }
            if let Ok(decoded)
                = <ProposalThresholdCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposalThreshold(decoded));
            }
            if let Ok(decoded)
                = <ProposalsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Proposals(decoded));
            }
            if let Ok(decoded)
                = <ProposeCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Propose(decoded));
            }
            if let Ok(decoded)
                = <ProposeWithTargetsAndValuesAndSignaturesAndCalldatasCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::ProposeWithTargetsAndValuesAndSignaturesAndCalldatas(decoded),
                );
            }
            if let Ok(decoded)
                = <QueueWithTargetsCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueueWithTargets(decoded));
            }
            if let Ok(decoded)
                = <QueueCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Queue(decoded));
            }
            if let Ok(decoded)
                = <QuorumCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Quorum(decoded));
            }
            if let Ok(decoded)
                = <QuorumVotesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuorumVotes(decoded));
            }
            if let Ok(decoded)
                = <RelayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Relay(decoded));
            }
            if let Ok(decoded)
                = <StateCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers_core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <TimelockCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Timelock(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded)
                = <VotingDelayCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotingDelay(decoded));
            }
            if let Ok(decoded)
                = <VotingPeriodCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VotingPeriod(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for GovernorCompatibilityBravoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BallotTypehash(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ClockMode(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CountingMode(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExtendedBallotTypehash(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Cancel(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CancelWithTargets(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CastVote(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::CastVoteBySig(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReason(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReasonAndParams(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::CastVoteWithReasonAndParamsBySig(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Clock(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Eip712Domain(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteWithTargets(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetActions(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetReceipt(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::GetVotes(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetVotesWithParams(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::HasVoted(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::HashProposal(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ProposalDeadline(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ProposalEta(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ProposalProposer(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ProposalSnapshot(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ProposalThreshold(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Proposals(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Propose(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ProposeWithTargetsAndValuesAndSignaturesAndCalldatas(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::QueueWithTargets(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Queue(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Quorum(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::QuorumVotes(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Relay(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Timelock(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::VotingDelay(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::VotingPeriod(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GovernorCompatibilityBravoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BallotTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClockMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::CountingMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtendedBallotTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cancel(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelWithTargets(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVote(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVoteBySig(element) => ::core::fmt::Display::fmt(element, f),
                Self::CastVoteWithReason(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVoteWithReasonAndParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CastVoteWithReasonAndParamsBySig(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Clock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithTargets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetActions(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReceipt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetVotesWithParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HasVoted(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalDeadline(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalEta(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalSnapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::Proposals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Propose(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeWithTargetsAndValuesAndSignaturesAndCalldatas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QueueWithTargets(element) => ::core::fmt::Display::fmt(element, f),
                Self::Queue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumVotes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Relay(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Timelock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::VotingPeriod(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BallotTypehashCall> for GovernorCompatibilityBravoCalls {
        fn from(value: BallotTypehashCall) -> Self {
            Self::BallotTypehash(value)
        }
    }
    impl ::core::convert::From<ClockModeCall> for GovernorCompatibilityBravoCalls {
        fn from(value: ClockModeCall) -> Self {
            Self::ClockMode(value)
        }
    }
    impl ::core::convert::From<CountingModeCall> for GovernorCompatibilityBravoCalls {
        fn from(value: CountingModeCall) -> Self {
            Self::CountingMode(value)
        }
    }
    impl ::core::convert::From<ExtendedBallotTypehashCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: ExtendedBallotTypehashCall) -> Self {
            Self::ExtendedBallotTypehash(value)
        }
    }
    impl ::core::convert::From<CancelCall> for GovernorCompatibilityBravoCalls {
        fn from(value: CancelCall) -> Self {
            Self::Cancel(value)
        }
    }
    impl ::core::convert::From<CancelWithTargetsCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: CancelWithTargetsCall) -> Self {
            Self::CancelWithTargets(value)
        }
    }
    impl ::core::convert::From<CastVoteCall> for GovernorCompatibilityBravoCalls {
        fn from(value: CastVoteCall) -> Self {
            Self::CastVote(value)
        }
    }
    impl ::core::convert::From<CastVoteBySigCall> for GovernorCompatibilityBravoCalls {
        fn from(value: CastVoteBySigCall) -> Self {
            Self::CastVoteBySig(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: CastVoteWithReasonCall) -> Self {
            Self::CastVoteWithReason(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonAndParamsCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: CastVoteWithReasonAndParamsCall) -> Self {
            Self::CastVoteWithReasonAndParams(value)
        }
    }
    impl ::core::convert::From<CastVoteWithReasonAndParamsBySigCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: CastVoteWithReasonAndParamsBySigCall) -> Self {
            Self::CastVoteWithReasonAndParamsBySig(value)
        }
    }
    impl ::core::convert::From<ClockCall> for GovernorCompatibilityBravoCalls {
        fn from(value: ClockCall) -> Self {
            Self::Clock(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for GovernorCompatibilityBravoCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<ExecuteWithTargetsCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: ExecuteWithTargetsCall) -> Self {
            Self::ExecuteWithTargets(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for GovernorCompatibilityBravoCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetActionsCall> for GovernorCompatibilityBravoCalls {
        fn from(value: GetActionsCall) -> Self {
            Self::GetActions(value)
        }
    }
    impl ::core::convert::From<GetReceiptCall> for GovernorCompatibilityBravoCalls {
        fn from(value: GetReceiptCall) -> Self {
            Self::GetReceipt(value)
        }
    }
    impl ::core::convert::From<GetVotesCall> for GovernorCompatibilityBravoCalls {
        fn from(value: GetVotesCall) -> Self {
            Self::GetVotes(value)
        }
    }
    impl ::core::convert::From<GetVotesWithParamsCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: GetVotesWithParamsCall) -> Self {
            Self::GetVotesWithParams(value)
        }
    }
    impl ::core::convert::From<HasVotedCall> for GovernorCompatibilityBravoCalls {
        fn from(value: HasVotedCall) -> Self {
            Self::HasVoted(value)
        }
    }
    impl ::core::convert::From<HashProposalCall> for GovernorCompatibilityBravoCalls {
        fn from(value: HashProposalCall) -> Self {
            Self::HashProposal(value)
        }
    }
    impl ::core::convert::From<NameCall> for GovernorCompatibilityBravoCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<ProposalDeadlineCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: ProposalDeadlineCall) -> Self {
            Self::ProposalDeadline(value)
        }
    }
    impl ::core::convert::From<ProposalEtaCall> for GovernorCompatibilityBravoCalls {
        fn from(value: ProposalEtaCall) -> Self {
            Self::ProposalEta(value)
        }
    }
    impl ::core::convert::From<ProposalProposerCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: ProposalProposerCall) -> Self {
            Self::ProposalProposer(value)
        }
    }
    impl ::core::convert::From<ProposalSnapshotCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: ProposalSnapshotCall) -> Self {
            Self::ProposalSnapshot(value)
        }
    }
    impl ::core::convert::From<ProposalThresholdCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: ProposalThresholdCall) -> Self {
            Self::ProposalThreshold(value)
        }
    }
    impl ::core::convert::From<ProposalsCall> for GovernorCompatibilityBravoCalls {
        fn from(value: ProposalsCall) -> Self {
            Self::Proposals(value)
        }
    }
    impl ::core::convert::From<ProposeCall> for GovernorCompatibilityBravoCalls {
        fn from(value: ProposeCall) -> Self {
            Self::Propose(value)
        }
    }
    impl ::core::convert::From<ProposeWithTargetsAndValuesAndSignaturesAndCalldatasCall>
    for GovernorCompatibilityBravoCalls {
        fn from(
            value: ProposeWithTargetsAndValuesAndSignaturesAndCalldatasCall,
        ) -> Self {
            Self::ProposeWithTargetsAndValuesAndSignaturesAndCalldatas(value)
        }
    }
    impl ::core::convert::From<QueueWithTargetsCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: QueueWithTargetsCall) -> Self {
            Self::QueueWithTargets(value)
        }
    }
    impl ::core::convert::From<QueueCall> for GovernorCompatibilityBravoCalls {
        fn from(value: QueueCall) -> Self {
            Self::Queue(value)
        }
    }
    impl ::core::convert::From<QuorumCall> for GovernorCompatibilityBravoCalls {
        fn from(value: QuorumCall) -> Self {
            Self::Quorum(value)
        }
    }
    impl ::core::convert::From<QuorumVotesCall> for GovernorCompatibilityBravoCalls {
        fn from(value: QuorumVotesCall) -> Self {
            Self::QuorumVotes(value)
        }
    }
    impl ::core::convert::From<RelayCall> for GovernorCompatibilityBravoCalls {
        fn from(value: RelayCall) -> Self {
            Self::Relay(value)
        }
    }
    impl ::core::convert::From<StateCall> for GovernorCompatibilityBravoCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall>
    for GovernorCompatibilityBravoCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<TimelockCall> for GovernorCompatibilityBravoCalls {
        fn from(value: TimelockCall) -> Self {
            Self::Timelock(value)
        }
    }
    impl ::core::convert::From<VersionCall> for GovernorCompatibilityBravoCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<VotingDelayCall> for GovernorCompatibilityBravoCalls {
        fn from(value: VotingDelayCall) -> Self {
            Self::VotingDelay(value)
        }
    }
    impl ::core::convert::From<VotingPeriodCall> for GovernorCompatibilityBravoCalls {
        fn from(value: VotingPeriodCall) -> Self {
            Self::VotingPeriod(value)
        }
    }
    ///Container type for all return fields from the `BALLOT_TYPEHASH` function with signature `BALLOT_TYPEHASH()` and selector `0xdeaaa7cc`
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
    pub struct BallotTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `CLOCK_MODE` function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`
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
    pub struct ClockModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `COUNTING_MODE` function with signature `COUNTING_MODE()` and selector `0xdd4e2ba5`
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
    pub struct CountingModeReturn(pub ::std::string::String);
    ///Container type for all return fields from the `EXTENDED_BALLOT_TYPEHASH` function with signature `EXTENDED_BALLOT_TYPEHASH()` and selector `0x2fe3e261`
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
    pub struct ExtendedBallotTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cancel` function with signature `cancel(address[],uint256[],bytes[],bytes32)` and selector `0x452115d6`
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
    pub struct CancelWithTargetsReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `castVote` function with signature `castVote(uint256,uint8)` and selector `0x56781388`
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
    pub struct CastVoteReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `castVoteBySig` function with signature `castVoteBySig(uint256,uint8,uint8,bytes32,bytes32)` and selector `0x3bccf4fd`
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
    pub struct CastVoteBySigReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `castVoteWithReason` function with signature `castVoteWithReason(uint256,uint8,string)` and selector `0x7b3c71d3`
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
    pub struct CastVoteWithReasonReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `castVoteWithReasonAndParams` function with signature `castVoteWithReasonAndParams(uint256,uint8,string,bytes)` and selector `0x5f398a14`
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
    pub struct CastVoteWithReasonAndParamsReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `castVoteWithReasonAndParamsBySig` function with signature `castVoteWithReasonAndParamsBySig(uint256,uint8,string,bytes,uint8,bytes32,bytes32)` and selector `0x03420181`
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
    pub struct CastVoteWithReasonAndParamsBySigReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `clock` function with signature `clock()` and selector `0x91ddadf4`
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
    pub struct ClockReturn(pub u64);
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers_core::types::U256,
        pub verifying_contract: ::ethers_core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers_core::types::U256>,
    }
    ///Container type for all return fields from the `execute` function with signature `execute(address[],uint256[],bytes[],bytes32)` and selector `0x2656227d`
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
    pub struct ExecuteWithTargetsReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getActions` function with signature `getActions(uint256)` and selector `0x328dd982`
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
    pub struct GetActionsReturn {
        pub targets: ::std::vec::Vec<::ethers_core::types::Address>,
        pub values: ::std::vec::Vec<::ethers_core::types::U256>,
        pub signatures: ::std::vec::Vec<::std::string::String>,
        pub calldatas: ::std::vec::Vec<::ethers_core::types::Bytes>,
    }
    ///Container type for all return fields from the `getReceipt` function with signature `getReceipt(uint256,address)` and selector `0xe23a9a52`
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
    pub struct GetReceiptReturn(pub Receipt);
    ///Container type for all return fields from the `getVotes` function with signature `getVotes(address,uint256)` and selector `0xeb9019d4`
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
    pub struct GetVotesReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `getVotesWithParams` function with signature `getVotesWithParams(address,uint256,bytes)` and selector `0x9a802a6d`
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
    pub struct GetVotesWithParamsReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `hasVoted` function with signature `hasVoted(uint256,address)` and selector `0x43859632`
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
    pub struct HasVotedReturn(pub bool);
    ///Container type for all return fields from the `hashProposal` function with signature `hashProposal(address[],uint256[],bytes[],bytes32)` and selector `0xc59057e4`
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
    pub struct HashProposalReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `proposalDeadline` function with signature `proposalDeadline(uint256)` and selector `0xc01f9e37`
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
    pub struct ProposalDeadlineReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `proposalEta` function with signature `proposalEta(uint256)` and selector `0xab58fb8e`
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
    pub struct ProposalEtaReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `proposalProposer` function with signature `proposalProposer(uint256)` and selector `0x143489d0`
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
    pub struct ProposalProposerReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `proposalSnapshot` function with signature `proposalSnapshot(uint256)` and selector `0x2d63f693`
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
    pub struct ProposalSnapshotReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `proposalThreshold` function with signature `proposalThreshold()` and selector `0xb58131b0`
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
    pub struct ProposalThresholdReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `proposals` function with signature `proposals(uint256)` and selector `0x013cf08b`
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
    pub struct ProposalsReturn {
        pub id: ::ethers_core::types::U256,
        pub proposer: ::ethers_core::types::Address,
        pub eta: ::ethers_core::types::U256,
        pub start_block: ::ethers_core::types::U256,
        pub end_block: ::ethers_core::types::U256,
        pub for_votes: ::ethers_core::types::U256,
        pub against_votes: ::ethers_core::types::U256,
        pub abstain_votes: ::ethers_core::types::U256,
        pub canceled: bool,
        pub executed: bool,
    }
    ///Container type for all return fields from the `propose` function with signature `propose(address[],uint256[],bytes[],string)` and selector `0x7d5e81e2`
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
    pub struct ProposeReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `propose` function with signature `propose(address[],uint256[],string[],bytes[],string)` and selector `0xda95691a`
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
    pub struct ProposeWithTargetsAndValuesAndSignaturesAndCalldatasReturn(
        pub ::ethers_core::types::U256,
    );
    ///Container type for all return fields from the `queue` function with signature `queue(address[],uint256[],bytes[],bytes32)` and selector `0x160cbed7`
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
    pub struct QueueWithTargetsReturn {
        pub proposal_id: ::ethers_core::types::U256,
    }
    ///Container type for all return fields from the `quorum` function with signature `quorum(uint256)` and selector `0xf8ce560a`
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
    pub struct QuorumReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `quorumVotes` function with signature `quorumVotes()` and selector `0x24bc1a64`
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
    pub struct QuorumVotesReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `state` function with signature `state(uint256)` and selector `0x3e4f49e6`
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
    pub struct StateReturn(pub u8);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `timelock` function with signature `timelock()` and selector `0xd33219b4`
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
    pub struct TimelockReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `votingDelay` function with signature `votingDelay()` and selector `0x3932abb1`
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
    pub struct VotingDelayReturn(pub ::ethers_core::types::U256);
    ///Container type for all return fields from the `votingPeriod` function with signature `votingPeriod()` and selector `0x02a251a3`
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
    pub struct VotingPeriodReturn(pub ::ethers_core::types::U256);
    ///`Receipt(bool,uint8,uint96)`
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
    pub struct Receipt {
        pub has_voted: bool,
        pub support: u8,
        pub votes: u128,
    }
}
