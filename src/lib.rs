//! [ChromaDB](https://www.trychroma.com/) client library for Rust.
//!
//! The library provides 2 modules to interact with the ChromaDB server via API V1:
//! * `client` - To interface with the ChromaDB server.
//! * `collection` - To interface with an associated ChromaDB collection.
//!
//! You can connect to ChromaDB by instantiating a [ChromaClient](crate::v1::ChromaClient)
//! ```
//! use chromadb::v1::client::{ChromaClient, ChromaClientOptions};
//! use chromadb::v1::collection::{ChromaCollection, GetResult};
//! use serde_json::json;
//!
//!# async fn doc_client_demo() -> anyhow::Result<()> {
//! // With default ChromaClientOptions
//! // Defaults to http://localhost:8000
//! let client: ChromaClient = ChromaClient::new(Default::default());
//!
//! // With custom ChromaClientOptions
//! let client: ChromaClient = ChromaClient::new(ChromaClientOptions { url: "<CHROMADB_URL>".into() });
//!
//! # Ok(())
//! # }
//! ```
//! Now that a client is instantiated, we can interface with the ChromaDB server and execute queries.
//!
//! A collection can be retrieved with the [create_collection](crate::v1::ChromaClient::create_collection), [get_or_create_collection](crate::v1::ChromaClient::get_or_create_collection), [get_collection](crate::v1::ChromaClient::get_collection) methods.
//!
//! ```
//!# use chromadb::v1::ChromaClient;
//!# use chromadb::v1::collection::{ChromaCollection, GetResult};
//!# use serde_json::json;
//!# async fn doc_client_create_collection(client: &ChromaClient) -> anyhow::Result<()> {
//! // Get or create a collection with the given name and no metadata.
//! let collection: ChromaCollection = client.get_or_create_collection("my_collection", None).await?;
//!
//! // Get the UUID of the collection
//! let collection_uuid = collection.id();
//! println!("Collection UUID: {}", collection_uuid);
//!
//! // Upsert some embeddings with documents and no metadata.
//! let result: bool = collection.upsert(
//!    vec!["demo-id-1", "demo-id-1", "demo-id-1"],
//!    Some(vec![vec![0.0_f64; 768]]),
//!    None,
//!    Some(vec![
//!        "Some document about 9 octopus recipies".into(),
//!        "Some other document about DCEU Superman Vs CW Superman".into()
//!    ]),
//!    None).await?;
//!
//! // Create a filter object to filter by document content.
//! let where_document = json!({
//!    "$contains": "Superman"
//!     });
//! 
//! // Get embeddings from a collection with filters and limit set to 1. 
//! // An empty IDs vec will return all embeddings.
//! let get_result: GetResult = collection.get(
//!    vec![],
//!    None,
//!    Some(1),
//!    None,
//!    Some(where_document),
//!    Some(vec![
//!        "documents",
//!        "embeddings"
//!    ])).await?;
//! println!("Get result: {:?}", get_result);
//!# Ok(())
//!# }
//! ```
//! Find more information about on the available filters and options in the [get()](crate::v1::ChromaCollection::get) documentation.
pub mod v1;
