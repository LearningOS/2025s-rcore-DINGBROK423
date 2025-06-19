DINGBROK423: chapter5练习
编程作业
关于之前的 syscall
你仍需要迁移上一章的 sys_get_time sys_mmap sys_munmap 以适应新的进程结构。不过，从本章节开始，不再要求维护 ``sys_trace`` 这一系统调用。

进程创建
大家一定好奇过为啥进程创建要用 fork + exec 这么一个奇怪的系统调用，就不能直接搞一个新进程吗？ 思而不学则殆，我们就来试一试！这章的编程练习请大家实现一个完全 DIY 的系统调用 spawn，用以创建一个新进程。

spawn 系统调用定义( 标准spawn看这里 )：

fn sys_spawn(path: *const u8) -> isize
syscall ID: 400

功能：新建子进程，使其执行目标程序。

说明：成功返回子进程id，否则返回 -1。

可能的错误：
无效的文件名。

小心

虽然测例很简单，但提醒读者 spawn 不必 像 fork 一样复制父进程的地址空间。

stride 调度算法
ch3 中我们实现的调度算法十分简单。现在我们要为我们的 os 实现一种带优先级的调度算法：stride 调度算法。

算法描述如下:

(1) 为每个进程设置一个当前 stride，表示该进程当前已经运行的“长度”。另外设置其对应的 pass 值（只与进程的优先权有关系），表示对应进程在调度后，stride 需要进行的累加值。

每次需要调度时，从当前 runnable 态的进程中选择 stride 最小的进程调度。对于获得调度的进程 P，将对应的 stride 加上其对应的步长 pass。

一个时间片后，回到上一步骤，重新调度当前 stride 最小的进程。

可以证明，如果令 P.pass = BigStride / P.priority 其中 P.priority 表示进程的优先权（大于 1），而 BigStride 表示一个预先定义的大常数，则该调度方案为每个进程分配的时间将与其优先级成正比。证明过程我们在这里略去，有兴趣的同学可以在网上查找相关资料。

其他实验细节：

stride 调度要求进程优先级 
，所以设定进程优先级 
 会导致错误。

进程初始 stride 设置为 0 即可。

进程初始优先级设置为 16。

为了实现该调度算法，内核还要增加 set_prio 系统调用

// syscall ID：140
// 设置当前进程优先级为 prio
// 参数：prio 进程优先级，要求 prio >= 2
// 返回值：如果输入合法则返回 prio，否则返回 -1
fn sys_set_priority(prio: isize) -> isize;
提示

你可以在TCB加入新的字段来支持优先级等。

为了减少整数除的误差，BIG_STRIDE 一般需要很大，但为了不至于发生反转现象（详见问答作业），或许选择一个适中的数即可，当然能进行溢出处理就更好了。

stride 算法要找到 stride 最小的进程，使用优先级队列是效率不错的办法，但是我们的实验测例很简单，所以效率完全不是问题。事实上，很推荐使用暴力扫一遍的办法找最小值。

注意设置进程的初始优先级。

注意

为了让大家能在本编程作业中使用 Vec 等数据结构，我们利用第三方库 buddy_system_allocator 为大家实现了堆内存分配器，相关代码位于 mm/heap_allocator 模块。

背景知识： Rust 中的动态内存分配

实验要求
实现分支：ch5。

实验目录请参考 ch3。注意在reports中放入lab1-3的所有报告。

通过所有测例。

在 os 目录下 make run BASE=2 加载所有测例， ch5_usertest 打包了所有你需要通过的测例， 你也可以通过修改这个文件调整本地测试的内容, 或者单独运行某测例来纠正特定的错误。 ch5_stride 检查 stride 调度算法是否满足公平性要求，六个子程序运行的次数应该大致与其优先级呈正比，测试通过标准是 
 
 
.

CI 的原理是用 ch5_usertest 替代 ch5b_initproc ，使内核在所有测例执行完后直接退出。

从本章开始，你的内核必须前向兼容，能通过前一章的所有测例。

GitHub Copilot: # 实现系统调用和Stride调度算法

我需要对代码进行几处修改，以实现spawn系统调用和stride调度算法。

## 步骤：
1. 实现`sys_spawn`系统调用
2. 实现`sys_set_priority`系统调用
3. 修改任务控制块(TCB)结构，添加stride和priority字段
4. 修改调度算法为stride调度

下面是具体的修改：

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\syscall\process.rs

修改`sys_spawn`和`sys_set_priority`函数实现：

````rust
// ...existing code...

/// YOUR JOB: Implement spawn.
/// HINT: fork + exec =/= spawn
pub fn sys_spawn(path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        // 创建新进程
        let new_task = Arc::new(TaskControlBlock::new(data));
        let new_pid = new_task.pid.0;
        // 添加新进程到调度器
        add_task(new_task);
        new_pid as isize
    } else {
        -1
    }
}

// YOUR JOB: Set task priority.
pub fn sys_set_priority(prio: isize) -> isize {
    trace!(
        "kernel:pid[{}] sys_set_priority",
        current_task().unwrap().pid.0
    );
    if prio < 2 {
        return -1;
    }
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    inner.priority = prio as usize;
    prio
}
// ...existing code...
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\task.rs

修改任务控制块以添加stride和priority字段:

````rust
// ...existing code...

pub struct TaskControlBlockInner {
    // ...existing code...
    pub stride: usize,
    pub priority: usize,
    // ...existing code...
}

impl TaskControlBlockInner {
    pub fn new() -> Self {
        // ...existing code...
        Self {
            // ...existing code...
            stride: 0,
            priority: 16,
            // ...existing code...
        }
    }
    // ...existing code...
}
// ...existing code...
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

修改任务管理器实现stride调度：

````rust
// ...existing code...

const BIG_STRIDE: usize = 1 << 20; // 适中的大常数

impl TaskManager {
    // ...existing code...
    
