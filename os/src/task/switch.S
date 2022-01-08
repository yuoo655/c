.altmacro
.macro SAVE_SN n
    sd s\n, (\n+1)*8(sp)
.endm
.macro LOAD_SN n
    ld s\n, (\n+1)*8(sp)
.endm
    .section .text
    .globl __switch
__switch:

    # __switch(
    #     current_task_cx_ptr: *mut TaskContext,
    #     next_task_cx_ptr: *const TaskContext
    # )
    

    # push TaskContext to current sp
    sd sp, 8(a0)
    # save ra 
    sd ra, 0(a0)

    #save s0-s11
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n + 1
    .endr


    # load TaskContext
    ld sp, 8(a1)
    # load a1
    ld ra, 0(a1)
    # load s0-s11
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n + 1
    .endr
    ret

