use std::usize;

const STACK_SIZE: usize = 1024;
const NPROC: usize = 9;

#[derive(Copy, Clone)]
enum ProcessStatus {
    FREE,
    READY,
    SLEEP,
    BLOCK,
    ZOMBIE
}

#[derive(Clone)]
struct Process {
    saved_sp     : u32,
    pid          : u32,
    ppid         : u32,
    status       : ProcessStatus,
    priority     : u32,
    event        : u32,
    exit_code    : u32,
    next         : Option<Box<Process>>,
    stack        : [u32; STACK_SIZE]
}

impl Process {
    fn new() -> Process {
        Process {
            saved_sp: 0, pid: 0, ppid: 0,
            status: ProcessStatus::FREE, priority: 1, event: 0, exit_code: 0,
            next: None, stack: [0; STACK_SIZE]
        }
    }
}

fn main() {
    let process = [Process::new(); NPROC];

    println!("Hello, world!");
}
