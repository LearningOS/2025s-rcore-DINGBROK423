//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

use crate::task::TASK_MANAGER;
use crate::task::SYSCALL_COUNTS;
// 实现系统调用
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0=> {
            let target_addr = _id as *const u8;
            let byte_value = unsafe { *target_addr };
            
            println!("trace request: 0, read value {} from address 0x{:x}", byte_value, _id);
            byte_value as isize
        }
        1=> {
            let target_addr = _id as *mut u8;  
            let byte_value = (_data & 0xff) as u8;
            unsafe{*target_addr = byte_value};
            println!("trace request: 1, wrote value {} to address 0x{:x}", byte_value, _id);
            0
        }
        2=> {
            let syscall_id = _id;
            let current_task = TASK_MANAGER.current_task();
            let counts = SYSCALL_COUNTS.exclusive_access();
            let call_count = counts[current_task][syscall_id];
            println!("trace request: 2, syscall {} called {} times by task {}", 
                     syscall_id, call_count, current_task);
            call_count as isize
        }
        _ => {
            println!("error: invalid trace request");
            -1
        }
    }
}
