# lab1 实验报告
#### 天津大学 王一丁
### 1.总体思路
**完成实验一要修改四个文件：**
1. **os/src/syscall/process.rs**
fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize 定义所在处
2. **os/src/config.rs**
添加宏：最大系统调用数
3. **os/src/task/mod.rs**
核心文件：添加系统计数功能
4. **os/src/syscall/mod.rs**
每次系统计数被调用都++

### 2.实验经历
1. **os/src/syscall/process.rs**
**0=>** 
一开始没读清题：“如果 trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务_id 地址处一个字节的无符号整数值。此时应忽略 data 参数。返回值为 id 地址处的值。” 以为_id是任务号，要找任务号为_id的任务的首地址。我还专门去找任务控制器的接口。后来仔细审了一下就是个地址解引用（私密马赛）
**1=>** 
掩码取低位，然后写入地址，unsafe打包
**2=>**
最难的一个，需要自主添加函数实现。需要访问当前任务，系统调用记录数组
重要的数据结构
```rust
pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}
/*inner 的类型是 UPSafeCell<TaskManagerInner>，它相当于一个“可变容器”（interior mutability）。
虽然 TASK_MANAGER 本身是用 lazy_static! 定义的 static ref，看起来是不可变的全局变量，但通过 UPSafeCell 我们可以在运行时取得对其中内容的独占可变引用，从而修改任务队列（tasks）或当前任务索引（current_task）。*/

```

```rust
pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}
//TaskManagerInner 里真正存储了任务列表和当前任务 ID：
```

```
UPSafeCell 和 exclusive_access()
UPSafeCell<T> 来自 crate::sync，它类似于单核环境下的 RefCell，但省去了运行时的借用检查。核心在于：

禁用中断：在单核时禁止上下文切换和中断，保证拿到的数据独占不被打断。
UnsafeCell：底层用 UnsafeCell<T> 存放数据，以便绕过 Rust 的默认不可变借用规则。
```

2. **os/src/task/mod.rs**
让我收获最大的地方是工程代码的严谨性
函数不添加注释说明不让过编译，进程调用数组开小了也会错
宏里面只能初始化，函数外界访问不到
```rust
UPSafeCell::new([[0; MAX_SYSCALL_NUM]; MAX_APP_NUM]) // 一开始开小了，导致原本对的前五个测试样例也WA了
```
```rust
pub fn current_task(&self) -> usize {
	let inner = self.inner.exclusive_access();
	let current = inner.current_task;
	drop(inner);
	current
}
// 封装一个供外界访问task的接口
在这段代码里：
​`exclusive_access()` 并不是简单地返回一个 `&mut TaskManagerInner`，而是返回了一个“临界区守护”（guard）——它在创建时关闭中断并借出数据，在销毁（`Drop`）时恢复中断并释放借用。  
​因此我们要：

1. 立即取出 `current` 字段后，显式调用 `drop(inner)`  
   – 触发 guard 的 `Drop` 实现，恢复中断  
   – 结束对 `inner` 的可变借用，避免后续再借用时冲突  
2. 最后再返回 `current`  

如果不写 `drop(inner)`，guard 会一直活到函数末尾才析构，中断恢复就会被延后，且整个函数体里一直保持着对 `inner` 的可变借用。
```
```rust
为什么需要 lazy_static!
Rust 的 static 变量要求在编译期就能 const 初始化，而很多类型（如带有运行时计算或 unsafe 的初始化）无法满足这一点。lazy_static! 通过“懒初始化”（first‐use）绕过了这个限制：

只有在真正第一次访问时才运行初始化代码
之后的访问直接拿到已经初始化好的值
```
