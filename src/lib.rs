use std::{
    sync::Arc,
    mem::size_of,
};

use byteorder::{
    ByteOrder,
    BigEndian,
};

pub type Key = Arc<[u8]>;
pub type Value = Arc<[u8]>;

#[derive(Debug, PartialEq)]
pub enum RepayStatus {
    Penalty,
    Reward,
    Front,
    Drop,
}

#[derive(Debug, PartialEq)]
pub enum LendMode {
    Block,
    Poll,
}

#[derive(Debug, PartialEq)]
pub enum AddMode {
    Head,
    Tail,
}

#[derive(Debug, PartialEq)]
pub enum GlobalReq {
    Ping,
    Count,
    Add { key: Key, value: Value, mode: AddMode, },
    Update(Key, Value),
    Lookup(Key),
    Remove(Key),
    Lend { timeout: u64, mode: LendMode, },
    Repay { lend_key: u64, key: Key, value: Value, status: RepayStatus, },
    Heartbeat { lend_key: u64, key: Key, timeout: u64, },
    Stats,
    Flush,
    Terminate,
}

#[derive(Debug, PartialEq)]
pub enum GlobalRep {
    Pong,
    Counted(usize),
    Added,
    Kept,
    Updated,
    NotFound,
    ValueFound(Value),
    ValueNotFound,
    Removed,
    NotRemoved,
    Lent { lend_key: u64, key: Key, value: Value, },
    QueueEmpty,
    Repaid,
    Heartbeaten,
    Skipped,
    StatsGot {
        ping: usize,
        count: usize,
        add: usize,
        update: usize,
        lookup: usize,
        remove: usize,
        lend: usize,
        repay: usize,
        heartbeat: usize,
        stats: usize,
    },
    Flushed,
    Terminated,
    Error(ProtoError),
}

