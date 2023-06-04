#![feature(async_closure)]
use std::future::Future;
use crate::ent::{Article, Tool};
use anyhow::Result;
use ply_rs_async_stub::{FromMsg, Ply, PlyH2};

mod ent {

    #[derive(Clone)]
    pub struct Article {
        pub(crate) id: String,
    }

    impl ply_rs_async_stub::ToMsg for Article {
        fn name() -> &'static str {
            "Article"
        }
        fn to_msg(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    impl ply_rs_async_stub::FromMsg for Article {
        fn name() -> &'static str {
            "Article"
        }

        fn from_msg(msg: Vec<u8>) -> Self {
            todo!()
        }
    }

    #[derive(Clone)]
    pub struct Tool {
        pub(crate) id: String,
    }

    impl ply_rs_async_stub::ToMsg for Tool {
        fn name() -> &'static str {
            "Tool"
        }
        fn to_msg(&self) -> Vec<u8> {
            Vec::new()
        }
    }

    impl ply_rs_async_stub::FromMsg for Tool {
        fn name() -> &'static str {
            "Tool"
        }

        fn from_msg(msg: Vec<u8>) -> Self {
            todo!()
        }
    }

}

mod repo {
    use anyhow::Result;
    // use ply_rs_async_stub::Ply;
    // use crate::{repo};
    use crate::ent::Article;
    use crate::Tool;

    #[derive(Clone)]
    pub struct Db {}

    impl Db {
        pub async fn save_article(&self, a: Article) -> Result<()> {
            todo!()
        }
        pub async fn save_tool(&self, t: Tool) -> Result<()> {
            todo!()
        }
    }
}

mod uc {
    use crate::ent::Article;
    use crate::repo::Db;
    use anyhow::Result;
    use ply_rs_async_stub::PlyH;
    use crate::Tool;

    pub async fn store_article(db: Db, plyh: PlyH, a: Article) -> Result<()> {
        plyh.update(a.clone());
        db.save_article(a);
        todo!()
    }

    pub async fn store_tool(db: Db, plyh: PlyH, t: Tool) -> Result<()> {
        plyh.update(t.clone());
        db.save_tool(t);
        todo!()
    }
}
#[tokio::main]
async fn main() {
    println!("xxx");
    let mut ply = Ply::new();
    let plyh = ply.plyh();

    let db = repo::Db {};

    ply.plyh2().register(async move |a:Article| {
        println!("{}",a.id);
        if a.id == "1" {
            return Err(())
        }
        Ok(())
    });
    ply.plyh2().register(async move |t:Tool| {
        println!("{}",t.id);
        if t.id == "1" {
            return Err(())
        }
        Ok(())
    });


    let a = Article {
        id: String::from("1234"),
    };
    let t = Tool {
        id: String::from("1234"),
    };

    uc::store_article(db.clone(), plyh.clone(), a).await.unwrap();
    uc::store_tool(db, plyh, t).await.unwrap();

    let x = ply.plyh2();
    x.run().await;
}
