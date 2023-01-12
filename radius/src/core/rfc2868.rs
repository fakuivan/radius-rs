// Code generated by machine generator; DO NOT EDIT.

//! Utility for rfc2868 packet.
//!
//! This module handles the packet according to the following definition:
//! ```text
//! //! # -*- text -*-
//! # Copyright (C) 2020 The FreeRADIUS Server project and contributors
//! # This work is licensed under CC-BY version 4.0 https://creativecommons.org/licenses/by/4.0
//! # Version $Id$
//! #
//! #    Attributes and values defined in RFC 2868.
//! #    http://www.ietf.org/rfc/rfc2868.txt
//! #
//! #    $Id$
//! #
//! ATTRIBUTE    Tunnel-Type                64    integer    has_tag
//! ATTRIBUTE    Tunnel-Medium-Type            65    integer    has_tag
//! ATTRIBUTE    Tunnel-Client-Endpoint            66    string    has_tag
//! ATTRIBUTE    Tunnel-Server-Endpoint            67    string    has_tag
//!
//! ATTRIBUTE    Tunnel-Password                69    string    has_tag,encrypt=2
//!
//! ATTRIBUTE    Tunnel-Private-Group-Id            81    string    has_tag
//! ATTRIBUTE    Tunnel-Assignment-Id            82    string    has_tag
//! ATTRIBUTE    Tunnel-Preference            83    integer    has_tag
//!
//! ATTRIBUTE    Tunnel-Client-Auth-Id            90    string    has_tag
//! ATTRIBUTE    Tunnel-Server-Auth-Id            91    string    has_tag
//!
//! #    Tunnel Type
//!
//! VALUE    Tunnel-Type            PPTP            1
//! VALUE    Tunnel-Type            L2F            2
//! VALUE    Tunnel-Type            L2TP            3
//! VALUE    Tunnel-Type            ATMP            4
//! VALUE    Tunnel-Type            VTP            5
//! VALUE    Tunnel-Type            AH            6
//! VALUE    Tunnel-Type            IP            7
//! VALUE    Tunnel-Type            MIN-IP            8
//! VALUE    Tunnel-Type            ESP            9
//! VALUE    Tunnel-Type            GRE            10
//! VALUE    Tunnel-Type            DVS            11
//! VALUE    Tunnel-Type            IP-in-IP        12
//!
//! #    Tunnel Medium Type
//!
//! VALUE    Tunnel-Medium-Type        IP            1
//! VALUE    Tunnel-Medium-Type        IPv4            1
//! VALUE    Tunnel-Medium-Type        IPv6            2
//! VALUE    Tunnel-Medium-Type        NSAP            3
//! VALUE    Tunnel-Medium-Type        HDLC            4
//! VALUE    Tunnel-Medium-Type        BBN-1822        5
//! VALUE    Tunnel-Medium-Type        IEEE-802        6
//! VALUE    Tunnel-Medium-Type        E.163            7
//! VALUE    Tunnel-Medium-Type        E.164            8
//! VALUE    Tunnel-Medium-Type        F.69            9
//! VALUE    Tunnel-Medium-Type        X.121            10
//! VALUE    Tunnel-Medium-Type        IPX            11
//! VALUE    Tunnel-Medium-Type        Appletalk        12
//! VALUE    Tunnel-Medium-Type        DecNet-IV        13
//! VALUE    Tunnel-Medium-Type        Banyan-Vines        14
//! VALUE    Tunnel-Medium-Type        E.164-NSAP        15
//! ```

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::avp as avp;
use crate::core::packet::Packet;
use crate::core::tag::Tag;

