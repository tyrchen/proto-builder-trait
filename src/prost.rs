use crate::utils::{
    derive_builder_attr, derive_builder_into_attr, derive_builder_option_attr, serde_attr,
    sqlx_from_row_attr, sqlx_type_attr,
};
use prost_build::Config;

/// provide extra attributes to the generated protobuf code easily
pub trait BuilderAttributes {
    /// add type attributes with `#[derive(serde::Serialize, serde::Deserialize)]`
    fn with_serde(&mut self, paths: &[&str], ser: bool, de: bool) -> &mut Self;
    /// add type attributes with `#[derive(sqlx::Type)]`
    fn with_sqlx_type(&mut self, paths: &[&str]) -> &mut Self;
    /// add type attributes with `#[derive(sqlx::FromRow)]`
    fn with_sqlx_from_row(&mut self, paths: &[&str]) -> &mut Self;
    /// add type attributes with `#[derive(derive_builder::Builder)]`
    fn with_derive_builder(&mut self, paths: &[&str]) -> &mut Self;
    /// add field attributes with `#[builder(setter(into), default)]`
    fn with_derive_builder_into(&mut self, path: &str, fields: &[&str]) -> &mut Self;
    /// add field attributes with `#[builder(setter(into, strip_option), default)]`
    fn with_derive_builder_option(&mut self, path: &str, fields: &[&str]) -> &mut Self;
    /// add type attributes
    fn with_type_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self;
    /// add field attributes
    fn with_field_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self;
}

impl BuilderAttributes for Config {
    fn with_serde(&mut self, paths: &[&str], ser: bool, de: bool) -> &mut Self {
        let attr = serde_attr(ser, de);

        paths
            .iter()
            .fold(self, |builder, ty| builder.type_attribute(ty, attr))
    }

    fn with_sqlx_type(&mut self, paths: &[&str]) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, sqlx_type_attr())
        })
    }

    fn with_sqlx_from_row(&mut self, paths: &[&str]) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, sqlx_from_row_attr())
        })
    }

    fn with_derive_builder(&mut self, paths: &[&str]) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, derive_builder_attr())
        })
    }

    fn with_derive_builder_into(&mut self, path: &str, fields: &[&str]) -> &mut Self {
        fields.iter().fold(self, |builder, field| {
            builder.field_attribute(format!("{}.{}", path, field), derive_builder_into_attr())
        })
    }

    fn with_derive_builder_option(&mut self, path: &str, fields: &[&str]) -> &mut Self {
        fields.iter().fold(self, |builder, field| {
            builder.field_attribute(format!("{}.{}", path, field), derive_builder_option_attr())
        })
    }

    fn with_type_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self {
        let attr = attributes.join("\n");

        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, attr.as_str())
        })
    }

    fn with_field_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self {
        let attr = attributes.join("\n");
        paths.iter().fold(self, |builder, ty| {
            builder.field_attribute(ty, attr.as_str())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_prost_build_with_extra_attributes_should_work() {
        let path = tempdir().unwrap();
        let filename = path.path().join("todo.rs");
        Config::default()
            .out_dir(path.path())
            .with_serde(&["todo.Todo", "todo.TodoStatus"], true, true)
            .with_derive_builder(&["todo.Todo"])
            .with_sqlx_type(&["todo.TodoStatus"])
            .with_derive_builder_into("todo.Todo", &["id", "title", "status", "description"])
            .with_derive_builder_option("todo.Todo", &["created_at", "updated_at"])
            .with_field_attributes(
                &["todo.Todo.created_at", "todo.Todo.updated_at"],
                &["#[derive(Copy)]"],
            )
            .compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"])
            .unwrap();
        insta::assert_snapshot!(fs::read_to_string(filename).unwrap(), @r###"
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
            #[derive(Copy)]
            pub created_at: ::core::option::Option<::prost_types::Timestamp>,
            #[prost(message, optional, tag="6")]
            #[builder(setter(into, strip_option), default)]
            #[derive(Copy)]
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
        "###);
    }
}
