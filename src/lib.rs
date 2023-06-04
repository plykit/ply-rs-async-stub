#![feature(fn_traits)]
#![feature(unboxed_closures)]

mod operation;
mod error;
mod msg;

pub use error::PlyError;
pub use operation::Operation;
pub use msg::Msg;
pub use msg::ToMsg;
pub use msg::FromMsg;

use std::collections::HashMap;
use tokio_stream::StreamExt;
use std::future::Future;
use std::marker::PhantomData;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
// use futures::future::BoxFuture;
use futures_util::future::BoxFuture;

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
    pub fn emitter<T:ToMsg>(&self) -> PlyEmitter<T> {
        println!("Creating emitter for {}",T::kind());

        PlyEmitter {
            phantom:PhantomData::default(),
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

#[derive(Clone)]
pub struct PlyEmitter<T:ToMsg> {
    phantom: PhantomData<T>,
    tx: Sender<Msg>,
}

impl PlyH {
    pub async fn update<E: ToMsg>(&self, entity: E) -> Result<(), PlyError> {
        let msg = Msg{
            op: Operation::Update,
            id: entity.id(),
            kind: String::from(E::kind()),
            bytes: entity.to_msg()
        };
        println!("Event: {} <{}>", msg.kind, String::from_utf8(entity.to_msg()).unwrap());
        //self.tx.send(msg).await.map_err(|e:tokio::sync::mpsc::error::SendError<Msg>| PlyError::SendError(e.into()))
        self.tx.send(msg).await.map_err(|e| PlyError::SendError(e.into()))
    }
}

impl<T:ToMsg> PlyEmitter<T> {
    pub async fn update(&self, entity: T) -> Result<(), PlyError> {
        let msg = Msg{
            op: Operation::Update,
            id: entity.id(),
            kind: String::from(T::kind()),
            bytes: entity.to_msg()
        };
        println!("Event: {} <{}>", msg.kind, String::from_utf8(entity.to_msg()).unwrap());
        //self.tx.send(msg).await.map_err(|e:tokio::sync::mpsc::error::SendError<Msg>| PlyError::SendError(e.into()))
        self.tx.send(msg).await.map_err(|e| PlyError::SendError(e.into()))
    }
}

// #[derive(Clone)]
pub struct PlyH2 {
    rx: Receiver<Msg>,
    tab: HashMap<String, Box<dyn Fn(Msg) -> BoxFuture<'static,Result<(), PlyError>>>>,
}

impl PlyH2 {
    pub fn register<Fut, T,F>(&mut self, f: F)
    where
        T: FromMsg,
        F: Fn(T) -> Fut + 'static,
        Fut: Future<Output = Result<(), PlyError>> + Send + 'static,
    {
        self.tab.insert(String::from(T::kind()), Box::new(move |msg:Msg| {
            match T::from_msg(msg.bytes) {
                Ok(t) => Box::pin(f(t)),
                Err(e) => Box::pin(std::future::ready(Err(e)))
            }
        }));
    }

    pub async fn run(mut self) {
        let mut s: ReceiverStream<Msg> = ReceiverStream::new(self.rx);
        while let Some(msg) = s.next().await {
            let e = self.tab.get(msg.kind.as_str()).unwrap();
            println!("                       Handling: {} <{}>", msg.kind.as_str(), String::from_utf8(msg.bytes.clone()).unwrap());

            let x = e(msg);

            x.await;
        }
    }
}
