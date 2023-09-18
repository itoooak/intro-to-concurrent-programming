use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    FutureExt,
};
use std::{
    future::Future,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};

struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &std::sync::Arc<Self>) {}
}

struct Hello {
    state: StateHello,
}

enum StateHello {
    Hello,
    World,
    End,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::Hello,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.state {
            StateHello::Hello => {
                print!("Hello, ");
                self.state = StateHello::World;
                Poll::Pending
            }
            StateHello::World => {
                println!("World!");
                self.state = StateHello::End;
                Poll::Pending
            }
            StateHello::End => Poll::Ready(()),
        }
    }
}

fn main() {
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    let _ = hello.as_mut().poll(&mut ctx);
    let _ = hello.as_mut().poll(&mut ctx);
    let _ = hello.as_mut().poll(&mut ctx);
}
