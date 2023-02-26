        .global  running, scheduler, tswitch
tswitch:
SAVE:   pushl %eax
        pushl %ebx
        pushl %ecx
        pushl %edx
        pushl %ebp
        pushl %esi
        pushl %edi
        pushfl
        movl  running,%ebx  # LOAD running          =: register ebx
        movl  %esp,4(%ebx)  # LOAD running.saved_sp =: (%ebx + 4)
FIND:   call  scheduler
RESUME: movl  running,%ebx  # LOAD running          =: register ebx
        movl  4(%ebx),%esp  # (%ebx + 4)        := running.saved_sp
        popfl
        popl  %edi
        popl  %esi
        popl  %ebp
        popl  %edx
        popl  %ecx
        popl  %ebx
        popl  %eax
        ret
# stack contents = |retPC|eax|ebx|ecx|edx|ebp|esi|edi|eflag|
#                    -1   -2  -3  -4  -5  -6  -7  -8   -9