    /// 获取下一个要执行的任务，使用stride调度算法
    pub fn fetch_task(&mut self) -> Option<Arc<TaskControlBlock>> {
        let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
        
        for (idx, task) in self.ready_queue.iter().enumerate() {
            let inner = task.inner_exclusive_access();
            if inner.task_status == TaskStatus::Ready {
                let stride = inner.stride;
                if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                    min_stride_task = Some((idx, stride));
                }
            }
        }
        
        if let Some((idx, _)) = min_stride_task {
            let task = self.ready_queue.remove(idx);
            // 更新stride值
            let mut inner = task.inner_exclusive_access();
            inner.task_status = TaskStatus::Running;
            let pass = BIG_STRIDE / inner.priority;
            inner.stride += pass;
            drop(inner);
            
            Some(task)
        } else {
            None
        }
    }
    // ...existing code...
}
// ...existing code...
````

现在系统支持了spawn系统调用和stride调度算法。spawn系统调用可以直接创建新进程而不需要像fork+exec那样先复制父进程的地址空间。而stride调度算法则确保了进程获得的CPU时间与其优先级成正比。

系统的初始设置是每个进程的初始stride为0，初始优先级为16。用户可以通过sys_set_priority系统调用来修改进程的优先级，但优先级必须大于等于2，否则会返回-1表示错误。

DINGBROK423: error[E0424]: expected value, found module `self`
  --> src/task/manager.rs:55:24
   |
51 | pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
   |        ---------- this function can't have a `self` parameter
...
55 |     for (idx, task) in self.ready_queue.iter().enumerate() {
   |                        ^^^^ `self` value is a keyword only available in methods with a `self` parameter

error[E0424]: expected value, found module `self`
  --> src/task/manager.rs:66:20
   |
51 | pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
   |        ---------- this function can't have a `self` parameter
...
66 |         let task = self.ready_queue.remove(idx);
   |                    ^^^^ `self` value is a keyword only available in methods with a `self` parameter

error[E0609]: no field `priority` on type `RefMut<'_, TaskControlBlockInner>`
   --> src/syscall/process.rs:236:11
    |
236 |     inner.priority = _prio as usize;
    |           ^^^^^^^^ unknown field
    |
    = note: available fields are: `trap_cx_ppn`, `base_size`, `task_cx`, `task_status`, `memory_set` ... and 5 others

Some errors have detailed explanations: E0424, E0609.
For more information about an error, try `rustc --explain E0424`.
error: could not compile `os` (bin "os") due to 3 previous errors
make[1]: *** [kernel] Error 101
Makefile:12: recipe for target 'kernel' failed
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)

GitHub Copilot: 我需要修复代码中的几个错误，主要是关于`self`的使用以及缺少的字段。

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\task.rs

首先，我们需要在任务控制块中添加stride和priority字段：

````rust
// ...existing code...
pub struct TaskControlBlockInner {
    // ...existing code...
    pub stride: usize,
    pub priority: usize,
}

impl TaskControlBlockInner {
    pub fn new(
        // ...existing code...
    ) -> Self {
        // ...existing code...
        Self {
            // ...existing code...
            stride: 0,
            priority: 16,
        }
    }
    // ...existing code...
}
// ...existing code...
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

接下来，修正`fetch_task`函数，不要使用`self`而是获取`TASK_MANAGER`的引用：

````rust
// ...existing code...

const BIG_STRIDE: usize = 1 << 20; // 适中的大常数

// ...existing code...

/// 获取下一个要执行的任务，使用stride调度算法
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    let mut task_manager = TASK_MANAGER.exclusive_access();
    let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
    
    for (idx, task) in task_manager.ready_queue.iter().enumerate() {
        let inner = task.inner_exclusive_access();
        if inner.task_status == TaskStatus::Ready {
            let stride = inner.stride;
            if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                min_stride_task = Some((idx, stride));
            }
        }
    }
    
    if let Some((idx, _)) = min_stride_task {
        let task = task_manager.ready_queue.remove(idx);
        // 更新stride值
        let mut inner = task.inner_exclusive_access();
        inner.task_status = TaskStatus::Running;
        let pass = BIG_STRIDE / inner.priority;
        inner.stride += pass;
        drop(inner);
        
        Some(task)
    } else {
        None
    }
}
// ...existing code...
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\syscall\process.rs

最后，修正`sys_set_priority`函数，确保使用正确的字段名：

````rust
// ...existing code...

// YOUR JOB: Set task priority.
pub fn sys_set_priority(_prio: isize) -> isize {
    trace!(
        "kernel:pid[{}] sys_set_priority",
        current_task().unwrap().pid.0
    );
    if _prio < 2 {
        return -1;
    }
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    inner.priority = _prio as usize;
    _prio
}
// ...existing code...
````

这些修改应该解决了编译错误，正确实现了stride调度算法和spawn系统调用。

DINGBROK423: error[E0609]: no field `priority` on type `RefMut<'_, TaskControlBlockInner>`
   --> src/syscall/process.rs:236:11
    |
236 |     inner.priority = _prio as usize;
    |           ^^^^^^^^ unknown field
    |
    = note: available fields are: `trap_cx_ppn`, `base_size`, `task_cx`, `task_status`, `memory_set` ... and 5 others

error[E0609]: no field `stride` on type `RefMut<'_, TaskControlBlockInner>`
  --> src/task/manager.rs:59:32
   |
59 |             let stride = inner.stride;
   |                                ^^^^^^ unknown field
   |
   = note: available fields are: `trap_cx_ppn`, `base_size`, `task_cx`, `task_status`, `memory_set` ... and 5 others

error[E0599]: no method named `inner_exclusive_access` found for enum `Option` in the current scope
  --> src/task/manager.rs:69:30
   |
69 |         let mut inner = task.inner_exclusive_access();
   |                              ^^^^^^^^^^^^^^^^^^^^^^ method not found in `Option<Arc<TaskControlBlock>>`
   |
note: the method `inner_exclusive_access` exists on the type `Arc<TaskControlBlock>`
  --> src/task/task.rs:35:5
   |
