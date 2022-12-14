//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport = NSString;
);

extern_static!(
    ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportUSB:
        &'static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport
);

extern_static!(
    ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportNFC:
        &'static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport
);

extern_static!(
    ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransportBluetooth:
        &'static ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport
);

extern_fn!(
    pub unsafe fn ASAuthorizationAllSupportedPublicKeyCredentialDescriptorTransports(
    ) -> NonNull<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>>;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor;

    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialDescriptor {
        #[method_id(@__retain_semantics Init initWithCredentialID:transports:)]
        pub unsafe fn initWithCredentialID_transports(
            this: Option<Allocated<Self>>,
            credentialID: &NSData,
            allowedTransports: &NSArray<
                ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport,
            >,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other transports)]
        pub unsafe fn transports(
            &self,
        ) -> Id<NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>, Shared>;

        #[method(setTransports:)]
        pub unsafe fn setTransports(
            &self,
            transports: &NSArray<ASAuthorizationSecurityKeyPublicKeyCredentialDescriptorTransport>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
