use std::{ffi::{c_void, CString}, sync::{atomic::AtomicBool, Arc, OnceLock}};

pub struct Context {
    pub run: Arc<AtomicBool>,
    pub ctx: *mut c_void,
    pub sock: *mut c_void,
}

impl Context {
    /// Create both the ZeroMQ Context and Socket(s) we'll be using for our
    /// requests.
    fn new() -> Self {
        let ctx = unsafe {
            zmq_sys::zmq_ctx_new()
        };
        assert_ne!(ctx, std::ptr::null() as *const c_void as *mut c_void);

        let sock = unsafe {
            let sock = zmq_sys::zmq_socket(ctx, zmq_sys::ZMQ_REP as i32);
            let addr = CString::new("tcp://localhost:5500").unwrap();
            zmq_sys::zmq_connect(sock, addr.as_ptr());
            sock
        };

        Self { run: Arc::new(AtomicBool::new(true)), ctx, sock }
    }

    pub fn raw_ctx_ptr(&self) -> *mut c_void {
        self.ctx
    }

    pub fn raw_sock_ptr(&self) -> *mut c_void {
        self.sock
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // So this operation doesn't block, we close the socket first, then we
        // close the context.

        self.run = AtomicBool::new(false).into();

        let mut e = unsafe { zmq_sys::zmq_close(self.sock) };
        while e == -1 {
            e = unsafe { zmq_sys::zmq_close(self.sock) };
        }

        e = unsafe { zmq_sys::zmq_ctx_shutdown(self.ctx) };
        while e == -1 {
            e = unsafe { zmq_sys::zmq_ctx_shutdown(self.ctx) };
        }
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

static ZMQ_CONTEXT_POINTER: OnceLock<Context> = OnceLock::new();

#[no_mangle]
pub fn get_zmq_context_pointer() -> *const c_void {
    get_context().raw_ctx_ptr()
}

#[no_mangle]
pub fn get_zmq_sock_pointer() -> *const c_void {
    get_context().raw_sock_ptr()
}

pub fn get_context() -> &'static Context {
    ZMQ_CONTEXT_POINTER.get_or_init(|| {
        Context::new()
    })
}
