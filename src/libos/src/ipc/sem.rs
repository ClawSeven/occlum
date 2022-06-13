use super::*;

use super::ipc::{ipc_perm_t, key_t};
use crate::fs::FileMode;
use crate::process::{do_getegid, do_geteuid, gid_t, uid_t, ThreadRef};
use crate::time::time_t;

use std::collections::{HashMap, HashSet};

type SemId = u32; // semaphore set identifier

// const SEMAEM: usize = 0;
const SEMMNI: u32 = 4096;

struct semid_ds_t {
    // ownership and permissions
    sem_perm: ipc_perm_t,
    // time of last semop
    sem_otime: time_t,
    // time of last change
    sem_ctime: time_t,
    // number of semaphores in set
    sem_nsems: u32,
}

// 需要细看,16-bits也够用的
bitflags! {
    struct SemFlags: u32 {
        const IPC_CREAT = 0o1000;
        const IPC_EXCL = 0o2000;

        /// read by owner
        const S_IRUSR = FileMode::S_IRUSR.bits() as u32;
        /// write by owner
        const S_IWUSR = FileMode::S_IWUSR.bits() as u32;
        /// execute/search by owner
        const S_IXUSR = FileMode::S_IXUSR.bits() as u32;
        /// read by group
        const S_IRGRP = FileMode::S_IRGRP.bits() as u32;
        /// write by group
        const S_IWGRP = FileMode::S_IWGRP.bits() as u32;
        /// execute/search by group
        const S_IXGRP = FileMode::S_IXGRP.bits() as u32;
        /// read by others
        const S_IROTH = FileMode::S_IROTH.bits() as u32;
        /// write by others
        const S_IWOTH = FileMode::S_IWOTH.bits() as u32;
        /// execute/search by others
        const S_IXOTH = FileMode::S_IXOTH.bits() as u32;
    }
}

struct sembuf_t {
    sem_num: u32, // semaphore number, unsigned short
    sem_op: i32,
    sem_flg: i32, // short
}

// one semaphore structure in semaphore sets
struct sem {
    // current value of semaphore
    semval: AtomicI32,
    // pid of the process last modified the semaphores
    pid: pid_t,
    sem_otime: time_t,
    // list_head
    // pending_alter: VecDeque<>,
    // pending_const: VecDeque<>,
}

// /* One sem_array data structure for each set of semaphores in the system. */
// struct sem_array {
//     // ipc_perm_kernel
//     sem_ctime: time_t,
//     sem_nsems: u64, // the number of semaphores in array
//     // complex_count: u64, //pending complex operations
//     sems: RwLock<Vec<sem>>,
// }

// struct SemManager {
//     semid: HashMap<SemId, semid_ds_t>,
// }

// struct sem_t {

// }

struct SemSet {
    semid: SemId,
    key: key_t,

    uid: uid_t,
    gid: gid_t,
    cuid: uid_t,
    cgid: gid_t,
    mode: FileMode,

    sem_nsems: u32, // the number of semaphores in array
    sem_array: Vec<sem>,
    sem_ctime: time_t,
    sem_otime: time_t,
    waiting_process: VecDeque<pid_t>,
}

struct SemIdManager {
    used_id: HashSet<SemId>,
    free_num: u32,
    last_alloc_id: SemId,
}

impl SemIdManager {
    fn new() -> Self {
        Self {
            used_id: HashSet::new(),
            free_num: SEMMNI,
            last_alloc_id: (SEMMNI - 1) as SemId,
        }
    }
}

struct SemManager {
    semid_manager: RwLock<SemIdManager>,
    semsets: HashMap<SemId, SemSet>,
}

// impl SemManager {
//     fn new() -> Self {

//     }

//     fn do_semget(&self, key: key_t, nsems: u32, semflg: SemFlags ) -> Result<usize> {

//     }

//     fn do_semctl() -> Result<usize> {

//     }

//     fn do_semop() -> Result<usize> {

//     }

// }
