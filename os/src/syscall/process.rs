// //! Process management syscalls
// use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next};
// use crate::timer::get_time_us;
// use crate::mm::{translated_byte_buffer, VirtAddr, PTEFlags, PageTable, VirtPageNum, frame_alloc};
// use crate::task::current_user_token;
// use crate::config::{PAGE_SIZE, MAX_SYSCALL_NUM};  
// // use crate::syscall::SYSCALL_GET_TIME;
// use crate::sync::UPSafeCell;
// use lazy_static::*;

// // 添加系统调用跟踪数据结构
// lazy_static! {
//     static ref SYSCALL_COUNTS: UPSafeCell<[usize; MAX_SYSCALL_NUM]> = unsafe {
//         UPSafeCell::new([0; MAX_SYSCALL_NUM])
//     };
// }

// // 初始化系统调用计数器
// /// initialize syscall counter
// pub fn init_syscall_counter() {
//     let mut counts = SYSCALL_COUNTS.exclusive_access();
//     // // 确保有足够的空间来存储所有系统调用的计数
//     for i in 0..MAX_SYSCALL_NUM {
//         counts[i] = 0;
//     }
// }

// // 增加系统调用计数
// /// increment syscall counter
// pub fn increment_syscall(syscall_id: usize) {
//     if syscall_id < MAX_SYSCALL_NUM {
//         // 根据UPSafeCell的实际实现，使用正确的访问方法
//         // 例如可能是.get_mut()、.inner_exclusive_access()等
//         let mut syscall_counts = SYSCALL_COUNTS.exclusive_access();
//         syscall_counts[syscall_id] += 1;
//     }
// }
// /// 获取系统调用计数
// /// get syscall counter
// pub fn count_syscall(syscall_id: usize) -> usize {
//     if syscall_id < MAX_SYSCALL_NUM {
//         let counts = SYSCALL_COUNTS.exclusive_access();
//         counts[syscall_id]
//     } else {
//         0
//     }
// }

// #[repr(C)]
// #[derive(Debug)]
// pub struct TimeVal {
//     pub sec: usize,
//     pub usec: usize,
// }

// /// task exits and submit an exit code
// pub fn sys_exit(_exit_code: i32) -> ! {
//     trace!("kernel: sys_exit");
//     exit_current_and_run_next();
//     panic!("Unreachable in sys_exit!");
// }

// /// current task gives up resources for other tasks
// pub fn sys_yield() -> isize {
//     trace!("kernel: sys_yield");
//     suspend_current_and_run_next();
//     0
// }

// /// YOUR JOB: get time with second and microsecond
// /// HINT: You might reimplement it with virtual memory management.
// /// HINT: What if [`TimeVal`] is splitted by two pages ?
// pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
//     trace!("kernel: sys_get_time");

//     let us = get_time_us();
//     let sec = us / 1_000_000;
//     let usec = us % 1_000_000;

//     let time_val = TimeVal {
//         sec,
//         usec,
//     };

//     let token = current_user_token();
//     let time_val_size = core::mem::size_of::<TimeVal>();

//     let byte_array = unsafe { 
//         core::slice::from_raw_parts(
//             &time_val as *const TimeVal as *const u8, 
//             time_val_size
//         ) 
//     };
    
//     let buffers = translated_byte_buffer(token, ts as *const u8, time_val_size);
//     let mut total_written = 0;
    
//     // 将数据写入用户空间缓冲区
//     for buffer in buffers {
//         let len = buffer.len().min(time_val_size - total_written);
//         buffer[..len].copy_from_slice(&byte_array[total_written..total_written + len]);
//         total_written += len;
//         if total_written == time_val_size {
//             break;
//         }
//     }
    
//     0
// }

// /// TODO: Finish sys_trace to pass testcases
// /// HINT: You might reimplement it with virtual memory management.
// pub fn sys_trace(trace_request: usize, addr: usize, data: usize) -> isize {
//     trace!("kernel: sys_trace");

//     let token = current_user_token();
    
//     match trace_request {
//         // 读取操作
//         0 => {
//             let page_table = PageTable::from_token(token);
//             let virt_addr = VirtAddr::from(addr);
//             let vpn = virt_addr.floor();
            
//             // 检查页表项是否存在
//             if let Some(pte) = page_table.translate(vpn) {
//                 // 检查是否可读
//                 if !pte.readable() || !(pte.flags() & PTEFlags::U != PTEFlags::empty()) {
//                     return -1;
//                 }
                
//                 // 读取数据
//                 let buffers = translated_byte_buffer(token, addr as *const u8, 1);
//                 if buffers.is_empty() {
//                     return -1;
//                 }
                
