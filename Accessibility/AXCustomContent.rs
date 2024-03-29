//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::Foundation::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AXCustomContentImportance(pub NSUInteger);
impl AXCustomContentImportance {
    #[doc(alias = "AXCustomContentImportanceDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "AXCustomContentImportanceHigh")]
    pub const High: Self = Self(1);
}

#[cfg(feature = "objc2")]
unsafe impl Encode for AXCustomContentImportance {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AXCustomContentImportance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AXCustomContent;

    unsafe impl ClassType for AXCustomContent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for AXCustomContent {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for AXCustomContent {}

unsafe impl NSObjectProtocol for AXCustomContent {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for AXCustomContent {}

extern_methods!(
    unsafe impl AXCustomContent {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customContentWithLabel:value:)]
        pub unsafe fn customContentWithLabel_value(label: &NSString, value: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other customContentWithAttributedLabel:attributedValue:)]
        pub unsafe fn customContentWithAttributedLabel_attributedValue(
            label: &NSAttributedString,
            value: &NSAttributedString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedLabel)]
        pub unsafe fn attributedLabel(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedValue)]
        pub unsafe fn attributedValue(&self) -> Id<NSAttributedString>;

        #[method(importance)]
        pub unsafe fn importance(&self) -> AXCustomContentImportance;

        #[method(setImportance:)]
        pub unsafe fn setImportance(&self, importance: AXCustomContentImportance);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSArray")]
pub type AXCustomContentReturnBlock = *mut Block<dyn Fn() -> *mut NSArray<AXCustomContent>>;

extern_protocol!(
    pub unsafe trait AXCustomContentProvider: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other accessibilityCustomContent)]
        unsafe fn accessibilityCustomContent(&self) -> Id<NSArray<AXCustomContent>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAccessibilityCustomContent:)]
        unsafe fn setAccessibilityCustomContent(
            &self,
            accessibility_custom_content: Option<&NSArray<AXCustomContent>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[optional]
        #[method(accessibilityCustomContentBlock)]
        unsafe fn accessibilityCustomContentBlock(&self) -> AXCustomContentReturnBlock;

        #[cfg(feature = "Foundation_NSArray")]
        #[optional]
        #[method(setAccessibilityCustomContentBlock:)]
        unsafe fn setAccessibilityCustomContentBlock(
            &self,
            accessibility_custom_content_block: AXCustomContentReturnBlock,
        );
    }

    unsafe impl ProtocolType for dyn AXCustomContentProvider {}
);
