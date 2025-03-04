pub use libwing;
pub use allo_isolate::store_dart_post_cobject;

/// # Safety
///
/// This function should not be called with null
#[no_mangle]
pub unsafe extern "C" fn dart_wing_console_read(handle: *mut libwing::WingConsoleHandle, send_port: i64) {
    let console = unsafe { &mut (*handle).console };
    std::thread::spawn(move || {
        let send_port = allo_isolate::Isolate::new(send_port);
        loop {
            if let Ok(response) = console.read() {
                let res = Box::into_raw(Box::new(libwing::ResponseHandle { response }));
                send_port.post(res);
            } else {
                send_port.post(0_i64);
                break;
            }
        }
    });
}

/// # Safety
///
/// This function should not be called with null
#[no_mangle]
pub unsafe extern "C" fn dart_wing_console_read_meter(handle: *mut libwing::WingConsoleHandle, send_port: i64) {
    let console = unsafe { &mut (*handle).console };
    std::thread::spawn(move || {
        let send_port = allo_isolate::Isolate::new(send_port);
        loop {
            if let Ok((id, data)) = console.read_meters() {
                send_port.post((id, data));
            } else {
                send_port.post(0_i64);
                break;
            }
        }
    });
}
