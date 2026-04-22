use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
#[cfg(target_arch = "wasm32")]
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T>>>;

#[cfg(not(target_arch = "wasm32"))]
pub trait TaskHandle: Send + Sync + 'static {
    fn abort(&self);
}

#[cfg(target_arch = "wasm32")]
pub trait TaskHandle: 'static {
    fn abort(&self);
}

#[cfg(not(target_arch = "wasm32"))]
pub trait Runtime: Clone + Send + Sync + 'static {
    type TaskHandle: TaskHandle;

    fn spawn(&self, fut: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) -> Self::TaskHandle;
    fn sleep(&self, duration: Duration) -> BoxFuture<()>;
}

#[cfg(target_arch = "wasm32")]
pub trait Runtime: Clone + 'static {
    type TaskHandle: TaskHandle;

    fn spawn(&self, fut: Pin<Box<dyn Future<Output = ()> + 'static>>) -> Self::TaskHandle;
    fn sleep(&self, duration: Duration) -> BoxFuture<()>;
}
