use super::*;

use crate::process::{gid_t, uid_t};

#[allow(non_camel_case_types)]
pub type key_t = u32;
pub type ShmId = u32;
pub type CmdId = u32;

// #define SEMOP		 1
// #define SEMGET		 2
// #define SEMCTL		 3
// #define SEMTIMEDOP	 4
// #define MSGSND		11
// #define MSGRCV		12
// #define MSGGET		13
// #define MSGCTL		14
// #define SHMAT		21
// #define SHMDT		22
// #define SHMGET		23
// #define SHMCTL		24

pub const IPC_PRIVATE: key_t = 0;

// For cmd in shmctl()
pub const IPC_RMID: CmdId = 0;
pub const IPC_SET: CmdId = 1;
pub const IPC_STAT: CmdId = 2;
pub const IPC_INFO: CmdId = 3;
pub const SHM_LOCK: CmdId = 11;
pub const SHM_UNLOCK: CmdId = 12;
pub const SHM_STAT: CmdId = 13;
pub const SHM_INFO: CmdId = 14;
pub const SHM_STAT_ANY: CmdId = 15;

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct ipc_perm_t {
    pub key: key_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: u16,
    pub pad1: u16,
    pub seq: u16,
    pub pad2: u16,
    pub unused1: u64,
    pub unused2: u64,
}
