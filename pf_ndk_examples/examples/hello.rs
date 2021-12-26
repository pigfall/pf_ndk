use pf_ndk_glue::app::AndroidApp;
use std::sync::{Arc,Mutex,Condvar};

#[cfg_attr(target_os = "android", pf_ndk_glue::main())]
pub fn run(arc_app_cond:Arc<(Mutex<AndroidApp>,Condvar)>) {
    println!("hello world");
}
