//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(feature = "NSString")]
pub type NSErrorDomain = NSString;

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSCocoaErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSPOSIXErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSOSStatusErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSMachErrorDomain: &'static NSErrorDomain;
}

#[cfg(feature = "NSString")]
pub type NSErrorUserInfoKey = NSString;

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSUnderlyingErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSMultipleUnderlyingErrorsKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSLocalizedDescriptionKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSLocalizedFailureReasonErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSLocalizedRecoverySuggestionErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSLocalizedRecoveryOptionsErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSRecoveryAttempterErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSHelpAnchorErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSDebugDescriptionErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSLocalizedFailureErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSStringEncodingErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSURLErrorKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSFilePathErrorKey: &'static NSErrorUserInfoKey;
}

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSError;

    unsafe impl ClassType for NSError {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSError {}

unsafe impl Sync for NSError {}

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSError {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSError {}

unsafe impl NSObjectProtocol for NSError {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSError {}

extern_methods!(
    unsafe impl NSError {
        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Init initWithDomain:code:userInfo:)]
        pub unsafe fn initWithDomain_code_userInfo(
            this: Allocated<Self>,
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other errorWithDomain:code:userInfo:)]
        pub unsafe fn errorWithDomain_code_userInfo(
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, AnyObject>>,
        ) -> Retained<Self>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other domain)]
        pub fn domain(&self) -> Retained<NSErrorDomain>;

        #[method(code)]
        pub fn code(&self) -> NSInteger;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub fn userInfo(&self) -> Retained<NSDictionary<NSErrorUserInfoKey, AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedDescription)]
        pub fn localizedDescription(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedFailureReason)]
        pub unsafe fn localizedFailureReason(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other localizedRecoverySuggestion)]
        pub unsafe fn localizedRecoverySuggestion(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other localizedRecoveryOptions)]
        pub unsafe fn localizedRecoveryOptions(&self) -> Option<Retained<NSArray<NSString>>>;

        #[method_id(@__retain_semantics Other recoveryAttempter)]
        pub unsafe fn recoveryAttempter(&self) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other underlyingErrors)]
        pub unsafe fn underlyingErrors(&self) -> Retained<NSArray<NSError>>;

        #[cfg(feature = "NSString")]
        #[method(setUserInfoValueProviderForDomain:provider:)]
        pub unsafe fn setUserInfoValueProviderForDomain_provider(
            error_domain: &NSErrorDomain,
            provider: Unknown,
        );

        #[cfg(feature = "NSString")]
        #[method(userInfoValueProviderForDomain:)]
        pub unsafe fn userInfoValueProviderForDomain(error_domain: &NSErrorDomain) -> Unknown;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSError {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_category!(
    /// Category "NSErrorRecoveryAttempting" on [`NSObject`].
    #[doc(alias = "NSErrorRecoveryAttempting")]
    pub unsafe trait NSObjectNSErrorRecoveryAttempting {
        #[method(attemptRecoveryFromError:optionIndex:delegate:didRecoverSelector:contextInfo:)]
        unsafe fn attemptRecoveryFromError_optionIndex_delegate_didRecoverSelector_contextInfo(
            &self,
            error: &NSError,
            recovery_option_index: NSUInteger,
            delegate: Option<&AnyObject>,
            did_recover_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(attemptRecoveryFromError:optionIndex:)]
        unsafe fn attemptRecoveryFromError_optionIndex(
            &self,
            error: &NSError,
            recovery_option_index: NSUInteger,
        ) -> bool;
    }

    unsafe impl NSObjectNSErrorRecoveryAttempting for NSObject {}
);
