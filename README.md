# run 
python ./lkm.py

# usage
[doc.md](./doc.md)


# requirements

qemu 5.2 
```shell
# 安装编译所需的依赖包
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 python3-pip

# 可能还需安装ninja 过程略

# 下载源码包
wget https://download.qemu.org/qemu-5.2.0.tar.xz

# 编译安装并配置 RISC-V 支持
cd qemu-5.2.0
./configure --target-list=riscv64-softmmu,riscv64-linux-user
make -j$(nproc)

# 请注意，qemu-5.2.0 的父目录可以随着你的实际安装位置灵活调整
export PATH=$PATH:$HOME/qemu-5.2.0/build
export PATH=$PATH:$HOME/qemu-5.2.0/build/riscv64-softmmu
export PATH=$PATH:$HOME/qemu-5.2.0/build/riscv64-linux-user
```



# 运行演示
在4核cpu上 .8个用户进程, 每个用户进程下有多个协程运行演示,每个协程输出自身编号

因为时钟中断的原因,中间输出的信息有点乱.


```c
[rustsbi] RustSBI version 0.2.0-alpha.9
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Implementation: RustSBI-QEMU Version 0.0.2
[rustsbi-dtb] Hart count: cluster0 with 4 cores
[rustsbi] misa: RV64ACDFIMSU
[rustsbi] mideleg: ssoft, stimer, sext (0x222)
[rustsbi] medeleg: ima, ia, la, sa, uecall, ipage, lpage, spage (0xb1a3)
[rustsbi] enter supervisor 0x80200000
last 15773 Physical Frames.
.text [0x80200000, 0x8021a000)
.rodata [0x8021a000, 0x80221000)
.data [0x80221000, 0x80222000)
.bss [0x80222000, 0x84263000)
ekernel  MEMORY_END [0x84263000, 0x88000000)
mapping .text section
mapping .rodata section
mapping .data section
mapping .bss section
mapping physical memory
start: 0x87000000 end: 0x87800000
mapping memory-mapped registers
[kernel] Hello, world!
remap_test passed!
loader list app
/**** APPS ****
1
2
3
4
5
6
7
8
initproc
user_shell
basic_rt
**************/
trying to add user test
[hart 0] Start 1
[hart 0] Start 2
[hart 0] Start 3
[hart 0]Hello
[hart 3]init done satp: 0x8000000000084263
[hart 1]init done satp: 0x8000000000084263
[hart 2]init done satp: 0x8000000000084263
[hart 1]Hello
[hart 2]Hello
[hart 1]run user task
[hart 3]Hello
[hart 2]run user task
[hart 3]run user task
run_tasks
[hart 0]run user task
run_tasks
run_tasks
run_tasks
[user6] Hello world from user mode program!
[user5] Hello world from user mode program!
[user4] Hello world from user mode program!
[user8] Hello world from user mode program!
[user2] Hello world from user mode program!
[user3] Hello world from user mode program!
[user1] Hello world from user mode program!
[user7] Hello world from user mode program!

>>>> will switch_to thread 0 in idle_main!
thread_main-------------
thread_main running, no task:
>>>> will switch_to thread 0 in idle_main!
thread_main-------------
thread_main running, no task: false
[hart 3] [user6] 666

>>>> will switch_to thread 0 in idle_main!
thread_main-------------
thread_main running, no task: false
[hart false
[hart 3] [user4] 666
thread_main running, no task: false
[hart 3] [user4] 9
thread_main running, no task: false
[hart 3] [user4] 8
thread_main running, no task:
>>>> will switch_to thread 0 in idle_main!
thread_main-------------
thread_main running, no task: false

>>>> will switch_to thread 0 in idle_main!
thread_main-------------
thread_main running, no task: false
[hart 3] [user2] 666thread_main running, no task: false
[hart 3] [user6] 9
thread_main running, no task: false
[hart 3] [user6] 8
thread_main running, no task: false
[hart 3] [user6] 7
thread_main running, no task: false3] [user5] 666
thread_main running, no task: false
[hart 3] [user5] 9
thread_main running, no task: false
[hart 3] [user5] 8
thread_main running, no task: false
[hart 3] [user5] false
[hart 3] [user4] 7
thread_main running, no task: false
[hart 3] [user4] 6
thread_main running, no task: false
[hart 3] [user4] 5
thread_main running, no task: false
[hart 3] [user4] 4
thread_main running, no task: [hart 3] [user8] 666
thread_main running, no task: false
[hart 3] [user8] 9
thread_main running, no task: false
[hart 3] [user8] 8

thread_main running, no task: false
[hart 3] [user2] 9
thread_main running, no task: false
[hart 3] [user2] 8
thread_main running, no task:
[hart 3] [user6] 6
thread_main running, no task: false
[hart 3] [user6] 5
thread_main running, no task: false
[hart 3] [user6] 4
thread_main running, no task: false
[hart 3] [user6] 3
thread_main running, no task: false7
thread_main running, no task: false
[hart 3] [user5] 6
thread_main running, no task: false
[hart 3] [user5] 5
thread_main running, no task: false
[hart 3] [user5] 4
thread_main running, no task: false
[hart false
[hart 3] [user4] 3
thread_main running, no task: false
[hart 3] [user4] 2
thread_main running, no task: false
[hart 3] [user4] 1
thread_main running, no task: false
[hart 3] [user4] 0
thread_main running, no task: false
[hart 3] [user8] 7
thread_main running, no task: false
[hart 3] [user8] 6
thread_main running, no task: false
[hart 3] [user8] 5
thread_main running, no task: false
[hart 3] [user8] 4
false
[hart 3] [user2] 7
thread_main running, no task: false
[hart 3] [user2] 6
thread_main running, no task: false
[hart 3] [user2] 5
thread_main running, no task: false
[hart 3] [user2] 4
thread_main running, no task: false
[hart 3] [user6] 2
thread_main running, no task: false
[hart 3] [user6] 1
thread_main running, no task: false
[hart 3] [user6] 0
thread_main running, no task: true
no task
user exit
3] [user5] 3
thread_main running, no task: false
[hart 2] [user5] 2
thread_main running, no task: false
[hart 2] [user5] 1
thread_main running, no task: false
[hart 2] [user5] 0
thread_main running, no task: true
no task
thread_main running, no task: true
no task
user exit
thread_main running, no task: false
[hart 0] [user8] 3
thread_main running, no task: false
[hart 0] [user8] 2
thread_main running, no task: false
[hart 0] [user8] 1
thread_main running, no task: false
[hart 0] [user8] 0

[hart 0] [user2] 3
thread_main running, no task: false
[hart 0] [user2] 2
thread_main running, no task: false
[hart 0] [user2] 1
thread_main running, no task: false
[hart 0] [user2] 0
thread_main running, no task: trueuser exit
exit_current_and_run_next schedule
thread_main running, no task: true
no task
exit_current_and_run_next schedule
user exit
exit_current_and_run_next schedule
exit_current_and_run_next schedule

no task
user exit

>>>> will switch_to thread exit_current_and_run_next schedule
0 in idle_main!
thread_main-------------
thread_main running, no task: false
[hart 2] [user3] 666
thread_main running, no task: false
[hart 2] [user3] 9
thread_main running, no task: false
[hart 2] [user3]
>>>> will switch_to thread 8
0 in idle_main!
thread_main running, no task: thread_main-------------
false
[hart 2] [user3] 7thread_main running, no task: 
falsethread_main running, no task:
false
[hart 2] [user3] [hart 6
3thread_main running, no task: ] [user1] false666

[hart 2] [user3] 5
thread_main running, no task: thread_main running, no task: falsefalse

[hart [hart 23] [user3] ] [user1] 49

thread_main running, no task: thread_main running, no task: falsefalse

[hart [hart 03] [user1] ] [user3] 83

thread_main running, no task: thread_main running, no task: falsefalse

[hart [hart 23] [user3] ] [user1] 27

thread_main running, no task: thread_main running, no task: falsefalse

[hart [hart 32] [user3] ] [user1] 16

thread_main running, no task: thread_main running, no task: false
false[hart
0] [user3] [hart 03
] [user1] thread_main running, no task: 5
truethread_main running, no task:
falseno task

user exit
[hart 3] [user1] 4
thread_main running, no task: exit_current_and_run_next schedule
false
[hart 3] [user1] 3
thread_main running, no task: false
[hart 3] [user1] 2
thread_main running, no task: false
[hart 3] [user1] 1
thread_main running, no task: false
[hart 3] [user1] 0
thread_main running, no task: true
no task
user exit
exit_current_and_run_next schedule

>>>> will switch_to thread 0 in idle_main!
thread_main-------------
thread_main running, no task: false
[hart 2] [user7] 666
thread_main running, no task: false
[hart 2] [user7] 9
thread_main running, no task: false
[hart 2] [user7] 8
thread_main running, no task: false
[hart 2] [user7] 7
thread_main running, no task: false
[hart 2] [user7] 6
thread_main running, no task: false
[hart 2] [user7] 5
thread_main running, no task: false
[hart 2] [user7] 4
thread_main running, no task: false
[hart 2] [user7] 3
thread_main running, no task: false
[hart 2] [user7] 2
thread_main running, no task: false
[hart 2] [user7] 1
thread_main running, no task: false
[hart 2] [user7] 0
thread_main running, no task: true
no task
user exit
exit_current_and_run_next schedule

```