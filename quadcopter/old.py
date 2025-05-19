#!/usr/bin/env python3
import subprocess
import time
import sys
import os
import signal

def run_probe_rs():
    """Run `probe-rs gdb` in the background and ensure it's running."""
    probe_rs_process = subprocess.Popen(
        ["probe-rs", "gdb", "--chip", "esp32c3"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    time.sleep(2)  # Give probe-rs time to start
    if probe_rs_process.poll() is not None:
        # If the process is already dead, it failed to start
        print("Error: probe-rs failed to start.")
        sys.exit(1)

    print("probe-rs is running...")
    return probe_rs_process

def run_gdb(debug_executable):
    """Run riscv32-esp-elf-gdb with the given debug executable."""
    gdb_command = [
        "riscv32-esp-elf-gdb", debug_executable, "--eval-command", "target remote localhost:1337", "--eval-command", "monitor reset",
    ]
    gdb_process = subprocess.Popen(gdb_command)
    gdb_process.wait()  # Wait for GDB to finish
    return gdb_process

def main():
    # Get the debug executable path (assuming the current crate is built and you know the path)
    debug_executable = "./target/riscv32imc-unknown-none-elf/debug/quadcopter"

    if not os.path.exists(debug_executable):
        print(f"Error: Executable {debug_executable} not found.")
        sys.exit(1)

    # Run probe-rs in the background
    probe_rs_process = run_probe_rs()

    # Run riscv32-esp-elf-gdb in the foreground
    try:
        run_gdb(debug_executable)
    except KeyboardInterrupt:
        print("GDB exited, closing probe-rs.")

    # Once GDB exits, terminate the probe-rs process
    print("Terminating probe-rs process.")
    probe_rs_process.terminate()  # Gracefully terminate probe-rs
    try:
        probe_rs_process.wait(timeout=5)  # Wait for probe-rs to exit
    except subprocess.TimeoutExpired:
        print("probe-rs did not terminate gracefully, killing the process.")
        probe_rs_process.kill()  # Force kill if it doesn't exit within timeout

if __name__ == "__main__":
    main()
