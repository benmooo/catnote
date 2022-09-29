use bson::oid::ObjectId;
use juniper::graphql_object;

pub struct InsertId {
    insert_id: ObjectId,
}

impl InsertId {
    pub fn new(insert_id: ObjectId) -> Self {
        Self { insert_id }
    }
}

#[graphql_object]
impl InsertId {
    fn id(&self) -> String {
        self.insert_id.to_hex()
    }
}
