use cgp::prelude::*;

use crate::types::client_id::ClientId;
use crate::types::cosmos::height::Height;

#[derive(Debug, HasField, HasFields)]
pub struct CreateClientResponse {
    pub client_id: ClientId,
    pub height: Height,
}