35 |     pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `Arc<TaskControlBlock>` value, propagating an `Option::None` value to the caller
   |
69 |         let mut inner = task?.inner_exclusive_access();
   |                             +

error[E0308]: mismatched types
  --> src/task/manager.rs:75:9
   |
66 | /     if let Some((idx, _)) = min_stride_task {
67 | |         let task = task_manager.ready_queue.remove(idx);
68 | |         // 更新stride值
69 | |         let mut inner = task.inner_exclusive_access();
...  |
75 | |         Some(task)
   | |         ^^^^^^^^^^ expected `()`, found `Option<Option<Arc<...>>>`
76 | |     } else {
77 | |         None;
78 | |     }
   | |_____- expected this to be `()`
   |
   = note: expected unit type `()`
                   found enum `Option<Option<Arc<TaskControlBlock>>>`

Some errors have detailed explanations: E0308, E0599, E0609.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `os` (bin "os") due to 4 previous errors
make[1]: *** [kernel] Error 101
Makefile:12: recipe for target 'kernel' failed
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)
['Test 04_5 ummap OK1458040847!', 'current time_msec = (\\d+)', 'Test sleep OK1458040847!', 'Test spawn0 OK1458040847!', 'Test 04_6 ummap2 OK1458040847!', 'Test sleep1 passed1458040847!', 'Test 04_1 OK1458040847!', 'Test wait OK1458040847!', 'get_time OK1458040847! (\\d+)', 'Test waitpid OK1458040847!', 'time_msec = (\\d+) after sleeping (\\d+) ticks, delta = (\\d+)ms!', 'Test set_priority OK1458040847!', 'Test 04_4 test OK1458040847!'] ['FAIL: T.T', 'Test sbrk failed!']

GitHub Copilot: 我需要修复代码中的几个问题，主要涉及结构体字段的位置和类型错误。

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\task.rs

在TaskControlBlockInner中添加stride和priority字段，而不是在TaskControlBlock中：

````rust
/// Task control block structure
///
/// Directly save the contents that will not change during running
pub struct TaskControlBlock {
    // Immutable
    /// Process identifier
    pub pid: PidHandle,

    /// Kernel stack corresponding to PID
    pub kernel_stack: KernelStack,

    /// Mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

// ...existing code...

pub struct TaskControlBlockInner {
    /// The physical page number of the frame where the trap context is placed
    pub trap_cx_ppn: PhysPageNum,

    /// Application data can only appear in areas
    /// where the application address space is lower than base_size
    pub base_size: usize,

    /// Save task context
    pub task_cx: TaskContext,

    /// Maintain the execution status of the current process
    pub task_status: TaskStatus,

    /// Application address space
    pub memory_set: MemorySet,

    /// Parent process of the current process.
    /// Weak will not affect the reference count of the parent
    pub parent: Option<Weak<TaskControlBlock>>,

    /// A vector containing TCBs of all child processes of the current process
    pub children: Vec<Arc<TaskControlBlock>>,

    /// It is set when active exit or execution error occurs
    pub exit_code: i32,

    /// Heap bottom
    pub heap_bottom: usize,

    /// Program break
    pub program_brk: usize,
    
    /// The stride value for stride scheduling
    pub stride: usize,

    /// The priority of the task, used for scheduling
    pub priority: usize,
}

impl TaskControlBlockInner {
    // ...existing code...
}

impl TaskControlBlock {
    /// Create a new process
    ///
    /// At present, it is only used for the creation of initproc
    pub fn new(elf_data: &[u8]) -> Self {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        // push a task context which goes to trap_return to the top of kernel stack
        let task_control_block = Self {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    trap_cx_ppn,
                    base_size: user_sp,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory_set,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                    heap_bottom: user_sp,
                    program_brk: user_sp,
                    stride: 0,
                    priority: 16,
                })
            },
        };
        // ...existing code...
        task_control_block
    }

    /// parent process fork the child process
    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        // ---- access parent PCB exclusively
        let mut parent_inner = self.inner_exclusive_access();
        // copy user space(include trap context)
        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);
        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = pid_alloc();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let task_control_block = Arc::new(TaskControlBlock {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    trap_cx_ppn,
                    base_size: parent_inner.base_size,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory_set,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                    heap_bottom: parent_inner.heap_bottom,
                    program_brk: parent_inner.program_brk,
                    stride: parent_inner.stride,
                    priority: parent_inner.priority,
                })
            },
        });
        // ...existing code...
        task_control_block
        // ...existing code...
    }

    // ...existing code...
}
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

修复fetch_task函数的实现，修正类型错误和控制流问题：

````rust
/// Take a process out of the ready queue
use crate::task::TaskStatus;
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    let mut task_manager = TASK_MANAGER.exclusive_access();
    let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
        
    for (idx, task) in task_manager.ready_queue.iter().enumerate() {
        let inner = task.inner_exclusive_access();
        if inner.task_status == TaskStatus::Ready {
            let stride = inner.stride;
            if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                min_stride_task = Some((idx, stride));
            }
        }
    }
    
    if let Some((idx, _)) = min_stride_task {
        let task = task_manager.ready_queue.remove(idx);
        // 更新stride值
        let mut inner = task.inner_exclusive_access();
        inner.task_status = TaskStatus::Running;
        let pass = BIG_STRIDE / inner.priority;
        inner.stride += pass;
        drop(inner);
        
        Some(task)
    } else {
        // 如果没有找到合适的任务，使用原来的FIFO调度
        task_manager.fetch()
    }
}
````

至此，我们已经修复了代码中的主要问题：

1. 将stride和priority字段移动到了TaskControlBlockInner结构体中
2. 修正了fetch_task函数中的类型错误和控制流问题
3. 确保了sys_set_priority可以正确访问优先级字段

这些修改应该能解决编译错误，并正确实现stride调度算法和spawn系统调用。

找到具有 2 个许可证类型的类似代码

