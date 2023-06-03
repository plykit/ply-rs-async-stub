#![feature(fn_traits)]
#![feature(unboxed_closures)]


use std::future::Future;

#[derive(Debug)]
pub struct Error{}

pub trait ToMsg {
    fn to_msg(&self) -> Vec<u8>;
}

pub trait FromMsg {
    fn name() -> &'static str;
    fn from_msg(msg: Vec<u8>) -> Self;
}



pub struct Ply {

}

impl Ply {
    pub fn plyh(&self) -> PlyH {
        PlyH{}
    }
    pub fn plyh2<Fut,T>(&self) -> PlyH2<Fut,T> {
        PlyH2{ tab: Default::default() }
    }

}


#[derive(Clone)]
pub struct PlyH {

}

impl PlyH {
    pub async fn update<E: ToMsg>(&self, entity: E) -> Result<(),Error>{
        todo!()
    }
}

// #[derive(Clone)]
pub struct PlyH2<Fut,T> {
    tab: HashMap<String, Box<dyn Fn(T) -> Fut>>,
}

impl<Fut,T> PlyH2<Fut,T> {
    pub fn register<F>(&mut self, f: F)
        where
            T: FromMsg,
            F: Fn(T) -> Fut + 'static,
            Fut: Future<Output=Result<(), ()>>
    {
        self.tab.insert(String::from(T::name()), Box::new(f));
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // poll event log
        // type,bytes = nextEvent()
        //

        let a = self.tab.get("").unwrap();
        let b: Vec<u8> = vec![];

        a(b).await.expect("TODO: panic message")
    }
}


// use async_trait::async_trait;
use std::collections::HashMap;
use std::env::Args;
// use std::future::Future;
// use derive_more::Display;
// use std::marker::PhantomData;

// pub struct Entity {}
//
// // pub enum Error {
// //     Connect,
// // }
// //
// // impl std::error::Error for Error {}
//
//
// // #[derive(Clone)]
// struct Entry<T, F> {
//     name: String,
//     // de: fn(Vec<u8>) -> T,
//     f: F,
// }
//
// impl<T, F> FnMut<Args> for Entry<T, F> {
//     fn call_mut(&mut self, args: Args) -> Self::Output {
//         todo!()
//     }
// }
//
// impl<T, F> FnOnce<Args> for Entry<T, F> {
//     type Output = ();
//
//     fn call_once(self, args: Args) -> Self::Output {
//         todo!()
//     }
// }
//
// impl<T, F> Fn<Args> for Entry<T, F> {
//     fn call(&self, args: Args) -> Self::Output {
//         self.f(args.)
//     }
// }
//
// impl<T, F, Fut, G> Entry<T, F> {
//     fn new(name: &str, de: fn(Vec<u8>) -> T, g: G) -> Entry<T, F> where
//         T: Into<Entity> + From<Vec<u8>>,
//         F: Fn(T) -> Fut,
//         Fut: Future<Output=Result<(), ()>>
//     {
//         let h = || {};
//         Entry {
//             name: String::from(name),
//             f: h,
//         }
//     }
// }

