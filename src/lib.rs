#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::collections::HashMap;
use std::future;
use tokio_stream::StreamExt;
use std::future::Future;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
// use futures::future::BoxFuture;
use futures_util::future::BoxFuture;

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
    pub fn plyh2(&mut self) -> PlyH2 {
        let rx = self.rx.take().expect("plyh2 can only be called once");
        PlyH2 {
            rx,
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
        let msg = Msg{
            name: String::from(E::name()),
            bytes: entity.to_msg()
        };
        println!("Event: {} <{}>",msg.name,String::from_utf8(entity.to_msg()).unwrap());
        self.tx.send(msg).await.map_err(|e| Error{})
    }
}

// #[derive(Clone)]
pub struct PlyH2 {
    rx: Receiver<Msg>,
    tab: HashMap<String, Box<dyn Fn(Msg) -> BoxFuture<'static,Result<(), Error>>>>,
}

impl PlyH2 {
    pub fn register<Fut, T,F>(&mut self, f: F)
    where
        T: FromMsg,
        F: Fn(T) -> Fut + 'static,
        Fut: Future<Output = Result<(), Error>> + Send + 'static,
    {
        self.tab.insert(String::from(T::name()), Box::new(move |msg:Msg| {
            let t:T = T::from_msg(msg.bytes);
            Box::pin(f(t))
        }));
    }

    pub async fn run(mut self) {
        let mut s: ReceiverStream<Msg> = ReceiverStream::new(self.rx);
        while let Some(msg) = s.next().await {
            let e = self.tab.get(msg.name.as_str()).unwrap();
            println!("                       Handling: {} <{}>",msg.name.as_str(), String::from_utf8(msg.bytes.clone()).unwrap());

            let x = e(msg);

            x.await;
        }
    }
}