pub const TUNNEL_TYPE_TYPE: AVPType = 64;
/// Delete all of `tunnel_type` values from a packet.
pub fn delete_tunnel_type(packet: &mut Packet) {
    packet.delete(TUNNEL_TYPE_TYPE);
}
/// Add `tunnel_type` tagged value-defined integer value to a packet.
pub fn add_tunnel_type(packet: &mut Packet, tag: Option<&Tag>, value: TunnelType) {
    packet.add(avp::avp_from_integral!(u32, 3)(
        TUNNEL_TYPE_TYPE, Some(avp::to_unused_tag(tag)), value
    ));
}
/// Lookup a `tunnel_type` tagged value-defined integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_type`, it returns `None`.
pub fn lookup_tunnel_type(packet: &Packet) -> Option<Result<(TunnelType, Tag), AVPError>> {
    packet.lookup(TUNNEL_TYPE_TYPE).map(|v| {
        let (v, t) = v.encode_tagged_u32()?;
        Ok((v as TunnelType, t))
    })
}
/// Lookup all of the `tunnel_type` tagged value-defined integer value from a packet.
pub fn lookup_all_tunnel_type(packet: &Packet) -> Result<Vec<(TunnelType, Tag)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_TYPE_TYPE) {
        let (v, t) = avp.encode_tagged_u32()?;
        vec.push((v as TunnelType, t))
    }
    Ok(vec)
}

pub const TUNNEL_MEDIUM_TYPE_TYPE: AVPType = 65;
/// Delete all of `tunnel_medium_type` values from a packet.
pub fn delete_tunnel_medium_type(packet: &mut Packet) {
    packet.delete(TUNNEL_MEDIUM_TYPE_TYPE);
}
/// Add `tunnel_medium_type` tagged value-defined integer value to a packet.
pub fn add_tunnel_medium_type(packet: &mut Packet, tag: Option<&Tag>, value: TunnelMediumType) {
    packet.add(avp::avp_from_integral!(u32, 3)(
        TUNNEL_MEDIUM_TYPE_TYPE, Some(avp::to_unused_tag(tag)), value as u32
    ));
}
/// Lookup a `tunnel_medium_type` tagged value-defined integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_medium_type`, it returns `None`.
pub fn lookup_tunnel_medium_type(
    packet: &Packet,
) -> Option<Result<(TunnelMediumType, Tag), AVPError>> {
    packet.lookup(TUNNEL_MEDIUM_TYPE_TYPE).map(|v| {
        let (v, t) = v.encode_tagged_u32()?;
        Ok((v as TunnelMediumType, t))
    })
}
/// Lookup all of the `tunnel_medium_type` tagged value-defined integer value from a packet.
pub fn lookup_all_tunnel_medium_type(
    packet: &Packet,
) -> Result<Vec<(TunnelMediumType, Tag)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_MEDIUM_TYPE_TYPE) {
        let (v, t) = avp.encode_tagged_u32()?;
        vec.push((v as TunnelMediumType, t))
    }
    Ok(vec)
}

pub const TUNNEL_CLIENT_ENDPOINT_TYPE: AVPType = 66;
/// Delete all of `tunnel_client_endpoint` values from a packet.
pub fn delete_tunnel_client_endpoint(packet: &mut Packet) {
    packet.delete(TUNNEL_CLIENT_ENDPOINT_TYPE);
}
/// Add `tunnel_client_endpoint` tagged string value to a packet.
pub fn add_tunnel_client_endpoint(packet: &mut Packet, tag: Option<&Tag>, value: &str) {
    packet.add(AVP::from_tagged_string(
        TUNNEL_CLIENT_ENDPOINT_TYPE,
        tag,
        value,
    ));
}
/// Lookup a `tunnel_client_endpoint` tagged string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_client_endpoint`, it returns `None`.
pub fn lookup_tunnel_client_endpoint(
    packet: &Packet,
) -> Option<Result<(String, Option<Tag>), AVPError>> {
    packet
        .lookup(TUNNEL_CLIENT_ENDPOINT_TYPE)
        .map(|v| v.encode_tagged_string())
}
/// Lookup all of the `tunnel_client_endpoint` tagged string value from a packet.
pub fn lookup_all_tunnel_client_endpoint(
    packet: &Packet,
) -> Result<Vec<(String, Option<Tag>)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_CLIENT_ENDPOINT_TYPE) {
        vec.push(avp.encode_tagged_string()?)
    }
    Ok(vec)
}

