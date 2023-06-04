#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::collections::HashMap;
use tokio_stream::StreamExt;
use std::future::Future;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug)]
pub struct Error {}

pub trait ToMsg {
    fn name() -> &'static str;
    fn to_msg(&self) -> Vec<u8>;
}

pub trait FromMsg {
    fn name() -> &'static str;
    fn from_msg(msg: Vec<u8>) -> Self;
}

struct Msg {
    pub name: String,
    pub bytes: Vec<u8>,
}

pub struct Ply {
    tx: Sender<Msg>,
    rx: Option<Receiver<Msg>>,
}

impl Ply {
    pub fn new() -> Ply {
        let (tx, rx) = mpsc::channel(100);
        Ply { tx, rx: Some(rx) }
    }
    pub fn plyh(&self) -> PlyH {
        PlyH {
            tx: self.tx.clone(),
        }
    }
    pub fn plyh2<Fut, T>(&mut self) -> PlyH2<Fut, T> where
    Fut: Future<Output = Result<(), ()>>
    {
        let rx = self.rx.take().expect("plyh2 can only be called once");
        PlyH2 {
            rx,
            tab2: Default::default(),
            tab: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct PlyH {
    tx: Sender<Msg>,
}

impl PlyH {
    pub async fn update<E: ToMsg>(&self, entity: E) -> Result<(), Error> {
        todo!()
    }
}

// #[derive(Clone)]
struct Entry<T> {
    name: String,
    de: fn(Vec<u8>) -> T,
    // f: F,
}

// #[derive(Clone)]
pub struct PlyH2<Fut, T> {
    rx: Receiver<Msg>,
    tab2: HashMap<String, Entry<T>>,
    tab: HashMap<String, Box<dyn Fn(T) -> Fut>>,
}

impl<Fut: std::future::Future, T> PlyH2<Fut, T> {
    pub fn register<F>(&mut self, f: F)
    where
        T: FromMsg,
        F: Fn(T) -> Fut + 'static,
        Fut: Future<Output = Result<(), ()>>,
    {
        let e = Entry {
            name: String::from(T::name()),
            de: T::from_msg,
        };
        self.tab2.insert(String::from(T::name()), e);
        self.tab.insert(String::from(T::name()), Box::new(f));
    }

    pub async fn run(mut self) {
        let mut s: ReceiverStream<Msg> = ReceiverStream::new(self.rx);
        while let Some(msg) = s.next().await {
            let e = self.tab2.get("").unwrap();
            let a = self.tab.get("").unwrap();

            let x = (e.de)(msg.bytes);

            a(x).await;
        }
    }
}
