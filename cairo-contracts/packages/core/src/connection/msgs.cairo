use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenInit {}

impl MsgConnOpenInitValidateBasic of ValidateBasic<MsgConnOpenInit> {
    fn validate_basic(self: @MsgConnOpenInit) {}
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenTry {}

impl MsgConnOpenTryValidateBasic of ValidateBasic<MsgConnOpenTry> {
    fn validate_basic(self: @MsgConnOpenTry) {}
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenAck {}

impl MsgConnOpenAckValidateBasic of ValidateBasic<MsgConnOpenAck> {
    fn validate_basic(self: @MsgConnOpenAck) {}
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct MsgConnOpenConfirm {}

impl MsgConnOpenConfirmValidateBasic of ValidateBasic<MsgConnOpenConfirm> {
    fn validate_basic(self: @MsgConnOpenConfirm) {}
}
