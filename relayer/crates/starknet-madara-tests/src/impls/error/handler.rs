use core::array::TryFromSliceError;
use core::convert::Infallible;
use core::num::{ParseIntError, TryFromIntError};
use core::str::Utf8Error;
use std::string::FromUtf8Error;

use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use eyre::Report;
use hermes_cairo_encoding_components::impls::{
    DecodeBoolError, NonEmptyBuffer, UnexpectedEndOfBuffer, VariantIndexOutOfBound,
};
use hermes_core::chain_components::impls::InvalidTimeoutReceipt;
use hermes_core::chain_components::traits::{
    EmptyMessageResponse, HasClientIdType, HasOutgoingPacketType,
};
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType, HasHeightType};
use hermes_core::relayer_components::chain::impls::NoConsensusStateAtLessThanHeight;
use hermes_core::relayer_components::transaction::impls::TxNoResponseError;
use hermes_core::relayer_components::transaction::traits::HasTxHashType;
use hermes_core::test_components::chain::impls::{
    EventualAmountTimeoutError, MissingSendPacketEventError,
};
use hermes_cosmos_core::protobuf_encoding_components::impls::{
    InvalidWireType, RequiredFieldTagNotFound, TypeUrlMismatchError, UnsupportedWireType,
};
use hermes_error::handlers::{
    DebugError, DisplayError, HandleInfallible, ReportError, ReturnError,
};
use hermes_error::types::Error;
use hermes_prelude::*;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_starknet_chain_components::impls::{ConsensusStateNotFound, ContractAddressNotFound};
use hermes_starknet_chain_components::types::UnknownEvent;
use ibc::core::channel::types::error::ChannelError;
use ibc::core::client::types::error::ClientError;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use ibc::primitives::TimestampError;
use prost::{DecodeError, EncodeError};
use starknet_v13::accounts::{single_owner, AccountError};
use starknet_v13::core::types::contract::{ComputeClassHashError, JsonError};
use starknet_v13::core::types::{RevertedInvocation, StarknetError};
use starknet_v13::providers::ProviderError;
use starknet_v13::signers::local_wallet;
use url::ParseError;

use crate::impls::{
    RaiseAccountError, RaiseProviderError, RaiseStarknetError, UnexpectedTransactionTraceType,
};

pub struct HandleMadaraChainError;

pub type SignError = single_owner::SignError<local_wallet::SignError>;

delegate_components! {
    HandleMadaraChainError {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TryFromIntError,
            Utf8Error,
            ParseIntError,
            FromUtf8Error,
            SignError,
            TryFromSliceError,
            TokioRuntimeError,
            serde_json::error::Error,
            toml::ser::Error,
            reqwest::Error,
            JsonError,
            EncodeError,
            DecodeError,
            ParseError,
            DecodingError,
            ClientError,
            ChannelError,
            TimestampError,
            IdentifierError,
            ComputeClassHashError,
            StarknetSierraCompilationError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            RevertedInvocation,
            UnexpectedTransactionTraceType,
            UnexpectedEndOfBuffer,
            NonEmptyBuffer,
            VariantIndexOutOfBound,
            DecodeBoolError,
            TypeUrlMismatchError,
            InvalidWireType,
            UnsupportedWireType,
            RequiredFieldTagNotFound,
            ContractAddressNotFound,
            EmptyMessageResponse,
            ConsensusStateNotFound,
            MissingSendPacketEventError,
            <'a> UnknownEvent<'a>,
            <'a, Chain: HasAddressType + HasAmountType> EventualAmountTimeoutError<'a, Chain>,
            <'a, Chain: HasTxHashType> TxNoResponseError<'a, Chain>,
            <'a, Chain: HasClientIdType<Counterparty>, Counterparty: HasHeightType>
                NoConsensusStateAtLessThanHeight<'a, Chain, Counterparty>,
            <'a, Chain: HasHeightType, Counterparty: HasOutgoingPacketType<Chain>>
                InvalidTimeoutReceipt<'a, Chain, Counterparty>,
        ]:
            DebugError,
        StarknetError: RaiseStarknetError,
        ProviderError: RaiseProviderError,
        AccountError<SignError>: RaiseAccountError,
    }
}
