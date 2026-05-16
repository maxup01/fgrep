use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();
    let ebpf_dir = workspace_root.join("ebpf");

    println!("cargo:rerun-if-changed={}", ebpf_dir.join("src").display());

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = format!("{}/ebpf-target", out_dir);

    // Use rustup to invoke cargo with nightly, bypassing the inherited RUSTC
    let status = Command::new("rustup")
        .args([
            "run",
            "nightly",
            "cargo",
            "build",
            "--release",
            "--target-dir",
            &target_dir,
        ])
        .env_remove("RUSTC")
        .env_remove("RUSTC_WORKSPACE_WRAPPER")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env("PATH", env::var("PATH").unwrap())
        .current_dir(&ebpf_dir)
        .status()
        .expect("Failed to spawn rustup");

    assert!(status.success(), "eBPF build failed");

    let ebpf_path = format!("{}/bpfel-unknown-none/release/ebpf", target_dir);

    assert!(
        fs::exists(&ebpf_path).expect("Failed during file existence check"),
        "eBPF binary not found at: {}",
        ebpf_path
    );

    println!("cargo:rustc-env=EBPF_PATH={}", ebpf_path);
}
