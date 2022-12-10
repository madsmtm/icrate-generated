//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSAttributedStringKey = NSString;

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSAttributedString;

    unsafe impl ClassType for NSAttributedString {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Other string)]
        pub fn string(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other attributesAtIndex:effectiveRange:)]
        pub unsafe fn attributesAtIndex_effectiveRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSAttributedStringEnumerationOptions {
        NSAttributedStringEnumerationReverse = 1 << 1,
        NSAttributedStringEnumerationLongestEffectiveRangeNotRequired = 1 << 20,
    }
);

extern_methods!(
    /// NSExtendedAttributedString
    unsafe impl NSAttributedString {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other attribute:atIndex:effectiveRange:)]
        pub unsafe fn attribute_atIndex_effectiveRange(
            &self,
            attrName: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other attributedSubstringFromRange:)]
        pub unsafe fn attributedSubstringFromRange(
            &self,
            range: NSRange,
        ) -> Id<NSAttributedString, Shared>;

        #[method_id(@__retain_semantics Other attributesAtIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attributesAtIndex_longestEffectiveRange_inRange(
            &self,
            location: NSUInteger,
            range: NSRangePointer,
            rangeLimit: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;

        #[method_id(@__retain_semantics Other attribute:atIndex:longestEffectiveRange:inRange:)]
        pub unsafe fn attribute_atIndex_longestEffectiveRange_inRange(
            &self,
            attrName: &NSAttributedStringKey,
            location: NSUInteger,
            range: NSRangePointer,
            rangeLimit: NSRange,
        ) -> Option<Id<Object, Shared>>;

        #[method(isEqualToAttributedString:)]
        pub unsafe fn isEqualToAttributedString(&self, other: &NSAttributedString) -> bool;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub fn initWithString(this: Option<Allocated<Self>>, str: &NSString) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Option<Allocated<Self>>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub fn initWithAttributedString(
            this: Option<Allocated<Self>>,
            attrStr: &NSAttributedString,
        ) -> Id<Self, Shared>;

        #[method(enumerateAttributesInRange:options:usingBlock:)]
        pub unsafe fn enumerateAttributesInRange_options_usingBlock(
            &self,
            enumerationRange: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: &Block<
                (
                    NonNull<NSDictionary<NSAttributedStringKey, Object>>,
                    NSRange,
                    NonNull<Bool>,
                ),
                (),
            >,
        );

        #[method(enumerateAttribute:inRange:options:usingBlock:)]
        pub unsafe fn enumerateAttribute_inRange_options_usingBlock(
            &self,
            attrName: &NSAttributedStringKey,
            enumerationRange: NSRange,
            opts: NSAttributedStringEnumerationOptions,
            block: &Block<(*mut Object, NSRange, NonNull<Bool>), ()>,
        );
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    pub struct NSMutableAttributedString;

    unsafe impl ClassType for NSMutableAttributedString {
        type Super = NSAttributedString;
    }
);

extern_methods!(
    unsafe impl NSMutableAttributedString {
        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, str: &NSString);

        #[method(setAttributes:range:)]
        pub unsafe fn setAttributes_range(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            range: NSRange,
        );
    }
);

