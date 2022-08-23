/// Certificate schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSchema {
    #[prost(string, repeated, tag="1")]
    pub attribute_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl CertificateSchema {
    pub fn new() -> CertificateSchema {
        ::std::default::Default::default()
    }

    // repeated string attribute_name = 1;


    pub fn get_attribute_name(&self) -> &[::std::string::String] {
        &self.attribute_name
    }
}
/// String to string mapping pairs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringToStringPair {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
impl StringToStringPair {
    pub fn new() -> StringToStringPair {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn get_value(&self) -> &str {
        &self.value
    }
}
/// Certificate template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateTemplate {
    #[prost(message, optional, tag="1")]
    pub certificate_schema: ::core::option::Option<CertificateSchema>,
    #[prost(string, tag="2")]
    pub template_correctness_proof: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub template_public_key: ::core::option::Option<TemplatePublicKey>,
}
impl CertificateTemplate {
    pub fn new() -> CertificateTemplate {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_certificate_schema(&mut self, v: CertificateSchema) {
        self.certificate_schema = Some(v);
    }

    pub fn set_template_public_key(&mut self, v: TemplatePublicKey) {
        self.template_public_key = Some(v);
    }

    // Take field
    pub fn take_certificate_schema(&mut self) -> CertificateSchema {
        self.certificate_schema.take().unwrap_or_else(|| CertificateSchema::new())
    }

    // string template_correctness_proof = 2;
    pub fn get_template_correctness_proof(&self) -> &str {
        &self.template_correctness_proof
    }
    pub fn clear_template_correctness_proof(&mut self) {
        self.template_correctness_proof.clear();
    }

    // Param is passed by value, moved
    pub fn set_template_correctness_proof(&mut self, v: ::std::string::String) {
        self.template_correctness_proof = v;
    }
    pub fn get_template_public_key(&self) -> &TemplatePublicKey {
        self.template_public_key.as_ref().unwrap()
    }
}
/// Template public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplatePublicKey {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
impl TemplatePublicKey {
    pub fn new() -> TemplatePublicKey {
        ::std::default::Default::default()
    }
    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }
}
/// Template private key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemplatePrivateKey {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
impl TemplatePrivateKey {
    pub fn new() -> TemplatePrivateKey {
        ::std::default::Default::default()
    }

    // string key = 1;


    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }
}
/// Certificate attribute dictionary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeDict {
    #[prost(message, repeated, tag="1")]
    pub pair: ::prost::alloc::vec::Vec<StringToStringPair>,
}
impl AttributeDict {
    pub fn new() -> AttributeDict {
        ::std::default::Default::default()
    }

    // repeated .scd.StringToStringPair pair = 1;


    pub fn get_pair(&self) -> &[StringToStringPair] {
        &self.pair
    }
}
/// Certificate secret blind factors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlindedCertificateSecret {
    #[prost(string, tag="1")]
    pub blinded_certificate_secrets: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub blinded_certificate_secrets_correctness_proof: ::prost::alloc::string::String,
}
/// Certificate signature signed by an issuer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSignature {
    #[prost(string, tag="1")]
    pub certificate_signature: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub signature_correctness_proof: ::prost::alloc::string::String,
}
impl CertificateSignature {
    pub fn new() -> CertificateSignature {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_certificate_signature(&mut self, v: ::std::string::String) {
        self.certificate_signature = v;
    }
    // Param is passed by value, moved
    pub fn set_signature_correctness_proof(&mut self, v: ::std::string::String) {
        self.signature_correctness_proof = v;
    }
}
/// Predicate rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Predicate {
    #[prost(string, tag="1")]
    pub attribute_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub predicate_type: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub predicate_value: u64,
}
/// Verification rule set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationRuleSet {
    #[prost(string, repeated, tag="1")]
    pub revealed_attribute_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub attribute_predicate: ::prost::alloc::vec::Vec<Predicate>,
}
/// Request of signing a new certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignCertificateRequest {
    #[prost(message, optional, tag="1")]
    pub certificate_attribute_dict: ::core::option::Option<AttributeDict>,
    #[prost(string, tag="2")]
    pub blinded_certificate_secrets: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub blinded_certificate_secrets_correctness_proof: ::prost::alloc::string::String,
}
impl SignCertificateRequest {
    pub fn new() -> SignCertificateRequest {
        ::std::default::Default::default()
    }

