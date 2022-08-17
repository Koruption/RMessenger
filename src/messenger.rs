use uuid::Uuid;

#[allow(dead_code)]
struct Receiver<'a, T> {
    id: String,
    cb: Box<&'a dyn Fn(&'a T)>
}

impl <'a, T> Receiver<'a, T> 
{
    fn new(_cb: &'a impl Fn(&'a T)) -> Receiver<'a, T> {
        return Receiver { 
            id: Uuid::new_v4().to_string(),
            cb: Box::new(_cb),
        };
    }
}

#[allow(dead_code)]
pub struct Sender<'a, K>
{
    receivers: Vec<Receiver<'a, K>>,
    name: String,
}

impl <'a, T>Sender<'a, T> {

    pub fn new(_name: String) -> Sender<'a, T> {
        Sender {
            receivers: Vec::new(),
            name: _name,
        }
    }

    pub fn tap(&mut self, _cb: &'a impl Fn(&'a T)) -> &mut Sender<'a, T> {
        self.receivers.push(Receiver::<T>::new(_cb));
        return self;
    }

    pub fn send(&self, message: &'a T) {
        for rec in &self.receivers {
            (rec.cb)(message)
        }
    }
}
