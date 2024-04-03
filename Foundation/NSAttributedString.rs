//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSAttributedStringKey = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSAttributedStringFormattingContextKey = NSString;

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInflectionConceptsKey: &'static NSAttributedStringFormattingContextKey;
}

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSAttributedString;

    unsafe impl ClassType for NSAttributedString {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableAttributedString>;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSAttributedString {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSAttributedString {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSMutableCopying for NSAttributedString {}

unsafe impl NSObjectProtocol for NSAttributedString {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSAttributedString {}

extern_methods!(
    unsafe impl NSAttributedString {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other string)]
        pub fn string(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other attributesAtIndex:effectiveRange:)]
        pub unsafe fn attributesAtIndex_effectiveRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for NSAttributedString {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringEnumerationOptions(pub NSUInteger);
impl NSAttributedStringEnumerationOptions {
    pub const NSAttributedStringEnumerationReverse: Self = Self(1 << 1);
    pub const NSAttributedStringEnumerationLongestEffectiveRangeNotRequired: Self = Self(1 << 20);
}

unsafe impl Encode for NSAttributedStringEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSExtendedAttributedString
    unsafe impl NSAttributedString {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attribute:atIndex:effectiveRange:)]
        pub unsafe fn attribute_atIndex_effectiveRange(
            &self,
            attr_name: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSRange")]
        #[method_id(@__retain_semantics Other attributedSubstringFromRange:)]
        pub unsafe fn attributedSubstringFromRange(&self, range: NSRange)
            -> Id<NSAttributedString>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other attributesAtIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attributesAtIndex_longestEffectiveRange_inRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
            range_limit: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attribute:atIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attribute_atIndex_longestEffectiveRange_inRange(
            &self,
            attr_name: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
            range_limit: NSRange,
        ) -> Option<Id<AnyObject>>;

        #[method(isEqualToAttributedString:)]
        pub unsafe fn isEqualToAttributedString(&self, other: &NSAttributedString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub fn initWithString(this: Allocated<Self>, str: &NSString) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Allocated<Self>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub fn initWithAttributedString(
            this: Allocated<Self>,
            attr_str: &NSAttributedString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString",
            feature = "block2"
        ))]
        #[method(enumerateAttributesInRange:options:usingBlock:)]
        pub unsafe fn enumerateAttributesInRange_options_usingBlock(
            &self,
            enumeration_range: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: &Block<
                dyn Fn(
                        NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
                        NSRange,
                        NonNull<Bool>,
                    ) + '_,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString",
            feature = "block2"
        ))]
        #[method(enumerateAttribute:inRange:options:usingBlock:)]
        pub unsafe fn enumerateAttribute_inRange_options_usingBlock(
            &self,
            attr_name: &NSAttributedStringKey,
            enumeration_range: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: &Block<dyn Fn(*mut AnyObject, NSRange, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSExtendedAttributedString
    unsafe impl NSMutableAttributedString {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub fn initWithString(this: Allocated<Self>, str: &NSString) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Allocated<Self>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub fn initWithAttributedString(
            this: Allocated<Self>,
            attr_str: &NSAttributedString,
        ) -> Id<Self>;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableAttributedString;

    unsafe impl ClassType for NSMutableAttributedString {
        #[inherits(NSObject)]
        type Super = NSAttributedString;
        type Mutability = MutableWithImmutableSuperclass<NSAttributedString>;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSMutableAttributedString {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSMutableAttributedString {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSMutableCopying for NSMutableAttributedString {}

unsafe impl NSObjectProtocol for NSMutableAttributedString {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSMutableAttributedString {}

extern_methods!(
    unsafe impl NSMutableAttributedString {
        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(
            &mut self,
            range: NSRange,
            str: &NSString,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method(setAttributes:range:)]
        pub unsafe fn setAttributes_range(
            &mut self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            range: NSRange,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableAttributedString {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

impl DefaultId for NSMutableAttributedString {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableAttributedString
    unsafe impl NSMutableAttributedString {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other mutableString)]
        pub unsafe fn mutableString(&self) -> Id<NSMutableString>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method(addAttribute:value:range:)]
        pub unsafe fn addAttribute_value_range(
            &mut self,
            name: &NSAttributedStringKey,
            value: &AnyObject,
            range: NSRange,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSRange",
            feature = "Foundation_NSString"
        ))]
        #[method(addAttributes:range:)]
        pub unsafe fn addAttributes_range(
            &mut self,
            attrs: &NSDictionary<NSAttributedStringKey, AnyObject>,
            range: NSRange,
        );

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method(removeAttribute:range:)]
        pub unsafe fn removeAttribute_range(
            &mut self,
            name: &NSAttributedStringKey,
            range: NSRange,
        );

        #[cfg(feature = "Foundation_NSRange")]
        #[method(replaceCharactersInRange:withAttributedString:)]
        pub unsafe fn replaceCharactersInRange_withAttributedString(
            &mut self,
            range: NSRange,
            attr_string: &NSAttributedString,
        );

        #[method(insertAttributedString:atIndex:)]
        pub unsafe fn insertAttributedString_atIndex(
            &mut self,
            attr_string: &NSAttributedString,
            loc: NSUInteger,
        );

        #[method(appendAttributedString:)]
        pub unsafe fn appendAttributedString(&mut self, attr_string: &NSAttributedString);

        #[cfg(feature = "Foundation_NSRange")]
        #[method(deleteCharactersInRange:)]
        pub unsafe fn deleteCharactersInRange(&mut self, range: NSRange);

        #[method(setAttributedString:)]
        pub fn setAttributedString(&mut self, attr_string: &NSAttributedString);

        #[method(beginEditing)]
        pub unsafe fn beginEditing(&mut self);

        #[method(endEditing)]
        pub unsafe fn endEditing(&mut self);
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSInlinePresentationIntent(pub NSUInteger);
impl NSInlinePresentationIntent {
    #[doc(alias = "NSInlinePresentationIntentEmphasized")]
    pub const Emphasized: Self = Self(1 << 0);
    #[doc(alias = "NSInlinePresentationIntentStronglyEmphasized")]
    pub const StronglyEmphasized: Self = Self(1 << 1);
    #[doc(alias = "NSInlinePresentationIntentCode")]
    pub const Code: Self = Self(1 << 2);
    #[doc(alias = "NSInlinePresentationIntentStrikethrough")]
    pub const Strikethrough: Self = Self(1 << 5);
    #[doc(alias = "NSInlinePresentationIntentSoftBreak")]
    pub const SoftBreak: Self = Self(1 << 6);
    #[doc(alias = "NSInlinePresentationIntentLineBreak")]
    pub const LineBreak: Self = Self(1 << 7);
    #[doc(alias = "NSInlinePresentationIntentInlineHTML")]
    pub const InlineHTML: Self = Self(1 << 8);
    #[doc(alias = "NSInlinePresentationIntentBlockHTML")]
    pub const BlockHTML: Self = Self(1 << 9);
}

unsafe impl Encode for NSInlinePresentationIntent {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSInlinePresentationIntent {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInlinePresentationIntentAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSAlternateDescriptionAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSImageURLAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSLanguageIdentifierAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMarkdownSourcePositionAttributeName: &'static NSAttributedStringKey;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringMarkdownParsingFailurePolicy(pub NSInteger);
impl NSAttributedStringMarkdownParsingFailurePolicy {
    pub const NSAttributedStringMarkdownParsingFailureReturnError: Self = Self(0);
    pub const NSAttributedStringMarkdownParsingFailureReturnPartiallyParsedIfPossible: Self =
        Self(1);
}

unsafe impl Encode for NSAttributedStringMarkdownParsingFailurePolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringMarkdownParsingFailurePolicy {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringMarkdownInterpretedSyntax(pub NSInteger);
impl NSAttributedStringMarkdownInterpretedSyntax {
    #[doc(alias = "NSAttributedStringMarkdownInterpretedSyntaxFull")]
    pub const Full: Self = Self(0);
    #[doc(alias = "NSAttributedStringMarkdownInterpretedSyntaxInlineOnly")]
    pub const InlineOnly: Self = Self(1);
    #[doc(alias = "NSAttributedStringMarkdownInterpretedSyntaxInlineOnlyPreservingWhitespace")]
    pub const InlineOnlyPreservingWhitespace: Self = Self(2);
}

unsafe impl Encode for NSAttributedStringMarkdownInterpretedSyntax {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringMarkdownInterpretedSyntax {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributedStringMarkdownSourcePosition;

    unsafe impl ClassType for NSAttributedStringMarkdownSourcePosition {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSAttributedStringMarkdownSourcePosition {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSAttributedStringMarkdownSourcePosition {}

unsafe impl NSObjectProtocol for NSAttributedStringMarkdownSourcePosition {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSAttributedStringMarkdownSourcePosition {}

extern_methods!(
    unsafe impl NSAttributedStringMarkdownSourcePosition {
        #[method(startLine)]
        pub unsafe fn startLine(&self) -> NSInteger;

        #[method(startColumn)]
        pub unsafe fn startColumn(&self) -> NSInteger;

        #[method(endLine)]
        pub unsafe fn endLine(&self) -> NSInteger;

        #[method(endColumn)]
        pub unsafe fn endColumn(&self) -> NSInteger;

        #[method_id(@__retain_semantics Init initWithStartLine:startColumn:endLine:endColumn:)]
        pub unsafe fn initWithStartLine_startColumn_endLine_endColumn(
            this: Allocated<Self>,
            start_line: NSInteger,
            start_column: NSInteger,
            end_line: NSInteger,
            end_column: NSInteger,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSRange", feature = "Foundation_NSString"))]
        #[method(rangeInString:)]
        pub unsafe fn rangeInString(&self, string: &NSString) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAttributedStringMarkdownSourcePosition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributedStringMarkdownParsingOptions;

    unsafe impl ClassType for NSAttributedStringMarkdownParsingOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSAttributedStringMarkdownParsingOptions {}

unsafe impl NSObjectProtocol for NSAttributedStringMarkdownParsingOptions {}

extern_methods!(
    unsafe impl NSAttributedStringMarkdownParsingOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(allowsExtendedAttributes)]
        pub unsafe fn allowsExtendedAttributes(&self) -> bool;

        #[method(setAllowsExtendedAttributes:)]
        pub unsafe fn setAllowsExtendedAttributes(&self, allows_extended_attributes: bool);

        #[method(interpretedSyntax)]
        pub unsafe fn interpretedSyntax(&self) -> NSAttributedStringMarkdownInterpretedSyntax;

        #[method(setInterpretedSyntax:)]
        pub unsafe fn setInterpretedSyntax(
            &self,
            interpreted_syntax: NSAttributedStringMarkdownInterpretedSyntax,
        );

        #[method(failurePolicy)]
        pub unsafe fn failurePolicy(&self) -> NSAttributedStringMarkdownParsingFailurePolicy;

        #[method(setFailurePolicy:)]
        pub unsafe fn setFailurePolicy(
            &self,
            failure_policy: NSAttributedStringMarkdownParsingFailurePolicy,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLanguageCode:)]
        pub unsafe fn setLanguageCode(&self, language_code: Option<&NSString>);

        #[method(appliesSourcePositionAttributes)]
        pub unsafe fn appliesSourcePositionAttributes(&self) -> bool;

        #[method(setAppliesSourcePositionAttributes:)]
        pub unsafe fn setAppliesSourcePositionAttributes(
            &self,
            applies_source_position_attributes: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSAttributedStringMarkdownParsingOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSAttributedStringCreateFromMarkdown
    unsafe impl NSAttributedString {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:_)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Allocated<Self>,
            markdown_file: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Allocated<Self>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Allocated<Self>,
            markdown_string: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSAttributedStringCreateFromMarkdown
    unsafe impl NSMutableAttributedString {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:_)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Allocated<Self>,
            markdown_file: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Allocated<Self>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Allocated<Self>,
            markdown_string: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSAttributedStringFormattingOptions(pub NSUInteger);
impl NSAttributedStringFormattingOptions {
    pub const NSAttributedStringFormattingInsertArgumentAttributesWithoutMerging: Self =
        Self(1 << 0);
    pub const NSAttributedStringFormattingApplyReplacementIndexAttribute: Self = Self(1 << 1);
}

unsafe impl Encode for NSAttributedStringFormattingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSAttributedStringFormattingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// NSAttributedStringFormatting
    unsafe impl NSAttributedString {}
);

extern_methods!(
    /// NSMutableAttributedStringFormatting
    unsafe impl NSMutableAttributedString {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSReplacementIndexAttributeName: &'static NSAttributedStringKey;
}

extern_methods!(
    /// NSMorphology
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Other attributedStringByInflectingString)]
        pub unsafe fn attributedStringByInflectingString(&self) -> Id<NSAttributedString>;
    }
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSMorphologyAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInflectionRuleAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInflectionAgreementArgumentAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInflectionAgreementConceptAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInflectionReferentConceptAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSInflectionAlternativeAttributeName: &'static NSAttributedStringKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSPresentationIntentAttributeName: &'static NSAttributedStringKey;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPresentationIntentKind(pub NSInteger);
impl NSPresentationIntentKind {
    #[doc(alias = "NSPresentationIntentKindParagraph")]
    pub const Paragraph: Self = Self(0);
    #[doc(alias = "NSPresentationIntentKindHeader")]
    pub const Header: Self = Self(1);
    #[doc(alias = "NSPresentationIntentKindOrderedList")]
    pub const OrderedList: Self = Self(2);
    #[doc(alias = "NSPresentationIntentKindUnorderedList")]
    pub const UnorderedList: Self = Self(3);
    #[doc(alias = "NSPresentationIntentKindListItem")]
    pub const ListItem: Self = Self(4);
    #[doc(alias = "NSPresentationIntentKindCodeBlock")]
    pub const CodeBlock: Self = Self(5);
    #[doc(alias = "NSPresentationIntentKindBlockQuote")]
    pub const BlockQuote: Self = Self(6);
    #[doc(alias = "NSPresentationIntentKindThematicBreak")]
    pub const ThematicBreak: Self = Self(7);
    #[doc(alias = "NSPresentationIntentKindTable")]
    pub const Table: Self = Self(8);
    #[doc(alias = "NSPresentationIntentKindTableHeaderRow")]
    pub const TableHeaderRow: Self = Self(9);
    #[doc(alias = "NSPresentationIntentKindTableRow")]
    pub const TableRow: Self = Self(10);
    #[doc(alias = "NSPresentationIntentKindTableCell")]
    pub const TableCell: Self = Self(11);
}

unsafe impl Encode for NSPresentationIntentKind {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPresentationIntentKind {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSPresentationIntentTableColumnAlignment(pub NSInteger);
impl NSPresentationIntentTableColumnAlignment {
    #[doc(alias = "NSPresentationIntentTableColumnAlignmentLeft")]
    pub const Left: Self = Self(0);
    #[doc(alias = "NSPresentationIntentTableColumnAlignmentCenter")]
    pub const Center: Self = Self(1);
    #[doc(alias = "NSPresentationIntentTableColumnAlignmentRight")]
    pub const Right: Self = Self(2);
}

unsafe impl Encode for NSPresentationIntentTableColumnAlignment {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSPresentationIntentTableColumnAlignment {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPresentationIntent;

    unsafe impl ClassType for NSPresentationIntent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSPresentationIntent {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSPresentationIntent {}

unsafe impl NSObjectProtocol for NSPresentationIntent {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSSecureCoding for NSPresentationIntent {}

extern_methods!(
    unsafe impl NSPresentationIntent {
        #[method(intentKind)]
        pub unsafe fn intentKind(&self) -> NSPresentationIntentKind;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other parentIntent)]
        pub unsafe fn parentIntent(&self) -> Option<Id<NSPresentationIntent>>;

        #[method_id(@__retain_semantics Other paragraphIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn paragraphIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other headerIntentWithIdentity:level:nestedInsideIntent:)]
        pub unsafe fn headerIntentWithIdentity_level_nestedInsideIntent(
            identity: NSInteger,
            level: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other codeBlockIntentWithIdentity:languageHint:nestedInsideIntent:)]
        pub unsafe fn codeBlockIntentWithIdentity_languageHint_nestedInsideIntent(
            identity: NSInteger,
            language_hint: Option<&NSString>,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other thematicBreakIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn thematicBreakIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other orderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn orderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other unorderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn unorderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other listItemIntentWithIdentity:ordinal:nestedInsideIntent:)]
        pub unsafe fn listItemIntentWithIdentity_ordinal_nestedInsideIntent(
            identity: NSInteger,
            ordinal: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other blockQuoteIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn blockQuoteIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other tableIntentWithIdentity:columnCount:alignments:nestedInsideIntent:)]
        pub unsafe fn tableIntentWithIdentity_columnCount_alignments_nestedInsideIntent(
            identity: NSInteger,
            column_count: NSInteger,
            alignments: &NSArray<NSNumber>,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other tableHeaderRowIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn tableHeaderRowIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other tableRowIntentWithIdentity:row:nestedInsideIntent:)]
        pub unsafe fn tableRowIntentWithIdentity_row_nestedInsideIntent(
            identity: NSInteger,
            row: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method_id(@__retain_semantics Other tableCellIntentWithIdentity:column:nestedInsideIntent:)]
        pub unsafe fn tableCellIntentWithIdentity_column_nestedInsideIntent(
            identity: NSInteger,
            column: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent>;

        #[method(identity)]
        pub unsafe fn identity(&self) -> NSInteger;

        #[method(ordinal)]
        pub unsafe fn ordinal(&self) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other columnAlignments)]
        pub unsafe fn columnAlignments(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[method(columnCount)]
        pub unsafe fn columnCount(&self) -> NSInteger;

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other languageHint)]
        pub unsafe fn languageHint(&self) -> Option<Id<NSString>>;

        #[method(column)]
        pub unsafe fn column(&self) -> NSInteger;

        #[method(row)]
        pub unsafe fn row(&self) -> NSInteger;

        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        #[method(isEquivalentToPresentationIntent:)]
        pub unsafe fn isEquivalentToPresentationIntent(&self, other: &NSPresentationIntent)
            -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSPresentationIntent {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