DINGBROK423: error[E0599]: no method named `inner_exclusive_access` found for enum `Option` in the current scope
  --> src/task/manager.rs:69:30
   |
69 |         let mut inner = task.inner_exclusive_access();
   |                              ^^^^^^^^^^^^^^^^^^^^^^ method not found in `Option<Arc<TaskControlBlock>>`
   |
note: the method `inner_exclusive_access` exists on the type `Arc<TaskControlBlock>`
  --> src/task/task.rs:29:5
   |
29 |     pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `Arc<TaskControlBlock>` value, propagating an `Option::None` value to the caller
   |
69 |         let mut inner = task?.inner_exclusive_access();
   |                             +

error[E0308]: mismatched types
   --> src/task/manager.rs:75:14
    |
75  |         Some(task)
    |         ---- ^^^^ expected `Arc<TaskControlBlock>`, found `Option<Arc<...>>`
    |         |
    |         arguments to this enum variant are incorrect
    |
    = note: expected struct `Arc<_>`
                 found enum `Option<Arc<_>>`
help: the type constructed contains `Option<Arc<TaskControlBlock>>` due to the type of the argument passed
   --> src/task/manager.rs:75:9
    |
75  |         Some(task)
    |         ^^^^^----^
    |              |
    |              this argument influences the type of `Some`
note: tuple variant defined here
   --> /home/k423/.rustup/toolchains/nightly-2024-05-02-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:579:5
    |
579 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: use the `?` operator to extract the `Option<Arc<TaskControlBlock>>` value, propagating an `Option::None` value to the caller
    |
75  |         Some(task?)
    |                  +

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `os` (bin "os") due to 2 previous errors
make[1]: *** [kernel] Error 101
Makefile:12: recipe for target 'kernel' failed
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)

GitHub Copilot: 我需要修复`fetch_task`函数中的错误，主要是关于Option类型的处理问题。

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

需要修复对`Option<Arc<TaskControlBlock>>`类型的处理：

````rust
/// Take a process out of the ready queue
use crate::task::TaskStatus;
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    let mut task_manager = TASK_MANAGER.exclusive_access();
    let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
        
    for (idx, task) in task_manager.ready_queue.iter().enumerate() {
        let inner = task.inner_exclusive_access();
        if inner.task_status == TaskStatus::Ready {
            let stride = inner.stride;
            if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                min_stride_task = Some((idx, stride));
            }
        }
    }
    
    if let Some((idx, _)) = min_stride_task {
        let task = task_manager.ready_queue.remove(idx);
        // 更新stride值
        let mut inner = task.inner_exclusive_access();
        inner.task_status = TaskStatus::Running;
        let pass = BIG_STRIDE / inner.priority;
        inner.stride += pass;
        drop(inner);
        
        return Some(task);
    } else {
        // 如果没有找到合适的任务，使用原来的FIFO调度
        return task_manager.fetch();
    }
}
````

主要修改了两点：

1. 在第75行，将`Some(task)`改为`return Some(task);`，避免嵌套的`Option`类型错误
2. 在最后一行，将`task_manager.fetch()`前面也加上`return`关键字以保持一致性

这样修改后，函数应该能够正确返回`Option<Arc<TaskControlBlock>>`类型的值，而不会有类型不匹配的问题。

DINGBROK423: error[E0599]: no method named `inner_exclusive_access` found for enum `Option` in the current scope
  --> src/task/manager.rs:69:30
   |
69 |         let mut inner = task.inner_exclusive_access();
   |                              ^^^^^^^^^^^^^^^^^^^^^^ method not found in `Option<Arc<TaskControlBlock>>`
   |
note: the method `inner_exclusive_access` exists on the type `Arc<TaskControlBlock>`
  --> src/task/task.rs:29:5
   |
29 |     pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `Arc<TaskControlBlock>` value, propagating an `Option::None` value to the caller
   |
69 |         let mut inner = task?.inner_exclusive_access();
   |                             +

error[E0308]: mismatched types
   --> src/task/manager.rs:75:21
    |
75  |         return Some(task);
    |                ---- ^^^^ expected `Arc<TaskControlBlock>`, found `Option<Arc<...>>`
    |                |
    |                arguments to this enum variant are incorrect
    |
    = note: expected struct `Arc<_>`
                 found enum `Option<Arc<_>>`
help: the type constructed contains `Option<Arc<TaskControlBlock>>` due to the type of the argument passed
   --> src/task/manager.rs:75:16
    |
75  |         return Some(task);
    |                ^^^^^----^
    |                     |
    |                     this argument influences the type of `Some`
note: tuple variant defined here
   --> /home/k423/.rustup/toolchains/nightly-2024-05-02-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:579:5
    |
579 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: use the `?` operator to extract the `Option<Arc<TaskControlBlock>>` value, propagating an `Option::None` value to the caller
    |
75  |         return Some(task?);
    |                         +

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `os` (bin "os") due to 2 previous errors
make[1]: *** [kernel] Error 101
Makefile:12: recipe for target 'kernel' failed
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)

GitHub Copilot: 我需要修复`fetch_task`函数中的类型错误问题。问题在于我们没有正确处理可能的`Option`类型。

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

````rust
/// Take a process out of the ready queue
use crate::task::TaskStatus;
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    let mut task_manager = TASK_MANAGER.exclusive_access();
    let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
        
    for (idx, task) in task_manager.ready_queue.iter().enumerate() {
        let inner = task.inner_exclusive_access();
        if inner.task_status == TaskStatus::Ready {
            let stride = inner.stride;
            if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                min_stride_task = Some((idx, stride));
            }
        }
    }
    
    if let Some((idx, _)) = min_stride_task {
        // 直接使用VecDeque的remove方法，它返回Arc<TaskControlBlock>而不是Option
        let task = task_manager.ready_queue.remove(idx);
        // 更新stride值
        let mut inner = task.inner_exclusive_access();
        inner.task_status = TaskStatus::Running;
        let pass = BIG_STRIDE / inner.priority;
        inner.stride += pass;
        drop(inner);
        
        // 直接返回任务，不需要再包装在Some中
        Some(task)
    } else {
        // 如果没有找到合适的任务，使用原来的FIFO调度
        task_manager.fetch()
    }
}
````

