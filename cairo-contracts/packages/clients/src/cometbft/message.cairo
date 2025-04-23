#[derive(Serde)]
pub enum ClientMessage {
    Update: Array<felt252>,
    Misbehaviour: Array<felt252>,
}