#[derive(Debug, PartialEq)]
pub enum ProtoError {
    NotEnoughDataForGlobalReqTag { required: usize, given: usize, },
    InvalidGlobalReqTag(u8),
    NotEnoughDataForGlobalReqAddKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqAddKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqAddValueLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqAddValue { required: usize, given: usize, },
    NotEnoughDataForGlobalReqAddMode { required: usize, given: usize, },
    InvalidGlobalReqAddModeTag(u8),
    NotEnoughDataForGlobalReqLendTimeout { required: usize, given: usize, },
    NotEnoughDataForGlobalReqLendMode { required: usize, given: usize, },
    InvalidGlobalReqLendModeTag(u8),
    NotEnoughDataForGlobalReqRepayLendKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRepayKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRepayKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRepayValueLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRepayValue { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRepayRepayStatus { required: usize, given: usize, },
    InvalidGlobalReqRepayRepayStatusTag(u8),
    NotEnoughDataForGlobalRepTag { required: usize, given: usize, },
    InvalidGlobalRepTag(u8),
    NotEnoughDataForGlobalRepCountCount { required: usize, given: usize, },
    NotEnoughDataForGlobalRepLentLendKey { required: usize, given: usize, },
    NotEnoughDataForGlobalRepLentKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalRepLentKey { required: usize, given: usize, },
    NotEnoughDataForGlobalRepLentValueLen { required: usize, given: usize, },
    NotEnoughDataForGlobalRepLentValue { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsCount { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsPing { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsAdd { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsUpdate { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsLookup { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsRemove { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsLend { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsRepay { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsHeartbeat { required: usize, given: usize, },
    NotEnoughDataForGlobalRepStatsStats { required: usize, given: usize, },
    NotEnoughDataForProtoErrorTag { required: usize, given: usize, },
    InvalidProtoErrorTag(u8),
    NotEnoughDataForProtoErrorRequired { required: usize, given: usize, },
    NotEnoughDataForProtoErrorGiven { required: usize, given: usize, },
    NotEnoughDataForProtoErrorInvalidTag { required: usize, given: usize, },
    DbQueueOutOfSync(Key),
    NotEnoughDataForProtoErrorDbQueueOutOfSyncKeyLen { required: usize, given: usize, },
    NotEnoughDataForProtoErrorDbQueueOutOfSyncKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqUpdateKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqUpdateKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqUpdateValueLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqUpdateValue { required: usize, given: usize, },
    NotEnoughDataForGlobalReqHeartbeatLendKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqHeartbeatKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqHeartbeatKey { required: usize, given: usize, },
    NotEnoughDataForGlobalReqHeartbeatTimeout { required: usize, given: usize, },
    NotEnoughDataForGlobalReqLookupKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqLookupKey { required: usize, given: usize, },
    NotEnoughDataForGlobalRepValueFoundValueLen { required: usize, given: usize, },
    NotEnoughDataForGlobalRepValueFoundValue { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRemoveKeyLen { required: usize, given: usize, },
    NotEnoughDataForGlobalReqRemoveKey { required: usize, given: usize, },
}

impl std::fmt::Display for ProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ProtoError {}

macro_rules! try_get {
    ($data:ident, $ty:ty, $reader:ident, $err:ident) =>
        (if $data.len() < size_of::<$ty>() {
            return Err(ProtoError::$err { required: size_of::<$ty>(), given: $data.len(), })
        } else {
            (BigEndian::$reader($data), &$data[size_of::<$ty>() ..])
        })
}

macro_rules! put_adv {
    ($area:expr, $ty:ty, $writer:ident, $value:expr) => ({
        let area = $area;
        BigEndian::$writer(area, $value);
        &mut area[size_of::<$ty>() ..]
    })
}

trait U8Support {
    fn read_u8(buf: &[u8]) -> u8;
    fn write_u8(buf: &mut [u8], n: u8);
}

impl U8Support for BigEndian {
    fn read_u8(buf: &[u8]) -> u8 {
        buf[0]
    }

    fn write_u8(buf: &mut [u8], n: u8) {
        buf[0] = n;
    }
}

macro_rules! try_get_vec {
    ($buf:expr, $err_len:ident, $err_val:ident) => ({
        let buf = $buf;
        let (len, buf) = try_get!(buf, u32, read_u32, $err_len);
        let len = len as usize;
        if buf.len() < len {
            return Err(ProtoError::$err_val { required: len, given: buf.len(), })
        } else {
            (Arc::from(&buf[0 .. len]), &buf[len ..])
        }
    })
}

macro_rules! put_vec_adv {
    ($area:expr, $vec:ident) => ({
        let src = &*$vec;
        let dst = $area;
        let src_len_value = src.len() as u32;
        let area = put_adv!(dst, u32, write_u32, src_len_value);
        unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), area.as_mut_ptr(), src.len()); }
        &mut area[src.len() ..]
    })
}

impl GlobalReq {
    pub fn decode<'a>(data: &'a [u8]) -> Result<(GlobalReq, &'a [u8]), ProtoError> {
        match try_get!(data, u8, read_u8, NotEnoughDataForGlobalReqTag) {
            (1, buf) =>
                Ok((GlobalReq::Count, buf)),
            (2, buf) => {
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqAddKeyLen, NotEnoughDataForGlobalReqAddKey);
                let (value, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqAddValueLen, NotEnoughDataForGlobalReqAddValue);
                let (mode, buf) = match try_get!(buf, u8, read_u8, NotEnoughDataForGlobalReqAddMode) {
                    (1, buf) => (AddMode::Head, buf),
                    (2, buf) => (AddMode::Tail, buf),
                    (mode_tag, _) => return Err(ProtoError::InvalidGlobalReqAddModeTag(mode_tag)),
                };

                Ok((GlobalReq::Add { key: key, value: value, mode: mode, }, buf))
            },
            (3, buf) => {
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqUpdateKeyLen, NotEnoughDataForGlobalReqUpdateKey);
                let (value, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqUpdateValueLen, NotEnoughDataForGlobalReqUpdateValue);
                Ok((GlobalReq::Update(key, value), buf))
            },
            (4, buf) => {
                let (timeout, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalReqLendTimeout);
                let (mode, buf) = match try_get!(buf, u8, read_u8, NotEnoughDataForGlobalReqLendMode) {
                    (1, buf) => (LendMode::Block, buf),
                    (2, buf) => (LendMode::Poll, buf),
                    (mode_tag, _) => return Err(ProtoError::InvalidGlobalReqLendModeTag(mode_tag)),
                };
                Ok((GlobalReq::Lend { timeout: timeout, mode: mode, }, buf))
            },
            (5, buf) => {
                let (lend_key, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalReqRepayLendKey);
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqRepayKeyLen, NotEnoughDataForGlobalReqRepayKey);
                let (value, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqRepayValueLen, NotEnoughDataForGlobalReqRepayValue);
                let (status, buf) = match try_get!(buf, u8, read_u8, NotEnoughDataForGlobalReqRepayRepayStatus) {
                    (1, buf) => (RepayStatus::Penalty, buf),
                    (2, buf) => (RepayStatus::Reward, buf),
                    (3, buf) => (RepayStatus::Front, buf),
                    (4, buf) => (RepayStatus::Drop, buf),
                    (status_tag, _) => return Err(ProtoError::InvalidGlobalReqRepayRepayStatusTag(status_tag)),
                };
                Ok((GlobalReq::Repay { lend_key: lend_key, key: key, value: value, status: status, }, buf))
            },
            (6, buf) => {
                let (lend_key, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalReqHeartbeatLendKey);
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqHeartbeatKeyLen, NotEnoughDataForGlobalReqHeartbeatKey);
                let (timeout, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalReqHeartbeatTimeout);
                Ok((GlobalReq::Heartbeat { lend_key: lend_key, key: key, timeout: timeout, }, buf))
            },
            (7, buf) =>
                Ok((GlobalReq::Stats, buf)),
            (8, buf) =>
                Ok((GlobalReq::Terminate, buf)),
            (9, buf) => {
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqLookupKeyLen, NotEnoughDataForGlobalReqLookupKey);
                Ok((GlobalReq::Lookup(key), buf))
            },
            (10, buf) =>
                Ok((GlobalReq::Flush, buf)),
            (11, buf) =>
                Ok((GlobalReq::Ping, buf)),
            (12, buf) => {
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalReqRemoveKeyLen, NotEnoughDataForGlobalReqRemoveKey);
                Ok((GlobalReq::Remove(key), buf))
            },
            (tag, _) =>
                return Err(ProtoError::InvalidGlobalReqTag(tag)),
        }
    }

    pub fn encode_len(&self) -> usize {
        size_of::<u8>() + match self {
            &GlobalReq::Ping | &GlobalReq::Count | &GlobalReq::Stats | &GlobalReq::Terminate | &GlobalReq::Flush => 0,
            &GlobalReq::Add { key: ref k, value: ref v, .. } => size_of::<u32>() * 2 + k.len() + v.len() + size_of::<u8>(),
            &GlobalReq::Update(ref key, ref value) => size_of::<u32>() * 2 + key.len() + value.len(),
            &GlobalReq::Lend { .. } => size_of::<u64>() + size_of::<u8>(),
            &GlobalReq::Repay { key: ref rkey, value: ref rvalue, .. } =>
                size_of::<u64>() + size_of::<u32>() * 2 + rkey.len() + rvalue.len() + size_of::<u8>(),
            &GlobalReq::Heartbeat { key: ref k, .. } => size_of::<u64>() + size_of::<u32>() + k.len() + size_of::<u64>(),
            &GlobalReq::Lookup(ref key) => size_of::<u32>() + key.len(),
            &GlobalReq::Remove(ref key) => size_of::<u32>() + key.len(),
        }
    }

    pub fn encode<'b>(&self, area: &'b mut [u8]) -> &'b mut [u8] {
        match self {
            &GlobalReq::Count =>
                put_adv!(area, u8, write_u8, 1),
            &GlobalReq::Add { key: ref k, value: ref v, mode: ref m, } => {
                let area = put_adv!(area, u8, write_u8, 2);
                let area = put_vec_adv!(area, k);
                let area = put_vec_adv!(area, v);
                put_adv!(area, u8, write_u8, match m {
                    &AddMode::Head => 1,
                    &AddMode::Tail => 2,
                })
            },
            &GlobalReq::Update(ref key, ref value) => {
                let area = put_adv!(area, u8, write_u8, 3);
                let area = put_vec_adv!(area, key);
                let area = put_vec_adv!(area, value);
                area
            },
            &GlobalReq::Lend { timeout: t, mode: ref m, } => {
                let area = put_adv!(area, u8, write_u8, 4);
                let area = put_adv!(area, u64, write_u64, t);
                put_adv!(area, u8, write_u8, match m {
                    &LendMode::Block => 1,
                    &LendMode::Poll => 2,
                })
            },
            &GlobalReq::Repay { lend_key: rlend_key, key: ref rkey, value: ref rvalue, status: ref rstatus } => {
                let area = put_adv!(area, u8, write_u8, 5);
                let area = put_adv!(area, u64, write_u64, rlend_key);
                let area = put_vec_adv!(area, rkey);
                let area = put_vec_adv!(area, rvalue);
                put_adv!(area, u8, write_u8, match rstatus {
                    &RepayStatus::Penalty => 1,
                    &RepayStatus::Reward => 2,
                    &RepayStatus::Front => 3,
                    &RepayStatus::Drop => 4,
                })
            },
            &GlobalReq::Heartbeat { lend_key: l, key: ref k, timeout: t, } => {
                let area = put_adv!(area, u8, write_u8, 6);
                let area = put_adv!(area, u64, write_u64, l);
                let area = put_vec_adv!(area, k);
                put_adv!(area, u64, write_u64, t)
            },
            &GlobalReq::Stats =>
                put_adv!(area, u8, write_u8, 7),
            &GlobalReq::Terminate =>
                put_adv!(area, u8, write_u8, 8),
            &GlobalReq::Lookup(ref key) => {
                let area = put_adv!(area, u8, write_u8, 9);
                let area = put_vec_adv!(area, key);
                area
            },
            &GlobalReq::Flush =>
                put_adv!(area, u8, write_u8, 10),
            &GlobalReq::Ping =>
                put_adv!(area, u8, write_u8, 11),
            &GlobalReq::Remove(ref key) => {
                let area = put_adv!(area, u8, write_u8, 12);
                let area = put_vec_adv!(area, key);
                area
            },
        }
    }
}

