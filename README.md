# Proto Builder Trait

This crate simplifies the work to add extra attributes for prost-build/tonic-build generated code.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sqlx = "..."
derive_builder = "..."
serde = "..."

[build-dependencies]
proto-builder-trait = "0.1"
```

In your build.rs:

```rust
use proto_builder_trait::ProtoBuilder;
use prost_build::Config;

fn main() {
    Config::default()
        .out_dir(path.path())
        .with_serde(&["todo.Todo", "todo.TodoStatus"], true, true)
        .with_derive_builder(&["todo.Todo"])
        .with_sqlx_type(&["todo.TodoStatus"])
        .with_derive_builder_into("todo.Todo", &["id", "title", "status", "description"])
        .with_derive_builder_option("todo.Todo", &["created_at", "updated_at"])
        .compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"])
        .unwrap();
}
```

This will generate the following code:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(derive_builder::Builder)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Todo {
    #[prost(string, tag="1")]
    #[builder(setter(into), default)]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    #[builder(setter(into), default)]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    #[builder(setter(into), default)]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="TodoStatus", tag="4")]
    #[builder(setter(into), default)]
    pub status: i32,
    #[prost(message, optional, tag="5")]
    #[builder(setter(into, strip_option), default)]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    #[builder(setter(into, strip_option), default)]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTodosRequest {
    #[prost(string, repeated, tag="1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTodoRequest {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTodoRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTodoResponse {
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(sqlx::Type)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TodoStatus {
    Doing = 0,
    Done = 1,
}
impl TodoStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TodoStatus::Doing => "TODO_STATUS_DOING",
            TodoStatus::Done => "TODO_STATUS_DONE",
        }
    }
}
```
