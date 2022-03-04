1:build qemu

2:build qemu-net

build:
	cd os && cargo build
	cd os && cargo build --release

qemu:
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-smp cpus=4 \
	-bios bootloader/rustsbi-qemu.bin \
	-device loader,file=os/target/riscv64gc-unknown-none-elf/release/os,addr=0x80200000 \
	-device loader,file=basic_rt/target/riscv64gc-unknown-none-elf/debug/basic_rt.bin,addr=0x87000000 \
	-drive file=user/target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-device virtio-net-device,netdev=usernet \
	-netdev user,id=usernet,hostfwd=tcp::3333-:22

qemu-net:
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-smp cpus=4 \
	-bios bootloader/rustsbi-qemu.bin \
	-device loader,file=os/target/riscv64gc-unknown-none-elf/release/os,addr=0x80200000 \
	-device loader,file=basic_rt/target/riscv64gc-unknown-none-elf/debug/basic_rt.bin,addr=0x87000000 \
	-drive file=user/target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no \
	-device virtio-net-device,netdev=tap0,mac=EE:BB:AA:EE:AA:AA,bus=virtio-mmio-bus.1 \








xx:
	-netdev tap,id=tap,ifname=tap,script=no,downscript=no \
	-device virtio-net-device,netdev=tap,bus=virtio-mmio-bus.2




debug:build qemu-debug

qemu-debug:
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-smp cpus=4 \
	-bios bootloader/rustsbi-qemu.bin \
	-device loader,file=os/target/riscv64gc-unknown-none-elf/debug/os,addr=0x80200000 \
	-device loader,file=basic_rt/target/riscv64gc-unknown-none-elf/debug/basic_rt.bin,addr=0x87000000 \
	-drive file=user/target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0 \
	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no \
	-device virtio-net-device,netdev=tap0,mac=EE:BB:AA:EE:AA:AA,bus=virtio-mmio-bus.1 \
	-S \
	-s