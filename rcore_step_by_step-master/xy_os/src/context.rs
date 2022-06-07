use riscv::register::{
    sstatus::Sstatus,
    scause::Scause,
};

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32], // General registers
    pub sstatus: Sstatus, // Supervisor Status Register
    pub sepc: usize, // Supervisor exception program counter
    pub stval: usize, // Supervisor trap value
    pub scause: Scause, // Scause register: record the cause of exception/interrupt/trap
}

#[repr(C)]
pub struct Context {
    content_addr: usize // 上下文内容存储的位置
}

impl Context {
    pub unsafe fn null() -> Context {
        Context { content_addr: 0 }
    }

    pub unsafe fn new_kernel_thread(
        entry: extern "C" fn(usize) -> !,
        arg: usize,
        kstack_top: usize,
        satp: usize ) -> Context {
        ContextContent::new_kernel_thread(entry, arg, kstack_top, satp).push_at(kstack_top)
    }

    pub unsafe fn new_user_thread(
        entry: usize,
        ustack_top : usize,
        kstack_top : usize,
        satp : usize
    ) -> Self {
        ContextContent::new_user_thread(entry, ustack_top, satp).push_at(kstack_top)
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch(&mut self, target: &mut Context) {
        // asm!(include_str!("process/switch.asm") :::: "volatile");
        asm!(
            r"
        .equ XLENB, 4
        .macro Load reg, mem
            lw \reg, \mem
        .endm
        .macro Store reg, mem
            sw \reg, \mem
        .endm"
        );
        // 请注意下面汇编中对a0以及a1中的值的使用和处理。这表明switch函数的调用将会改变它的参数指向的内存中存储的数据。
        asm!("
        // save from's registers
        addi  sp, sp, (-XLENB*14)
        Store sp, 0(a0)
        Store ra, 0*XLENB(sp)
        Store s0, 2*XLENB(sp)
        Store s1, 3*XLENB(sp)
        Store s2, 4*XLENB(sp)
        Store s3, 5*XLENB(sp)
        Store s4, 6*XLENB(sp)
        Store s5, 7*XLENB(sp)
        Store s6, 8*XLENB(sp)
        Store s7, 9*XLENB(sp)
        Store s8, 10*XLENB(sp)
        Store s9, 11*XLENB(sp)
        Store s10, 12*XLENB(sp)
        Store s11, 13*XLENB(sp)
        csrr  s11, satp
        Store s11, 1*XLENB(sp)

        // restore to's registers
        Load sp, 0(a1)
        Load s11, 1*XLENB(sp)
        csrw satp, s11
        Load ra, 0*XLENB(sp)
        Load s0, 2*XLENB(sp)
        Load s1, 3*XLENB(sp)
        Load s2, 4*XLENB(sp)
        Load s3, 5*XLENB(sp)
        Load s4, 6*XLENB(sp)
        Load s5, 7*XLENB(sp)
        Load s6, 8*XLENB(sp)
        Load s7, 9*XLENB(sp)
        Load s8, 10*XLENB(sp)
        Load s9, 11*XLENB(sp)
        Load s10, 12*XLENB(sp)
        Load s11, 13*XLENB(sp)
        addi sp, sp, (XLENB*14)

        Store zero, 0(a1)
        ret"
        : : : : "volatile" )
    }
}

#[repr(C)]
struct ContextContent {
    ra: usize, // 返回地址
    satp: usize, //　二级页表所在位置
    s: [usize; 12], // 被调用者保存的寄存器
    tf: TrapFrame,
}

extern "C" {
    fn __trapret();
}

use core::mem::zeroed;
use riscv::register::sstatus;
impl ContextContent {
    fn new_kernel_thread(entry: extern "C" fn(usize) -> !, arg: usize , kstack_top: usize, satp: usize) -> ContextContent {
        let mut content: ContextContent = unsafe { zeroed() };
        content.ra = entry as usize;
        content.satp = satp;
        content.s[0] = arg;
        let mut _sstatus = sstatus::read();
        _sstatus.set_spp(sstatus::SPP::Supervisor); // 代表 sret 之后的特权级仍为 Ｓ
        content.s[1] = _sstatus.bits();
        content
    }

    fn new_user_thread(entry : usize, ustack_top : usize, satp : usize) -> Self {
        ContextContent{
            ra: __trapret as usize,
            satp,
            s: [0;12],
            tf: {
                let mut tf: TrapFrame = unsafe { zeroed() };
                tf.x[2] = ustack_top;   // 栈顶 sp
                tf.sepc = entry;   // sepc 在调用 sret 之后将被被赋值给 PC
                tf.sstatus = sstatus::read();
                tf.sstatus.set_spie(true);
                tf.sstatus.set_sie(false);
                tf.sstatus.set_spp(sstatus::SPP::User);   // 代表 sret 之后的特权级为U
                tf
            },
        }
    }

    unsafe fn push_at(self, stack_top: usize) -> Context {
        let ptr = (stack_top as *mut ContextContent).sub(1);
        *ptr = self; // 拷贝 ContextContent
        Context { content_addr: ptr as usize }
    }
}
