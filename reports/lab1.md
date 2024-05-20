# 实现功能
修改syscall/mod.rs, process.rs和task/mod.rs, task.rs  
在 task/task.rs 实现结构体 TaskInfo 以及对应初始化函数，在 mod.rs 中实现 tasks 的初始化，以及对当前 task 的 get 和 set 操作，并提供 pub 函数供外部调用  
在 syscall/process.rs 中调用 get 操作，并且正确返回时间，在 mod.rs 中实现每次 syscall 的 times 加一操作。

# 问答题
### 一
程序出错行为：
ch2b_bad_address：向 0x0 写入无效数据 0
ch2b_bad_instructions：执行 sret 指令
ch2b_bad_register：读取 sstatus 寄存器
sbi 版本：
RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
### 二
1. 参考[系统调用](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/2application.html#term-call-syscall)或者 __alltraps 的`mv a0, sp`， a0 保存系统调用的返回值，当系统调用返回后， a0 作为输出寄存器保存系统调用的返回值。 __restore 用途：系统调用或者异常结束后，从保存在内核栈上的 Trap 上下文恢复寄存器，即s=>u。
2. 处理了`sstatus`,`sepc`和`sscratch`三个寄存器，参考[特权级切换相关的控制状态寄存器](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/4trap-handling.html#id4)，sstatus：SPP 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息，spec：当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址，sscratch：指向用户栈。
3. 跳过 x2(stack pointer) ：后面处理，见[L33-35](https://github.com/LearningOS/rCore-Tutorial-Code-2024S/blob/ch3/os/src/trap/trap.S#L33)，将 sscratch 的值读到寄存器 t2 并保存到内核栈上；跳过 x4(thread pointer) ：参考[用户栈与内核栈](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/4trap-handling.html#id7),除非我们手动出于一些特殊用途使用它，否则一般也不会被用到。
4. 该指令之后， sp 指向用户栈， sscratch 指向内核栈。
5. 参考[特权级切换的硬件控制机制](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/4trap-handling.html#trap-hw-mechanism)，发生状态切换在`sret`。当 CPU 完成 Trap 处理准备返回的时候，需要通过一条 S 特权级的特权指令 sret 来完成，这一条指令具体完成以下功能：CPU 会将当前的特权级按照 sstatus 的 SPP 字段设置为 U 或者 S ；CPU 会跳转到 sepc 寄存器指向的那条指令，然后继续执行。
6. 该指令之后， sp 指向内核栈， sscratch 指向用户栈。
7. 从 U 态进入 S 态在 trap.S 之前应该（我猜？）就已经完成了， __alltraps 只是完成设置寄存器等操作。

# 荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

[rCore-Tutorial-Book-v3 第二章：批处理系统](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/index.html)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。


# optional