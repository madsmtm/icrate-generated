//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSURLAuthenticationChallengeSender: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSURLAuthenticationChallenge",
            feature = "Foundation_NSURLCredential"
        ))]
        #[method(useCredential:forAuthenticationChallenge:)]
        unsafe fn useCredential_forAuthenticationChallenge(
            &self,
            credential: &NSURLCredential,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
        #[method(continueWithoutCredentialForAuthenticationChallenge:)]
        unsafe fn continueWithoutCredentialForAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
        #[method(cancelAuthenticationChallenge:)]
        unsafe fn cancelAuthenticationChallenge(&self, challenge: &NSURLAuthenticationChallenge);

        #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
        #[optional]
        #[method(performDefaultHandlingForAuthenticationChallenge:)]
        unsafe fn performDefaultHandlingForAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
        #[optional]
        #[method(rejectProtectionSpaceAndContinueWithChallenge:)]
        unsafe fn rejectProtectionSpaceAndContinueWithChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );
    }

    unsafe impl ProtocolType for dyn NSURLAuthenticationChallengeSender {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
    pub struct NSURLAuthenticationChallenge;

    #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
    unsafe impl ClassType for NSURLAuthenticationChallenge {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
unsafe impl NSCoding for NSURLAuthenticationChallenge {}

#[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
unsafe impl NSObjectProtocol for NSURLAuthenticationChallenge {}

#[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
unsafe impl NSSecureCoding for NSURLAuthenticationChallenge {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
    unsafe impl NSURLAuthenticationChallenge {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSURLCredential",
            feature = "Foundation_NSURLProtectionSpace",
            feature = "Foundation_NSURLResponse"
        ))]
        #[method_id(@__retain_semantics Init initWithProtectionSpace:proposedCredential:previousFailureCount:failureResponse:error:sender:)]
        pub unsafe fn initWithProtectionSpace_proposedCredential_previousFailureCount_failureResponse_error_sender(
            this: Option<Allocated<Self>>,
            space: &NSURLProtectionSpace,
            credential: Option<&NSURLCredential>,
            previous_failure_count: NSInteger,
            response: Option<&NSURLResponse>,
            error: Option<&NSError>,
            sender: &(impl NSURLAuthenticationChallengeSender + Message),
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithAuthenticationChallenge:sender:)]
        pub unsafe fn initWithAuthenticationChallenge_sender(
            this: Option<Allocated<Self>>,
            challenge: &NSURLAuthenticationChallenge,
            sender: &(impl NSURLAuthenticationChallengeSender + Message),
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURLProtectionSpace")]
        #[method_id(@__retain_semantics Other protectionSpace)]
        pub unsafe fn protectionSpace(&self) -> Id<NSURLProtectionSpace>;

        #[cfg(feature = "Foundation_NSURLCredential")]
        #[method_id(@__retain_semantics Other proposedCredential)]
        pub unsafe fn proposedCredential(&self) -> Option<Id<NSURLCredential>>;

        #[method(previousFailureCount)]
        pub unsafe fn previousFailureCount(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSURLResponse")]
        #[method_id(@__retain_semantics Other failureResponse)]
        pub unsafe fn failureResponse(&self) -> Option<Id<NSURLResponse>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;

        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSURLAuthenticationChallengeSender>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLAuthenticationChallenge")]
    unsafe impl NSURLAuthenticationChallenge {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
