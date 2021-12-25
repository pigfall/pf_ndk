use proc_macro::{TokenStream};
use quote::{ ToTokens,quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(attr: TokenStream,item_input: TokenStream)->TokenStream{
    let item_ast = parse_macro_input!(item_input as ItemFn);
    let f_name = &item_ast.sig.ident;
    let tk_stream = quote!{
        pub fn expand_run(){
            println!("insert by macro");
            #f_name();
        }

        #[no_mangle]
        unsafe extern "C" fn ANativeActivity_onCreate(
            activity: *mut std::os::raw::c_void,
            saved_state: *mut std::os::raw::c_void,
            saved_state_size: usize,
            ) {
            use pf_ndk_glue::log::info;
            pf_ndk_glue::android_logger::init_once(
                pf_ndk_glue::android_logger::Config::default()
                .with_min_level(pf_ndk_glue::log::Level::Trace)
                .with_tag("mytag")
                );
            info!("ANativeActivity_onCreate");
        }

        #item_ast

        
    };
    tk_stream.into()
}
