pub use android_logger;
pub use log;
pub use log::{info,error};
use std::thread;
use std::io::{BufRead, BufReader};
use std::ffi::{CStr, CString};
use std::os::raw;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::os::unix::prelude::RawFd;
use ndk_sys::{ANativeActivity,AInputQueue,ARect,ANativeWindow};
use std::ptr::NonNull;
use std::os::raw::c_void;

pub use pf_ndk_macro::main;

mod app;

pub unsafe  fn init(
    activity: *mut ndk_sys::ANativeActivity,
    _saved_state: *mut u8,
    _saved_state_size: usize,
    main: fn(),
    ){
    info!("activity {:?}",activity);

    // { 
    let mut logpipe: [RawFd; 2] = Default::default();
    libc::pipe(logpipe.as_mut_ptr());
    libc::dup2(logpipe[1], libc::STDOUT_FILENO);
    libc::dup2(logpipe[1], libc::STDERR_FILENO);
    thread::spawn(move || {
        //let tag = CStr::from_bytes_with_nul(b"RustStdoutStderr\0").unwrap();

        let file = File::from_raw_fd(logpipe[0]);
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if let Ok(len) = reader.read_line(&mut buffer) {
                if len == 0 {
                    break;
                } else if let Ok(msg) = CString::new(buffer.clone()) {
                    error!("{:?}",msg);
                    //android_log(Level::Info, tag, &msg);
                }
            }
        }
    });
    // }

    // {
    let mut activity = NonNull::new(activity).unwrap();
    let mut callbacks = activity.as_mut().callbacks.as_mut().unwrap();
    callbacks.onStart = Some(on_start);
    callbacks.onResume = Some(on_resume);
    callbacks.onSaveInstanceState = Some(on_save_instance_state);
    callbacks.onPause = Some(on_pause);
    callbacks.onStop = Some(on_stop);
    callbacks.onDestroy = Some(on_destroy);
    callbacks.onWindowFocusChanged = Some(on_window_focus_changed);
    callbacks.onNativeWindowCreated = Some(on_window_created);
    callbacks.onNativeWindowResized = Some(on_window_resized);
    callbacks.onNativeWindowRedrawNeeded = Some(on_window_redraw_needed);
    callbacks.onNativeWindowDestroyed = Some(on_window_destroyed);
    callbacks.onInputQueueCreated = Some(on_input_queue_created);
    callbacks.onInputQueueDestroyed = Some(on_input_queue_destroyed);
    callbacks.onContentRectChanged = Some(on_content_rect_changed);
    callbacks.onConfigurationChanged = Some(on_configuration_changed);
    callbacks.onLowMemory = Some(on_low_memory);
    // }
    
    // {
    let (app_mutex,cond )= &*app::android_app_create(activity,_saved_state,_saved_state_size);
    // activity.as_mut().instance =  app as *mut _ as *mut c_void ;
    info!("wating app to be running");
    let mut app =app_mutex.lock().unwrap();
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    while !app.running {
        app = cond.wait(app).unwrap();
    }
    // }
    
    info!("onCreate over");
}

unsafe extern "C" fn on_start(activity: *mut ANativeActivity){
    info!("on_start");
}

unsafe extern "C" fn on_resume(activity: *mut ANativeActivity){
    info!("on_resume");
}

unsafe extern "C" fn on_save_instance_state(activity: *mut ANativeActivity,_out_size: *mut ndk_sys::size_t)->*mut raw::c_void{
    info!("on_save_instance_state");
    // TODO
    std::ptr::null_mut()
}


unsafe extern "C" fn on_pause(activity: *mut ANativeActivity){
    info!("on_pause");
}

unsafe extern "C" fn on_stop(activity: *mut ANativeActivity){
    info!("on_stop");
}

unsafe extern "C" fn on_destroy(activity: *mut ANativeActivity){
    info!("on_destroy");
}

unsafe extern "C" fn on_window_focus_changed(
    activity: *mut ANativeActivity,
    has_focus: raw::c_int,
    ){
    info!("on_window_focus_changed");
}

unsafe extern "C" fn on_window_created(activity: *mut ANativeActivity,window: *mut ANativeWindow){
    info!("on_window_created {:?}",window);
}

unsafe extern "C" fn on_window_resized(activity: *mut ANativeActivity,window :*mut ANativeWindow){
    info!("on_window_resized");
}

unsafe extern "C" fn on_window_redraw_needed(activity: *mut ANativeActivity,window :*mut ANativeWindow){
    info!("on_window_redraw_needed");
}

unsafe extern "C" fn on_window_destroyed(activity: *mut ANativeActivity,window :*mut ANativeWindow){
    info!("on_window_destroyed");
}

unsafe extern "C" fn on_input_queue_created(activity: *mut ANativeActivity,queue: *mut AInputQueue){
    info!("on_input_queue_created");
}


unsafe extern "C" fn on_input_queue_destroyed(activity: *mut ANativeActivity,input_queue :*mut AInputQueue){
    info!("on_input_queue_destroyed");
}

unsafe extern "C" fn on_content_rect_changed(activity: *mut ANativeActivity,rect: *const ARect){
    info!("on_content_rect_changed");
}

unsafe extern "C" fn on_configuration_changed(activity: *mut ANativeActivity){
    info!("on_configuration_changed");
}

unsafe extern "C" fn on_low_memory(activity: *mut ANativeActivity){
    info!("on_low_memory");
}
