use hermes_prelude::*;

use crate::types::{ClientId, Height};

#[derive(Debug, HasField, HasFields)]
pub struct CreateClientResponse {
    pub client_id: ClientId,
    pub height: Height,
}