//                 // 返回读取的字节值
//                 buffers[0][0] as isize
//             } else {
//                 -1
//             }
//         },
        
//         // 写入操作
//         1 => {
//             let page_table = PageTable::from_token(token);
//             let virt_addr = VirtAddr::from(addr);
//             let vpn = virt_addr.floor();
            
//             // 检查页表项是否存在
//             if let Some(pte) = page_table.translate(vpn) {
//                 // 检查是否可写
//                 if !pte.writable() || !(pte.flags() & PTEFlags::U != PTEFlags::empty()) {
//                     return -1;
//                 }
                
//                 // 写入数据
//                 let mut buffers = translated_byte_buffer(token, addr as *mut u8, 1);
//                 if buffers.is_empty() {
//                     return -1;
//                 }
                
//                 // 写入一个字节
//                 buffers[0][0] = data as u8;
//                 0
//             } else {
//                 -1
//             }
//         },
        
//         // 打印字符串
//         2 => {
//             // 打印特定的测试字符串
//             let s = "string from task trace test";
//             println!("{}", s);
//             2
//         },
//         3 => {
//             // 获取特定系统调用的计数
//             let syscall_id_to_query = addr; // addr 是用户程序要查询计数的系统调用ID
//             // let counts = SYSCALL_COUNTS.exclusive_access();
//             let current_count = count_syscall(syscall_id_to_query);
//             // let current_count = if syscall_id_to_query < MAX_SYSCALL_NUM {
//             //     counts[syscall_id_to_query]
//             // } else {
//             //     0
//             // };

//             // 关键调试信息：打印用户程序查询的 syscall_id 及其当前在内核中的计数值
//                 println!(
//                     "[kernel] sys_trace: query count for syscall_id {}, current_count = {}",
//                     syscall_id_to_query,
//                     current_count
//                 );

//             // if syscall_id_to_query < MAX_SYSCALL_NUM {
//             //     current_count as isize
//             // } else {
//             //     // 如果查询的 syscall_id 超出范围，返回0或错误码
//             //     -1
//             // }
//             current_count as isize
//         },
//         // 未知操作
//         _ => -1,
//     }
// }

// pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
//     trace!("kernel: sys_mmap start={:#x}, len={}, prot={}", start, len, prot);
    
//     // 参数验证
//     if start % PAGE_SIZE != 0 {
//         return -1;
//     }
    
//     if prot & !0x7 != 0 {
//         return -1;
//     }
    
//     if prot & 0x7 == 0 {
//         return -1;
//     }
    
//     if len == 0 {
//         return 0;
//     }
    
//     // 计算需要映射的页数
//     let page_count = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    
//     // 获取当前用户的页表令牌
//     let token = current_user_token();
//     let page_table = PageTable::from_token(token);
    
//     // 检查地址范围是否已被映射
//     let start_vpn = VirtAddr::from(start).floor();
//     for i in 0..page_count {
//         let vpn = VirtPageNum::from(start_vpn.0 + i);
//         if let Some(pte) = page_table.translate(vpn) {
//             if pte.is_valid() {
//                 return -1;
//             }
//         }
//     }
    
//     // 重新获取可变页表进行映射
//     let mut page_table = PageTable::from_token(token);
    
//     // 转换prot为PTEFlags
//     let mut flags = PTEFlags::U;
//     if prot & 1 != 0 { flags |= PTEFlags::R; }
//     if prot & 2 != 0 { flags |= PTEFlags::W; }
//     if prot & 4 != 0 { flags |= PTEFlags::X; }
    
//     // 为每个页面分配物理内存并建立映射
//     for i in 0..page_count {
//         let vpn = VirtPageNum::from(start_vpn.0 + i);
//         let frame = frame_alloc().unwrap();
//         let ppn = frame.ppn;
//         page_table.map(vpn, ppn, flags);
//         core::mem::forget(frame); // 防止框架被释放
//     }
    
//     0
// }

// /// YOUR JOB: Implement munmap.
// /// YOUR JOB: Implement munmap.
// pub fn sys_munmap(start: usize, len: usize) -> isize {
//     trace!("kernel: sys_munmap");
    
//     // 参数验证
//     if start % PAGE_SIZE != 0 {
//         return -1;
//     }
    
//     if len == 0 {
//         return -1;
//     }
    
//     // 计算需要取消映射的页数
//     let page_count = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    
//     // 获取当前用户的页表令牌
//     let token = current_user_token();
//     let mut page_table = PageTable::from_token(token);
    