impl GlobalRep {
    pub fn decode<'a>(data: &'a [u8]) -> Result<(GlobalRep, &'a [u8]), ProtoError> {
        match try_get!(data, u8, read_u8, NotEnoughDataForGlobalRepTag) {
            (1, buf) => {
                let (count, buf) = try_get!(buf, u32, read_u32, NotEnoughDataForGlobalRepCountCount);
                Ok((GlobalRep::Counted(count as usize), buf))
            },
            (2, buf) =>
                Ok((GlobalRep::Added, buf)),
            (3, buf) =>
                Ok((GlobalRep::Kept, buf)),
            (4, buf) =>
                Ok((GlobalRep::Updated, buf)),
            (5, buf) =>
                Ok((GlobalRep::NotFound, buf)),
            (6, buf) => {
                let (lend_key, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepLentLendKey);
                let (key, buf) = try_get_vec!(buf, NotEnoughDataForGlobalRepLentKeyLen, NotEnoughDataForGlobalRepLentKey);
                let (value, buf) = try_get_vec!(buf, NotEnoughDataForGlobalRepLentValueLen, NotEnoughDataForGlobalRepLentValue);
                Ok((GlobalRep::Lent { lend_key: lend_key, key: key, value: value, }, buf))
            },
            (7, buf) =>
                Ok((GlobalRep::Repaid, buf)),
            (8, buf) =>
                Ok((GlobalRep::Heartbeaten, buf)),
            (9, buf) =>
                Ok((GlobalRep::Skipped, buf)),
            (10, buf) => {
                let (stats_ping, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsPing);
                let (stats_count, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsCount);
                let (stats_add, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsAdd);
                let (stats_update, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsUpdate);
                let (stats_lookup, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsLookup);
                let (stats_remove, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsRemove);
                let (stats_lend, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsLend);
                let (stats_repay, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsRepay);
                let (stats_heartbeat, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsHeartbeat);
                let (stats_stats, buf) = try_get!(buf, u64, read_u64, NotEnoughDataForGlobalRepStatsStats);
                Ok((GlobalRep::StatsGot {
                    ping: stats_ping as usize,
                    count: stats_count as usize,
                    add: stats_add as usize,
                    update: stats_update as usize,
                    lookup: stats_lookup as usize,
                    remove: stats_remove as usize,
                    lend: stats_lend as usize,
                    repay: stats_repay as usize,
                    heartbeat: stats_heartbeat as usize,
                    stats: stats_stats as usize,
                }, buf))
            },
            (11, buf) => {
                let (err, buf) = ProtoError::decode(buf)?;
                Ok((GlobalRep::Error(err), buf))
            },
            (12, buf) =>
                Ok((GlobalRep::Terminated, buf)),
            (13, buf) => {
                let (value, buf) = try_get_vec!(buf, NotEnoughDataForGlobalRepValueFoundValueLen, NotEnoughDataForGlobalRepValueFoundValue);
                Ok((GlobalRep::ValueFound(value), buf))
            },
            (14, buf) =>
                Ok((GlobalRep::ValueNotFound, buf)),
            (15, buf) =>
                Ok((GlobalRep::Flushed, buf)),
            (16, buf) =>
                Ok((GlobalRep::QueueEmpty, buf)),
            (17, buf) =>
                Ok((GlobalRep::Pong, buf)),
            (18, buf) =>
                Ok((GlobalRep::Removed, buf)),
            (19, buf) =>
                Ok((GlobalRep::NotRemoved, buf)),
            (tag, _) =>
                return Err(ProtoError::InvalidGlobalRepTag(tag)),
        }
    }

    pub fn encode_len(&self) -> usize {
        size_of::<u8>() + match self {
            &GlobalRep::Counted(..) => size_of::<u32>(),
            &GlobalRep::Added |
            &GlobalRep::Kept |
            &GlobalRep::Updated |
            &GlobalRep::NotFound |
            &GlobalRep::Repaid |
            &GlobalRep::Heartbeaten |
            &GlobalRep::Skipped |
            &GlobalRep::Terminated |
            &GlobalRep::ValueNotFound |
            &GlobalRep::Flushed |
            &GlobalRep::Pong |
            &GlobalRep::Removed |
            &GlobalRep::NotRemoved |
            &GlobalRep::QueueEmpty => 0,
            &GlobalRep::Lent { key: ref rkey, value: ref rvalue, .. } => size_of::<u64>() + size_of::<u32>() * 2 + rkey.len() + rvalue.len(),
            &GlobalRep::StatsGot { .. } => size_of::<u64>() * 10,
            &GlobalRep::Error(ref err) => err.encode_len(),
            &GlobalRep::ValueFound(ref value) => size_of::<u32>() + value.len(),
        }
    }

    pub fn encode<'b>(&self, area: &'b mut [u8]) -> &'b mut [u8] {
        match self {
            &GlobalRep::Counted(count) => {
                let area = put_adv!(area, u8, write_u8, 1);
                put_adv!(area, u32, write_u32, count as u32)
            },
            &GlobalRep::Added =>
                put_adv!(area, u8, write_u8, 2),
            &GlobalRep::Kept =>
                put_adv!(area, u8, write_u8, 3),
            &GlobalRep::Updated =>
                put_adv!(area, u8, write_u8, 4),
            &GlobalRep::NotFound =>
                put_adv!(area, u8, write_u8, 5),
            &GlobalRep::Lent { lend_key: rlend_key, key: ref rkey, value: ref rvalue, } => {
                let area = put_adv!(area, u8, write_u8, 6);
                let area = put_adv!(area, u64, write_u64, rlend_key);
                let area = put_vec_adv!(area, rkey);
                let area = put_vec_adv!(area, rvalue);
                area
            },
            &GlobalRep::Repaid =>
                put_adv!(area, u8, write_u8, 7),
            &GlobalRep::Heartbeaten =>
                put_adv!(area, u8, write_u8, 8),
            &GlobalRep::Skipped =>
                put_adv!(area, u8, write_u8, 9),
            &GlobalRep::StatsGot {
                ping: stats_ping,
                count: stats_count,
                add: stats_add,
                update: stats_update,
                lookup: stats_lookup,
                remove: stats_remove,
                lend: stats_lend,
                repay: stats_repay,
                heartbeat: stats_heartbeat,
                stats: stats_stats,
            } => {
                let area = put_adv!(area, u8, write_u8, 10);
                let area = put_adv!(area, u64, write_u64, stats_ping as u64);
                let area = put_adv!(area, u64, write_u64, stats_count as u64);
                let area = put_adv!(area, u64, write_u64, stats_add as u64);
                let area = put_adv!(area, u64, write_u64, stats_update as u64);
                let area = put_adv!(area, u64, write_u64, stats_lookup as u64);
                let area = put_adv!(area, u64, write_u64, stats_remove as u64);
                let area = put_adv!(area, u64, write_u64, stats_lend as u64);
                let area = put_adv!(area, u64, write_u64, stats_repay as u64);
                let area = put_adv!(area, u64, write_u64, stats_heartbeat as u64);
                let area = put_adv!(area, u64, write_u64, stats_stats as u64);
                area
            },
            &GlobalRep::Error(ref err) => {
                let area = put_adv!(area, u8, write_u8, 11);
                err.encode(area)
            },
            &GlobalRep::Terminated =>
                put_adv!(area, u8, write_u8, 12),
            &GlobalRep::ValueFound(ref value) => {
                let area = put_adv!(area, u8, write_u8, 13);
                let area = put_vec_adv!(area, value);
                area
            },
            &GlobalRep::ValueNotFound =>
                put_adv!(area, u8, write_u8, 14),
            &GlobalRep::Flushed =>
                put_adv!(area, u8, write_u8, 15),
            &GlobalRep::QueueEmpty =>
                put_adv!(area, u8, write_u8, 16),
            &GlobalRep::Pong =>
                put_adv!(area, u8, write_u8, 17),
            &GlobalRep::Removed =>
                put_adv!(area, u8, write_u8, 18),
            &GlobalRep::NotRemoved =>
                put_adv!(area, u8, write_u8, 19),
        }
    }
}

