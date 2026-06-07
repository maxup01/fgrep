#!/usr/bin/env python3
"""Build fgrep in release mode, install it, and add it to PATH (macOS/Linux/Windows)."""

import os
import platform
import shutil
import subprocess
from pathlib import Path

IS_WINDOWS = platform.system() == "Windows"

# Build
subprocess.run(["cargo", "build", "--release"], check=True)

# Install
dest_dir = (Path(os.environ["USERPROFILE"]) if IS_WINDOWS else Path.home()) / ".local" / "bin"
dest_dir.mkdir(parents=True, exist_ok=True)
exe = "fgrep.exe" if IS_WINDOWS else "fgrep"
dest = dest_dir / exe
shutil.copy2(f"target/release/{exe}", dest)
if not IS_WINDOWS:
    os.chmod(dest, 0o755)
print(f"Installed -> {dest}")

# Already on PATH?
on_path = str(dest_dir) in (os.path.normpath(p) for p in os.environ.get("PATH", "").split(os.pathsep) if p)
if on_path:
    print("Already on PATH. Done!")
    raise SystemExit

# Add to PATH
if IS_WINDOWS:
    cur = os.environ.get("PATH", "")
    subprocess.run(["setx", "PATH", f"{dest_dir};{cur}"], check=False)
    print("Added to user PATH (via setx). Open a NEW terminal to use fgrep.")
else:
    shell = os.environ.get("SHELL", "")
    if "fish" in shell:
        rc = Path.home() / ".config" / "fish" / "config.fish"
        rc.parent.mkdir(parents=True, exist_ok=True)
        line = f'\n# Added by install_fgrep.py\nset -gx PATH {dest_dir} $PATH\n'
    elif "zsh" in shell:
        rc = Path.home() / ".zshrc"
        line = f'\n# Added by install_fgrep.py\nexport PATH="{dest_dir}:$PATH"\n'
    else:
        rc = Path.home() / ".bashrc"
        line = f'\n# Added by install_fgrep.py\nexport PATH="{dest_dir}:$PATH"\n'

    existing = rc.read_text() if rc.is_file() else ""
    if str(dest_dir) not in existing:
        with open(rc, "a") as f:
            f.write(line)
        print(f"Added to PATH in {rc}")
    else:
        print(f"{rc} already references {dest_dir}")
    print(f"Run:  source {rc}   (or open a new terminal) to apply.")
