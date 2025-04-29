use core::array::TryFromSliceError;
use core::convert::Infallible;
use core::num::{ParseIntError, TryFromIntError};
use core::str::Utf8Error;
use std::string::FromUtf8Error;

use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use eyre::Report;
use hermes_cairo_encoding_components::impls::encode_mut::bool::DecodeBoolError;
use hermes_cairo_encoding_components::impls::encode_mut::end::NonEmptyBuffer;
use hermes_cairo_encoding_components::impls::encode_mut::felt::UnexpectedEndOfBuffer;
use hermes_cairo_encoding_components::impls::encode_mut::variant::VariantIndexOutOfBound;
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
use hermes_cosmos::error::handlers::{
    DebugError, DisplayError, HandleInfallible, ReportError, ReturnError,
};
use hermes_cosmos::error::types::Error;
use hermes_cosmos::protobuf_encoding_components::impls::{
    InvalidWireType, RequiredFieldTagNotFound, TypeUrlMismatchError, UnsupportedWireType,
};
use hermes_cosmos::runtime::types::error::TokioRuntimeError;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::{
    ConsensusStateNotFound, ContractAddressNotFound, RaiseAccountError, RaiseProviderError,
    RaiseStarknetError, UnexpectedTransactionTraceType,
};
use hermes_starknet_chain_components::types::UnknownEvent;
use ibc::core::channel::types::error::ChannelError;
use ibc::core::client::types::error::ClientError;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use ibc::primitives::TimestampError;
use prost::{DecodeError, EncodeError};
use starknet::accounts::{single_owner, AccountError};
use starknet::core::types::contract::{ComputeClassHashError, JsonError};
use starknet::core::types::{RevertedInvocation, StarknetError};
use starknet::providers::ProviderError;
use starknet::signers::local_wallet;
use url::ParseError;

pub struct HandleStarknetChainError;

pub type SignError = single_owner::SignError<local_wallet::SignError>;

delegate_components! {
    HandleStarknetChainError {
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
