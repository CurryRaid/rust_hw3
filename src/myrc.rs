use std::ops::Deref;

// 实现简易的引用计数指针MyRc,类似于std::rc::RC
// 1. MyRc<T>的实例可以通过MyRc::new(value)创建，其中value是T类型的值
// 2. MyRc<T>实现了Deref trait，可以通过*操作符获取T类型的值
// 3. MyRc<T>实现了Drop trait，当MyRc<T>实例被drop时，会将引用计数减1，当引用计数为0时，释放内存
// 4. MyRc<T>实现了Clone trait，可以通过clone()方法创建MyRc<T>实例的副本，引用计数加1

pub struct MyRc<T> {
    pub ptr: *mut MyRcInner<T>,
}
pub struct MyRcInner<T> {
    pub value: T,
    pub count: usize,
}
impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(MyRcInner { value, count: 1 });
        MyRc {
            ptr: Box::into_raw(inner),
        }
    }
    pub fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).count += 1;
        }
        MyRc { ptr: self.ptr }
    }
    pub fn get_count(&self) -> usize {
        unsafe { (*self.ptr).count }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).count -= 1;
            if (*self.ptr).count == 0 {
                println!("last one drop");
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr).value }
    }
}
