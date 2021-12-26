use std::ptr::NonNull;
use std::sync::{Mutex,Condvar,Arc};
use ndk_sys::{ ANativeActivity, AInputQueue, ARect, ANativeWindow };
use std::os::raw::c_void;
use std::os::unix::prelude::RawFd;
use std::thread;
pub use log::{info,error};

#[derive(Debug)]
pub struct AndroidApp {
    pub running:bool,
    saved_state: Option<*mut u8>,
    saved_state_size:Option<usize>,
    cmd_msg_read:RawFd,
    cmd_msg_write:RawFd,
    cmd_poll_source:Option<AndroidPollSource>,
    input_poll_source:Option<AndroidPollSource>,
    looper:Option<*const ndk_sys::ALooper>,
}
unsafe impl Send for AndroidApp {}
unsafe impl Sync for AndroidApp {}

impl AndroidApp {
    pub fn new(native_activity:NonNull<ANativeActivity>)->Self {
        return AndroidApp{
            saved_state:None,
            saved_state_size: None,
            cmd_msg_read:0,
            cmd_msg_write:0,
            cmd_poll_source:None,
            input_poll_source:None,
            looper:None,
            running:false,
        };
    }

}

pub unsafe fn android_app_create(activity: NonNull<ANativeActivity>,saved_state : *mut u8,saved_state_size:usize,main :fn(Arc<(Mutex<AndroidApp>,Condvar)>))->Arc<(Mutex<AndroidApp>,Condvar)>{

    let mut app = AndroidApp::new(activity);
    if !saved_state.is_null() {
        app.saved_state = Some(libc::malloc(saved_state_size) as *mut u8);
        app.saved_state_size = Some(saved_state_size);
        libc::memcpy(app.saved_state.unwrap() as *mut c_void,saved_state as *mut c_void,saved_state_size);
    }
    let mut cmd_msg_pipe : [RawFd; 2] = Default::default();
    libc::pipe(cmd_msg_pipe.as_mut_ptr());
    app.cmd_msg_read=cmd_msg_pipe[0];
    app.cmd_msg_write=cmd_msg_pipe[1];

    let arc_app_cond = Arc::new((Mutex::new(app),Condvar::new()));
    //{
    let mut app_for_entry = Arc::clone(&arc_app_cond) ;
    thread::spawn(move ||{
        android_app_entry(app_for_entry,main);
    });
    //}

    return arc_app_cond;
}

pub fn android_app_entry(arc_app_cond: Arc<(Mutex< AndroidApp>,Condvar)>,main:fn(Arc<(Mutex<AndroidApp>,Condvar)>)){
    android_app_entry_init_looper(arc_app_cond.clone());
     main(arc_app_cond.clone());
}


pub fn android_app_entry_init_looper(arc_app_cond: Arc<(Mutex< AndroidApp>,Condvar)>){
    let (app_mutex , cond)= &*arc_app_cond;
    let mut app = app_mutex.lock().unwrap();
    info!("android_app_entry {:?}",app);
    app.cmd_poll_source = Some(
            AndroidPollSource{
                id: NDK_GLUE_LOOPER_EVENT_PIPE_IDENT,
                arc_app_cond:arc_app_cond.clone(),
            }  
        );
    app.input_poll_source =Some(
            AndroidPollSource{
                id: NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT ,
                arc_app_cond:arc_app_cond.clone(),

            }
        );
    let looper = unsafe {
        ndk_sys::ALooper_prepare(ndk_sys:: ALOOPER_PREPARE_ALLOW_NON_CALLBACKS.try_into().unwrap())
    };
    app.looper = Some(looper);
    app.running=true;
    info!("sleeping");
    thread::sleep(std::time::Duration::from_millis(3000));
    cond.notify_all();
}


#[derive(Debug)]
struct AndroidPollSource {
    id: i32,
    arc_app_cond: Arc<(Mutex<AndroidApp>,Condvar)>,
}


pub const NDK_GLUE_LOOPER_EVENT_PIPE_IDENT: i32 = 0;

pub const NDK_GLUE_LOOPER_INPUT_QUEUE_IDENT: i32 = 1;
