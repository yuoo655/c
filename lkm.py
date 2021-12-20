import os

os.system("cd tiny_kernel && rustup target add riscv64gc-unknown-none-elf")
os.system("cd tiny_kernel && cargo install cargo-binutils --vers ~0.2")
os.system("cd tiny_kernel && rustup component add llvm-tools-preview")
os.system("cd tiny_kernel && rustup component add rust-src")
# os.system("cd user && cargo clean")
os.system("cd user && cargo build --release")

# os.system("cd tiny_kernel && cargo clean")
# os.system("cd tiny_kernel && python build.py")

# os.system('cd tiny_kernel && cargo clean')
os.system('cd tiny_kernel && cargo build')

os.system("cd easy-fs-fuse && cargo run --release -- -s ../user/src/bin/ -t ../user/target/riscv64gc-unknown-none-elf/release/")

os.system("cd os && cargo clean")
os.system("cd os && cargo build --release")

os.system("qemu-system-riscv64 \
-machine virt \
-nographic \
-bios bootloader/rustsbi-qemu.bin \
-device loader,file=os/target/riscv64gc-unknown-none-elf/release/os,addr=0x80200000 \
-drive file=user/target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0 \
-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 ")
# -device loader,file=tiny_kernel/target/riscv64gc-unknown-none-elf/release/tiny_kernel.bin,addr=0x86000000 \