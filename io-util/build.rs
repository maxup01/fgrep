use std::fs;

fn main() {
    const PATH_TO_EBPF_RELEASE: &str = "../ebpf/target/bpfel-unknown-none/release/ebpf";
    const PATH_TO_EBPF_DEBUG: &str = "../ebpf/target/bpfel-unknown-none/debug/ebpf";

    if fs::exists(PATH_TO_EBPF_RELEASE).expect("Failed during file existence check") {
        println!("cargo:rustc-env=EBPF_PATH={}", PATH_TO_EBPF_RELEASE);
    } else if fs::exists(PATH_TO_EBPF_DEBUG).expect("Failed during file existence check") {
        println!("cargo:rustc-env=EBPF_PATH={}", PATH_TO_EBPF_DEBUG)
    } else {
        panic!("Ebpf program not found at certain locations");
    }
}
