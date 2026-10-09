use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};
use std::sync::Mutex;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPMAssertionCreateWithName(
        kind: CFStringRef,
        level: u32,
        name: CFStringRef,
        id: *mut u32,
    ) -> i32;
    fn IOPMAssertionRelease(id: u32) -> i32;
}

struct Assertion(u32);

impl Assertion {
    fn new(kind: &str) -> Result<Self, String> {
        let kind = CFString::new(kind);
        let name = CFString::new("DCD fullscreen clock");
        let mut id = 0;
        let result = unsafe {
            IOPMAssertionCreateWithName(
                kind.as_concrete_TypeRef(),
                255,
                name.as_concrete_TypeRef(),
                &mut id,
            )
        };
        if result != 0 {
            return Err(format!("IOPMAssertionCreateWithName: {result:#x}"));
        }
        Ok(Self(id))
    }
}

impl Drop for Assertion {
    fn drop(&mut self) {
        let result = unsafe { IOPMAssertionRelease(self.0) };
        if result != 0 {
            tracing::error!("Unable to release clock power assertion: {result:#x}");
        }
    }
}

#[derive(Default)]
pub struct ClockKeepAwake(Mutex<Option<[Assertion; 2]>>);

impl ClockKeepAwake {
    pub fn acquire(&self) -> Result<(), String> {
        let mut active = self.0.lock().map_err(|error| error.to_string())?;
        if active.is_none() {
            *active = Some([
                Assertion::new("PreventUserIdleDisplaySleep")?,
                Assertion::new("PreventUserIdleSystemSleep")?,
            ]);
        }
        Ok(())
    }

    pub fn release(&self) {
        let mut active = self.0.lock().unwrap_or_else(|error| error.into_inner());
        active.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{base::CFRelease, dictionary::CFDictionaryRef};

    extern "C" {
        fn IOPMAssertionCopyProperties(id: u32) -> CFDictionaryRef;
    }

    fn exists(id: u32) -> bool {
        unsafe {
            let properties = IOPMAssertionCopyProperties(id);
            if properties.is_null() {
                return false;
            }
            CFRelease(properties.cast());
            true
        }
    }

    #[test]
    #[ignore = "requires a running macOS power management service"]
    fn native_clock_assertions_are_idempotent_and_released() {
        let guard = ClockKeepAwake::default();
        guard.acquire().unwrap();
        let ids = guard
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .each_ref()
            .map(|value| value.0);
        assert!(ids.into_iter().all(exists));
        guard.acquire().unwrap();
        assert_eq!(
            guard
                .0
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .each_ref()
                .map(|value| value.0),
            ids
        );
        guard.release();
        assert!(ids.into_iter().all(|id| !exists(id)));
        guard.release();
        guard.acquire().unwrap();
        let ids = guard
            .0
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .each_ref()
            .map(|value| value.0);
        drop(guard);
        assert!(ids.into_iter().all(|id| !exists(id)));
    }
}