pub const TUNNEL_SERVER_ENDPOINT_TYPE: AVPType = 67;
/// Delete all of `tunnel_server_endpoint` values from a packet.
pub fn delete_tunnel_server_endpoint(packet: &mut Packet) {
    packet.delete(TUNNEL_SERVER_ENDPOINT_TYPE);
}
/// Add `tunnel_server_endpoint` tagged string value to a packet.
pub fn add_tunnel_server_endpoint(packet: &mut Packet, tag: Option<&Tag>, value: &str) {
    packet.add(AVP::from_tagged_string(
        TUNNEL_SERVER_ENDPOINT_TYPE,
        tag,
        value,
    ));
}
/// Lookup a `tunnel_server_endpoint` tagged string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_server_endpoint`, it returns `None`.
pub fn lookup_tunnel_server_endpoint(
    packet: &Packet,
) -> Option<Result<(String, Option<Tag>), AVPError>> {
    packet
        .lookup(TUNNEL_SERVER_ENDPOINT_TYPE)
        .map(|v| v.encode_tagged_string())
}
/// Lookup all of the `tunnel_server_endpoint` tagged string value from a packet.
pub fn lookup_all_tunnel_server_endpoint(
    packet: &Packet,
) -> Result<Vec<(String, Option<Tag>)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_SERVER_ENDPOINT_TYPE) {
        vec.push(avp.encode_tagged_string()?)
    }
    Ok(vec)
}

pub const TUNNEL_PASSWORD_TYPE: AVPType = 69;
/// Delete all of `tunnel_password` values from a packet.
pub fn delete_tunnel_password(packet: &mut Packet) {
    packet.delete(TUNNEL_PASSWORD_TYPE);
}
/// Add `tunnel_password` tunnel-password value to a packet.
pub fn add_tunnel_password(
    packet: &mut Packet,
    tag: Option<&Tag>,
    value: &[u8],
) -> Result<(), AVPError> {
    packet.add(AVP::from_tunnel_password(
        TUNNEL_PASSWORD_TYPE,
        tag,
        value,
        packet.get_secret(),
        packet.get_authenticator(),
    )?);
    Ok(())
}
/// Lookup a `tunnel_password` tunnel-password value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_password`, it returns `None`.
pub fn lookup_tunnel_password(packet: &Packet) -> Option<Result<(Vec<u8>, Tag), AVPError>> {
    packet
        .lookup(TUNNEL_PASSWORD_TYPE)
        .map(|v| v.encode_tunnel_password(packet.get_secret(), packet.get_authenticator()))
}
/// Lookup all of the `tunnel_password` tunnel-password value from a packet.
pub fn lookup_all_tunnel_password(packet: &Packet) -> Result<Vec<(Vec<u8>, Tag)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_PASSWORD_TYPE) {
        vec.push(avp.encode_tunnel_password(packet.get_secret(), packet.get_authenticator())?)
    }
    Ok(vec)
}

pub const TUNNEL_PRIVATE_GROUP_ID_TYPE: AVPType = 81;
/// Delete all of `tunnel_private_group_id` values from a packet.
pub fn delete_tunnel_private_group_id(packet: &mut Packet) {
    packet.delete(TUNNEL_PRIVATE_GROUP_ID_TYPE);
}
/// Add `tunnel_private_group_id` tagged string value to a packet.
pub fn add_tunnel_private_group_id(packet: &mut Packet, tag: Option<&Tag>, value: &str) {
    packet.add(AVP::from_tagged_string(
        TUNNEL_PRIVATE_GROUP_ID_TYPE,
        tag,
        value,
    ));
}
/// Lookup a `tunnel_private_group_id` tagged string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_private_group_id`, it returns `None`.
pub fn lookup_tunnel_private_group_id(
    packet: &Packet,
) -> Option<Result<(String, Option<Tag>), AVPError>> {
    packet
        .lookup(TUNNEL_PRIVATE_GROUP_ID_TYPE)
        .map(|v| v.encode_tagged_string())
}
/// Lookup all of the `tunnel_private_group_id` tagged string value from a packet.
pub fn lookup_all_tunnel_private_group_id(
    packet: &Packet,
) -> Result<Vec<(String, Option<Tag>)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_PRIVATE_GROUP_ID_TYPE) {
        vec.push(avp.encode_tagged_string()?)
    }
    Ok(vec)
}

