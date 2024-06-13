//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use `sec_protocol_options_t` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
    pub struct NWTLSParameters;

    unsafe impl ClassType for NWTLSParameters {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NWTLSParameters {}

extern_methods!(
    unsafe impl NWTLSParameters {
        #[deprecated = "Use `sec_protocol_options_set_tls_resumption_enabled` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method_id(@__retain_semantics Other TLSSessionID)]
        pub unsafe fn TLSSessionID(&self) -> Option<Retained<NSData>>;

        #[deprecated = "Use `sec_protocol_options_set_tls_resumption_enabled` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method(setTLSSessionID:)]
        pub unsafe fn setTLSSessionID(&self, tls_session_id: Option<&NSData>);

        #[deprecated = "Use `sec_protocol_options_append_tls_ciphersuite` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method_id(@__retain_semantics Other SSLCipherSuites)]
        pub unsafe fn SSLCipherSuites(&self) -> Option<Retained<NSSet<NSNumber>>>;

        #[deprecated = "Use `sec_protocol_options_append_tls_ciphersuite` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method(setSSLCipherSuites:)]
        pub unsafe fn setSSLCipherSuites(&self, ssl_cipher_suites: Option<&NSSet<NSNumber>>);

        #[deprecated = "Use `sec_protocol_options_set_min_tls_protocol_version` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method(minimumSSLProtocolVersion)]
        pub unsafe fn minimumSSLProtocolVersion(&self) -> NSUInteger;

        #[deprecated = "Use `sec_protocol_options_set_min_tls_protocol_version` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method(setMinimumSSLProtocolVersion:)]
        pub unsafe fn setMinimumSSLProtocolVersion(&self, minimum_ssl_protocol_version: NSUInteger);

        #[deprecated = "Use `sec_protocol_options_set_max_tls_protocol_version` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method(maximumSSLProtocolVersion)]
        pub unsafe fn maximumSSLProtocolVersion(&self) -> NSUInteger;

        #[deprecated = "Use `sec_protocol_options_set_max_tls_protocol_version` in Security framework instead, see deprecation notice in <NetworkExtension/NWTLSParameters.h>"]
        #[method(setMaximumSSLProtocolVersion:)]
        pub unsafe fn setMaximumSSLProtocolVersion(&self, maximum_ssl_protocol_version: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NWTLSParameters {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