    // .scd.AttributeDict certificate_attribute_dict = 1;


    pub fn get_certificate_attribute_dict(&self) -> &AttributeDict {
        self.certificate_attribute_dict.as_ref().unwrap()
    }
    pub fn get_blinded_certificate_secrets(&self) -> &str {
        &self.blinded_certificate_secrets
    }
    pub fn get_blinded_certificate_secrets_correctness_proof(&self) -> &str {
        &self.blinded_certificate_secrets_correctness_proof
    }
}
/// Request of verifying the selected information from a certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyRequest {
    #[prost(message, optional, tag="1")]
    pub certificate_template: ::core::option::Option<CertificateTemplate>,
    #[prost(string, tag="2")]
    pub verification_proof: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub verification_nonce: ::prost::alloc::string::String,
}
/// Return data to FFI interface used by SCD.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScdResult {
    /// Used by the issuer.
    #[prost(message, optional, tag="1")]
    pub certificate_template: ::core::option::Option<CertificateTemplate>,
    #[prost(message, optional, tag="2")]
    pub template_private_key: ::core::option::Option<TemplatePrivateKey>,
    #[prost(string, tag="3")]
    pub issuer_nonce: ::prost::alloc::string::String,
    /// Used by the user.
    #[prost(message, optional, tag="4")]
    pub certificate_signature: ::core::option::Option<CertificateSignature>,
    #[prost(string, tag="5")]
    pub user_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub sign_certificate_request: ::core::option::Option<SignCertificateRequest>,
    #[prost(string, tag="7")]
    pub user_private_key: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub certificate_secrets_blinding_factors: ::prost::alloc::string::String,
    /// Used by the verifier.
    #[prost(string, tag="9")]
    pub verification_nonce: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub verify_request: ::core::option::Option<VerifyRequest>,
    #[prost(message, optional, tag="11")]
    pub revealed_attribute_dict: ::core::option::Option<AttributeDict>,
    #[prost(bool, tag="12")]
    pub bool_result: bool,
}
/// Generated client implementations.
pub mod selective_disclosure_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SelectiveDisclosureClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SelectiveDisclosureClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SelectiveDisclosureClient<T>
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
        ) -> SelectiveDisclosureClient<InterceptedService<T, F>>
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
            SelectiveDisclosureClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CertificateSchema>,
        ) -> Result<tonic::Response<super::CertificateTemplate>, tonic::Status> {
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
                "/scd.SelectiveDisclosure/CreateTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod selective_disclosure_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SelectiveDisclosureServer.
    #[async_trait]
    pub trait SelectiveDisclosure: Send + Sync + 'static {
        async fn create_template(
            &self,
            request: tonic::Request<super::CertificateSchema>,
        ) -> Result<tonic::Response<super::CertificateTemplate>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SelectiveDisclosureServer<T: SelectiveDisclosure> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SelectiveDisclosure> SelectiveDisclosureServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SelectiveDisclosureServer<T>
    where
        T: SelectiveDisclosure,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/scd.SelectiveDisclosure/CreateTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTemplateSvc<T: SelectiveDisclosure>(pub Arc<T>);
                    impl<
                        T: SelectiveDisclosure,
                    > tonic::server::UnaryService<super::CertificateSchema>
                    for CreateTemplateSvc<T> {
                        type Response = super::CertificateTemplate;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CertificateSchema>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_template(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTemplateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SelectiveDisclosure> Clone for SelectiveDisclosureServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SelectiveDisclosure> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SelectiveDisclosure> tonic::server::NamedService
    for SelectiveDisclosureServer<T> {
        const NAME: &'static str = "scd.SelectiveDisclosure";
    }
}