pub const TUNNEL_ASSIGNMENT_ID_TYPE: AVPType = 82;
/// Delete all of `tunnel_assignment_id` values from a packet.
pub fn delete_tunnel_assignment_id(packet: &mut Packet) {
    packet.delete(TUNNEL_ASSIGNMENT_ID_TYPE);
}
/// Add `tunnel_assignment_id` tagged string value to a packet.
pub fn add_tunnel_assignment_id(packet: &mut Packet, tag: Option<&Tag>, value: &str) {
    packet.add(AVP::from_tagged_string(
        TUNNEL_ASSIGNMENT_ID_TYPE,
        tag,
        value,
    ));
}
/// Lookup a `tunnel_assignment_id` tagged string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_assignment_id`, it returns `None`.
pub fn lookup_tunnel_assignment_id(
    packet: &Packet,
) -> Option<Result<(String, Option<Tag>), AVPError>> {
    packet
        .lookup(TUNNEL_ASSIGNMENT_ID_TYPE)
        .map(|v| v.encode_tagged_string())
}
/// Lookup all of the `tunnel_assignment_id` tagged string value from a packet.
pub fn lookup_all_tunnel_assignment_id(
    packet: &Packet,
) -> Result<Vec<(String, Option<Tag>)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_ASSIGNMENT_ID_TYPE) {
        vec.push(avp.encode_tagged_string()?)
    }
    Ok(vec)
}

pub const TUNNEL_PREFERENCE_TYPE: AVPType = 83;
/// Delete all of `tunnel_preference` values from a packet.
pub fn delete_tunnel_preference(packet: &mut Packet) {
    packet.delete(TUNNEL_PREFERENCE_TYPE);
}
/// Add `tunnel_preference` tagged integer value to a packet.
pub fn add_tunnel_preference(packet: &mut Packet, tag: Option<&Tag>, value: u32) {
    packet.add(AVP::from_tagged_u32(TUNNEL_PREFERENCE_TYPE, tag, value));
}
/// Lookup a `tunnel_preference` tagged integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_preference`, it returns `None`.
pub fn lookup_tunnel_preference(packet: &Packet) -> Option<Result<(u32, Tag), AVPError>> {
    packet
        .lookup(TUNNEL_PREFERENCE_TYPE)
        .map(|v| v.encode_tagged_u32())
}
/// Lookup all of the `tunnel_preference` tagged integer value from a packet.
pub fn lookup_all_tunnel_preference(packet: &Packet) -> Result<Vec<(u32, Tag)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_PREFERENCE_TYPE) {
        vec.push(avp.encode_tagged_u32()?)
    }
    Ok(vec)
}

pub const TUNNEL_CLIENT_AUTH_ID_TYPE: AVPType = 90;
/// Delete all of `tunnel_client_auth_id` values from a packet.
pub fn delete_tunnel_client_auth_id(packet: &mut Packet) {
    packet.delete(TUNNEL_CLIENT_AUTH_ID_TYPE);
}
/// Add `tunnel_client_auth_id` tagged string value to a packet.
pub fn add_tunnel_client_auth_id(packet: &mut Packet, tag: Option<&Tag>, value: &str) {
    packet.add(AVP::from_tagged_string(
        TUNNEL_CLIENT_AUTH_ID_TYPE,
        tag,
        value,
    ));
}
/// Lookup a `tunnel_client_auth_id` tagged string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_client_auth_id`, it returns `None`.
pub fn lookup_tunnel_client_auth_id(
    packet: &Packet,
) -> Option<Result<(String, Option<Tag>), AVPError>> {
    packet
        .lookup(TUNNEL_CLIENT_AUTH_ID_TYPE)
        .map(|v| v.encode_tagged_string())
}
/// Lookup all of the `tunnel_client_auth_id` tagged string value from a packet.
pub fn lookup_all_tunnel_client_auth_id(
    packet: &Packet,
) -> Result<Vec<(String, Option<Tag>)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_CLIENT_AUTH_ID_TYPE) {
        vec.push(avp.encode_tagged_string()?)
    }
    Ok(vec)
}

