//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        NSShowControlGlyphs = 1 << 0,
        NSShowInvisibleGlyphs = 1 << 1,
        NSWantsBidiLevels = 1 << 2,
    }
);

extern_protocol!(
    pub unsafe trait NSGlyphStorage {
        #[method(insertGlyphs:length:forStartingGlyphAtIndex:characterIndex:)]
        unsafe fn insertGlyphs_length_forStartingGlyphAtIndex_characterIndex(
            &self,
            glyphs: NonNull<NSGlyph>,
            length: NSUInteger,
            glyph_index: NSUInteger,
            char_index: NSUInteger,
        );

        #[method(setIntAttribute:value:forGlyphAtIndex:)]
        unsafe fn setIntAttribute_value_forGlyphAtIndex(
            &self,
            attribute_tag: NSInteger,
            val: NSInteger,
            glyph_index: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        unsafe fn attributedString(&self) -> Id<NSAttributedString>;

        #[method(layoutOptions)]
        unsafe fn layoutOptions(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn NSGlyphStorage {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGlyphGenerator")]
    pub struct NSGlyphGenerator;

    #[cfg(feature = "AppKit_NSGlyphGenerator")]
    unsafe impl ClassType for NSGlyphGenerator {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSGlyphGenerator")]
unsafe impl NSObjectProtocol for NSGlyphGenerator {}

extern_methods!(
    #[cfg(feature = "AppKit_NSGlyphGenerator")]
    unsafe impl NSGlyphGenerator {
        #[method(generateGlyphsForGlyphStorage:desiredNumberOfCharacters:glyphIndex:characterIndex:)]
        pub unsafe fn generateGlyphsForGlyphStorage_desiredNumberOfCharacters_glyphIndex_characterIndex(
            &self,
            glyph_storage: &(impl NSGlyphStorage + Message),
            n_chars: NSUInteger,
            glyph_index: *mut NSUInteger,
            char_index: *mut NSUInteger,
        );

        #[method_id(@__retain_semantics Other sharedGlyphGenerator)]
        pub unsafe fn sharedGlyphGenerator() -> Id<NSGlyphGenerator>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSGlyphGenerator")]
    unsafe impl NSGlyphGenerator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
