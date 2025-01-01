## Cell


- `UnsafeCell`: Building block for interior mutability via unsafe pointer operation. Not safe for multithreaded hence generally wrapped inside a `Mutex` or `RwLock`.