pub const TUNNEL_SERVER_AUTH_ID_TYPE: AVPType = 91;
/// Delete all of `tunnel_server_auth_id` values from a packet.
pub fn delete_tunnel_server_auth_id(packet: &mut Packet) {
    packet.delete(TUNNEL_SERVER_AUTH_ID_TYPE);
}
/// Add `tunnel_server_auth_id` tagged string value to a packet.
pub fn add_tunnel_server_auth_id(packet: &mut Packet, tag: Option<&Tag>, value: &str) {
    packet.add(AVP::from_tagged_string(
        TUNNEL_SERVER_AUTH_ID_TYPE,
        tag,
        value,
    ));
}
/// Lookup a `tunnel_server_auth_id` tagged string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `tunnel_server_auth_id`, it returns `None`.
pub fn lookup_tunnel_server_auth_id(
    packet: &Packet,
) -> Option<Result<(String, Option<Tag>), AVPError>> {
    packet
        .lookup(TUNNEL_SERVER_AUTH_ID_TYPE)
        .map(|v| v.encode_tagged_string())
}
/// Lookup all of the `tunnel_server_auth_id` tagged string value from a packet.
pub fn lookup_all_tunnel_server_auth_id(
    packet: &Packet,
) -> Result<Vec<(String, Option<Tag>)>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(TUNNEL_SERVER_AUTH_ID_TYPE) {
        vec.push(avp.encode_tagged_string()?)
    }
    Ok(vec)
}

pub type TunnelMediumType = u32;
pub const TUNNEL_MEDIUM_TYPE_IP: TunnelMediumType = 1;
pub const TUNNEL_MEDIUM_TYPE_I_PV_4: TunnelMediumType = 1;
pub const TUNNEL_MEDIUM_TYPE_I_PV_6: TunnelMediumType = 2;
pub const TUNNEL_MEDIUM_TYPE_NSAP: TunnelMediumType = 3;
pub const TUNNEL_MEDIUM_TYPE_HDLC: TunnelMediumType = 4;
pub const TUNNEL_MEDIUM_TYPE_BBN_1822: TunnelMediumType = 5;
pub const TUNNEL_MEDIUM_TYPE_IEEE_802: TunnelMediumType = 6;
pub const TUNNEL_MEDIUM_TYPE_E_163: TunnelMediumType = 7;
pub const TUNNEL_MEDIUM_TYPE_E_164: TunnelMediumType = 8;
pub const TUNNEL_MEDIUM_TYPE_F_69: TunnelMediumType = 9;
pub const TUNNEL_MEDIUM_TYPE_X_121: TunnelMediumType = 10;
pub const TUNNEL_MEDIUM_TYPE_IPX: TunnelMediumType = 11;
pub const TUNNEL_MEDIUM_TYPE_APPLETALK: TunnelMediumType = 12;
pub const TUNNEL_MEDIUM_TYPE_DEC_NET_IV: TunnelMediumType = 13;
pub const TUNNEL_MEDIUM_TYPE_BANYAN_VINES: TunnelMediumType = 14;
pub const TUNNEL_MEDIUM_TYPE_E_164_NSAP: TunnelMediumType = 15;

pub type TunnelType = u32;
pub const TUNNEL_TYPE_PPTP: TunnelType = 1;
pub const TUNNEL_TYPE_L2F: TunnelType = 2;
pub const TUNNEL_TYPE_L2TP: TunnelType = 3;
pub const TUNNEL_TYPE_ATMP: TunnelType = 4;
pub const TUNNEL_TYPE_VTP: TunnelType = 5;
pub const TUNNEL_TYPE_AH: TunnelType = 6;
pub const TUNNEL_TYPE_IP: TunnelType = 7;
pub const TUNNEL_TYPE_MIN_IP: TunnelType = 8;
pub const TUNNEL_TYPE_ESP: TunnelType = 9;
pub const TUNNEL_TYPE_GRE: TunnelType = 10;
pub const TUNNEL_TYPE_DVS: TunnelType = 11;
pub const TUNNEL_TYPE_IP_IN_IP: TunnelType = 12;
