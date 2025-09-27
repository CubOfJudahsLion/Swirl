//! Module  :   FRP
//! Purpose :   A Functional Reactive Programming library. FRP models everything
//!             in terms of transformable streams connected in pipes.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu Feb 25 03:21:53 PM CST 2021
//! Modified:   Wed Mar  3 01:23:15 PM CST 2021


use std::rc::Rc;


pub struct Stream<T> {
    endpoint: Rc::
}


/*pub enum Message<E> {
    Event(E),
    End,
}


//trait Observer<E: Sized> {
//  fn generate(callback: dyn FnMut(dyn FnMut(&[Message<E>]))) -> Self;
//  fn never() -> Self;
//  fn once(val: E) -> Self;
//  fn later(val: E, ms: u64) -> Observer<E>;
//  fn interval(val: E, ms: u64) -> Observer<E>;
//  fn merge(streams: &[&Observer<E>]) -> Observer<E>;
//  fn filter(&self, filter_fn: FnMut(E) -> bool) -> Observer<F>;
//  fn map<F>(&mut self, mapper: FnMut(E) -> F) -> Observer<F>;
//  fn flat_map(&mut self, stream_mapper: FnMut(E) -> Observer<F>) -> Observer<F>;
//  fn observe(&mut self, Fn(E));
//}
 

#[allow(dead_code)]
struct StreamData<E> {
    open    : bool,
    chain   : Vec<Box<dyn FnMut(Message<E>)>>,
}


impl<E: Clone> StreamData<E> {
    #[allow(dead_code)]
    fn new() -> Self {
        StreamData {
            open    : true,
            chain   : Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn pass_down(&mut self, msg: &Message<E>) {
        for next in &mut self.chain {
            use Message::*;
            match msg {
                &End            =>  next(End),
                &Event(ref ev)  =>  next(Event(ev.clone())),
            }
         }
    }

    #[allow(dead_code)]
    fn process_messages(&mut self, msg_list: &[Message<E>]) {
        if self.open {
            for msg in msg_list {
                self.pass_down(msg);
                if let &Message::End = msg {
                    self.open = false;
                    return;
                }
            }
        }
    }
}


#[allow(dead_code)]
pub struct Stream<E>{
    rdata       : Rc<RefCell<StreamData<E>>>,
    generator   : Box<dyn FnOnce(&[Message<E>]) + 'static>,
}

impl<E: Clone + 'static> Stream<E> {
    #[allow(dead_code)]
    pub fn from_generator<F>(mut gen: F) -> Self
        where   F: FnOnce(Box<dyn FnMut(&[Message<E>])>) + 'static
    {
        let data        = StreamData::new();
        let rdata       = Rc::new(RefCell::new(data));
        let rdata_clone = rdata.clone();
        let generator   = Box::new(move |msg_list: &[Message<E>]| { rdata_clone.deref().borrow_mut().process_messages(msg_list);  });
        gen(generator);
        Stream {rdata, generator}
    }

    pub fn once(val: E) -> Self {
        Self::from_generator(|sink: Box<dyn FnMut(&[Message<E>])>| {
            let win         = window().expect_throw("recórcholis");
            let callback    = move || { sink(&[Message::Event(val), Message::End]); };
            let cbox        = Box::new(callback) as Box<dyn FnMut()>;
            let closure     = Closure::wrap(cbox).as_ref().unchecked_ref();
            win.set_timeout_with_callback_and_timeout_and_arguments_0(closure, 0);
        })
    }
}*/