主要修改：
1. 在第75行，将`return Some(task);`改为`Some(task)`，因为`task`已经是`Arc<TaskControlBlock>`类型，而不是`Option<Arc<TaskControlBlock>>`
2. 删除了`return`关键字，让函数使用表达式返回结果

这样修改后，`fetch_task`函数现在能够正确处理类型，不会再有类型不匹配的问题。

DINGBROK423: error[E0599]: no method named `inner_exclusive_access` found for enum `Option` in the current scope
  --> src/task/manager.rs:70:30
   |
70 |         let mut inner = task.inner_exclusive_access();
   |                              ^^^^^^^^^^^^^^^^^^^^^^ method not found in `Option<Arc<TaskControlBlock>>`
   |
note: the method `inner_exclusive_access` exists on the type `Arc<TaskControlBlock>`
  --> src/task/task.rs:29:5
   |
29 |     pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use the `?` operator to extract the `Arc<TaskControlBlock>` value, propagating an `Option::None` value to the caller
   |
70 |         let mut inner = task?.inner_exclusive_access();
   |                             +

error[E0308]: mismatched types
   --> src/task/manager.rs:77:14
    |
77  |         Some(task)
    |         ---- ^^^^ expected `Arc<TaskControlBlock>`, found `Option<Arc<...>>`
    |         |
    |         arguments to this enum variant are incorrect
    |
    = note: expected struct `Arc<_>`
                 found enum `Option<Arc<_>>`
help: the type constructed contains `Option<Arc<TaskControlBlock>>` due to the type of the argument passed
   --> src/task/manager.rs:77:9
    |
77  |         Some(task)
    |         ^^^^^----^
    |              |
    |              this argument influences the type of `Some`
note: tuple variant defined here
   --> /home/k423/.rustup/toolchains/nightly-2024-05-02-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:579:5
    |
579 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
help: use the `?` operator to extract the `Option<Arc<TaskControlBlock>>` value, propagating an `Option::None` value to the caller
    |
77  |         Some(task?)
    |                  +

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `os` (bin "os") due to 2 previous errors
make[1]: *** [kernel] Error 101
Makefile:12: recipe for target 'kernel' failed
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)
['Test sleep OK4533940847!', 'Test waitpid OK4533940847!', 'Test 04_6 ummap2 OK4533940847!', 'current time_msec = (\\d+)', 'Test sleep1 passed4533940847!', 'Test 04_5 ummap OK4533940847!', 'Test 04_1 OK4533940847!', 'get_time OK4533940847! (\\d+)', 'Test spawn0 OK4533940847!', 'time_msec = (\\d+) after sleeping (\\d+) ticks, delta = (\\d+)ms!', 'Test set_priority OK4533940847!', 'Test 04_4 test OK4533940847!', 'Test wait OK4533940847!'] ['FAIL: T.T', 'Test sbrk failed!']

GitHub Copilot: 我需要修复`fetch_task`函数中的类型错误。问题在于我混用了`task`变量，导致类型不匹配。

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

修复类型错误，确保正确处理`Arc<TaskControlBlock>`类型：

````rust
/// Take a process out of the ready queue
use crate::task::TaskStatus;
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    let mut task_manager = TASK_MANAGER.exclusive_access();
    let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
        
    for (idx, task) in task_manager.ready_queue.iter().enumerate() {
        let inner = task.inner_exclusive_access();
        if inner.task_status == TaskStatus::Ready {
            let stride = inner.stride;
            if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                min_stride_task = Some((idx, stride));
            }
        }
    }
    
    if let Some((idx, _)) = min_stride_task {
        // 使用remove方法获取任务，它直接返回Arc<TaskControlBlock>
        let task = task_manager.ready_queue.remove(idx).unwrap();
        // 更新stride值
        let mut inner = task.inner_exclusive_access();
        inner.task_status = TaskStatus::Running;
        let pass = BIG_STRIDE / inner.priority;
        inner.stride += pass;
        drop(inner);
        
        Some(task)
    } else {
        // 如果没有找到合适的任务，使用原来的FIFO调度
        task_manager.fetch()
    }
}
````

修复了两个关键问题：

1. 在第67行，将`task_manager.ready_queue.remove(idx)`改为`task_manager.ready_queue.remove(idx).unwrap()`，因为`VecDeque::remove`返回的是`Option<T>`，需要解包。

2. 在第77行，直接返回`Some(task)`，因为`task`现在是正确的`Arc<TaskControlBlock>`类型。

这样修改后，stride调度算法应该能够正确工作，按照stride值选择优先级最高的任务进行调度。

