// Code generated by machine generator; DO NOT EDIT.

use crate::avp::{AVPError, AVPType, AVP};
use crate::packet::Packet;

pub const DIGEST_RESPONSE_TYPE: AVPType = 103;
pub fn delete_digest_response(packet: &mut Packet) {
    packet.delete(DIGEST_RESPONSE_TYPE);
}
pub fn add_digest_response(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_RESPONSE_TYPE, value));
}
pub fn lookup_digest_response(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_RESPONSE_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_response(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_RESPONSE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_REALM_TYPE: AVPType = 104;
pub fn delete_digest_realm(packet: &mut Packet) {
    packet.delete(DIGEST_REALM_TYPE);
}
pub fn add_digest_realm(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_REALM_TYPE, value));
}
pub fn lookup_digest_realm(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_REALM_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_realm(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_REALM_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_NONCE_TYPE: AVPType = 105;
pub fn delete_digest_nonce(packet: &mut Packet) {
    packet.delete(DIGEST_NONCE_TYPE);
}
pub fn add_digest_nonce(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_NONCE_TYPE, value));
}
pub fn lookup_digest_nonce(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_NONCE_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_nonce(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_NONCE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_RESPONSE_AUTH_TYPE: AVPType = 106;
pub fn delete_digest_response_auth(packet: &mut Packet) {
    packet.delete(DIGEST_RESPONSE_AUTH_TYPE);
}
pub fn add_digest_response_auth(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_RESPONSE_AUTH_TYPE, value));
}
pub fn lookup_digest_response_auth(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_RESPONSE_AUTH_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_response_auth(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_RESPONSE_AUTH_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_NEXTNONCE_TYPE: AVPType = 107;
pub fn delete_digest_nextnonce(packet: &mut Packet) {
    packet.delete(DIGEST_NEXTNONCE_TYPE);
}
pub fn add_digest_nextnonce(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_NEXTNONCE_TYPE, value));
}
pub fn lookup_digest_nextnonce(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_NEXTNONCE_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_nextnonce(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_NEXTNONCE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_METHOD_TYPE: AVPType = 108;
pub fn delete_digest_method(packet: &mut Packet) {
    packet.delete(DIGEST_METHOD_TYPE);
}
pub fn add_digest_method(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_METHOD_TYPE, value));
}
pub fn lookup_digest_method(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_METHOD_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_method(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_METHOD_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_URI_TYPE: AVPType = 109;
pub fn delete_digest_uri(packet: &mut Packet) {
    packet.delete(DIGEST_URI_TYPE);
}
pub fn add_digest_uri(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_URI_TYPE, value));
}
pub fn lookup_digest_uri(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_URI_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_uri(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_URI_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_QOP_TYPE: AVPType = 110;
pub fn delete_digest_qop(packet: &mut Packet) {
    packet.delete(DIGEST_QOP_TYPE);
}
pub fn add_digest_qop(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_QOP_TYPE, value));
}
pub fn lookup_digest_qop(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_QOP_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_qop(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_QOP_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_ALGORITHM_TYPE: AVPType = 111;
pub fn delete_digest_algorithm(packet: &mut Packet) {
    packet.delete(DIGEST_ALGORITHM_TYPE);
}
pub fn add_digest_algorithm(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_ALGORITHM_TYPE, value));
}
pub fn lookup_digest_algorithm(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_ALGORITHM_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_algorithm(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_ALGORITHM_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_ENTITY_BODY_HASH_TYPE: AVPType = 112;
pub fn delete_digest_entity_body_hash(packet: &mut Packet) {
    packet.delete(DIGEST_ENTITY_BODY_HASH_TYPE);
}
pub fn add_digest_entity_body_hash(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_ENTITY_BODY_HASH_TYPE, value));
}
pub fn lookup_digest_entity_body_hash(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_ENTITY_BODY_HASH_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_entity_body_hash(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_ENTITY_BODY_HASH_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_C_NONCE_TYPE: AVPType = 113;
pub fn delete_digest_c_nonce(packet: &mut Packet) {
    packet.delete(DIGEST_C_NONCE_TYPE);
}
pub fn add_digest_c_nonce(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_C_NONCE_TYPE, value));
}
pub fn lookup_digest_c_nonce(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_C_NONCE_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_c_nonce(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_C_NONCE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_NONCE_COUNT_TYPE: AVPType = 114;
pub fn delete_digest_nonce_count(packet: &mut Packet) {
    packet.delete(DIGEST_NONCE_COUNT_TYPE);
}
pub fn add_digest_nonce_count(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_NONCE_COUNT_TYPE, value));
}
pub fn lookup_digest_nonce_count(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_NONCE_COUNT_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_nonce_count(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_NONCE_COUNT_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_USERNAME_TYPE: AVPType = 115;
pub fn delete_digest_username(packet: &mut Packet) {
    packet.delete(DIGEST_USERNAME_TYPE);
}
pub fn add_digest_username(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_USERNAME_TYPE, value));
}
pub fn lookup_digest_username(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_USERNAME_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_username(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_USERNAME_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_OPAQUE_TYPE: AVPType = 116;
pub fn delete_digest_opaque(packet: &mut Packet) {
    packet.delete(DIGEST_OPAQUE_TYPE);
}
pub fn add_digest_opaque(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_OPAQUE_TYPE, value));
}
pub fn lookup_digest_opaque(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_OPAQUE_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_opaque(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_OPAQUE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_AUTH_PARAM_TYPE: AVPType = 117;
pub fn delete_digest_auth_param(packet: &mut Packet) {
    packet.delete(DIGEST_AUTH_PARAM_TYPE);
}
pub fn add_digest_auth_param(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_AUTH_PARAM_TYPE, value));
}
pub fn lookup_digest_auth_param(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_AUTH_PARAM_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_auth_param(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_AUTH_PARAM_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_AKA_AUTS_TYPE: AVPType = 118;
pub fn delete_digest_aka_auts(packet: &mut Packet) {
    packet.delete(DIGEST_AKA_AUTS_TYPE);
}
pub fn add_digest_aka_auts(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_AKA_AUTS_TYPE, value));
}
pub fn lookup_digest_aka_auts(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DIGEST_AKA_AUTS_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_digest_aka_auts(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_AKA_AUTS_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_DOMAIN_TYPE: AVPType = 119;
pub fn delete_digest_domain(packet: &mut Packet) {
    packet.delete(DIGEST_DOMAIN_TYPE);
}
pub fn add_digest_domain(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_DOMAIN_TYPE, value));
}
pub fn lookup_digest_domain(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_DOMAIN_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_domain(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_DOMAIN_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_STALE_TYPE: AVPType = 120;
pub fn delete_digest_stale(packet: &mut Packet) {
    packet.delete(DIGEST_STALE_TYPE);
}
pub fn add_digest_stale(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_STALE_TYPE, value));
}
pub fn lookup_digest_stale(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_STALE_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_stale(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_STALE_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const DIGEST_HA1_TYPE: AVPType = 121;
pub fn delete_digest_ha1(packet: &mut Packet) {
    packet.delete(DIGEST_HA1_TYPE);
}
pub fn add_digest_ha1(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DIGEST_HA1_TYPE, value));
}
pub fn lookup_digest_ha1(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(DIGEST_HA1_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_digest_ha1(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DIGEST_HA1_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const SIP_AOR_TYPE: AVPType = 122;
pub fn delete_sip_aor(packet: &mut Packet) {
    packet.delete(SIP_AOR_TYPE);
}
pub fn add_sip_aor(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(SIP_AOR_TYPE, value));
}
pub fn lookup_sip_aor(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet.lookup(SIP_AOR_TYPE).map(|v| v.encode_string())
}
pub fn lookup_all_sip_aor(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(SIP_AOR_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}