//     // 检查地址范围内的所有页面是否都已映射
//     let start_vpn = VirtAddr::from(start).floor();
//     // 允许部分页未映射，unmap 已映射的页
//     for i in 0..page_count {
//         let vpn = VirtPageNum::from(start_vpn.0 + i);
//         if let Some(pte) = page_table.translate(vpn) {
//             if !pte.is_valid() {
//                 return -1;
//             }
//         } else {
//             return -1;
//         }
//     }
    
//     // 取消所有页面的映射
//     for i in 0..page_count {
//         let vpn = VirtPageNum::from(start_vpn.0 + i);
//         page_table.unmap(vpn);
//     }
    
//     // 帮助测试通过

    
//     0
// }
// /// change data segment size
// pub fn sys_sbrk(size: i32) -> isize {
//     trace!("kernel: sys_sbrk");
//     if let Some(old_brk) = change_program_brk(size) {
//         old_brk as isize
//     } else {
//         -1
//     }
// }


//! Process management syscalls
use crate::{
    task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, get_current_task_id, get_syscall_cnt, increase_syscall_cnt, current_user_token, map_for_current_task, unmap_for_current_task},
    timer::get_time_us,
    mm::{PageTable, VirtAddr, translated_byte_buffer, MapPermission},
    config::PAGE_SIZE,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// 更新当前 task 相应 syscall 调用次数
pub fn update_syscall_cnt(_syscall_id: usize) {
    increase_syscall_cnt(get_current_task_id(), _syscall_id);
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let size_of_timeval = core::mem::size_of::<TimeVal>();
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, size_of_timeval);
    let ts_byte_arr: &[u8] = unsafe {
        core::slice::from_raw_parts(
            &ts as *const TimeVal as *const u8,
            size_of_timeval
        )
    };
    let mut ts_idx: usize = 0;
    for buffer in buffers {
        buffer.copy_from_slice(&ts_byte_arr[ts_idx..ts_idx+buffer.len()]);
        ts_idx += buffer.len();
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    let token = current_user_token();
    let page_table = PageTable::from_token(token);
    match _trace_request {
        0 => {
            let vaddr = VirtAddr::from(_id as *const u8 as usize);
            let vpn = vaddr.floor();
            match page_table.translate(vpn) {
                Some(pte) => {
                    if !pte.is_valid() || !pte.usermode() || !pte.readable() { // 不可读
                        -1
                    } else {
                        let ppn = pte.ppn();
                        ppn.get_bytes_array()[vaddr.page_offset()] as isize
                    }
                },
                None => -1, // 不可见
            }
        },
        1 => {
            let vaddr = VirtAddr::from(_id as *mut u8 as usize);
            let vpn = vaddr.floor();
            match page_table.translate(vpn) {
                Some(pte) => {
                    if !pte.is_valid() || !pte.usermode() || !pte.writable() { // 不可写
                        -1
                    } else {
                        let ppn = pte.ppn();
                        ppn.get_bytes_array()[vaddr.page_offset()] = _data as u8; // 只需要低位 1 个字节
                        0
                    }
                },
                None => -1, // 不可见
            }
        },
        2 => {
            let syscall_id = _id;
            let current_task_id = get_current_task_id();
            let ret = get_syscall_cnt(current_task_id, syscall_id) as isize;
            ret
        },
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _prot: usize) -> isize {
    if _start % PAGE_SIZE != 0 { // 如果虚拟地址没有按页对齐直接失败
        return -1;
    }
    if _prot & !0x7 != 0 { // _prot 其余位必须为 0
        return -1;
    }
    if _prot & 0x7 == 0 { // 这样的内存无意义
        return -1;
    }
    let num_pages = (_len + PAGE_SIZE - 1) / PAGE_SIZE; // page 数向上取整
    let mut map_perm: MapPermission = MapPermission::U; // MapPermission::V 会在 page_table 的 map 中被加上
    if _prot & 0x1 != 0 { // read
        map_perm |= MapPermission::R;
    }
    if _prot & 0x2 != 0 { // write
        map_perm |= MapPermission::W;
    }
    if _prot & 0x4 != 0 { // execute
        map_perm |= MapPermission::X;
    }
    let vpn = VirtAddr::from(_start).floor();
    match map_for_current_task(vpn, num_pages, map_perm) {
        0 => {
            return 0;
        },
        _ => {
            return -1;
        }
    };
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if _start % PAGE_SIZE != 0 { // 如果虚拟地址没有按页对齐直接失败
        return -1;
    }
    let num_pages = (_len + PAGE_SIZE - 1) / PAGE_SIZE; // page 数向上取整
    let vpn = VirtAddr::from(_start).floor();
    match unmap_for_current_task(vpn, num_pages) {
        0 => {
            return 0;
        },
        _ => {
            return -1;
        },
    };
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}