DINGBROK423: [rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!
remap_test passed!
after initproc!
/**** APPS ****
ch2b_bad_address
ch2b_bad_instructions
ch2b_bad_register
ch2b_hello_world
ch2b_power_3
ch2b_power_5
ch2b_power_7
ch3_sleep
ch3_sleep1
ch3_trace
ch3b_yield0
ch3b_yield1
ch3b_yield2
ch4_mmap0
ch4_mmap1
ch4_mmap2
ch4_mmap3
ch4_trace1
ch4_unmap
ch4_unmap2
ch4b_sbrk
ch5_exit0
ch5_exit1
ch5_getpid
ch5_setprio
ch5_spawn0
ch5_spawn1
ch5_stride
ch5_stride0
ch5_stride1
ch5_stride2
ch5_stride3
ch5_stride4
ch5_stride5
ch5_usertest
ch5b_exit
ch5b_forktest
ch5b_forktest2
ch5b_forktest_simple
ch5b_forktree
ch5b_initproc
ch5b_user_shell
ch5b_usertest
**************/
Usertests: Running ch2b_hello_world
Usertests: Running ch2b_power_3
Usertests: Running ch2b_power_5
Usertests: Running ch2b_power_7
Usertests: Running ch3b_yield0
Usertests: Running ch3b_yield1
Usertests: Running ch3b_yield2
Usertests: Running ch3_sleep
Usertests: Running ch3_sleep1
Usertests: Running ch4_mmap0
Usertests: Running ch4_mmap1
Usertests: Running ch4_mmap2
Usertests: Running ch4_mmap3
Usertests: Running ch4_unmap
Usertests: Running ch4_unmap2
Usertests: Running ch5_spawn0
Usertests: Running ch5_spawn1
Usertests: Running ch5_setprio
Usertests: Test ch2b_hello_world in Process 1 exited with code 0
Panicked at src/bin/ch5_usertest.rs:48, assertion `left == right` failed
  left: 1
 right: -1
[kernel] Idle process exit with exit_code -1 ...
[kernel] Panicked at src/task/mod.rs:72 All applications completed!
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)
['Test sleep1 passed1303540847!', 'get_time OK1303540847! (\\d+)', 'Test 04_5 ummap OK1303540847!', 'Test 04_6 ummap2 OK1303540847!', 'Test spawn0 OK1303540847!', 'Test 04_4 test OK1303540847!', 'Test 04_1 OK1303540847!', 'Test set_priority OK1303540847!', 'current time_msec = (\\d+)', 'Test wait OK1303540847!', 'Test sleep OK1303540847!', 'Test waitpid OK1303540847!', 'time_msec = (\\d+) after sleeping (\\d+) ticks, delta = (\\d+)ms!'] ['FAIL: T.T', 'Test sbrk failed!']
[FAIL] not found <Test sleep1 passed1303540847!>
[FAIL] not found <get_time OK1303540847! (\d+)>
[FAIL] not found <Test 04_5 ummap OK1303540847!>
[FAIL] not found <Test 04_6 ummap2 OK1303540847!>
[FAIL] not found <Test spawn0 OK1303540847!>
[FAIL] not found <Test 04_4 test OK1303540847!>
[FAIL] not found <Test 04_1 OK1303540847!>
[FAIL] not found <Test set_priority OK1303540847!>
[FAIL] not found <current time_msec = (\d+)>
[FAIL] not found <Test wait OK1303540847!>
[FAIL] not found <Test sleep OK1303540847!>
[FAIL] not found <Test waitpid OK1303540847!>
[FAIL] not found <time_msec = (\d+) after sleeping (\d+) ticks, delta = (\d+)ms!>
[PASS] not found <FAIL: T.T>
[PASS] not found <Test sbrk failed!>

Test passed1303540847: 2/15

GitHub Copilot: 根据错误信息，问题出现在`fetch_task`函数中，它不应该设置任务状态为`Running`，因为这会导致调度逻辑出现问题。让我修复这个问题以及`sys_get_time`中的参数错误。

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

修复`fetch_task`函数，不要在这里设置任务状态：

````rust
/// Take a process out of the ready queue
use crate::task::TaskStatus;
/// Fetch a task from the ready queue using stride scheduling
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    let mut task_manager = TASK_MANAGER.exclusive_access();
    let mut min_stride_task: Option<(usize, usize)> = None; // (index, stride)
        
    for (idx, task) in task_manager.ready_queue.iter().enumerate() {
        let inner = task.inner_exclusive_access();
        if inner.task_status == TaskStatus::Ready {
            let stride = inner.stride;
            if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                min_stride_task = Some((idx, stride));
            }
        }
    }
    
    if let Some((idx, _)) = min_stride_task {
        // 使用remove方法获取任务
        let task = task_manager.ready_queue.remove(idx).unwrap();
        // 更新stride值，但不设置状态为Running（这由processor处理）
        let mut inner = task.inner_exclusive_access();
        let pass = BIG_STRIDE / inner.priority;
        inner.stride += pass;
        drop(inner);
        
        Some(task)
    } else {
        // 如果没有找到合适的任务，使用原来的FIFO调度
        task_manager.fetch()
    }
}
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\syscall\process.rs

修复`sys_get_time`函数中的参数错误：

````rust
/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!(
        "kernel:pid[{}] sys_get_time",
        current_task().unwrap().pid.0
    );
    let us=get_time_us();
    let sec =us / 1_000_000;
    let usec = us % 1_000_000;
    let time_val = TimeVal { sec, usec };
    let token = current_user_token();
    let time_val_size = core::mem::size_of::<TimeVal>();
    let byte_array = unsafe{
        core::slice::from_raw_parts(
            &time_val as *const TimeVal as *const u8,
            time_val_size
        )
    };
    let buffers = translated_byte_buffer(token, _ts as *const u8, time_val_size);
    let mut total_len=0;
    for buffer in buffers{
        let len = buffer.len().min(time_val_size - total_len);
        buffer[..len].copy_from_slice(&byte_array[total_len..total_len + len]);
        total_len += len;
        if total_len == time_val_size {
           break;
        }
    }
    0
}
````

主要修改：

1. **任务管理器**：在`fetch_task`中移除了设置任务状态为`Running`的代码，因为这应该由`processor.rs`中的`run_tasks`函数处理
2. **系统调用**：修复了`sys_get_time`中的参数错误，将`_tz as *const u8`改为`_ts as *const u8`，因为时间值应该写到`_ts`指向的地址

这些修改应该能解决调度问题，让测试能够正常运行。

DINGBROK423: [kernel] trap_handler:  Exception(StorePageFault) in application, bad addr = 0x10000000, bad instruction = 0x474, kernel killed it.
Panicked at src/bin/ch4_mmap1.rs:18, assertion `left == right` failed
  left: 0
 right: -1
Panicked at src/bin/ch4_mmap2.rs:18, assertion `left == right` failed
  left: 0
 right: -1
Panicked at src/bin/ch4_mmap3.rs:18, assertion `left == right` failed
  left: 0
 right: -1
Panicked at src/bin/ch4_unmap.rs:18, assertion `left == right` failed
  left: 0
 right: -1
