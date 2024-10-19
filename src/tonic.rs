use crate::utils::{
    derive_builder_attr, serde_as_attr, serde_attr, sqlx_from_row_attr, sqlx_type_attr,
};
use tonic_build::Builder;

/// provide extra attributes to the generated protobuf code easily
pub trait BuilderAttributes {
    /// add type attributes with `#[derive(serde::Serialize, serde::Deserialize)]`
    fn with_serde(self, paths: &[&str], ser: bool, de: bool, extra_attrs: Option<&[&str]>) -> Self;
    fn with_serde_as(self, path: &str, fields: &[(&[&str], &str)]) -> Self;
    /// add type attributes with `#[derive(sqlx::Type)]`
    fn with_sqlx_type(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes with `#[derive(sqlx::FromRow)]`
    fn with_sqlx_from_row(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes with `#[derive(derive_builder::Builder)]`
    fn with_derive_builder(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes with `#[derive(strum::EnumString)]`
    fn with_strum(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self;
    /// add type attributes
    fn with_type_attributes(self, paths: &[&str], attributes: &[&str]) -> Self;
    /// add field attributes
    fn with_field_attributes(self, paths: &[&str], attributes: &[&str]) -> Self;
    /// add optional type attributes
    fn with_optional_type_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self;
    /// add optional field attributes
    fn with_optional_field_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self;
}

/// provide extra attributes to the generated protobuf code easily
impl BuilderAttributes for Builder {
    fn with_serde(self, paths: &[&str], ser: bool, de: bool, extra_attrs: Option<&[&str]>) -> Self {
        let attr = serde_attr(ser, de);

        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, attr)
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_serde_as(self, path: &str, fields: &[(&[&str], &str)]) -> Self {
        let serde_attr = serde_as_attr();
        let builder = self.type_attribute(path, serde_attr);
        fields.iter().fold(builder, |builder, (paths, attr)| {
            paths.iter().fold(builder, |builder, p| {
                let p = format!("{}.{}", path, p);
                builder.field_attribute(p, attr)
            })
        })
    }

    fn with_sqlx_type(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, sqlx_type_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_sqlx_from_row(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, sqlx_from_row_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_derive_builder(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, derive_builder_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_strum(self, paths: &[&str], extra_attrs: Option<&[&str]>) -> Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(
                    ty,
                    "#[derive(strum::EnumString, strum::Display, strum::EnumIter)]",
                )
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_type_attributes(self, paths: &[&str], attributes: &[&str]) -> Self {
        let attr = attributes.join("\n");

        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, attr.as_str())
        })
    }

    fn with_field_attributes(self, paths: &[&str], attributes: &[&str]) -> Self {
        let attr = attributes.join("\n");
        paths.iter().fold(self, |builder, ty| {
            builder.field_attribute(ty, attr.as_str())
        })
    }

    fn with_optional_type_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self {
        if let Some(attributes) = attributes {
            self.with_type_attributes(paths, attributes)
        } else {
            self
        }
    }

    fn with_optional_field_attributes(self, paths: &[&str], attributes: Option<&[&str]>) -> Self {
        if let Some(attributes) = attributes {
            self.with_field_attributes(paths, attributes)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_tonic_build_with_extra_attributes_should_work() {
        let path = tempdir().unwrap();
        let filename = path.path().join("todo.rs");
        tonic_build::configure()
            .out_dir(path.path())
            .with_serde(
                &["todo.Todo", "todo.TodoStatus"],
                true,
                true,
                Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
            )
            .with_serde_as(
                "todo.Todo",
                &[(
                    &["status", "created_at"],
                    r#"#[serde_as(as = "DisplayFromStr")]"#,
                )],
            )
            .with_derive_builder(
                &["todo.Todo"],
                Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
            )
            .with_sqlx_type(&["todo.TodoStatus"], None)
            .with_strum(
                &["todo.TodoStatus"],
                Some(&[r#"#[strum(ascii_case_insensitive, serialize_all = "snake_case")]"#]),
            )
            .with_field_attributes(
                &["todo.Todo.created_at", "todo.Todo.updated_at"],
                &["#[derive(Copy)]"],
            )
            .compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"])
            .unwrap();
        insta::assert_snapshot!(fs::read_to_string(filename).unwrap(), @r###"
        // This file is @generated by prost-build.
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(derive_builder::Builder)]
        #[builder(setter(into, strip_option), default)]
        #[builder(build_fn(name = "private_build"))]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Todo {
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub title: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub description: ::prost::alloc::string::String,
            #[prost(enumeration = "TodoStatus", tag = "4")]
            #[serde_as(as = "DisplayFromStr")]
            pub status: i32,
            #[prost(message, optional, tag = "5")]
            #[serde_as(as = "DisplayFromStr")]
            #[derive(Copy)]
            pub created_at: ::core::option::Option<::prost_types::Timestamp>,
            #[prost(message, optional, tag = "6")]
            #[derive(Copy)]
            pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GetTodosRequest {
            #[prost(string, repeated, tag = "1")]
            pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CreateTodoRequest {
            #[prost(string, tag = "1")]
            pub title: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub description: ::prost::alloc::string::String,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeleteTodoRequest {
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
        }
        #[derive(Clone, Copy, PartialEq, ::prost::Message)]
        pub struct DeleteTodoResponse {}
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[derive(sqlx::Type)]
        #[derive(strum::EnumString, strum::Display, strum::EnumIter)]
        #[strum(ascii_case_insensitive, serialize_all = "snake_case")]
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
                    Self::Doing => "TODO_STATUS_DOING",
                    Self::Done => "TODO_STATUS_DONE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TODO_STATUS_DOING" => Some(Self::Doing),
                    "TODO_STATUS_DONE" => Some(Self::Done),
                    _ => None,
                }
            }
        }
        /// Generated client implementations.
        pub mod todo_service_client {
            #![allow(
                unused_variables,
                dead_code,
                missing_docs,
                clippy::wildcard_imports,
                clippy::let_unit_value,
            )]
            use tonic::codegen::*;
            use tonic::codegen::http::Uri;
            #[derive(Debug, Clone)]
            pub struct TodoServiceClient<T> {
                inner: tonic::client::Grpc<T>,
            }
            impl TodoServiceClient<tonic::transport::Channel> {
                /// Attempt to create a new client by connecting to a given endpoint.
                pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
                where
                    D: TryInto<tonic::transport::Endpoint>,
                    D::Error: Into<StdError>,
                {
                    let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                    Ok(Self::new(conn))
                }
            }
            impl<T> TodoServiceClient<T>
            where
                T: tonic::client::GrpcService<tonic::body::BoxBody>,
                T::Error: Into<StdError>,
                T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
                <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
            {
                pub fn new(inner: T) -> Self {
                    let inner = tonic::client::Grpc::new(inner);
                    Self { inner }
                }
                pub fn with_origin(inner: T, origin: Uri) -> Self {
                    let inner = tonic::client::Grpc::with_origin(inner, origin);
                    Self { inner }
                }
                pub fn with_interceptor<F>(
                    inner: T,
                    interceptor: F,
                ) -> TodoServiceClient<InterceptedService<T, F>>
                where
                    F: tonic::service::Interceptor,
                    T::ResponseBody: Default,
                    T: tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                        Response = http::Response<
                            <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                        >,
                    >,
                    <T as tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                    >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
                {
                    TodoServiceClient::new(InterceptedService::new(inner, interceptor))
                }
                /// Compress requests with the given encoding.
                ///
                /// This requires the server to support it otherwise it might respond with an
                /// error.
                #[must_use]
                pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                    self.inner = self.inner.send_compressed(encoding);
                    self
                }
                /// Enable decompressing responses.
                #[must_use]
                pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                    self.inner = self.inner.accept_compressed(encoding);
                    self
                }
                /// Limits the maximum size of a decoded message.
                ///
                /// Default: `4MB`
                #[must_use]
                pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                    self.inner = self.inner.max_decoding_message_size(limit);
                    self
                }
                /// Limits the maximum size of an encoded message.
                ///
                /// Default: `usize::MAX`
                #[must_use]
                pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                    self.inner = self.inner.max_encoding_message_size(limit);
                    self
                }
                pub async fn get_todos(
                    &mut self,
                    request: impl tonic::IntoRequest<super::GetTodosRequest>,
                ) -> std::result::Result<
                    tonic::Response<tonic::codec::Streaming<super::Todo>>,
                    tonic::Status,
                > {
                    self.inner
                        .ready()
                        .await
                        .map_err(|e| {
                            tonic::Status::unknown(
                                format!("Service was not ready: {}", e.into()),
                            )
                        })?;
                    let codec = tonic::codec::ProstCodec::default();
                    let path = http::uri::PathAndQuery::from_static(
                        "/todo.TodoService/GetTodos",
                    );
                    let mut req = request.into_request();
                    req.extensions_mut().insert(GrpcMethod::new("todo.TodoService", "GetTodos"));
                    self.inner.server_streaming(req, path, codec).await
                }
                pub async fn create_todo(
                    &mut self,
                    request: impl tonic::IntoRequest<super::CreateTodoRequest>,
                ) -> std::result::Result<tonic::Response<super::Todo>, tonic::Status> {
                    self.inner
                        .ready()
                        .await
                        .map_err(|e| {
                            tonic::Status::unknown(
                                format!("Service was not ready: {}", e.into()),
                            )
                        })?;
                    let codec = tonic::codec::ProstCodec::default();
                    let path = http::uri::PathAndQuery::from_static(
                        "/todo.TodoService/CreateTodo",
                    );
                    let mut req = request.into_request();
                    req.extensions_mut()
                        .insert(GrpcMethod::new("todo.TodoService", "CreateTodo"));
                    self.inner.unary(req, path, codec).await
                }
                pub async fn update_todo(
                    &mut self,
                    request: impl tonic::IntoRequest<super::Todo>,
                ) -> std::result::Result<tonic::Response<super::Todo>, tonic::Status> {
                    self.inner
                        .ready()
                        .await
                        .map_err(|e| {
                            tonic::Status::unknown(
                                format!("Service was not ready: {}", e.into()),
                            )
                        })?;
                    let codec = tonic::codec::ProstCodec::default();
                    let path = http::uri::PathAndQuery::from_static(
                        "/todo.TodoService/UpdateTodo",
                    );
                    let mut req = request.into_request();
                    req.extensions_mut()
                        .insert(GrpcMethod::new("todo.TodoService", "UpdateTodo"));
                    self.inner.unary(req, path, codec).await
                }
                pub async fn delete_todo(
                    &mut self,
                    request: impl tonic::IntoRequest<super::DeleteTodoRequest>,
                ) -> std::result::Result<
                    tonic::Response<super::DeleteTodoResponse>,
                    tonic::Status,
                > {
                    self.inner
                        .ready()
                        .await
                        .map_err(|e| {
                            tonic::Status::unknown(
                                format!("Service was not ready: {}", e.into()),
                            )
                        })?;
                    let codec = tonic::codec::ProstCodec::default();
                    let path = http::uri::PathAndQuery::from_static(
                        "/todo.TodoService/DeleteTodo",
                    );
                    let mut req = request.into_request();
                    req.extensions_mut()
                        .insert(GrpcMethod::new("todo.TodoService", "DeleteTodo"));
                    self.inner.unary(req, path, codec).await
                }
            }
        }
        /// Generated server implementations.
        pub mod todo_service_server {
            #![allow(
                unused_variables,
                dead_code,
                missing_docs,
                clippy::wildcard_imports,
                clippy::let_unit_value,
            )]
            use tonic::codegen::*;
            /// Generated trait containing gRPC methods that should be implemented for use with TodoServiceServer.
            #[async_trait]
            pub trait TodoService: std::marker::Send + std::marker::Sync + 'static {
                /// Server streaming response type for the GetTodos method.
                type GetTodosStream: tonic::codegen::tokio_stream::Stream<
                        Item = std::result::Result<super::Todo, tonic::Status>,
                    >
                    + std::marker::Send
                    + 'static;
                async fn get_todos(
                    &self,
                    request: tonic::Request<super::GetTodosRequest>,
                ) -> std::result::Result<tonic::Response<Self::GetTodosStream>, tonic::Status>;
                async fn create_todo(
                    &self,
                    request: tonic::Request<super::CreateTodoRequest>,
                ) -> std::result::Result<tonic::Response<super::Todo>, tonic::Status>;
                async fn update_todo(
                    &self,
                    request: tonic::Request<super::Todo>,
                ) -> std::result::Result<tonic::Response<super::Todo>, tonic::Status>;
                async fn delete_todo(
                    &self,
                    request: tonic::Request<super::DeleteTodoRequest>,
                ) -> std::result::Result<
                    tonic::Response<super::DeleteTodoResponse>,
                    tonic::Status,
                >;
            }
            #[derive(Debug)]
            pub struct TodoServiceServer<T> {
                inner: Arc<T>,
                accept_compression_encodings: EnabledCompressionEncodings,
                send_compression_encodings: EnabledCompressionEncodings,
                max_decoding_message_size: Option<usize>,
                max_encoding_message_size: Option<usize>,
            }
            impl<T> TodoServiceServer<T> {
                pub fn new(inner: T) -> Self {
                    Self::from_arc(Arc::new(inner))
                }
                pub fn from_arc(inner: Arc<T>) -> Self {
                    Self {
                        inner,
                        accept_compression_encodings: Default::default(),
                        send_compression_encodings: Default::default(),
                        max_decoding_message_size: None,
                        max_encoding_message_size: None,
                    }
                }
                pub fn with_interceptor<F>(
                    inner: T,
                    interceptor: F,
                ) -> InterceptedService<Self, F>
                where
                    F: tonic::service::Interceptor,
                {
                    InterceptedService::new(Self::new(inner), interceptor)
                }
                /// Enable decompressing requests with the given encoding.
                #[must_use]
                pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
                    self.accept_compression_encodings.enable(encoding);
                    self
                }
                /// Compress responses with the given encoding, if the client supports it.
                #[must_use]
                pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
                    self.send_compression_encodings.enable(encoding);
                    self
                }
                /// Limits the maximum size of a decoded message.
                ///
                /// Default: `4MB`
                #[must_use]
                pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
                    self.max_decoding_message_size = Some(limit);
                    self
                }
                /// Limits the maximum size of an encoded message.
                ///
                /// Default: `usize::MAX`
                #[must_use]
                pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
                    self.max_encoding_message_size = Some(limit);
                    self
                }
            }
            impl<T, B> tonic::codegen::Service<http::Request<B>> for TodoServiceServer<T>
            where
                T: TodoService,
                B: Body + std::marker::Send + 'static,
                B::Error: Into<StdError> + std::marker::Send + 'static,
            {
                type Response = http::Response<tonic::body::BoxBody>;
                type Error = std::convert::Infallible;
                type Future = BoxFuture<Self::Response, Self::Error>;
                fn poll_ready(
                    &mut self,
                    _cx: &mut Context<'_>,
                ) -> Poll<std::result::Result<(), Self::Error>> {
                    Poll::Ready(Ok(()))
                }
                fn call(&mut self, req: http::Request<B>) -> Self::Future {
                    match req.uri().path() {
                        "/todo.TodoService/GetTodos" => {
                            #[allow(non_camel_case_types)]
                            struct GetTodosSvc<T: TodoService>(pub Arc<T>);
                            impl<
                                T: TodoService,
                            > tonic::server::ServerStreamingService<super::GetTodosRequest>
                            for GetTodosSvc<T> {
                                type Response = super::Todo;
                                type ResponseStream = T::GetTodosStream;
                                type Future = BoxFuture<
                                    tonic::Response<Self::ResponseStream>,
                                    tonic::Status,
                                >;
                                fn call(
                                    &mut self,
                                    request: tonic::Request<super::GetTodosRequest>,
                                ) -> Self::Future {
                                    let inner = Arc::clone(&self.0);
                                    let fut = async move {
                                        <T as TodoService>::get_todos(&inner, request).await
                                    };
                                    Box::pin(fut)
                                }
                            }
                            let accept_compression_encodings = self.accept_compression_encodings;
                            let send_compression_encodings = self.send_compression_encodings;
                            let max_decoding_message_size = self.max_decoding_message_size;
                            let max_encoding_message_size = self.max_encoding_message_size;
                            let inner = self.inner.clone();
                            let fut = async move {
                                let method = GetTodosSvc(inner);
                                let codec = tonic::codec::ProstCodec::default();
                                let mut grpc = tonic::server::Grpc::new(codec)
                                    .apply_compression_config(
                                        accept_compression_encodings,
                                        send_compression_encodings,
                                    )
                                    .apply_max_message_size_config(
                                        max_decoding_message_size,
                                        max_encoding_message_size,
                                    );
                                let res = grpc.server_streaming(method, req).await;
                                Ok(res)
                            };
                            Box::pin(fut)
                        }
                        "/todo.TodoService/CreateTodo" => {
                            #[allow(non_camel_case_types)]
                            struct CreateTodoSvc<T: TodoService>(pub Arc<T>);
                            impl<
                                T: TodoService,
                            > tonic::server::UnaryService<super::CreateTodoRequest>
                            for CreateTodoSvc<T> {
                                type Response = super::Todo;
                                type Future = BoxFuture<
                                    tonic::Response<Self::Response>,
                                    tonic::Status,
                                >;
                                fn call(
                                    &mut self,
                                    request: tonic::Request<super::CreateTodoRequest>,
                                ) -> Self::Future {
                                    let inner = Arc::clone(&self.0);
                                    let fut = async move {
                                        <T as TodoService>::create_todo(&inner, request).await
                                    };
                                    Box::pin(fut)
                                }
                            }
                            let accept_compression_encodings = self.accept_compression_encodings;
                            let send_compression_encodings = self.send_compression_encodings;
                            let max_decoding_message_size = self.max_decoding_message_size;
                            let max_encoding_message_size = self.max_encoding_message_size;
                            let inner = self.inner.clone();
                            let fut = async move {
                                let method = CreateTodoSvc(inner);
                                let codec = tonic::codec::ProstCodec::default();
                                let mut grpc = tonic::server::Grpc::new(codec)
                                    .apply_compression_config(
                                        accept_compression_encodings,
                                        send_compression_encodings,
                                    )
                                    .apply_max_message_size_config(
                                        max_decoding_message_size,
                                        max_encoding_message_size,
                                    );
                                let res = grpc.unary(method, req).await;
                                Ok(res)
                            };
                            Box::pin(fut)
                        }
                        "/todo.TodoService/UpdateTodo" => {
                            #[allow(non_camel_case_types)]
                            struct UpdateTodoSvc<T: TodoService>(pub Arc<T>);
                            impl<T: TodoService> tonic::server::UnaryService<super::Todo>
                            for UpdateTodoSvc<T> {
                                type Response = super::Todo;
                                type Future = BoxFuture<
                                    tonic::Response<Self::Response>,
                                    tonic::Status,
                                >;
                                fn call(
                                    &mut self,
                                    request: tonic::Request<super::Todo>,
                                ) -> Self::Future {
                                    let inner = Arc::clone(&self.0);
                                    let fut = async move {
                                        <T as TodoService>::update_todo(&inner, request).await
                                    };
                                    Box::pin(fut)
                                }
                            }
                            let accept_compression_encodings = self.accept_compression_encodings;
                            let send_compression_encodings = self.send_compression_encodings;
                            let max_decoding_message_size = self.max_decoding_message_size;
                            let max_encoding_message_size = self.max_encoding_message_size;
                            let inner = self.inner.clone();
                            let fut = async move {
                                let method = UpdateTodoSvc(inner);
                                let codec = tonic::codec::ProstCodec::default();
                                let mut grpc = tonic::server::Grpc::new(codec)
                                    .apply_compression_config(
                                        accept_compression_encodings,
                                        send_compression_encodings,
                                    )
                                    .apply_max_message_size_config(
                                        max_decoding_message_size,
                                        max_encoding_message_size,
                                    );
                                let res = grpc.unary(method, req).await;
                                Ok(res)
                            };
                            Box::pin(fut)
                        }
                        "/todo.TodoService/DeleteTodo" => {
                            #[allow(non_camel_case_types)]
                            struct DeleteTodoSvc<T: TodoService>(pub Arc<T>);
                            impl<
                                T: TodoService,
                            > tonic::server::UnaryService<super::DeleteTodoRequest>
                            for DeleteTodoSvc<T> {
                                type Response = super::DeleteTodoResponse;
                                type Future = BoxFuture<
                                    tonic::Response<Self::Response>,
                                    tonic::Status,
                                >;
                                fn call(
                                    &mut self,
                                    request: tonic::Request<super::DeleteTodoRequest>,
                                ) -> Self::Future {
                                    let inner = Arc::clone(&self.0);
                                    let fut = async move {
                                        <T as TodoService>::delete_todo(&inner, request).await
                                    };
                                    Box::pin(fut)
                                }
                            }
                            let accept_compression_encodings = self.accept_compression_encodings;
                            let send_compression_encodings = self.send_compression_encodings;
                            let max_decoding_message_size = self.max_decoding_message_size;
                            let max_encoding_message_size = self.max_encoding_message_size;
                            let inner = self.inner.clone();
                            let fut = async move {
                                let method = DeleteTodoSvc(inner);
                                let codec = tonic::codec::ProstCodec::default();
                                let mut grpc = tonic::server::Grpc::new(codec)
                                    .apply_compression_config(
                                        accept_compression_encodings,
                                        send_compression_encodings,
                                    )
                                    .apply_max_message_size_config(
                                        max_decoding_message_size,
                                        max_encoding_message_size,
                                    );
                                let res = grpc.unary(method, req).await;
                                Ok(res)
                            };
                            Box::pin(fut)
                        }
                        _ => {
                            Box::pin(async move {
                                let mut response = http::Response::new(empty_body());
                                let headers = response.headers_mut();
                                headers
                                    .insert(
                                        tonic::Status::GRPC_STATUS,
                                        (tonic::Code::Unimplemented as i32).into(),
                                    );
                                headers
                                    .insert(
                                        http::header::CONTENT_TYPE,
                                        tonic::metadata::GRPC_CONTENT_TYPE,
                                    );
                                Ok(response)
                            })
                        }
                    }
                }
            }
            impl<T> Clone for TodoServiceServer<T> {
                fn clone(&self) -> Self {
                    let inner = self.inner.clone();
                    Self {
                        inner,
                        accept_compression_encodings: self.accept_compression_encodings,
                        send_compression_encodings: self.send_compression_encodings,
                        max_decoding_message_size: self.max_decoding_message_size,
                        max_encoding_message_size: self.max_encoding_message_size,
                    }
                }
            }
            /// Generated gRPC service name
            pub const SERVICE_NAME: &str = "todo.TodoService";
            impl<T> tonic::server::NamedService for TodoServiceServer<T> {
                const NAME: &'static str = SERVICE_NAME;
            }
        }
        "###);
    }
}
