# run 
python ./lkm.py

# usage
[doc.md](./doc.md)


# 运行演示

```rust
[rustsbi] RustSBI version 0.2.0-alpha.1
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Platform: QEMU (Version 0.2.0)
[rustsbi] misa: RV64ACDFIMSU
[rustsbi] mideleg: 0x222
[rustsbi] medeleg: 0xb1ab
[rustsbi-dtb] Hart count: cluster0 with 4 cores
[rustsbi] Kernel entry: 0x80200000
[kernel] Hello, world!
last 11677 Physical Frames.
.text [0x80200000, 0x8021a000)
.rodata [0x8021a000, 0x80221000)
.data [0x80221000, 0x80222000)
.bss [0x80222000, 0x85263000)
ekernel  MEMORY_END [0x85263000, 0x88000000)
mapping .text section
mapping .rodata section
mapping .data section
mapping .bss section
mapping physical memory
start: 0x87000000 end: 0x87800000
mapping memory-mapped registers
remap_test passed!
loader list app
/**** APPS ****
1
2
3
4
initproc
usertests
user_shell
basic_rt
**************/
trying to add add user test
[hart 0] Start 1
[hart 0] Start 2
[hart 0] Start 3
[hart 0]Hello
[hart 1]init done satp: 0x8000000000085263
[hart 2]init done satp: 0x8000000000085263
[hart 3]init done satp: 0x8000000000085263
[hart 1]Hello
[hart 3]Hello        
[hart 2]Hello        
[hart 3]run user task
[hart 1]run user task
[hart 2]run user task
run_tasks
run_tasks
run_tasks
[hart 0]run user task
task pid: 1 running  
task pid: 2 running  
task pid: 0 running  
run_tasks
[hart 3]switching    
[hart 1]switching
task pid: 3 running
[hart 2]switching
[hart 0]switching
[user1] Hello world from user mode program!
[user2] Hello world from user mode program!
[user3] Hello world from user mode program!
[user4] Hello world from user mode program!
init_environment at 0x8600053e
init_environment at 0x8600053e
init_environment at 0x8600053e
init_environment at 0x8600053e
init_cpu at 0x86012c2e
init_cpu at 0x86012c2e
init_cpu at 0x86012c2e
init_cpu at 0x86012c2e
cpu_run at 0x86012d9e
cpu_run at 0x86012d9e
cpu_run at 0x86012d9e
add_user_task at 0x86011d78
init_environment
init_cpu
init_cpu_test
add thread_main done
test task addr :0x544
add_task

>>>> will switch_to thread 0 in idle_main!
[hart 0] [user4] 666
[hart 0] [user4] 9
[hart 0] [user4] 8
[hart 0] [user4] 7
[hart 0] [user4] 6
[hart 0] [user4] 5
[hart 0] [user4] 4
[hart 0] [user4] 3
[hart 0] [user4] 2
[hart 0] [user4] 1
[hart 0] [user4] 0
cpu_run at 0x86012d9e
add_user_task at 0x86011d78
init_environment
init_cpu
init_cpu_test
add thread_main done
test task addr :0x5c6
add_task

>>>> will switch_to thread 0 in idle_main!
[hart 3] [user2] 666
[hart 3] [user2] 9
[hart 3] [user2] 8
[hart 3] [user2] 7
[hart 3] [user2] 6
[hart 3] [user2] 5
[hart 3] [user2] 4
[hart 3] [user2] 3
[hart 3] [user2] 2
[hart 3] [user2] 1
[hart 3] [user2] 0
add_user_task at 0x86011d78
init_environment
init_cpu
init_cpu_test
add thread_main done
test task addr :0x5c6
add_task

>>>> will switch_to thread 0 in idle_main!
[hart 1] [user1] 666
[hart 1] [user1] 9
[hart 1] [user1] 8
[hart 1] [user1] 7
[hart 1] [user1] 6
[hart 1] [user1] 5
[hart 1] [user1] 4
[hart 1] [user1] 3
[hart 1] [user1] 2
[hart 1] [user1] 1
[hart 1] [user1] 0
add_user_task at 0x86011d78
init_environment
init_cpu
init_cpu_test
add thread_main done
test task addr :0x5c6
add_task

>>>> will switch_to thread 0 in idle_main!
[hart 2] [user3] 666
[hart 2] [user3] 9
[hart 2] [user3] 8
[hart 2] [user3] 7
[hart 2] [user3] 6
[hart 2] [user3] 5
[hart 2] [user3] 4
[hart 2] [user3] 3
[hart 2] [user3] 2
[hart 2] [user3] 1
[hart 2] [user3] 0

```