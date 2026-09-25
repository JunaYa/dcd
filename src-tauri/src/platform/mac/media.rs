use std::{ffi::c_void, sync::OnceLock};

// MediaRemote is private: resolve it at runtime so its absence never prevents startup.
type SendCommand = unsafe extern "C" fn(u32, *const c_void) -> bool;
const PAUSE: u32 = 1;

fn sender() -> Result<SendCommand, &'static str> {
    static SENDER: OnceLock<Result<SendCommand, &'static str>> = OnceLock::new();
    *SENDER.get_or_init(|| unsafe {
        let library = libc::dlopen(
            c"/System/Library/PrivateFrameworks/MediaRemote.framework/MediaRemote".as_ptr(),
            libc::RTLD_LAZY | libc::RTLD_LOCAL,
        );
        if library.is_null() {
            return Err("MediaRemote is unavailable");
        }
        let symbol = libc::dlsym(library, c"MRMediaRemoteSendCommand".as_ptr());
        if symbol.is_null() {
            libc::dlclose(library);
            return Err("MediaRemote pause command is unavailable");
        }
        // Keep the framework loaded for the cached function and its asynchronous XPC work.
        Ok(std::mem::transmute::<*mut c_void, SendCommand>(symbol))
    })
}

pub fn pause_media() -> Result<(), &'static str> {
    let send = sender()?;
    // A dedicated pause is idempotent; never send the play/pause toggle or auto-resume.
    if unsafe { send(PAUSE, std::ptr::null()) } {
        Ok(())
    } else {
        Err("MediaRemote rejected the pause request")
    }
}
