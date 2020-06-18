use mongodb::{
    bson::{doc, Bson, Document},
    sync::{Client, Database, Collection, Cursor},
    options::{FindOptions, FindOneOptions, UpdateOptions, UpdateModifications, InsertOneOptions},
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
    /// 以下为了快速使用作简单封装，如需高级操作请自行 coll.method 原函数
    pub fn insert_one(&self, doc: Document) {
        self.coll.insert_one(doc, None);
    }
    pub fn insert_many(&self, docs: Vec<Document>) {
        self.coll.insert_many(docs, None);
    }

    pub fn delete_one(&self) {}

    pub fn delete_many(&self) {}

    pub fn update_one(&self) {}

    pub fn update_many(&self) {}

    pub fn find_one(&self, filter: Document, options: Document) -> Option<Document> {
        let find_options = FindOneOptions::builder().projection(options).build();
        self.coll.find_one(filter, find_options).unwrap()
    }

    pub fn find(&self, filter: Document, options: Document) -> Option<Vec<Document>> {
        let find_options = FindOptions::builder().projection(options).build();
        self.coll.find_one(filter, find_options).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mongo() {
        let client = QAMongoClient::new("mongodb://192.168.2.117:27017");
        let coll = client.database("cifitest").collection("mama");
        coll.insert_one(doc! {"name":"qa1","age":33});
        let docs = vec![
            doc! {"name":"qa2","age":10},
            doc! {"name":"qa3","age":19},
            doc! {"name":"qa4","age":19},
            doc! {"name":"qa5","age":20},
        ];
        coll.insert_many(docs);
        let findone = coll.find_one(doc! {}, doc! {});
        println!("{:?}", findone.unwrap());
    }
}

