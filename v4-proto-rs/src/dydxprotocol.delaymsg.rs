// This file is @generated by prost-build.
/// BlockMessageIds stores the id of each message that should be processed at a
/// given block height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMessageIds {
    /// ids stores a list of DelayedMessage ids that should be processed at a given
    /// block height.
    #[prost(uint32, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for BlockMessageIds {
    const NAME: &'static str = "BlockMessageIds";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.BlockMessageIds".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.BlockMessageIds".into()
    }
}
/// DelayedMessage is a message that is delayed until a certain block height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelayedMessage {
    /// The ID of the delayed message.
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// The message to be executed.
    #[prost(message, optional, tag = "2")]
    pub msg: ::core::option::Option<::prost_types::Any>,
    /// The block height at which the message should be executed.
    #[prost(uint32, tag = "3")]
    pub block_height: u32,
}
impl ::prost::Name for DelayedMessage {
    const NAME: &'static str = "DelayedMessage";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.DelayedMessage".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.DelayedMessage".into()
    }
}
/// GenesisState defines the delaymsg module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// delayed_messages is a list of delayed messages.
    #[prost(message, repeated, tag = "1")]
    pub delayed_messages: ::prost::alloc::vec::Vec<DelayedMessage>,
    /// next_delayed_message_id is the id to be assigned to next delayed message.
    #[prost(uint32, tag = "2")]
    pub next_delayed_message_id: u32,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.GenesisState".into()
    }
}
/// QueryNextDelayedMessageIdRequest is the request type for the
/// NextDelayedMessageId RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextDelayedMessageIdRequest {}
impl ::prost::Name for QueryNextDelayedMessageIdRequest {
    const NAME: &'static str = "QueryNextDelayedMessageIdRequest";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.QueryNextDelayedMessageIdRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.QueryNextDelayedMessageIdRequest".into()
    }
}
/// QueryNextDelayedMessageIdResponse is the response type for the
/// NextDelayedMessageId RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextDelayedMessageIdResponse {
    #[prost(uint32, tag = "1")]
    pub next_delayed_message_id: u32,
}
impl ::prost::Name for QueryNextDelayedMessageIdResponse {
    const NAME: &'static str = "QueryNextDelayedMessageIdResponse";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.QueryNextDelayedMessageIdResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.QueryNextDelayedMessageIdResponse".into()
    }
}
/// QueryMessageRequest is the request type for the Message RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMessageRequest {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
impl ::prost::Name for QueryMessageRequest {
    const NAME: &'static str = "QueryMessageRequest";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.QueryMessageRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.QueryMessageRequest".into()
    }
}
/// QueryGetMessageResponse is the response type for the Message RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMessageResponse {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<DelayedMessage>,
}
impl ::prost::Name for QueryMessageResponse {
    const NAME: &'static str = "QueryMessageResponse";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.QueryMessageResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.QueryMessageResponse".into()
    }
}
/// QueryBlockMessageIdsRequest is the request type for the BlockMessageIds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockMessageIdsRequest {
    #[prost(uint32, tag = "1")]
    pub block_height: u32,
}
impl ::prost::Name for QueryBlockMessageIdsRequest {
    const NAME: &'static str = "QueryBlockMessageIdsRequest";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.QueryBlockMessageIdsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.QueryBlockMessageIdsRequest".into()
    }
}
/// QueryGetBlockMessageIdsResponse is the response type for the BlockMessageIds
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockMessageIdsResponse {
    #[prost(uint32, repeated, tag = "1")]
    pub message_ids: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for QueryBlockMessageIdsResponse {
    const NAME: &'static str = "QueryBlockMessageIdsResponse";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.QueryBlockMessageIdsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.QueryBlockMessageIdsResponse".into()
    }
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> QueryClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        /// Queries the next DelayedMessage's id.
        pub async fn next_delayed_message_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNextDelayedMessageIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryNextDelayedMessageIdResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.delaymsg.Query/NextDelayedMessageId",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "dydxprotocol.delaymsg.Query",
                        "NextDelayedMessageId",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Queries the DelayedMessage by id.
        pub async fn message(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryMessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.delaymsg.Query/Message",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.delaymsg.Query", "Message"));
            self.inner.unary(req, path, codec).await
        }
        /// Queries the DelayedMessages at a given block height.
        pub async fn block_message_ids(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBlockMessageIdsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBlockMessageIdsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.delaymsg.Query/BlockMessageIds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("dydxprotocol.delaymsg.Query", "BlockMessageIds"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// MsgDelayMessage is a request type for the DelayMessage method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelayMessage {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The message to be delayed.
    #[prost(message, optional, tag = "2")]
    pub msg: ::core::option::Option<::prost_types::Any>,
    /// The number of blocks to delay the message for.
    #[prost(uint32, tag = "3")]
    pub delay_blocks: u32,
}
impl ::prost::Name for MsgDelayMessage {
    const NAME: &'static str = "MsgDelayMessage";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.MsgDelayMessage".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.MsgDelayMessage".into()
    }
}
/// MsgDelayMessageResponse is a response type for the DelayMessage method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelayMessageResponse {
    /// The id of the created delayed message.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for MsgDelayMessageResponse {
    const NAME: &'static str = "MsgDelayMessageResponse";
    const PACKAGE: &'static str = "dydxprotocol.delaymsg";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.delaymsg.MsgDelayMessageResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.delaymsg.MsgDelayMessageResponse".into()
    }
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> MsgClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        /// DelayMessage delays the execution of a message for a given number of
        /// blocks.
        pub async fn delay_message(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelayMessage>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDelayMessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/dydxprotocol.delaymsg.Msg/DelayMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.delaymsg.Msg", "DelayMessage"));
            self.inner.unary(req, path, codec).await
        }
    }
}
