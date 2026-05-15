use std::fs;

fn main() {
    const RELATIVE_PATH_TO_EBPF_RELEASE: &str = "../ebpf/target/bpfel-unknown-none/release/ebpf";
    const RELATIVE_PATH_TO_EBPF_DEBUG: &str = "../ebpf/target/bpfel-unknown-none/debug/ebpf";

    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get directory containing manifest");

    let absolute_path_to_ebpf_release =
        format!("{}/{}", manifest_dir, RELATIVE_PATH_TO_EBPF_RELEASE);
    let absolute_path_to_ebpf_debug = format!("{}/{}", manifest_dir, RELATIVE_PATH_TO_EBPF_DEBUG);

    if fs::exists(absolute_path_to_ebpf_release.as_str())
        .expect("Failed during file existence check")
    {
        println!(
            "cargo:rustc-env=EBPF_PATH={}",
            absolute_path_to_ebpf_release
        );
    } else if fs::exists(absolute_path_to_ebpf_debug.as_str())
        .expect("Failed during file existence check")
    {
        println!("cargo:rustc-env=EBPF_PATH={}", absolute_path_to_ebpf_debug)
    } else {
        panic!("Ebpf program not found at certain locations");
    }
}
