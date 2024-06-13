//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-uniform-type-identifiers")]
#[cfg(target_vendor = "apple")]
use objc2_uniform_type_identifiers::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAdaptiveImageGlyph;

    unsafe impl ClassType for NSAdaptiveImageGlyph {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSAdaptiveImageGlyph {}

unsafe impl Sync for NSAdaptiveImageGlyph {}

unsafe impl CTAdaptiveImageProviding for NSAdaptiveImageGlyph {}

unsafe impl NSCoding for NSAdaptiveImageGlyph {}

unsafe impl NSCopying for NSAdaptiveImageGlyph {}

unsafe impl NSObjectProtocol for NSAdaptiveImageGlyph {}

unsafe impl NSSecureCoding for NSAdaptiveImageGlyph {}

extern_methods!(
    unsafe impl NSAdaptiveImageGlyph {
        #[method_id(@__retain_semantics Init initWithImageContent:)]
        pub unsafe fn initWithImageContent(
            this: Allocated<Self>,
            image_content: &NSData,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other imageContent)]
        pub unsafe fn imageContent(&self) -> Retained<NSData>;

        #[method_id(@__retain_semantics Other contentIdentifier)]
        pub unsafe fn contentIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other contentDescription)]
        pub unsafe fn contentDescription(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Other contentType)]
        pub unsafe fn contentType() -> Retained<UTType>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAdaptiveImageGlyph {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category on [`NSAttributedString`].
    pub unsafe trait NSAttributedStringAdaptiveImageGlyphConveniences {
        #[method_id(@__retain_semantics Other attributedStringWithAdaptiveImageGlyph:attributes:)]
        unsafe fn attributedStringWithAdaptiveImageGlyph_attributes(
            adaptive_image_glyph: &NSAdaptiveImageGlyph,
            attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Retained<Self>;
    }

    unsafe impl NSAttributedStringAdaptiveImageGlyphConveniences for NSAttributedString {}
);
