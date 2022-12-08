//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct ASAccountAuthenticationModificationViewController;

    unsafe impl ClassType for ASAccountAuthenticationModificationViewController {
        type Super = ASViewController;
    }
);

extern_methods!(
    unsafe impl ASAccountAuthenticationModificationViewController {
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(
            &self,
        ) -> Id<ASAccountAuthenticationModificationExtensionContext, Shared>;

        #[method(convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier:existingCredential:userInfo:)]
        pub unsafe fn convertAccountToSignInWithAppleWithoutUserInteractionForServiceIdentifier_existingCredential_userInfo(
            &self,
            serviceIdentifier: &ASCredentialServiceIdentifier,
            existingCredential: &ASPasswordCredential,
            userInfo: Option<&NSDictionary>,
        );

        #[method(prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier:existingCredential:userInfo:)]
        pub unsafe fn prepareInterfaceToConvertAccountToSignInWithAppleForServiceIdentifier_existingCredential_userInfo(
            &self,
            serviceIdentifier: &ASCredentialServiceIdentifier,
            existingCredential: &ASPasswordCredential,
            userInfo: Option<&NSDictionary>,
        );

        #[method(changePasswordWithoutUserInteractionForServiceIdentifier:existingCredential:newPassword:userInfo:)]
        pub unsafe fn changePasswordWithoutUserInteractionForServiceIdentifier_existingCredential_newPassword_userInfo(
            &self,
            serviceIdentifier: &ASCredentialServiceIdentifier,
            existingCredential: &ASPasswordCredential,
            newPassword: &NSString,
            userInfo: Option<&NSDictionary>,
        );

        #[method(prepareInterfaceToChangePasswordForServiceIdentifier:existingCredential:newPassword:userInfo:)]
        pub unsafe fn prepareInterfaceToChangePasswordForServiceIdentifier_existingCredential_newPassword_userInfo(
            &self,
            serviceIdentifier: &ASCredentialServiceIdentifier,
            existingCredential: &ASPasswordCredential,
            newPassword: &NSString,
            userInfo: Option<&NSDictionary>,
        );

        #[method(cancelRequest)]
        pub unsafe fn cancelRequest(&self);
    }
);