extern_methods!(
    /// NSExtendedMutableAttributedString
    unsafe impl NSMutableAttributedString {
        #[method_id(@__retain_semantics Other mutableString)]
        pub unsafe fn mutableString(&self) -> Id<NSMutableString, Shared>;

        #[method(addAttribute:value:range:)]
        pub unsafe fn addAttribute_value_range(
            &self,
            name: &NSAttributedStringKey,
            value: &Object,
            range: NSRange,
        );

        #[method(addAttributes:range:)]
        pub unsafe fn addAttributes_range(
            &self,
            attrs: &NSDictionary<NSAttributedStringKey, Object>,
            range: NSRange,
        );

        #[method(removeAttribute:range:)]
        pub unsafe fn removeAttribute_range(&self, name: &NSAttributedStringKey, range: NSRange);

        #[method(replaceCharactersInRange:withAttributedString:)]
        pub unsafe fn replaceCharactersInRange_withAttributedString(
            &self,
            range: NSRange,
            attrString: &NSAttributedString,
        );

        #[method(insertAttributedString:atIndex:)]
        pub unsafe fn insertAttributedString_atIndex(
            &self,
            attrString: &NSAttributedString,
            loc: NSUInteger,
        );

        #[method(appendAttributedString:)]
        pub unsafe fn appendAttributedString(&self, attrString: &NSAttributedString);

        #[method(deleteCharactersInRange:)]
        pub unsafe fn deleteCharactersInRange(&self, range: NSRange);

        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attrString: &NSAttributedString);

        #[method(beginEditing)]
        pub unsafe fn beginEditing(&self);

        #[method(endEditing)]
        pub unsafe fn endEditing(&self);
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSInlinePresentationIntent {
        NSInlinePresentationIntentEmphasized = 1 << 0,
        NSInlinePresentationIntentStronglyEmphasized = 1 << 1,
        NSInlinePresentationIntentCode = 1 << 2,
        NSInlinePresentationIntentStrikethrough = 1 << 5,
        NSInlinePresentationIntentSoftBreak = 1 << 6,
        NSInlinePresentationIntentLineBreak = 1 << 7,
        NSInlinePresentationIntentInlineHTML = 1 << 8,
        NSInlinePresentationIntentBlockHTML = 1 << 9,
    }
);

