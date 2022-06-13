use super::*;

mod ipc;
mod sem;
mod shm;
mod syscalls;

pub use self::ipc::key_t;
pub use self::shm::{shmids_t, SHM_MANAGER};
pub use self::syscalls::{do_shmat, do_shmctl, do_shmdt, do_shmget};
