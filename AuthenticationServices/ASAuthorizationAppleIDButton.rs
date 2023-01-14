//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationAppleIDButtonType {
        ASAuthorizationAppleIDButtonTypeSignIn = 0,
        ASAuthorizationAppleIDButtonTypeContinue = 1,
        ASAuthorizationAppleIDButtonTypeSignUp = 2,
        ASAuthorizationAppleIDButtonTypeDefault = ASAuthorizationAppleIDButtonTypeSignIn,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASAuthorizationAppleIDButtonStyle {
        ASAuthorizationAppleIDButtonStyleWhite = 0,
        ASAuthorizationAppleIDButtonStyleWhiteOutline = 1,
        ASAuthorizationAppleIDButtonStyleBlack = 2,
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDButton")]
    unsafe impl ASAuthorizationAppleIDButton {
        #[method_id(@__retain_semantics Other buttonWithType:style:)]
        pub unsafe fn buttonWithType_style(
            r#type: ASAuthorizationAppleIDButtonType,
            style: ASAuthorizationAppleIDButtonStyle,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithAuthorizationButtonType:authorizationButtonStyle:)]
        pub unsafe fn initWithAuthorizationButtonType_authorizationButtonStyle(
            this: Option<Allocated<Self>>,
            r#type: ASAuthorizationAppleIDButtonType,
            style: ASAuthorizationAppleIDButtonStyle,
        ) -> Id<Self, Shared>;

        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);
    }
);
