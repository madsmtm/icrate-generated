//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSFontAttributeName: &'static NSAttributedStringKey);

extern_static!(NSParagraphStyleAttributeName: &'static NSAttributedStringKey);

extern_static!(NSForegroundColorAttributeName: &'static NSAttributedStringKey);

extern_static!(NSBackgroundColorAttributeName: &'static NSAttributedStringKey);

extern_static!(NSLigatureAttributeName: &'static NSAttributedStringKey);

extern_static!(NSKernAttributeName: &'static NSAttributedStringKey);

extern_static!(NSTrackingAttributeName: &'static NSAttributedStringKey);

extern_static!(NSStrikethroughStyleAttributeName: &'static NSAttributedStringKey);

extern_static!(NSUnderlineStyleAttributeName: &'static NSAttributedStringKey);

extern_static!(NSStrokeColorAttributeName: &'static NSAttributedStringKey);

extern_static!(NSStrokeWidthAttributeName: &'static NSAttributedStringKey);

extern_static!(NSShadowAttributeName: &'static NSAttributedStringKey);

extern_static!(NSTextEffectAttributeName: &'static NSAttributedStringKey);

extern_static!(NSAttachmentAttributeName: &'static NSAttributedStringKey);

extern_static!(NSLinkAttributeName: &'static NSAttributedStringKey);

extern_static!(NSBaselineOffsetAttributeName: &'static NSAttributedStringKey);

extern_static!(NSUnderlineColorAttributeName: &'static NSAttributedStringKey);

extern_static!(NSStrikethroughColorAttributeName: &'static NSAttributedStringKey);

extern_static!(NSObliquenessAttributeName: &'static NSAttributedStringKey);

extern_static!(NSExpansionAttributeName: &'static NSAttributedStringKey);

extern_static!(NSWritingDirectionAttributeName: &'static NSAttributedStringKey);

extern_static!(NSVerticalGlyphFormAttributeName: &'static NSAttributedStringKey);

extern_static!(NSCursorAttributeName: &'static NSAttributedStringKey);

extern_static!(NSToolTipAttributeName: &'static NSAttributedStringKey);

extern_static!(NSMarkedClauseSegmentAttributeName: &'static NSAttributedStringKey);

extern_static!(NSTextAlternativesAttributeName: &'static NSAttributedStringKey);

extern_static!(NSSpellingStateAttributeName: &'static NSAttributedStringKey);

extern_static!(NSSuperscriptAttributeName: &'static NSAttributedStringKey);

extern_static!(NSGlyphInfoAttributeName: &'static NSAttributedStringKey);

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSUnderlineStyle {
        NSUnderlineStyleNone = 0x00,
        NSUnderlineStyleSingle = 0x01,
        NSUnderlineStyleThick = 0x02,
        NSUnderlineStyleDouble = 0x09,
        NSUnderlineStylePatternSolid = 0x0000,
        NSUnderlineStylePatternDot = 0x0100,
        NSUnderlineStylePatternDash = 0x0200,
        NSUnderlineStylePatternDashDot = 0x0300,
        NSUnderlineStylePatternDashDotDot = 0x0400,
        NSUnderlineStyleByWord = 0x8000,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWritingDirectionFormatType {
        NSWritingDirectionEmbedding = 0 << 1,
        NSWritingDirectionOverride = 1 << 1,
    }
);

typed_enum!(
    pub type NSTextEffectStyle = NSString;
);

extern_static!(NSTextEffectLetterpressStyle: &'static NSTextEffectStyle);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSpellingState {
        NSSpellingStateSpellingFlag = 1 << 0,
        NSSpellingStateGrammarFlag = 1 << 1,
    }
);

extern_methods!(
    /// NSAttributedStringAttributeFixing
    unsafe impl NSMutableAttributedString {
        #[method(fixAttributesInRange:)]
        pub unsafe fn fixAttributesInRange(&self, range: NSRange);

        #[method(fixFontAttributeInRange:)]
        pub unsafe fn fixFontAttributeInRange(&self, range: NSRange);

        #[method(fixParagraphStyleAttributeInRange:)]
        pub unsafe fn fixParagraphStyleAttributeInRange(&self, range: NSRange);

        #[method(fixAttachmentAttributeInRange:)]
        pub unsafe fn fixAttachmentAttributeInRange(&self, range: NSRange);
    }
);

typed_extensible_enum!(
    pub type NSAttributedStringDocumentType = NSString;
);

extern_static!(NSPlainTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSRTFTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSRTFDTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSHTMLTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSMacSimpleTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSDocFormatTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSWordMLTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSWebArchiveTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSOfficeOpenXMLTextDocumentType: &'static NSAttributedStringDocumentType);

extern_static!(NSOpenDocumentTextDocumentType: &'static NSAttributedStringDocumentType);

typed_enum!(
    pub type NSTextLayoutSectionKey = NSString;
);

extern_static!(NSTextLayoutSectionOrientation: &'static NSTextLayoutSectionKey);

extern_static!(NSTextLayoutSectionRange: &'static NSTextLayoutSectionKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextScalingType {
        NSTextScalingStandard = 0,
        NSTextScalingiOS = 1,
    }
);

typed_extensible_enum!(
    pub type NSAttributedStringDocumentAttributeKey = NSString;
);

extern_static!(NSDocumentTypeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSConvertedDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSCocoaVersionDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSFileTypeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSTitleDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSCompanyDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSCopyrightDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSSubjectDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSAuthorDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSKeywordsDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSCommentDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSEditorDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSCreationTimeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(
    NSModificationTimeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(NSManagerDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSCategoryDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSAppearanceDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(
    NSCharacterEncodingDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(
    NSDefaultAttributesDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(NSPaperSizeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSLeftMarginDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSRightMarginDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSTopMarginDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSBottomMarginDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSViewSizeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSViewZoomDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSViewModeDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSReadOnlyDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSBackgroundColorDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(
    NSHyphenationFactorDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(
    NSDefaultTabIntervalDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(NSTextLayoutSectionsAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(
    NSExcludedElementsDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(
    NSTextEncodingNameDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

extern_static!(NSPrefixSpacesDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(NSTextScalingDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey);

extern_static!(
    NSSourceTextScalingDocumentAttribute: &'static NSAttributedStringDocumentAttributeKey
);

typed_extensible_enum!(
    pub type NSAttributedStringDocumentReadingOptionKey = NSString;
);

extern_static!(NSDocumentTypeDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey);

extern_static!(
    NSDefaultAttributesDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_static!(
    NSCharacterEncodingDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_static!(
    NSTextEncodingNameDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_static!(NSBaseURLDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey);

extern_static!(NSTimeoutDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey);

extern_static!(NSWebPreferencesDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey);

extern_static!(
    NSWebResourceLoadDelegateDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_static!(
    NSTextSizeMultiplierDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_static!(NSFileTypeDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey);

extern_static!(
    NSTargetTextScalingDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_static!(
    NSSourceTextScalingDocumentOption: &'static NSAttributedStringDocumentReadingOptionKey
);

extern_methods!(
    /// NSAttributedStringDocumentFormats
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Init initWithURL:options:documentAttributes:error:_)]
        pub unsafe fn initWithURL_options_documentAttributes_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithData:options:documentAttributes:error:_)]
        pub unsafe fn initWithData_options_documentAttributes_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other dataFromRange:documentAttributes:error:_)]
        pub unsafe fn dataFromRange_documentAttributes_error(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other fileWrapperFromRange:documentAttributes:error:_)]
        pub unsafe fn fileWrapperFromRange_documentAttributes_error(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<NSFileWrapper, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initWithRTF:documentAttributes:)]
        pub unsafe fn initWithRTF_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithRTFD:documentAttributes:)]
        pub unsafe fn initWithRTFD_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithHTML:documentAttributes:)]
        pub unsafe fn initWithHTML_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithHTML:baseURL:documentAttributes:)]
        pub unsafe fn initWithHTML_baseURL_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            base: &NSURL,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithDocFormat:documentAttributes:)]
        pub unsafe fn initWithDocFormat_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithHTML:options:documentAttributes:)]
        pub unsafe fn initWithHTML_options_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithRTFDFileWrapper:documentAttributes:)]
        pub unsafe fn initWithRTFDFileWrapper_documentAttributes(
            this: Option<Allocated<Self>>,
            wrapper: &NSFileWrapper,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other RTFFromRange:documentAttributes:)]
        pub unsafe fn RTFFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other RTFDFromRange:documentAttributes:)]
        pub unsafe fn RTFDFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other RTFDFileWrapperFromRange:documentAttributes:)]
        pub unsafe fn RTFDFileWrapperFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSFileWrapper, Shared>>;

        #[method_id(@__retain_semantics Other docFormatFromRange:documentAttributes:)]
        pub unsafe fn docFormatFromRange_documentAttributes(
            &self,
            range: NSRange,
            dict: &NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<NSData, Shared>>;
    }
);

extern_methods!(
    /// NSMutableAttributedStringDocumentFormats
    unsafe impl NSMutableAttributedString {
        #[method(readFromURL:options:documentAttributes:error:_)]
        pub unsafe fn readFromURL_options_documentAttributes_error(
            &self,
            url: &NSURL,
            opts: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(readFromData:options:documentAttributes:error:_)]
        pub unsafe fn readFromData_options_documentAttributes_error(
            &self,
            data: &NSData,
            opts: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSAttributedStringKitAdditions
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Other fontAttributesInRange:)]
        pub unsafe fn fontAttributesInRange(
            &self,
            range: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;

        #[method_id(@__retain_semantics Other rulerAttributesInRange:)]
        pub unsafe fn rulerAttributesInRange(
            &self,
            range: NSRange,
        ) -> Id<NSDictionary<NSAttributedStringKey, Object>, Shared>;

        #[method(containsAttachmentsInRange:)]
        pub unsafe fn containsAttachmentsInRange(&self, range: NSRange) -> bool;

        #[method(lineBreakBeforeIndex:withinRange:)]
        pub unsafe fn lineBreakBeforeIndex_withinRange(
            &self,
            location: NSUInteger,
            aRange: NSRange,
        ) -> NSUInteger;

        #[method(lineBreakByHyphenatingBeforeIndex:withinRange:)]
        pub unsafe fn lineBreakByHyphenatingBeforeIndex_withinRange(
            &self,
            location: NSUInteger,
            aRange: NSRange,
        ) -> NSUInteger;

        #[method(doubleClickAtIndex:)]
        pub unsafe fn doubleClickAtIndex(&self, location: NSUInteger) -> NSRange;

        #[method(nextWordFromIndex:forward:)]
        pub unsafe fn nextWordFromIndex_forward(
            &self,
            location: NSUInteger,
            isForward: bool,
        ) -> NSUInteger;

        #[method(rangeOfTextBlock:atIndex:)]
        pub unsafe fn rangeOfTextBlock_atIndex(
            &self,
            block: &NSTextBlock,
            location: NSUInteger,
        ) -> NSRange;

        #[method(rangeOfTextTable:atIndex:)]
        pub unsafe fn rangeOfTextTable_atIndex(
            &self,
            table: &NSTextTable,
            location: NSUInteger,
        ) -> NSRange;

        #[method(rangeOfTextList:atIndex:)]
        pub unsafe fn rangeOfTextList_atIndex(
            &self,
            list: &NSTextList,
            location: NSUInteger,
        ) -> NSRange;

        #[method(itemNumberInTextList:atIndex:)]
        pub unsafe fn itemNumberInTextList_atIndex(
            &self,
            list: &NSTextList,
            location: NSUInteger,
        ) -> NSInteger;
    }
);

extern_methods!(
    /// NSAttributedStringPasteboardAdditions
    unsafe impl NSAttributedString {
        #[method_id(@__retain_semantics Other textTypes)]
        pub unsafe fn textTypes() -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other textUnfilteredTypes)]
        pub unsafe fn textUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;
    }
);

extern_methods!(
    /// NSMutableAttributedStringKitAdditions
    unsafe impl NSMutableAttributedString {
        #[method(superscriptRange:)]
        pub unsafe fn superscriptRange(&self, range: NSRange);

        #[method(subscriptRange:)]
        pub unsafe fn subscriptRange(&self, range: NSRange);

        #[method(unscriptRange:)]
        pub unsafe fn unscriptRange(&self, range: NSRange);

        #[method(applyFontTraits:range:)]
        pub unsafe fn applyFontTraits_range(&self, traitMask: NSFontTraitMask, range: NSRange);

        #[method(setAlignment:range:)]
        pub unsafe fn setAlignment_range(&self, alignment: NSTextAlignment, range: NSRange);

        #[method(setBaseWritingDirection:range:)]
        pub unsafe fn setBaseWritingDirection_range(
            &self,
            writingDirection: NSWritingDirection,
            range: NSRange,
        );
    }
);

extern_static!(NSUnderlinePatternSolid: NSUnderlineStyle = NSUnderlineStylePatternSolid);

extern_static!(NSUnderlinePatternDot: NSUnderlineStyle = NSUnderlineStylePatternDot);

extern_static!(NSUnderlinePatternDash: NSUnderlineStyle = NSUnderlineStylePatternDash);

extern_static!(NSUnderlinePatternDashDot: NSUnderlineStyle = NSUnderlineStylePatternDashDot);

extern_static!(NSUnderlinePatternDashDotDot: NSUnderlineStyle = NSUnderlineStylePatternDashDotDot);

extern_static!(NSUnderlineByWord: NSUnderlineStyle = NSUnderlineStyleByWord);

extern_static!(NSCharacterShapeAttributeName: &'static NSAttributedStringKey);

extern_static!(NSUsesScreenFontsDocumentAttribute: &'static NSAttributedStringKey);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSNoUnderlineStyle = 0,
        NSSingleUnderlineStyle = 1,
    }
);

extern_static!(NSUnderlineStrikethroughMask: NSUInteger);

extern_static!(NSUnderlineByWordMask: NSUInteger);

extern_methods!(
    /// NSDeprecatedKitAdditions
    unsafe impl NSAttributedString {
        #[method(containsAttachments)]
        pub unsafe fn containsAttachments(&self) -> bool;

        #[method_id(@__retain_semantics Other textFileTypes)]
        pub unsafe fn textFileTypes() -> Id<NSArray, Shared>;

        #[method_id(@__retain_semantics Other textPasteboardTypes)]
        pub unsafe fn textPasteboardTypes() -> Id<NSArray, Shared>;

        #[method_id(@__retain_semantics Other textUnfilteredFileTypes)]
        pub unsafe fn textUnfilteredFileTypes() -> Id<NSArray, Shared>;

        #[method_id(@__retain_semantics Other textUnfilteredPasteboardTypes)]
        pub unsafe fn textUnfilteredPasteboardTypes() -> Id<NSArray, Shared>;

        #[method_id(@__retain_semantics Init initWithURL:documentAttributes:)]
        pub unsafe fn initWithURL_documentAttributes(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            dict: *mut *mut NSDictionary,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithPath:documentAttributes:)]
        pub unsafe fn initWithPath_documentAttributes(
            this: Option<Allocated<Self>>,
            path: &NSString,
            dict: *mut *mut NSDictionary,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other URLAtIndex:effectiveRange:)]
        pub unsafe fn URLAtIndex_effectiveRange(
            &self,
            location: NSUInteger,
            effectiveRange: NSRangePointer,
        ) -> Option<Id<NSURL, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecatedKitAdditions
    unsafe impl NSMutableAttributedString {
        #[method(readFromURL:options:documentAttributes:)]
        pub unsafe fn readFromURL_options_documentAttributes(
            &self,
            url: &NSURL,
            options: &NSDictionary,
            dict: *mut *mut NSDictionary,
        ) -> bool;

        #[method(readFromData:options:documentAttributes:)]
        pub unsafe fn readFromData_options_documentAttributes(
            &self,
            data: &NSData,
            options: &NSDictionary,
            dict: *mut *mut NSDictionary,
        ) -> bool;
    }
);