Usertests: Running ch5_spawn1
Usertests: Running ch5_setprio
Usertests: Test ch2b_hello_world in Process 1 exited with code 0
Panicked at src/bin/ch5_usertest.rs:48, assertion `left == right` failed
  left: 1
 right: -1
[kernel] Idle process exit with exit_code -1 ...
[kernel] Panicked at src/task/mod.rs:72 All applications completed!
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/os'
python3 check/ch5.py < stdout-ch5 || (\
        make restore ; \
        exit 1 ; \
)
['time_msec = (\\d+) after sleeping (\\d+) ticks, delta = (\\d+)ms!', 'Test sleep1 passed5074140847!', 'Test 04_4 test OK5074140847!', 'Test waitpid OK5074140847!', 'current time_msec = (\\d+)', 'Test spawn0 OK5074140847!', 'Test 04_6 ummap2 OK5074140847!', 'Test 04_1 OK5074140847!', 'get_time OK5074140847! (\\d+)', 'Test sleep OK5074140847!', 'Test wait OK5074140847!', 'Test 04_5 ummap OK5074140847!', 'Test set_priority OK5074140847!'] ['FAIL: T.T', 'Test sbrk failed!']
[FAIL] not found <time_msec = (\d+) after sleeping (\d+) ticks, delta = (\d+)ms!>
[FAIL] not found <Test sleep1 passed5074140847!>
[FAIL] not found <Test 04_4 test OK5074140847!>
[FAIL] not found <Test waitpid OK5074140847!>
[PASS] found <current time_msec = (\d+)>
[FAIL] not found <Test spawn0 OK5074140847!>
[FAIL] not found <Test 04_6 ummap2 OK5074140847!>
[FAIL] not found <Test 04_1 OK5074140847!>
[PASS] found <get_time OK5074140847! (\d+)>
[FAIL] not found <Test sleep OK5074140847!>
[FAIL] not found <Test wait OK5074140847!>
[FAIL] not found <Test 04_5 ummap OK5074140847!>
[FAIL] not found <Test set_priority OK5074140847!>
[PASS] not found <FAIL: T.T>
[PASS] not found <Test sbrk failed!>

Test passed5074140847: 4/15
Traceback (most recent call last):
  File "check/ch5.py", line 24, in <module>
    base.test(EXPECTED_5, NOT_EXPECTED_4b)
  File "/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/ci-user/check/base.py", line 27, in test
    assert count == total
AssertionError
make[1]: Entering directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/ci-user'
Restoring original files from temp-* dirs
make[1]: Leaving directory '/home/k423/Desktop/rust/rustclass2/2025s-rcore-DINGBROK423/ci-user'
Makefile:115: recipe for target 'test' failed
make: *** [test] Error 1

GitHub Copilot: 根据错误信息，主要问题有两个：

1. mmap/munmap 系统调用返回了错误值
2. spawn 系统调用实现有问题

让我修复这些问题：

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\mod.rs

需要修复 `map_for_current_task` 和 `unmap_for_current_task` 函数中的任务获取方式：

````rust
// ...existing code...

/// Map a new area for the current task
pub fn map_for_current_task(start_vpn: VirtPageNum, num_pages: usize, map_perm: MapPermission) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    let memory_set = &mut inner.memory_set;
    
    let mut end_vpn = start_vpn;
    for _ in 0..num_pages {
        if let Some(pte) = memory_set.translate(end_vpn) {
            if pte.is_valid() { // vpn 已经被映射到了已经存在的物理页
                return -1;
            }
        }
        end_vpn.step();
    }
    let start_va = VirtAddr::from(start_vpn);
    let end_va = VirtAddr::from(end_vpn);
    // 一块新的 MapArea，会自动 map PageTable
    memory_set.insert_framed_area(start_va, end_va, map_perm);
    return 0;
}

///
pub fn unmap_for_current_task(start_vpn: VirtPageNum, num_pages: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    let memory_set = &mut inner.memory_set;
    
    let mut end_vpn = start_vpn;
    for _ in 0..num_pages {
        if let Some(pte) = memory_set.translate(end_vpn) {
            if !pte.is_valid() { // 不能解除不存在的映射
                return -1;
            }
            // PageTable 里面的 frames 存储了页表所有节点对应的物理帧，这些帧用来存 pte
            // 而 MapArea 里面的 data_frames 存储了页表三级节点中的最后一级（叶节点）指向的物理帧，这些帧用来存数据
            // 这里并没有释放掉 MapArea，但从 PageTable 中去掉了对 MapArea 中相应 data_frames 的映射
            //（当然，可能并非该 MapArea 全部 data_frames），因此该 MapArea 中再无法通过 PageTable 映射到的 data_frames
            // 对后续也没有影响了，没有被取消映射因此仍可以通过 PageTable 映射到的哪些 data_frames 没有受到影响  
            memory_set.unmap_from_page_table(end_vpn);
            end_vpn.step();
        } else { // 不能解除不存在的映射
            return -1;
        }
    }
    return 0;
}
````

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\syscall\process.rs

修复 spawn 系统调用，确保创建的新进程不继承父进程的关系：

````rust
// ...existing code...

/// YOUR JOB: Implement spawn.
/// HINT: fork + exec =/= spawn
use crate::task::TaskControlBlock;
pub fn sys_spawn(_path: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_spawn",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, _path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        // 创建新进程
        let new_task = Arc::new(TaskControlBlock::new(data));
        let new_pid = new_task.pid.0;
        // 将新进程设置为当前进程的子进程
        let current = current_task().unwrap();
        let mut current_inner = current.inner_exclusive_access();
        let mut new_inner = new_task.inner_exclusive_access();
        new_inner.parent = Some(Arc::downgrade(&current));
        current_inner.children.push(new_task.clone());
        drop(new_inner);
        drop(current_inner);
        // 添加新进程到调度器
        add_task(new_task);
        new_pid as isize
    } else {
        -1
    }
}

// ...existing code...
````

主要修改：

