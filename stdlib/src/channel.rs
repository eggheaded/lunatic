use std::mem::{size_of, zeroed};
use std::marker::PhantomData;

#[repr(C)]
pub struct __wasi_iovec_t {
    pub buf: i32,
    pub buf_len: i32,
}

mod stdlib {
    use super::__wasi_iovec_t;

    #[link(wasm_import_module = "lunatic")]
    extern "C" {
        pub fn channel(bound: i32) -> i32;
        pub fn send(channel: i32, data: *const __wasi_iovec_t);
        pub fn receive(channel: i32, data: *const __wasi_iovec_t);
    }
}

#[derive(Copy, Clone)]
pub struct Channel<T> {
    id: i32,
    phantom: PhantomData<T>
}

impl<T: Copy> Channel<T> {
    /// If `bound` is 0, returns an unbound channel.
    pub fn new(bound: usize) -> Self {
        let id = unsafe { stdlib::channel(bound as i32) };
        Self { id, phantom: PhantomData }
    }

    pub fn send(&self, value: T) {
        let data = __wasi_iovec_t {
            buf: &value as *const T as i32,
            buf_len: size_of::<T>() as i32
        };
    
        unsafe { stdlib::send(self.id, &data as *const __wasi_iovec_t); }
    }

    pub fn receive(&self) -> T {
        let result: T = unsafe { zeroed() };
    
        let data = __wasi_iovec_t {
            buf: &result as *const T as i32,
            buf_len: size_of::<T>() as i32
        };
    
        unsafe { stdlib::receive(self.id, &data as *const __wasi_iovec_t); }
        result
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub unsafe fn from_id(id: i32) -> Self {
        Self { id, phantom: PhantomData }
    }
}