macro_rules! decode_not_enough {
    ($buf:ident, $pe_type:ident) => ({
        let (required, given_buf) = try_get!($buf, u32, read_u32, NotEnoughDataForProtoErrorRequired);
        let (given, rest) = try_get!(given_buf, u32, read_u32, NotEnoughDataForProtoErrorGiven);
        Ok((ProtoError::$pe_type { required: required as usize, given: given as usize, }, rest))
    })
}

macro_rules! encode_not_enough {
    ($area:ident, $tag:expr, $required:expr, $given: expr) => ({
        let area = put_adv!($area, u8, write_u8, $tag);
        let area = put_adv!(area, u32, write_u32, $required as u32);
        put_adv!(area, u32, write_u32, $given as u32)
    })
}

macro_rules! decode_tag {
    ($buf:ident, $pe_type:ident) => ({
        let (tag, rest) = try_get!($buf, u8, read_u8, NotEnoughDataForProtoErrorInvalidTag);
        Ok((ProtoError::$pe_type(tag), rest))
    })
}

macro_rules! encode_tag {
    ($area:ident, $tag:expr, $invalid_tag:expr) => ({
        let area = put_adv!($area, u8, write_u8, $tag);
        put_adv!(area, u8, write_u8, $invalid_tag)
    })
}

