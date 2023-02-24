#[derive(Debug)]
#[repr(C)]
pub enum Status {
    Success,
    Failure,
}

#[repr(C)]
pub struct CMyLib {
    _f: [u8; 0],
}
pub type MyLibHandle = *mut CMyLib;
impl CMyLib {
    pub fn handle() -> MyLibHandle {
        std::ptr::null_mut()
    }
}

#[link(name = "mylib")]
extern "C" {
    fn mylib_create(handle_ptr: *mut MyLibHandle) -> Status;
    fn mylib_destroy(handle: MyLibHandle) -> Status;
    fn mylib_set(handle: MyLibHandle, value: i32) -> Status;
    fn mylib_dump(handle: MyLibHandle) -> Status;
}

pub struct MyLib {
    handle: MyLibHandle,
}

impl MyLib {
    pub fn create() -> Result<Self, String> {
        let mut handle = CMyLib::handle();
        match unsafe { mylib_create(std::ptr::addr_of_mut!(handle)) } {
            Status::Success => Ok(Self { handle }),
            err_code => Err(format!("Failed to create MyLib. Error: {err_code:?}")),
        }
    }

    pub fn set(&mut self, val: i32) {
        let Status::Success = (unsafe {
            mylib_set(self.handle, val)
        }) else {
            panic!("Failed to set value");
        };
    }

    pub fn dump(&mut self) {
        let Status::Success = (unsafe {
            mylib_dump(self.handle)
        }) else {
            panic!("Failed to dump mylib");
        };
    }
}

impl Drop for MyLib {
    fn drop(&mut self) {
        let Status::Success = (unsafe { mylib_destroy(self.handle) })
        else {
            panic!("Something went wrong");
        };
    }
}

//
// Run with LD_LIBRARY_PATH=../shared_lib/build:$LD_LIBRARY_PATH cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ml = MyLib::create().unwrap();
        ml.dump();
        ml.set(123);
        ml.dump();
    }
}
