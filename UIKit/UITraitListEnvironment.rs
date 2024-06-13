//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIListEnvironment(pub NSInteger);
impl UIListEnvironment {
    #[doc(alias = "UIListEnvironmentUnspecified")]
    pub const Unspecified: Self = Self(0);
    #[doc(alias = "UIListEnvironmentNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "UIListEnvironmentPlain")]
    pub const Plain: Self = Self(2);
    #[doc(alias = "UIListEnvironmentGrouped")]
    pub const Grouped: Self = Self(3);
    #[doc(alias = "UIListEnvironmentInsetGrouped")]
    pub const InsetGrouped: Self = Self(4);
    #[doc(alias = "UIListEnvironmentSidebar")]
    pub const Sidebar: Self = Self(5);
    #[doc(alias = "UIListEnvironmentSidebarPlain")]
    pub const SidebarPlain: Self = Self(6);
}

unsafe impl Encode for UIListEnvironment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIListEnvironment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITraitListEnvironment;

    unsafe impl ClassType for UITraitListEnvironment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for UITraitListEnvironment {}

#[cfg(feature = "UITrait")]
unsafe impl UINSIntegerTraitDefinition for UITraitListEnvironment {}

#[cfg(feature = "UITrait")]
unsafe impl UITraitDefinition for UITraitListEnvironment {}

extern_methods!(
    unsafe impl UITraitListEnvironment {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UITraitListEnvironment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