impl ProtoError {
    pub fn decode<'a>(data: &'a [u8]) -> Result<(ProtoError, &'a [u8]), ProtoError> {
        match try_get!(data, u8, read_u8, NotEnoughDataForProtoErrorTag) {
            (1, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqTag),
            (2, buf) => decode_tag!(buf, InvalidGlobalReqTag),
            (3, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqAddKeyLen),
            (4, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqAddKey),
            (5, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqAddValueLen),
            (6, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqAddValue),
            (7, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqLendTimeout),
            (8, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRepayLendKey),
            (9, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRepayKeyLen),
            (10, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRepayKey),
            (11, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRepayValueLen),
            (12, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRepayValue),
            (13, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRepayRepayStatus),
            (14, buf) => decode_tag!(buf, InvalidGlobalReqRepayRepayStatusTag),
            (15, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepTag),
            (16, buf) => decode_tag!(buf, InvalidGlobalRepTag),
            (17, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepCountCount),
            (18, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepLentLendKey),
            (19, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepLentKeyLen),
            (20, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepLentKey),
            (21, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepLentValueLen),
            (22, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepLentValue),
            (23, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsCount),
            (24, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsAdd),
            (25, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsUpdate),
            (26, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsLookup),
            (27, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsLend),
            (28, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsRepay),
            (29, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsHeartbeat),
            (30, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsStats),
            (31, buf) => decode_not_enough!(buf, NotEnoughDataForProtoErrorTag),
            (32, buf) => decode_tag!(buf, InvalidProtoErrorTag),
            (33, buf) => decode_not_enough!(buf, NotEnoughDataForProtoErrorRequired),
            (34, buf) => decode_not_enough!(buf, NotEnoughDataForProtoErrorGiven),
            (35, buf) => decode_not_enough!(buf, NotEnoughDataForProtoErrorInvalidTag),
            (36, buf) => {
                let (key, buf) =
                    try_get_vec!(buf, NotEnoughDataForProtoErrorDbQueueOutOfSyncKeyLen, NotEnoughDataForProtoErrorDbQueueOutOfSyncKey);
                Ok((ProtoError::DbQueueOutOfSync(key), buf))
            },
            (37, buf) => decode_not_enough!(buf, NotEnoughDataForProtoErrorDbQueueOutOfSyncKeyLen),
            (38, buf) => decode_not_enough!(buf, NotEnoughDataForProtoErrorDbQueueOutOfSyncKey),
            (39, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqUpdateKeyLen),
            (40, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqUpdateKey),
            (41, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqUpdateValueLen),
            (42, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqUpdateValue),
            (43, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqHeartbeatLendKey),
            (44, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqHeartbeatKeyLen),
            (45, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqHeartbeatKey),
            (46, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqHeartbeatTimeout),
            (47, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqLookupKeyLen),
            (48, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqLookupKey),
            (49, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepValueFoundValueLen),
            (50, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepValueFoundValue),
            (51, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqLendMode),
            (52, buf) => decode_tag!(buf, InvalidGlobalReqLendModeTag),
            (53, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqAddMode),
            (54, buf) => decode_tag!(buf, InvalidGlobalReqAddModeTag),
            (55, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsPing),
            (56, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalRepStatsRemove),
            (57, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRemoveKeyLen),
            (58, buf) => decode_not_enough!(buf, NotEnoughDataForGlobalReqRemoveKey),
            (tag, _) => return Err(ProtoError::InvalidProtoErrorTag(tag)),
        }
    }

    pub fn encode_len(&self) -> usize {
        size_of::<u8>() + match self {
            &ProtoError::NotEnoughDataForGlobalReqTag { .. } |
            &ProtoError::NotEnoughDataForGlobalReqAddKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqAddKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqAddValueLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqAddValue { .. } |
            &ProtoError::NotEnoughDataForGlobalReqLendTimeout { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRepayLendKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRepayKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRepayKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRepayValueLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRepayValue { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRepayRepayStatus { .. } |
            &ProtoError::NotEnoughDataForGlobalRepTag { .. } |
            &ProtoError::NotEnoughDataForGlobalRepCountCount { .. } |
            &ProtoError::NotEnoughDataForGlobalRepLentLendKey { .. } |
            &ProtoError::NotEnoughDataForGlobalRepLentKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalRepLentKey { .. } |
            &ProtoError::NotEnoughDataForGlobalRepLentValueLen { .. } |
            &ProtoError::NotEnoughDataForGlobalRepLentValue { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsPing { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsCount { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsAdd { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsUpdate { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsLookup { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsRemove { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsLend { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsRepay { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsHeartbeat { .. } |
            &ProtoError::NotEnoughDataForGlobalRepStatsStats { .. } |
            &ProtoError::NotEnoughDataForProtoErrorTag { .. } |
            &ProtoError::NotEnoughDataForProtoErrorRequired { .. } |
            &ProtoError::NotEnoughDataForProtoErrorGiven { .. } |
            &ProtoError::NotEnoughDataForProtoErrorInvalidTag { .. } |
            &ProtoError::NotEnoughDataForProtoErrorDbQueueOutOfSyncKeyLen { .. } |
            &ProtoError::NotEnoughDataForProtoErrorDbQueueOutOfSyncKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqUpdateKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqUpdateKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqUpdateValueLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqUpdateValue { .. } |
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatLendKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatTimeout { .. } |
            &ProtoError::NotEnoughDataForGlobalReqLookupKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqLookupKey { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRemoveKeyLen { .. } |
            &ProtoError::NotEnoughDataForGlobalReqRemoveKey { .. } |
            &ProtoError::NotEnoughDataForGlobalRepValueFoundValueLen { .. } |
            &ProtoError::NotEnoughDataForGlobalRepValueFoundValue { .. } |
            &ProtoError::NotEnoughDataForGlobalReqLendMode { .. } |
            &ProtoError::NotEnoughDataForGlobalReqAddMode { .. } =>
                size_of::<u32>() + size_of::<u32>(),
            &ProtoError::InvalidGlobalRepTag(..) |
            &ProtoError::InvalidGlobalReqTag(..) |
            &ProtoError::InvalidGlobalReqRepayRepayStatusTag(..) |
            &ProtoError::InvalidProtoErrorTag(..) |
            &ProtoError::InvalidGlobalReqLendModeTag(..) |
            &ProtoError::InvalidGlobalReqAddModeTag(..) =>
                size_of::<u8>(),
            &ProtoError::DbQueueOutOfSync(ref key) => size_of::<u32>() + key.len(),

        }
    }

    pub fn encode<'b>(&self, area: &'b mut [u8]) -> &'b mut [u8] {
        match self {
            &ProtoError::NotEnoughDataForGlobalReqTag { required: r, given: g, } => encode_not_enough!(area, 1, r, g),
            &ProtoError::InvalidGlobalReqTag(tag) => encode_tag!(area, 2, tag),
            &ProtoError::NotEnoughDataForGlobalReqAddKeyLen { required: r, given: g, } => encode_not_enough!(area, 3, r, g),
            &ProtoError::NotEnoughDataForGlobalReqAddKey { required: r, given: g, } => encode_not_enough!(area, 4, r, g),
            &ProtoError::NotEnoughDataForGlobalReqAddValueLen { required: r, given: g, } => encode_not_enough!(area, 5, r, g),
            &ProtoError::NotEnoughDataForGlobalReqAddValue { required: r, given: g, } => encode_not_enough!(area, 6, r, g),
            &ProtoError::NotEnoughDataForGlobalReqLendTimeout { required: r, given: g, } => encode_not_enough!(area, 7, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRepayLendKey { required: r, given: g, } => encode_not_enough!(area, 8, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRepayKeyLen { required: r, given: g, } => encode_not_enough!(area, 9, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRepayKey { required: r, given: g, } => encode_not_enough!(area, 10, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRepayValueLen { required: r, given: g, } => encode_not_enough!(area, 11, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRepayValue { required: r, given: g, } => encode_not_enough!(area, 12, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRepayRepayStatus { required: r, given: g, } => encode_not_enough!(area, 13, r, g),
            &ProtoError::InvalidGlobalReqRepayRepayStatusTag(tag) => encode_tag!(area, 14, tag),
            &ProtoError::NotEnoughDataForGlobalRepTag { required: r, given: g, } => encode_not_enough!(area, 15, r, g),
            &ProtoError::InvalidGlobalRepTag(tag) => encode_tag!(area, 16, tag),
            &ProtoError::NotEnoughDataForGlobalRepCountCount { required: r, given: g, } => encode_not_enough!(area, 17, r, g),
            &ProtoError::NotEnoughDataForGlobalRepLentLendKey { required: r, given: g, } => encode_not_enough!(area, 18, r, g),
            &ProtoError::NotEnoughDataForGlobalRepLentKeyLen { required: r, given: g, } => encode_not_enough!(area, 19, r, g),
            &ProtoError::NotEnoughDataForGlobalRepLentKey { required: r, given: g, } => encode_not_enough!(area, 20, r, g),
            &ProtoError::NotEnoughDataForGlobalRepLentValueLen { required: r, given: g, } => encode_not_enough!(area, 21, r, g),
            &ProtoError::NotEnoughDataForGlobalRepLentValue { required: r, given: g, } => encode_not_enough!(area, 22, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsCount { required: r, given: g, } => encode_not_enough!(area, 23, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsAdd { required: r, given: g, } => encode_not_enough!(area, 24, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsUpdate { required: r, given: g, } => encode_not_enough!(area, 25, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsLookup { required: r, given: g, } => encode_not_enough!(area, 26, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsLend { required: r, given: g, } => encode_not_enough!(area, 27, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsRepay { required: r, given: g, } => encode_not_enough!(area, 28, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsHeartbeat { required: r, given: g, } => encode_not_enough!(area, 29, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsStats { required: r, given: g, } => encode_not_enough!(area, 30, r, g),
            &ProtoError::NotEnoughDataForProtoErrorTag { required: r, given: g, } => encode_not_enough!(area, 31, r, g),
            &ProtoError::InvalidProtoErrorTag(tag) => encode_tag!(area, 32, tag),
            &ProtoError::NotEnoughDataForProtoErrorRequired { required: r, given: g, } => encode_not_enough!(area, 33, r, g),
            &ProtoError::NotEnoughDataForProtoErrorGiven { required: r, given: g, } => encode_not_enough!(area, 34, r, g),
            &ProtoError::NotEnoughDataForProtoErrorInvalidTag { required: r, given: g, } => encode_not_enough!(area, 35, r, g),
            &ProtoError::DbQueueOutOfSync(ref key) => {
                let area = put_adv!(area, u8, write_u8, 36);
                let area = put_vec_adv!(area, key);
                area
            },
            &ProtoError::NotEnoughDataForProtoErrorDbQueueOutOfSyncKeyLen { required: r, given: g, } => encode_not_enough!(area, 37, r, g),
            &ProtoError::NotEnoughDataForProtoErrorDbQueueOutOfSyncKey { required: r, given: g, } => encode_not_enough!(area, 38, r, g),
            &ProtoError::NotEnoughDataForGlobalReqUpdateKeyLen { required: r, given: g, } => encode_not_enough!(area, 39, r, g),
            &ProtoError::NotEnoughDataForGlobalReqUpdateKey { required: r, given: g, } => encode_not_enough!(area, 40, r, g),
            &ProtoError::NotEnoughDataForGlobalReqUpdateValueLen { required: r, given: g, } => encode_not_enough!(area, 41, r, g),
            &ProtoError::NotEnoughDataForGlobalReqUpdateValue { required: r, given: g, } => encode_not_enough!(area, 42, r, g),
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatLendKey { required: r, given: g, } => encode_not_enough!(area, 43, r, g),
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatKeyLen { required: r, given: g, } => encode_not_enough!(area, 44, r, g),
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatKey { required: r, given: g, } => encode_not_enough!(area, 45, r, g),
            &ProtoError::NotEnoughDataForGlobalReqHeartbeatTimeout { required: r, given: g, } => encode_not_enough!(area, 46, r, g),
            &ProtoError::NotEnoughDataForGlobalReqLookupKeyLen { required: r, given: g, } => encode_not_enough!(area, 47, r, g),
            &ProtoError::NotEnoughDataForGlobalReqLookupKey { required: r, given: g, } => encode_not_enough!(area, 48, r, g),
            &ProtoError::NotEnoughDataForGlobalRepValueFoundValueLen { required: r, given: g, } => encode_not_enough!(area, 49, r, g),
            &ProtoError::NotEnoughDataForGlobalRepValueFoundValue { required: r, given: g, } => encode_not_enough!(area, 50, r, g),
            &ProtoError::NotEnoughDataForGlobalReqLendMode { required: r, given: g, } => encode_not_enough!(area, 51, r, g),
            &ProtoError::InvalidGlobalReqLendModeTag(tag) => encode_tag!(area, 52, tag),
            &ProtoError::NotEnoughDataForGlobalReqAddMode { required: r, given: g, } => encode_not_enough!(area, 53, r, g),
            &ProtoError::InvalidGlobalReqAddModeTag(tag) => encode_tag!(area, 54, tag),
            &ProtoError::NotEnoughDataForGlobalRepStatsPing { required: r, given: g, } => encode_not_enough!(area, 55, r, g),
            &ProtoError::NotEnoughDataForGlobalRepStatsRemove { required: r, given: g, } => encode_not_enough!(area, 56, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRemoveKeyLen { required: r, given: g, } => encode_not_enough!(area, 57, r, g),
            &ProtoError::NotEnoughDataForGlobalReqRemoveKey { required: r, given: g, } => encode_not_enough!(area, 58, r, g),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use super::{Key, Value, RepayStatus, LendMode, AddMode, GlobalReq, GlobalRep, ProtoError};

    macro_rules! defassert_encode_decode {
        ($name:ident, $ty:ty, $class:ident) => (fn $name(r: $ty) {
            let bytes_required = r.encode_len();
            let mut area: Vec<_> = (0 .. bytes_required).map(|_| 0).collect();
            assert!(r.encode(&mut area).len() == 0);
            let (assert_r, rest) = $class::decode(&area).unwrap();
            assert_eq!(rest.len(), 0);
            assert_eq!(r, assert_r);
        })
    }

    defassert_encode_decode!(assert_encode_decode_req, GlobalReq, GlobalReq);
    defassert_encode_decode!(assert_encode_decode_rep, GlobalRep, GlobalRep);

    fn dummy_key_value() -> (Key, Value) {
        (Arc::new("some key".as_bytes().to_owned()),
         Arc::new("some value".as_bytes().to_owned()))
    }

    #[test]
    fn globalreq_ping() {
        assert_encode_decode_req(GlobalReq::Ping);
    }

    #[test]
    fn globalreq_count() {
        assert_encode_decode_req(GlobalReq::Count);
    }

    #[test]
    fn globalreq_add_head() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Add { key: key, value: value, mode: AddMode::Head });
    }

    #[test]
    fn globalreq_add_tail() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Add { key: key, value: value, mode: AddMode::Tail });
    }

    #[test]
    fn globalreq_update() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Update(key, value));
    }

    #[test]
    fn globalreq_lookup() {
        let (key, _) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Lookup(key));
    }

    #[test]
    fn globalreq_remove() {
        let (key, _) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Remove(key));
    }

    #[test]
    fn globalreq_lend_block() {
        assert_encode_decode_req(GlobalReq::Lend { timeout: 177, mode: LendMode::Block, });
    }

    #[test]
    fn globalreq_lend_poll() {
        assert_encode_decode_req(GlobalReq::Lend { timeout: 177, mode: LendMode::Poll, });
    }

    #[test]
    fn globalreq_repay_penalty() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Repay { lend_key: 177, key: key, value: value, status: RepayStatus::Penalty, });
    }

    #[test]
    fn globalreq_repay_reward() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Repay { lend_key: 177, key: key, value: value, status: RepayStatus::Reward, });
    }

    #[test]
    fn globalreq_repay_front() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Repay { lend_key: 177, key: key, value: value, status: RepayStatus::Front, });
    }

    #[test]
    fn globalreq_repay_drop() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Repay { lend_key: 177, key: key, value: value, status: RepayStatus::Drop, });
    }

    #[test]
    fn globalreq_heartbeat() {
        let (key, _) = dummy_key_value();
        assert_encode_decode_req(GlobalReq::Heartbeat { lend_key: 177, key: key, timeout: 177, });
    }

    #[test]
    fn globalreq_stats() {
        assert_encode_decode_req(GlobalReq::Stats);
    }

    #[test]
    fn globalreq_terminate() {
        assert_encode_decode_req(GlobalReq::Terminate);
    }

    #[test]
    fn globalrep_pong() {
        assert_encode_decode_rep(GlobalRep::Pong);
    }

    #[test]
    fn globalrep_counted() {
        assert_encode_decode_rep(GlobalRep::Counted(97));
    }

    #[test]
    fn globalrep_added() {
        assert_encode_decode_rep(GlobalRep::Added);
    }

    #[test]
    fn globalrep_kept() {
        assert_encode_decode_rep(GlobalRep::Kept);
    }

    #[test]
    fn globalrep_updated() {
        assert_encode_decode_rep(GlobalRep::Updated);
    }

    #[test]
    fn globalrep_notfound() {
        assert_encode_decode_rep(GlobalRep::NotFound);
    }

    #[test]
    fn globalrep_valuefound() {
        let (_, value) = dummy_key_value();
        assert_encode_decode_rep(GlobalRep::ValueFound(value));
    }

    #[test]
    fn globalrep_valuenotfound() {
        assert_encode_decode_rep(GlobalRep::ValueNotFound);
    }

    #[test]
    fn globalrep_lent() {
        let (key, value) = dummy_key_value();
        assert_encode_decode_rep(GlobalRep::Lent { lend_key: 177, key: key, value: value, });
    }

    #[test]
    fn globalrep_queueempty() {
        assert_encode_decode_rep(GlobalRep::QueueEmpty);
    }

    #[test]
    fn globalrep_repaid() {
        assert_encode_decode_rep(GlobalRep::Repaid);
    }

    #[test]
    fn globalrep_heartbeaten() {
        assert_encode_decode_rep(GlobalRep::Heartbeaten);
    }

    #[test]
    fn globalrep_skipped() {
        assert_encode_decode_rep(GlobalRep::Skipped);
    }

    #[test]
    fn globalrep_stats() {
        assert_encode_decode_rep(GlobalRep::StatsGot {
            ping: 77,
            count: 177,
            add: 277,
            update: 377,
            lookup: 477,
            remove: 577,
            lend: 677,
            repay: 777,
            heartbeat: 877,
            stats: 977,
        });
    }

    #[test]
    fn globalrep_flushed() {
        assert_encode_decode_rep(GlobalRep::Flushed);
    }

    #[test]
    fn globalrep_terminated() {
        assert_encode_decode_rep(GlobalRep::Terminated);
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqtag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqTag { required: 177, given: 277, }));
    }

    #[test]
    fn globalrep_error_invalidglobalreqtag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::InvalidGlobalReqTag(177)));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqaddkeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqAddKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqaddkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqAddKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqaddvaluelen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqAddValueLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqaddvalue() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqAddValue { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqlendtimeout() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqLendTimeout { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqrepaylendkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRepayLendKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqrepaykeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRepayKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqrepaykey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRepayKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqrepayvaluelen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRepayValueLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqrepayvalue() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRepayValue { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqrepayrepaystatus() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRepayRepayStatus { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_invalidglobalreqrepayrepaystatustag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::InvalidGlobalReqRepayRepayStatusTag(177)));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreptag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepTag { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_invalidglobalreptag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::InvalidGlobalRepTag(177)));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepcountcount() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepCountCount { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreplentlendkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepLentLendKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreplentkeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepLentKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreplentkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepLentKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreplentvaluelen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepLentValueLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreplentvalue() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepLentValue { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatscount() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsCount { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatsadd() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsAdd { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatsupdate() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsUpdate { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatslookup() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsLookup { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatslend() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsLend { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatsrepay() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsRepay { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatsheartbeat() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsHeartbeat { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrepstatsstats() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalRepStatsStats { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforprotoerrortag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForProtoErrorTag { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_invalidprotoerrortag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::InvalidProtoErrorTag(177)));
    }

    #[test]
    fn globalrep_error_notenoughdataforprotoerrorrequired() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForProtoErrorRequired { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforprotoerrorgiven() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForProtoErrorGiven { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforprotoerrorinvalidtag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForProtoErrorInvalidTag { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforprotoerrordbqueueoutofsynckey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForProtoErrorDbQueueOutOfSyncKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforprotoerrordbqueueoutofsynckeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForProtoErrorDbQueueOutOfSyncKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_dbqueueoutofsync() {
        let (key, _) = dummy_key_value();
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::DbQueueOutOfSync(key)));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrequpdatekeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqUpdateKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrequpdatekey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqUpdateKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrequpdatevaluelen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqUpdateValueLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalrequpdatevalue() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqUpdateValue { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqheartbeatlendkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqHeartbeatLendKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqheartbeatkeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqHeartbeatKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqheartbeatkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqHeartbeatKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqheartbeattimeout() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqHeartbeatTimeout { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_invalidglobalreqlendmodetag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::InvalidGlobalReqLendModeTag(177)));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqlendmode() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqLendMode { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_invalidglobalreqaddmodetag() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::InvalidGlobalReqAddModeTag(177)));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqaddmode() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqAddMode { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqlookupkeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqLookupKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqlookupkey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqLookupKey { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqremovekeylen() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRemoveKeyLen { required: 177, given: 177, }));
    }

    #[test]
    fn globalrep_error_notenoughdataforglobalreqremovekey() {
        assert_encode_decode_rep(GlobalRep::Error(ProtoError::NotEnoughDataForGlobalReqRemoveKey { required: 177, given: 177, }));
    }
}
