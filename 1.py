import os

# os.system("cd basic_rt && rustup target add riscv64gc-unknown-none-elf")
# os.system("cd basic_rt && cargo install cargo-binutils --vers ~0.2")
# os.system("cd basic_rt && rustup component add llvm-tools-preview")
# os.system("cd basic_rt && rustup component add rust-src")
os.system("cd user && cargo clean")
os.system("cd user && cargo build --release")


os.system('cd basic_rt && cargo clean')
os.system('cd basic_rt && cargo build')

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
# -device loader,file=basic_rt/target/riscv64gc-unknown-none-elf/release/basic_rt.bin,addr=0x86000000 \