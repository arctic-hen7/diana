use serde::{Serialize, Deserialize};
use async_graphql::{
    ScalarType,
    Scalar,
    Value as GQLValue,
    InputValueResult,
    InputValueError,
};
use mongodb::{
    bson::{
        doc,
        oid::ObjectId as MongoObjectId,
        Document
    },
    results::InsertOneResult
};

use crate::errors::*;

// A custom GraphQL scalar type for MongoDB object ids (like `_id`)
#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectId(MongoObjectId);
impl ObjectId {
    // Creates a new instance of the scalar from a string
    pub fn new(hex_string: &str) -> Result<Self> {
        let new_oid = Self(
            MongoObjectId::with_string(hex_string)?
        );

        Ok(new_oid)
    }
    // Creates a new instance of the scalar from a raw MongoDB ObjectId
    pub fn from_oid(oid: &MongoObjectId) -> Self {
        Self(
            oid.clone()
        )
    }
    // Returns just the ObjectId itself for interfacing directly with the MongoDb driver
    pub fn to_raw(&self) -> MongoObjectId {
        self.0.clone()
    }
    // Creates a document so we can search by `_id`
    pub fn as_find_clause(&self) -> Document {
        doc!{
            "_id": self.to_raw()
        }
    }
    // Creates a document so we can search by `_id` just after an insertion (insert then return what was inserted)
    // This needs the result of an `insert_one` operation (loop through and create that if you'ure using `insert_many`)
    pub fn find_clause_from_insertion_res(insertion_res: InsertOneResult) -> Result<Document> {
        let oid = Self::from_oid(
            insertion_res.inserted_id.as_object_id().ok_or(
                ErrorKind::OidSerializationFailed
            )?
        );

        Ok(doc!{
            "_id": oid.to_raw()
        })
    }
}
#[Scalar]
impl ScalarType for ObjectId {
    // Parses an input hex string and turns it into an ObjectId
    // This uses an `async_graphql` error type because it will be called directly from resolvers
    fn parse(value: GQLValue) -> InputValueResult<Self> {
        if let GQLValue::String(value) = &value {
            Ok(ObjectId(MongoObjectId::with_string(&value)?))
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }

    // Transforms an ObjectId into a hex string
    fn to_value(&self) -> GQLValue {
        GQLValue::String(self.0.to_hex())
    }
}
