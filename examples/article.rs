#![feature(async_closure)]
use std::future::Future;
use std::time::Duration;
use crate::ent::{Article, Tool};
use anyhow::Result;
use tokio::time;
use ply_rs_async_stub::{FromMsg, Ply, PlyH2};

mod ent {
    use ply_rs_async_stub::PlyError;

    #[derive(Clone)]
    pub struct Article {
        pub(crate) id: String,
    }

    impl ply_rs_async_stub::ToMsg for Article {
        fn kind() -> &'static str {
            "Article"
        }

        fn id(&self) -> String {
            self.id.clone()
        }

        fn to_msg(&self) -> Vec<u8> {
            Vec::from(self.id.as_str())
        }
    }

    impl ply_rs_async_stub::FromMsg for Article {
        fn kind() -> &'static str {
            "Article"
        }

        fn from_msg(msg: Vec<u8>) -> Result<Self,PlyError> {
            Ok(Self{ id: String::from_utf8(msg).unwrap() })
        }
    }

    #[derive(Clone)]
    pub struct Tool {
        pub(crate) id: String,
    }

    impl ply_rs_async_stub::ToMsg for Tool {
        fn kind() -> &'static str {
            "Tool"
        }
        fn id(&self) -> String {
            self.id.clone()
        }
        fn to_msg(&self) -> Vec<u8> {
            Vec::from(self.id.as_str())
        }
    }

    impl ply_rs_async_stub::FromMsg for Tool {
        fn kind() -> &'static str {
            "Tool"
        }

        fn from_msg(msg: Vec<u8>) -> Result<Self,PlyError> {
            Ok(Self{ id: String::from_utf8(msg).unwrap() })
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
    use ply_rs_async_stub::{PlyEmitter, PlyH};
    use crate::Tool;

    pub async fn store_article(db: Db, plyh: PlyEmitter<Article>, a: Article) -> Result<()> {
        plyh.update(a.clone()).await.unwrap();
        db.save_article(a);
        Ok(())
    }

    pub async fn store_tool(db: Db, plyh: PlyEmitter<Tool>, t: Tool) -> Result<()> {
        plyh.update(t.clone()).await.unwrap();
        db.save_tool(t);
        Ok(())
    }
}
#[tokio::main]
async fn main() {
    println!("xxx");
    let mut ply = Ply::new();
    let article_emitter = ply.emitter::<Article>();
    let tool_emitter = ply.emitter::<Tool>();

    let db = repo::Db {};

    let mut plyh2 = ply.plyh2();

    plyh2.register(async move |a:Article| {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("                                                 do sth with article {}...",a.id);
        if a.id == "1" {
            return Err(ply_rs_async_stub::PlyError::UnknownOperation("xx".into()))
        }
        Ok(())
    });
    plyh2.register(async move |t:Tool| {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("                                                 do sth with tool {}...",t.id);
        if t.id == "1" {
            return Err(ply_rs_async_stub::PlyError::UnknownOperation("xx".into()))
        }
        Ok(())
    });




    tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(100));


            loop {
                interval.tick().await;
                let a = Article {
                    id: String::from("1234"),
                };
                let t = Tool {
                    id: String::from("1234"),
                };
                uc::store_article(db.clone(), article_emitter.clone(), a).await.unwrap();
                uc::store_tool(db.clone(), tool_emitter.clone(), t).await.unwrap();
            }
        });



    plyh2.run().await;
}
