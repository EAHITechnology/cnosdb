use std::time::{SystemTime, UNIX_EPOCH};

use bytes::BufMut;
use trace::info;

use models::FieldId;

use crate::byte_utils;

use super::*;

pub fn encode_inverted_max_index_key(tab: &str, tag_key: &[u8]) -> Vec<u8> {
    tab.as_bytes()
        .iter()
        .chain(".".as_bytes())
        .chain(tag_key)
        .chain(">".as_bytes())
        .cloned()
        .collect()
}

pub fn encode_inverted_min_index_key(tab: &str, tag_key: &[u8]) -> Vec<u8> {
    tab.as_bytes()
        .iter()
        .chain(".".as_bytes())
        .chain(tag_key)
        .chain("<".as_bytes())
        .cloned()
        .collect()
}

pub fn encode_inverted_index_key(tab: &str, tag_key: &[u8], tag_val: &[u8]) -> Vec<u8> {
    tab.as_bytes()
        .iter()
        .chain(".".as_bytes())
        .chain(tag_key)
        .chain("=".as_bytes())
        .chain(tag_val)
        .cloned()
        .collect()
}

pub fn decode_series_id_list(data: &[u8]) -> IndexResult<Vec<u64>> {
    if data.len() % 8 != 0 {
        return Err(IndexError::DecodeSeriesIDList);
    }

    let count = data.len() / 8;
    let mut list: Vec<u64> = Vec::with_capacity(count);
    for i in 0..count {
        let id = byte_utils::decode_be_u64(&data[i * 8..]);
        list.push(id);
    }

    Ok(list)
}

pub fn encode_series_id_list(list: &[u64]) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(list.len() * 8);
    for i in list {
        data.put_u64(*i);
    }

    data
}
