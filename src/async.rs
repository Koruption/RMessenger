use futures::executor::block_on;
use core::future::Future;

struct AsyncSender<'a, K>
{
    receivers: Vec<AsyncReceiver<'a, K>>,
    name: String,
}

struct AsyncReceiver<'a, T> {
    id: String,
    // cb: Box<dyn Fn(&'a Message<T>)>
    cb: Box<&'a dyn Fn(&'a T) -> dyn Future<Output=Option<bool>>>,
}

impl <'a, T> AsyncReceiver<'a, T> 
{
    fn new(_cb: &'a impl Fn(&'a T) -> dyn Future<Output=Option<bool>>) -> AsyncReceiver<'a, T> {
        return AsyncReceiver { 
            id: Uuid::new_v4().to_string(),
            cb: Box::new(_cb),
        };
    }
}


impl <'a, T>AsyncSender<'a, T> {
    fn new(_name: String) -> AsyncSender<'a, T> {
        AsyncSender {
            receivers: Vec::new(),
            name: _name,
        }
    }

    fn tap(&mut self, _cb: &'a impl Fn(&'a T) -> dyn Future<Output=Option<bool>>) -> &mut AsyncSender<'a, T> {
        self.receivers.push(AsyncReceiver::<T>::new(_cb));
        return self;
    }

    fn get_cbs(&self, message: &'a T) -> Vec<&dyn Fn(&'a T) -> (dyn futures::Future<Output = Option<bool>> + 'static)> {
        let cbs = Vec::new();
        for &rec in self.receivers.iter() {
            let f = *rec.cb;
            cbs.push(f.call(message))
        }
        cbs
    }

    async fn send(&self, message: &'a T, parallel: bool) {
        if (parallel) {
            futures::try_join!(self.get_cbs());
            return 
        }
        for rec in &self.receivers {
            (rec.cb)(message).await
        }
        return 
    }
}
