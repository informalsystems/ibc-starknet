use alloc::vec;
use alloc::vec::Vec;

use ibc_core::connection::types::proto::v1::ConnectionEnd;
use starknet_core::types::Felt;

use crate::encoding::utils::{packed_bytes_to_felt, packed_str_to_felt, parse_client_id};

pub fn connection_end_to_felts(connection_end: &ConnectionEnd) -> Vec<Felt> {
    let mut felts = vec![];

    felts.push(Felt::from(connection_end.state));

    // 2. client_id
    let (client_type, seq) = parse_client_id(&connection_end.client_id);
    felts.push(client_type);
    felts.push(seq);

    // 3. counterparty.client_id
    let counterparty = connection_end.clone().counterparty.expect("missing counterparty in connection end");
    let (cp_client_type, cp_seq) = parse_client_id(&counterparty.client_id);
    felts.push(cp_client_type);
    felts.push(cp_seq);

    // 4. connection_id = None (as 4 zero felts)
    match counterparty.connection_id.as_str() {
        "" => {
            // No connection ID present — emit 4 felts of zero
            felts.extend([Felt::ZERO; 4]);
        }
        non_empty => {
            let conn_id_bytes = non_empty.as_bytes();
            let conn_id_felt = packed_bytes_to_felt(conn_id_bytes);
            felts.push(Felt::ZERO);
            felts.push(conn_id_felt); // actual content
            felts.push(Felt::from(conn_id_bytes.len() as u64)); // length
            felts.push(Felt::ZERO); // padding
        }
    }

    // 5. prefix = "ibc"
    let prefix_felt = packed_bytes_to_felt(&counterparty.prefix.expect("connection end is missing prefix").key_prefix);
    felts.push(prefix_felt);

    // 6. version.identifier = "1"
    let version = connection_end
        .versions
        .first()
        .expect("expected at least one version");

    let identifier_bytes = version.identifier.as_bytes();
    assert!(identifier_bytes.len() == 1, "identifier must be length 1");

    felts.push(Felt::from(3u8)); // marker
    felts.push(Felt::from(0u8)); // padding
    felts.push(Felt::from(identifier_bytes[0])); // ASCII of "1" or similar

    // 7. version.features
    for (i, feature) in version.features.iter().enumerate() {
        let tag = match i {
            0 => 1u8,
            1 => 13u8,
            _ => panic!("Unexpected feature count > 2"),
        };
        felts.push(Felt::from(tag));
        felts.push(Felt::from(0u8));
        felts.push(packed_str_to_felt(feature));
    }

    // 8. delay_period
    felts.push(Felt::from(15u8)); // struct marker

    let delay_secs = connection_end.delay_period;
    felts.push(Felt::from(delay_secs)); // seconds
    felts.push(Felt::ZERO);

    felts
}
