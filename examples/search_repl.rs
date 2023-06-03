use ply_rs_stub::Ply;

struct Article {
    id: String
}

struct Db {
}


impl Db {
    fn save_article(&self, a: Article) {
    }

}


mod uc {

fn store_article(ply: Ply, a: Article) {

}
}
fn main() {
    println!("xxx");
    let p = Ply{};

    let db = Db{
    };



    let a = Article { id: String::from("1234") };
    db.save_artcile(a)
}