extern_static!(NSInlinePresentationIntentAttributeName: &'static NSAttributedStringKey);

extern_static!(NSAlternateDescriptionAttributeName: &'static NSAttributedStringKey);

extern_static!(NSImageURLAttributeName: &'static NSAttributedStringKey);

extern_static!(NSLanguageIdentifierAttributeName: &'static NSAttributedStringKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSAttributedStringMarkdownParsingFailurePolicy {
        NSAttributedStringMarkdownParsingFailureReturnError = 0,
        NSAttributedStringMarkdownParsingFailureReturnPartiallyParsedIfPossible = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSAttributedStringMarkdownInterpretedSyntax {
        NSAttributedStringMarkdownInterpretedSyntaxFull = 0,
        NSAttributedStringMarkdownInterpretedSyntaxInlineOnly = 1,
        NSAttributedStringMarkdownInterpretedSyntaxInlineOnlyPreservingWhitespace = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSAttributedStringMarkdownParsingOptions;

    unsafe impl ClassType for NSAttributedStringMarkdownParsingOptions {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAttributedStringMarkdownParsingOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(allowsExtendedAttributes)]
        pub unsafe fn allowsExtendedAttributes(&self) -> bool;

        #[method(setAllowsExtendedAttributes:)]
        pub unsafe fn setAllowsExtendedAttributes(&self, allowsExtendedAttributes: bool);

        #[method(interpretedSyntax)]
        pub unsafe fn interpretedSyntax(&self) -> NSAttributedStringMarkdownInterpretedSyntax;

        #[method(setInterpretedSyntax:)]
        pub unsafe fn setInterpretedSyntax(
            &self,
            interpretedSyntax: NSAttributedStringMarkdownInterpretedSyntax,
        );

        #[method(failurePolicy)]
        pub unsafe fn failurePolicy(&self) -> NSAttributedStringMarkdownParsingFailurePolicy;

        #[method(setFailurePolicy:)]
        pub unsafe fn setFailurePolicy(
            &self,
            failurePolicy: NSAttributedStringMarkdownParsingFailurePolicy,
        );

        #[method_id(@__retain_semantics Other languageCode)]
        pub unsafe fn languageCode(&self) -> Option<Id<NSString, Shared>>;

        #[method(setLanguageCode:)]
        pub unsafe fn setLanguageCode(&self, languageCode: Option<&NSString>);
    }
);

extern_methods!(
    /// NSAttributedStringCreateFromMarkdown
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdownFile: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdownString: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            baseURL: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSAttributedStringFormattingOptions {
        NSAttributedStringFormattingInsertArgumentAttributesWithoutMerging = 1 << 0,
        NSAttributedStringFormattingApplyReplacementIndexAttribute = 1 << 1,
    }
);

extern_methods!(
    /// NSAttributedStringFormatting
    unsafe impl NSAttributedString {}
);

extern_methods!(
    /// NSMutableAttributedStringFormatting
    unsafe impl NSMutableAttributedString {}
);

extern_static!(NSReplacementIndexAttributeName: &'static NSAttributedStringKey);

extern_methods!(
    /// NSMorphology
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Other attributedStringByInflectingString)]
        pub unsafe fn attributedStringByInflectingString(&self) -> Id<NSAttributedString, Shared>;
    }
);

extern_static!(NSMorphologyAttributeName: &'static NSAttributedStringKey);

extern_static!(NSInflectionRuleAttributeName: &'static NSAttributedStringKey);

extern_static!(NSInflectionAlternativeAttributeName: &'static NSAttributedStringKey);

extern_static!(NSPresentationIntentAttributeName: &'static NSAttributedStringKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPresentationIntentKind {
        NSPresentationIntentKindParagraph = 0,
        NSPresentationIntentKindHeader = 1,
        NSPresentationIntentKindOrderedList = 2,
        NSPresentationIntentKindUnorderedList = 3,
        NSPresentationIntentKindListItem = 4,
        NSPresentationIntentKindCodeBlock = 5,
        NSPresentationIntentKindBlockQuote = 6,
        NSPresentationIntentKindThematicBreak = 7,
        NSPresentationIntentKindTable = 8,
        NSPresentationIntentKindTableHeaderRow = 9,
        NSPresentationIntentKindTableRow = 10,
        NSPresentationIntentKindTableCell = 11,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPresentationIntentTableColumnAlignment {
        NSPresentationIntentTableColumnAlignmentLeft = 0,
        NSPresentationIntentTableColumnAlignmentCenter = 1,
        NSPresentationIntentTableColumnAlignmentRight = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPresentationIntent;

    unsafe impl ClassType for NSPresentationIntent {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPresentationIntent {
        #[method(intentKind)]
        pub unsafe fn intentKind(&self) -> NSPresentationIntentKind;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other parentIntent)]
        pub unsafe fn parentIntent(&self) -> Option<Id<NSPresentationIntent, Shared>>;

        #[method_id(@__retain_semantics Other paragraphIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn paragraphIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other headerIntentWithIdentity:level:nestedInsideIntent:)]
        pub unsafe fn headerIntentWithIdentity_level_nestedInsideIntent(
            identity: NSInteger,
            level: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other codeBlockIntentWithIdentity:languageHint:nestedInsideIntent:)]
        pub unsafe fn codeBlockIntentWithIdentity_languageHint_nestedInsideIntent(
            identity: NSInteger,
            languageHint: Option<&NSString>,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other thematicBreakIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn thematicBreakIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other orderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn orderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other unorderedListIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn unorderedListIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other listItemIntentWithIdentity:ordinal:nestedInsideIntent:)]
        pub unsafe fn listItemIntentWithIdentity_ordinal_nestedInsideIntent(
            identity: NSInteger,
            ordinal: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other blockQuoteIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn blockQuoteIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other tableIntentWithIdentity:columnCount:alignments:nestedInsideIntent:)]
        pub unsafe fn tableIntentWithIdentity_columnCount_alignments_nestedInsideIntent(
            identity: NSInteger,
            columnCount: NSInteger,
            alignments: &NSArray<NSNumber>,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other tableHeaderRowIntentWithIdentity:nestedInsideIntent:)]
        pub unsafe fn tableHeaderRowIntentWithIdentity_nestedInsideIntent(
            identity: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other tableRowIntentWithIdentity:row:nestedInsideIntent:)]
        pub unsafe fn tableRowIntentWithIdentity_row_nestedInsideIntent(
            identity: NSInteger,
            row: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method_id(@__retain_semantics Other tableCellIntentWithIdentity:column:nestedInsideIntent:)]
        pub unsafe fn tableCellIntentWithIdentity_column_nestedInsideIntent(
            identity: NSInteger,
            column: NSInteger,
            parent: Option<&NSPresentationIntent>,
        ) -> Id<NSPresentationIntent, Shared>;

        #[method(identity)]
        pub unsafe fn identity(&self) -> NSInteger;

        #[method(ordinal)]
        pub unsafe fn ordinal(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other columnAlignments)]
        pub unsafe fn columnAlignments(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[method(columnCount)]
        pub unsafe fn columnCount(&self) -> NSInteger;

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other languageHint)]
        pub unsafe fn languageHint(&self) -> Option<Id<NSString, Shared>>;

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