1. **任务映射函数**：修复了 `map_for_current_task` 和 `unmap_for_current_task` 中直接通过 `current_task()` 获取当前任务，而不是通过任务管理器
2. **spawn 系统调用**：确保新创建的进程被正确设置为当前进程的子进程，这样在进程管理中能够正确处理父子关系

这些修改应该能够解决 mmap/munmap 和 spawn 系统调用的问题，让更多的测试通过。

DINGBROK423: ok，全部通过。仔细讲解一下之前你给我提过的所有修改建议都是怎么改的，修改了哪部分

GitHub Copilot: 我来详细讲解一下为了实现章节5的练习，我们进行的所有修改：

## 1. 任务控制块结构修改

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\task.rs

**修改内容：**
- 在 `TaskControlBlockInner` 结构体中添加了两个新字段：
  ```rust
  /// The number of pages allocated for the user stack
  pub stride: usize,
  
  /// The priority of the task, used for scheduling
  pub priority: usize,
  ```
- 在 `TaskControlBlock::new()` 和 `fork()` 方法中初始化这些字段：
  ```rust
  stride: 0,      // 初始stride值为0
  priority: 16,   // 初始优先级为16
  ```

**为什么这样修改：**
- stride调度算法需要为每个进程维护stride值（已运行长度）
- 需要优先级字段来计算每次调度后stride的增量（pass值）

## 2. Stride调度算法实现

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\manager.rs

**修改内容：**
- 添加了常量：
  ```rust
  const BIG_STRIDE: usize = 1 << 20; // 适中的大常数
  ```
- 完全重写了 `fetch_task()` 函数：
  ```rust
  pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
      let mut task_manager = TASK_MANAGER.exclusive_access();
      let mut min_stride_task: Option<(usize, usize)> = None;
      
      // 遍历所有任务，找到stride最小的Ready任务
      for (idx, task) in task_manager.ready_queue.iter().enumerate() {
          let inner = task.inner_exclusive_access();
          if inner.task_status == TaskStatus::Ready {
              let stride = inner.stride;
              if min_stride_task.is_none() || stride < min_stride_task.unwrap().1 {
                  min_stride_task = Some((idx, stride));
              }
          }
      }
      
      if let Some((idx, _)) = min_stride_task {
          let task = task_manager.ready_queue.remove(idx).unwrap();
          // 更新stride值：stride += BIG_STRIDE / priority
          let mut inner = task.inner_exclusive_access();
          let pass = BIG_STRIDE / inner.priority;
          inner.stride += pass;
          drop(inner);
          Some(task)
      } else {
          // 回退到FIFO调度
          task_manager.fetch()
      }
  }
  ```

**为什么这样修改：**
- 实现stride调度核心逻辑：每次选择stride最小的就绪进程
- 选中进程后更新其stride值，确保高优先级进程获得更多CPU时间
- pass = BIG_STRIDE / priority 确保优先级高的进程stride增长慢，更容易被再次选中

## 3. 系统调用实现

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\syscall\process.rs

**A. sys_spawn 实现：**
```rust
pub fn sys_spawn(_path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, _path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        // 创建新进程（不复制父进程地址空间）
        let new_task = Arc::new(TaskControlBlock::new(data));
        let new_pid = new_task.pid.0;
        
        // 建立父子关系
        let current = current_task().unwrap();
        let mut current_inner = current.inner_exclusive_access();
        let mut new_inner = new_task.inner_exclusive_access();
        new_inner.parent = Some(Arc::downgrade(&current));
        current_inner.children.push(new_task.clone());
        
        add_task(new_task);
        new_pid as isize
    } else {
        -1
    }
}
```

**B. sys_set_priority 实现：**
```rust
pub fn sys_set_priority(_prio: isize) -> isize {
    if _prio < 2 {
        return -1;  // 优先级必须>=2
    }
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    inner.priority = _prio as usize;
    _prio
}
```

**C. sys_get_time 修复：**
```rust
// 修复了参数错误，使用_ts而不是_tz作为输出地址
let buffers = translated_byte_buffer(token, _ts as *const u8, time_val_size);
```

**为什么这样修改：**
- spawn与fork+exec不同：直接创建新进程而不复制父进程地址空间
- set_priority允许动态调整进程优先级，影响stride调度
- get_time修复确保时间值正确写入用户空间

## 4. 内存管理函数修复

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\mod.rs

**修改内容：**
```rust
pub fn map_for_current_task(start_vpn: VirtPageNum, num_pages: usize, map_perm: MapPermission) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    let memory_set = &mut inner.memory_set;
    // ... 映射逻辑
}

pub fn unmap_for_current_task(start_vpn: VirtPageNum, num_pages: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    let memory_set = &mut inner.memory_set;
    // ... 解映射逻辑
}
```

**为什么这样修改：**
- 之前的代码试图通过TASK_MANAGER.get_mut()获取任务，但这个方法不存在
- 改为直接通过current_task()获取当前任务，这是正确的方式
- 确保mmap/munmap系统调用能正常工作

## 5. 调度逻辑协调

### \home\k423\Desktop\rust\rustclass2\2025s-rcore-DINGBROK423\os\src\task\processor.rs

**关键协调：**
- `fetch_task()` 只负责选择任务和更新stride，不设置Running状态
- `run_tasks()` 中的 `task_inner.task_status = TaskStatus::Running;` 负责状态管理
- 这样分工避免了状态管理混乱

## 总结

整个修改过程实现了：

1. **数据结构扩展**：为进程添加stride和priority字段
2. **调度算法升级**：从简单FIFO改为stride调度
3. **新系统调用**：实现spawn（直接创建进程）和set_priority（动态调整优先级）
4. **兼容性维护**：修复现有系统调用（get_time, mmap, munmap）
5. **架构协调**：确保新的调度逻辑与现有进程管理架构兼容

这些修改使操作系统支持了优先级调度，进程可以根据其优先级获得相应比例的CPU时间，同时保持了与之前章节的兼容性。