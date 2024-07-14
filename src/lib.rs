use aig::Aig;
use std::ffi::{c_char, c_void, CString};

extern "C" {
    fn Abc_FrameGetGlobalFrame() -> *mut c_void;
    fn Abc_Stop();
    fn Cmd_CommandExecute(pAbc: *mut c_void, sCommand: *const c_char) -> i32;
}

pub struct Abc {
    ptr: *mut c_void,
}

impl Drop for Abc {
    fn drop(&mut self) {
        unsafe { Abc_Stop() };
    }
}

impl Abc {
    pub fn new() -> Self {
        let ptr = unsafe { Abc_FrameGetGlobalFrame() };
        assert!(!ptr.is_null(), "init abc failed");
        Self { ptr }
    }

    pub fn execute_command(&mut self, command: &str) {
        let c = CString::new(command).unwrap();
        let res = unsafe { Cmd_CommandExecute(self.ptr, c.as_ptr()) };
        assert!(res == 0, "abc execute {command} failed");
    }

    pub fn read_aig(&mut self, aig: &Aig) {
        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let path = tmpfile.path().as_os_str().to_str().unwrap();
        aig.to_file(path);
        let command = format!("read_aiger {};", path);
        let command = CString::new(command).unwrap();
        let res = unsafe { Cmd_CommandExecute(self.ptr, command.as_ptr()) };
        assert!(res == 0, "abc read aig failed");
    }

    pub fn write_aig(&mut self) -> Aig {
        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let path = tmpfile.path().as_os_str().to_str().unwrap();
        let command = format!("write_aiger {};", path);
        let command = CString::new(command).unwrap();
        let res = unsafe { Cmd_CommandExecute(self.ptr, command.as_ptr()) };
        assert!(res == 0, "abc write aig failed");
        Aig::from_file(path)
    }
}