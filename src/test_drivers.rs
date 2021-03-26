use crate::wycheproof::{*, self};
use syscalls;

pub trait TestDriverOps {
    fn sig_verify(&self, test: &WycheproofTestCase) -> bool;
}

pub struct KeyctlTestDriver;
impl TestDriverOps for KeyctlTestDriver {
    fn sig_verify(&self, test: &WycheproofTestCase) -> bool {
        let mut rv;

        unsafe {
            rv = syscalls::syscall(syscalls::SYS_add_key, &syscalls::SyscallArgs::new(1, 2, 3, 4, 5, 6)).unwrap();
        }
        println!("verify {} {}", test.msg, rv);
        return true;
    }
}
