use std::os::raw::c_char;

extern "C" {
    pub fn open(pathname: *const c_char, flags:i32) -> i32;
}

#[cfg(test)]
mod syscall {
    use crate::open;
    use std::ffi::CString;

    #[test]
    fn sample_syscall_by_hand(){
        let path = CString::new("Cargo.toml").expect("Path is not valid");
        let flags = 0;  // readonly
        let res = unsafe{ open( (&path).as_ptr(),flags) };
        println!("Open Result: {}",res);
        assert!(res > 0);
    }
    #[test]
    fn sample_syscall_by_libc() {
        let path = CString::new("Cargo.toml").expect("Path is not valid");
        let res =  unsafe { libc::open((&path).as_ptr(), libc::O_RDONLY) };
        println!("Open Result: {}",res);
        assert!(res > 0);
    }
}
