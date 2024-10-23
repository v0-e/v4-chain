// This file is @generated by prost-build.
/// AffiliateTiers defines the affiliate tiers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateTiers {
    /// All affiliate tiers
    #[prost(message, repeated, tag = "1")]
    pub tiers: ::prost::alloc::vec::Vec<affiliate_tiers::Tier>,
}
/// Nested message and enum types in `AffiliateTiers`.
pub mod affiliate_tiers {
    /// Tier defines an affiliate tier.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tier {
        /// Required all-time referred volume in quote quantums.
        #[prost(uint64, tag = "1")]
        pub req_referred_volume_quote_quantums: u64,
        /// Required currently staked native tokens (in whole coins).
        #[prost(uint32, tag = "2")]
        pub req_staked_whole_coins: u32,
        /// Taker fee share in parts-per-million.
        #[prost(uint32, tag = "3")]
        pub taker_fee_share_ppm: u32,
    }
    impl ::prost::Name for Tier {
        const NAME: &'static str = "Tier";
        const PACKAGE: &'static str = "dydxprotocol.affiliates";
        fn full_name() -> ::prost::alloc::string::String {
            "dydxprotocol.affiliates.AffiliateTiers.Tier".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/dydxprotocol.affiliates.AffiliateTiers.Tier".into()
        }
    }
}
impl ::prost::Name for AffiliateTiers {
    const NAME: &'static str = "AffiliateTiers";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AffiliateTiers".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AffiliateTiers".into()
    }
}
/// AffiliateWhitelist specifies the whitelisted affiliates.
/// If an address is in the whitelist, then the affiliate fee share in
/// this object will override fee share from the regular affiliate tiers above.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateWhitelist {
    /// All affiliate whitelist tiers.
    #[prost(message, repeated, tag = "1")]
    pub tiers: ::prost::alloc::vec::Vec<affiliate_whitelist::Tier>,
}
/// Nested message and enum types in `AffiliateWhitelist`.
pub mod affiliate_whitelist {
    /// Tier defines an affiliate whitelist tier.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tier {
        /// List of unique whitelisted addresses.
        #[prost(string, repeated, tag = "1")]
        pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Taker fee share in parts-per-million.
        #[prost(uint32, tag = "2")]
        pub taker_fee_share_ppm: u32,
    }
    impl ::prost::Name for Tier {
        const NAME: &'static str = "Tier";
        const PACKAGE: &'static str = "dydxprotocol.affiliates";
        fn full_name() -> ::prost::alloc::string::String {
            "dydxprotocol.affiliates.AffiliateWhitelist.Tier".into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/dydxprotocol.affiliates.AffiliateWhitelist.Tier".into()
        }
    }
}
impl ::prost::Name for AffiliateWhitelist {
    const NAME: &'static str = "AffiliateWhitelist";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AffiliateWhitelist".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AffiliateWhitelist".into()
    }
}
/// GenesisState defines generis state of `x/affiliates`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// The list of affiliate tiers
    #[prost(message, optional, tag = "1")]
    pub affiliate_tiers: ::core::option::Option<AffiliateTiers>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.GenesisState".into()
    }
}
/// AffiliateInfoRequest is the request type for the Query/AffiliateInfo RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateInfoRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for AffiliateInfoRequest {
    const NAME: &'static str = "AffiliateInfoRequest";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AffiliateInfoRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AffiliateInfoRequest".into()
    }
}
/// AffiliateInfoResponse is the response type for the Query/AffiliateInfo RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateInfoResponse {
    /// Whether the address is a whitelisted affiliate (VIP).
    #[prost(bool, tag = "1")]
    pub is_whitelisted: bool,
    /// If `is_whiteslisted == false`, the affiliate's tier qualified through
    /// regular affiliate program.
    #[prost(uint32, tag = "2")]
    pub tier: u32,
    /// The affiliate's taker fee share in parts-per-million (for both VIP and
    /// regular affiliate).
    #[prost(uint32, tag = "3")]
    pub fee_share_ppm: u32,
    /// The affiliate's all-time referred volume in quote quantums.
    #[prost(bytes = "vec", tag = "4")]
    pub referred_volume: ::prost::alloc::vec::Vec<u8>,
    /// The affiliate's currently staked native tokens (in whole coins).
    #[prost(bytes = "vec", tag = "5")]
    pub staked_amount: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for AffiliateInfoResponse {
    const NAME: &'static str = "AffiliateInfoResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AffiliateInfoResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AffiliateInfoResponse".into()
    }
}
/// ReferredByRequest is the request type for the Query/ReferredBy RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferredByRequest {
    /// The address to query.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for ReferredByRequest {
    const NAME: &'static str = "ReferredByRequest";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.ReferredByRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.ReferredByRequest".into()
    }
}
/// ReferredByResponse is the response type for the Query/ReferredBy RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReferredByResponse {
    /// The affiliate's address that referred the queried address.
    #[prost(string, tag = "1")]
    pub affiliate_address: ::prost::alloc::string::String,
}
impl ::prost::Name for ReferredByResponse {
    const NAME: &'static str = "ReferredByResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.ReferredByResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.ReferredByResponse".into()
    }
}
/// AllAffiliateTiersRequest is the request type for the Query/AllAffiliateTiers
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAffiliateTiersRequest {}
impl ::prost::Name for AllAffiliateTiersRequest {
    const NAME: &'static str = "AllAffiliateTiersRequest";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AllAffiliateTiersRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AllAffiliateTiersRequest".into()
    }
}
/// AllAffiliateTiersResponse is the response type for the
/// Query/AllAffiliateTiers RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllAffiliateTiersResponse {
    /// All affiliate tiers information.
    #[prost(message, optional, tag = "1")]
    pub tiers: ::core::option::Option<AffiliateTiers>,
}
impl ::prost::Name for AllAffiliateTiersResponse {
    const NAME: &'static str = "AllAffiliateTiersResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AllAffiliateTiersResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AllAffiliateTiersResponse".into()
    }
}
/// AffiliateWhitelistRequest is the request type for the
/// Query/AffiliateWhitelist RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateWhitelistRequest {}
impl ::prost::Name for AffiliateWhitelistRequest {
    const NAME: &'static str = "AffiliateWhitelistRequest";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AffiliateWhitelistRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AffiliateWhitelistRequest".into()
    }
}
/// AffiliateWhitelistResponse is the response type for the
/// Query/AffiliateWhitelist RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateWhitelistResponse {
    #[prost(message, optional, tag = "1")]
    pub whitelist: ::core::option::Option<AffiliateWhitelist>,
}
impl ::prost::Name for AffiliateWhitelistResponse {
    const NAME: &'static str = "AffiliateWhitelistResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.AffiliateWhitelistResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.AffiliateWhitelistResponse".into()
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
        /// Query AffiliateInfo returns the affiliate info for a given address.
        pub async fn affiliate_info(
            &mut self,
            request: impl tonic::IntoRequest<super::AffiliateInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AffiliateInfoResponse>,
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
                "/dydxprotocol.affiliates.Query/AffiliateInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("dydxprotocol.affiliates.Query", "AffiliateInfo"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query ReferredBy returns the affiliate that referred a given address.
        pub async fn referred_by(
            &mut self,
            request: impl tonic::IntoRequest<super::ReferredByRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReferredByResponse>,
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
                "/dydxprotocol.affiliates.Query/ReferredBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("dydxprotocol.affiliates.Query", "ReferredBy"));
            self.inner.unary(req, path, codec).await
        }
        /// Query AllAffiliateTiers returns all affiliate tiers.
        pub async fn all_affiliate_tiers(
            &mut self,
            request: impl tonic::IntoRequest<super::AllAffiliateTiersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AllAffiliateTiersResponse>,
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
                "/dydxprotocol.affiliates.Query/AllAffiliateTiers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("dydxprotocol.affiliates.Query", "AllAffiliateTiers"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Query AffiliateWhitelist returns the affiliate whitelist.
        pub async fn affiliate_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::AffiliateWhitelistRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AffiliateWhitelistResponse>,
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
                "/dydxprotocol.affiliates.Query/AffiliateWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "dydxprotocol.affiliates.Query",
                        "AffiliateWhitelist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Message to register a referee-affiliate relationship
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAffiliate {
    /// Address of the referee
    #[prost(string, tag = "1")]
    pub referee: ::prost::alloc::string::String,
    /// Address of the affiliate
    #[prost(string, tag = "2")]
    pub affiliate: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRegisterAffiliate {
    const NAME: &'static str = "MsgRegisterAffiliate";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.MsgRegisterAffiliate".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.MsgRegisterAffiliate".into()
    }
}
/// Response to MsgRegisterAffiliate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAffiliateResponse {}
impl ::prost::Name for MsgRegisterAffiliateResponse {
    const NAME: &'static str = "MsgRegisterAffiliateResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.MsgRegisterAffiliateResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.MsgRegisterAffiliateResponse".into()
    }
}
/// Message to update affiliate tiers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAffiliateTiers {
    /// Authority sending this message. Will be sent by gov
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Updated affiliate tiers information
    #[prost(message, optional, tag = "2")]
    pub tiers: ::core::option::Option<AffiliateTiers>,
}
impl ::prost::Name for MsgUpdateAffiliateTiers {
    const NAME: &'static str = "MsgUpdateAffiliateTiers";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.MsgUpdateAffiliateTiers".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.MsgUpdateAffiliateTiers".into()
    }
}
/// Response to MsgUpdateAffiliateTiers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAffiliateTiersResponse {}
impl ::prost::Name for MsgUpdateAffiliateTiersResponse {
    const NAME: &'static str = "MsgUpdateAffiliateTiersResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.MsgUpdateAffiliateTiersResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.MsgUpdateAffiliateTiersResponse".into()
    }
}
/// Message to update affiliate whitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAffiliateWhitelist {
    /// Authority sending this message. Will be sent by gov
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Updated affiliate whitelist information
    #[prost(message, optional, tag = "2")]
    pub whitelist: ::core::option::Option<AffiliateWhitelist>,
}
impl ::prost::Name for MsgUpdateAffiliateWhitelist {
    const NAME: &'static str = "MsgUpdateAffiliateWhitelist";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.MsgUpdateAffiliateWhitelist".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.MsgUpdateAffiliateWhitelist".into()
    }
}
/// Response to MsgUpdateAffiliateWhitelist
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateAffiliateWhitelistResponse {}
impl ::prost::Name for MsgUpdateAffiliateWhitelistResponse {
    const NAME: &'static str = "MsgUpdateAffiliateWhitelistResponse";
    const PACKAGE: &'static str = "dydxprotocol.affiliates";
    fn full_name() -> ::prost::alloc::string::String {
        "dydxprotocol.affiliates.MsgUpdateAffiliateWhitelistResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/dydxprotocol.affiliates.MsgUpdateAffiliateWhitelistResponse".into()
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
        /// RegisterAffiliate registers a referee-affiliate relationship
        pub async fn register_affiliate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRegisterAffiliate>,
        ) -> std::result::Result<
            tonic::Response<super::MsgRegisterAffiliateResponse>,
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
                "/dydxprotocol.affiliates.Msg/RegisterAffiliate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("dydxprotocol.affiliates.Msg", "RegisterAffiliate"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateAffiliateTiers updates affiliate tiers
        pub async fn update_affiliate_tiers(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAffiliateTiers>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateAffiliateTiersResponse>,
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
                "/dydxprotocol.affiliates.Msg/UpdateAffiliateTiers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "dydxprotocol.affiliates.Msg",
                        "UpdateAffiliateTiers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateAffiliateWhitelist updates affiliate whitelist
        pub async fn update_affiliate_whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateAffiliateWhitelist>,
        ) -> std::result::Result<
            tonic::Response<super::MsgUpdateAffiliateWhitelistResponse>,
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
                "/dydxprotocol.affiliates.Msg/UpdateAffiliateWhitelist",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "dydxprotocol.affiliates.Msg",
                        "UpdateAffiliateWhitelist",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
