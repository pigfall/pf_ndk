#[cfg_attr(target_os = "android", pf_ndk_glue::main())]
pub fn main() {
    println!("hello world");
}
