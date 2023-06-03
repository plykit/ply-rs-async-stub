use crate::ent::Article;
use anyhow::Result;
use ply_rs_async_stub::{FromMsg, Ply};

mod ent {

    #[derive(Clone)]
    pub struct Article {
        pub(crate) id: String,
    }

    impl ply_rs_async_stub::ToMsg for Article {
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

}

mod repo {
    use anyhow::Result;
    // use ply_rs_async_stub::Ply;
    // use crate::{repo};
    use crate::ent::Article;

    pub struct Db {}

    impl Db {
        pub async fn save_article(&self, a: Article) -> Result<()> {
            todo!()
        }
    }
}

mod uc {
    use crate::ent::Article;
    use crate::repo::Db;
    use anyhow::Result;
    use ply_rs_async_stub::PlyH;

    pub async fn store_article(db: Db, plyh: PlyH, a: Article) -> Result<()> {
        plyh.update(a.clone());
        db.save_article(a);
        todo!()
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("xxx");
    let ply = Ply {};
    let plyh = ply.plyh();

    let db = repo::Db {};

    ply.plyh2().register(|msg: Vec<u8>| {
        let a = Article::from_msg(msg);
        std::future::ready(Ok(()))
    });


    let a = Article {
        id: String::from("1234"),
    };

    uc::store_article(db, plyh, a).await
}
