use mongodb::{
    bson::{doc, Bson, Document},
    sync::{Client, Database, Collection, Cursor},
    options::{FindOptions,
              FindOneOptions,
              UpdateOptions,
              UpdateModifications,
              InsertOneOptions,
              DeleteOptions},
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    error::Result,
};
use serde::Serialize;

pub fn struct_to_doc<T>(value: T) -> Document
    where
        T: Serialize + std::fmt::Debug
{
    mongodb::bson::to_bson(&value).unwrap().as_document().unwrap().to_owned()
}

pub struct QAMongoClient {
    pub client: Client
}

impl QAMongoClient {
    pub fn new(uri: &str) -> Self {
        let client = Client::with_uri_str(uri).unwrap();
        Self {
            client: client
        }
    }
    pub fn database(&self, db: &str) -> QADataBase {
        QADataBase {
            db: self.client.database(db)
        }
    }
}

pub struct QADataBase {
    pub db: Database
}

impl QADataBase {
    pub fn collection(&self, coll: &str) -> QACollection {
        QACollection {
            coll: self.db.collection(coll)
        }
    }
}

pub struct QACollection {
    pub coll: Collection
}

impl QACollection {
    // 以下为了快速使用作简单封装，如需高级操作请自行 coll.method 原函数
    pub fn insert_one(&self, doc: Document, options: Document) -> Result<InsertOneResult> {
        self.coll.insert_one(doc, None)
    }

    pub fn insert_many(&self, docs: Vec<Document>, options: Document) -> Result<InsertManyResult> {
        self.coll.insert_many(docs, None)
    }

    pub fn delete_one(&self, query: Document, options: Document) -> Result<DeleteResult> {
        self.coll.delete_one(query, None)
    }

    pub fn delete_many(&self, query: Document, options: Document) -> Result<DeleteResult> {
        self.coll.delete_many(query, None)
    }

    pub fn update_one(&self, query: Document, update: Document, options: Document) -> Result<UpdateResult> {
        self.coll.update_one(query, update, None)
    }

    pub fn update_many(&self, query: Document, update: Document, options: Document) -> Result<UpdateResult> {
        // update 格式要求 doc! {"$set": { "age": 18}} 以 $操作为key
        self.coll.update_many(query, update, None)
    }

    pub fn find_one(&self, filter: Document, options: Document) -> Document {
        let find_options = FindOneOptions::builder().projection(options).build();
        match self.coll.find_one(filter, find_options).expect("Failed to execute find_one.") {
            Some(doc) => doc,
            None => Document::new()
        }
    }

    pub fn find(&self, filter: Document, options: Document) -> Vec<Document> {
        let find_options = FindOptions::builder().projection(options).build();
        self.coll.find(filter, find_options).expect("Failed to execute find.").map(|doc| doc.expect("Failed to execute cursor to document.")).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize)]
    struct A {
        name: String
    }

    #[test]
    fn test_mongo() {
        let client = QAMongoClient::new("mongodb://192.168.2.117:27017");
        let coll = client.database("cifitest").collection("mama");
        // test_insert(&coll);
        test_find(&coll);
        // test_update(&coll);
        // test_delete(&coll);
    }

    fn test_insert(coll: &QACollection) {
        coll.insert_one(doc! {"name":"qa1","age":33}, doc! {});
        let docs = vec![
            doc! {"name":"qa2","age":10},
            doc! {"name":"qa3","age":19},
            doc! {"name":"qa4","age":19},
            doc! {"name":"qa5","age":20},
        ];
        coll.insert_many(docs, doc! {});
    }

    fn test_find(coll: &QACollection) {
        let findone = coll.find_one(doc! {"name":"qa3"}, doc! {"_id":0,"age":1});
        println!("{:#?}", findone);
        let find = coll.find(doc! {}, doc! {"_id":0,"name":1});
        println!("{:#?}", find);
        let find = coll.find(struct_to_doc(A { name: "qa3".to_string() }), doc! {});
        println!("{:#?}", find);
    }

    fn test_update(coll: &QACollection) {
        coll.update_one(doc! {"name":"qa1"}, doc! {"name":"qa11","age":20}, doc! {});
        let find = coll.find(doc! {}, doc! {"_id":0});
        println!("{:#?}", find);
        coll.update_many(doc! {"name":"qa11"}, doc! {"$set": { "age": 4}}, doc! {});
        let find = coll.find(doc! {}, doc! {"_id":0});
        println!("{:#?}", find);
    }

    fn test_delete(coll: &QACollection) {
        let find = coll.find(doc! {}, doc! {"_id":0});
        println!("{:#?}", find);
        coll.delete_one(doc! {"name":"qa2"}, doc! {});
        let find = coll.find(doc! {}, doc! {"_id":0});
        println!("{:#?}", find);
        coll.delete_many(doc! {"name":"qa3"}, doc! {});
        let find = coll.find(doc! {}, doc! {"_id":0});
        println!("{:#?}", find);
    }
}

