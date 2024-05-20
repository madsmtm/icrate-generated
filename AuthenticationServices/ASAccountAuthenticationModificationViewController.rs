//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

#[cfg(target_os = "macos")]
unsafe impl NSCoding for ASAccountAuthenticationModificationViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSEditor for ASAccountAuthenticationModificationViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for ASAccountAuthenticationModificationViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for ASAccountAuthenticationModificationViewController {}

#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification
    for ASAccountAuthenticationModificationViewController
{
}

extern_methods!(
    #[cfg(target_os = "macos")]
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[cfg(feature = "ASAccountAuthenticationModificationExtensionContext")]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(
            &self,
        ) -> Id<ASAccountAuthenticationModificationExtensionContext>;

        #[cfg(all(
            feature = "ASCredentialServiceIdentifier",
            feature = "ASPasswordCredential"
        ))]
        #[method(convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier:existingCredential:userInfo:)]
        pub unsafe fn convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier_existingCredential_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "ASCredentialServiceIdentifier",
            feature = "ASPasswordCredential"
        ))]
        #[method(prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier:existingCredential:userInfo:)]
        pub unsafe fn prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier_existingCredential_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "ASCredentialServiceIdentifier",
            feature = "ASPasswordCredential"
        ))]
        #[method(changePasswordWithoutUserInteractionForServiceIdentifier:existingCredential:newPassword:userInfo:)]
        pub unsafe fn changePasswordWithoutUserInteractionForServiceIdentifier_existingCredential_newPassword_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            new_password: &NSString,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "ASCredentialServiceIdentifier",
            feature = "ASPasswordCredential"
        ))]
        #[method(prepareInterfaceToChangePasswordForServiceIdentifier:existingCredential:newPassword:userInfo:)]
        pub unsafe fn prepareInterfaceToChangePasswordForServiceIdentifier_existingCredential_newPassword_userInfo(
            &self,
            service_identifier: &ASCredentialServiceIdentifier,
            existing_credential: &ASPasswordCredential,
            new_password: &NSString,
            user_info: Option<&NSDictionary>,
        );

        #[method(cancelRequest)]
        pub unsafe fn cancelRequest(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(target_os = "macos")]
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(target_os = "macos")]
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(target_os = "macos")]
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
