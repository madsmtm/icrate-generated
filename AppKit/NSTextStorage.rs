//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextStorageEditActions {
        NSTextStorageEditedAttributes = 1 << 0,
        NSTextStorageEditedCharacters = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextStorage")]
    pub struct NSTextStorage;

    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl ClassType for NSTextStorage {
        #[inherits(NSAttributedString, NSObject)]
        type Super = NSMutableAttributedString;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(all(feature = "AppKit_NSLayoutManager", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other layoutManagers)]
        pub unsafe fn layoutManagers(&self) -> Id<NSArray<NSLayoutManager>, Shared>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(addLayoutManager:)]
        pub unsafe fn addLayoutManager(&self, a_layout_manager: &NSLayoutManager);

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(removeLayoutManager:)]
        pub unsafe fn removeLayoutManager(&self, a_layout_manager: &NSLayoutManager);

        #[method(editedMask)]
        pub unsafe fn editedMask(&self) -> NSTextStorageEditActions;

        #[method(editedRange)]
        pub unsafe fn editedRange(&self) -> NSRange;

        #[method(changeInLength)]
        pub unsafe fn changeInLength(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextStorageDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextStorageDelegate>);

        #[method(edited:range:changeInLength:)]
        pub unsafe fn edited_range_changeInLength(
            &self,
            edited_mask: NSTextStorageEditActions,
            edited_range: NSRange,
            delta: NSInteger,
        );

        #[method(processEditing)]
        pub unsafe fn processEditing(&self);

        #[method(fixesAttributesLazily)]
        pub unsafe fn fixesAttributesLazily(&self) -> bool;

        #[method(invalidateAttributesInRange:)]
        pub unsafe fn invalidateAttributesInRange(&self, range: NSRange);

        #[method(ensureAttributesAreFixedInRange:)]
        pub unsafe fn ensureAttributesAreFixedInRange(&self, range: NSRange);

        #[method_id(@__retain_semantics Other textStorageObserver)]
        pub unsafe fn textStorageObserver(&self) -> Option<Id<NSTextStorageObserving, Shared>>;

        #[method(setTextStorageObserver:)]
        pub unsafe fn setTextStorageObserver(
            &self,
            text_storage_observer: Option<&NSTextStorageObserving>,
        );
    }
);

extern_protocol!(
    pub struct NSTextStorageDelegate;

    unsafe impl ProtocolType for NSTextStorageDelegate {
        #[cfg(feature = "AppKit_NSTextStorage")]
        #[optional]
        #[method(textStorage:willProcessEditing:range:changeInLength:)]
        pub unsafe fn textStorage_willProcessEditing_range_changeInLength(
            &self,
            text_storage: &NSTextStorage,
            edited_mask: NSTextStorageEditActions,
            edited_range: NSRange,
            delta: NSInteger,
        );

        #[cfg(feature = "AppKit_NSTextStorage")]
        #[optional]
        #[method(textStorage:didProcessEditing:range:changeInLength:)]
        pub unsafe fn textStorage_didProcessEditing_range_changeInLength(
            &self,
            text_storage: &NSTextStorage,
            edited_mask: NSTextStorageEditActions,
            edited_range: NSRange,
            delta: NSInteger,
        );
    }
);

extern_static!(NSTextStorageWillProcessEditingNotification: &'static NSNotificationName);

extern_static!(NSTextStorageDidProcessEditingNotification: &'static NSNotificationName);

extern_protocol!(
    pub struct NSTextStorageObserving;

    unsafe impl ProtocolType for NSTextStorageObserving {
        #[cfg(feature = "AppKit_NSTextStorage")]
        #[method_id(@__retain_semantics Other textStorage)]
        pub unsafe fn textStorage(&self) -> Option<Id<NSTextStorage, Shared>>;

        #[cfg(feature = "AppKit_NSTextStorage")]
        #[method(setTextStorage:)]
        pub unsafe fn setTextStorage(&self, text_storage: Option<&NSTextStorage>);

        #[cfg(feature = "AppKit_NSTextStorage")]
        #[method(processEditingForTextStorage:edited:range:changeInLength:invalidatedRange:)]
        pub unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange(
            &self,
            text_storage: &NSTextStorage,
            edit_mask: NSTextStorageEditActions,
            new_char_range: NSRange,
            delta: NSInteger,
            invalidated_char_range: NSRange,
        );

        #[cfg(feature = "AppKit_NSTextStorage")]
        #[method(performEditingTransactionForTextStorage:usingBlock:)]
        pub unsafe fn performEditingTransactionForTextStorage_usingBlock(
            &self,
            text_storage: &NSTextStorage,
            transaction: &Block<(), ()>,
        );
    }
);

pub type NSTextStorageEditedOptions = NSUInteger;

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSAttributedStringDocumentFormats
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithURL:options:documentAttributes:error:_)]
        pub unsafe fn initWithURL_options_documentAttributes_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Init initWithData:options:documentAttributes:error:_)]
        pub unsafe fn initWithData_options_documentAttributes_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Init initWithRTF:documentAttributes:)]
        pub unsafe fn initWithRTF_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Init initWithRTFD:documentAttributes:)]
        pub unsafe fn initWithRTFD_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Init initWithHTML:documentAttributes:)]
        pub unsafe fn initWithHTML_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithHTML:baseURL:documentAttributes:)]
        pub unsafe fn initWithHTML_baseURL_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            base: &NSURL,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Init initWithDocFormat:documentAttributes:)]
        pub unsafe fn initWithDocFormat_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Init initWithHTML:options:documentAttributes:)]
        pub unsafe fn initWithHTML_options_documentAttributes(
            this: Option<Allocated<Self>>,
            data: &NSData,
            options: &NSDictionary<NSAttributedStringDocumentReadingOptionKey, Object>,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSFileWrapper"
        ))]
        #[method_id(@__retain_semantics Init initWithRTFDFileWrapper:documentAttributes:)]
        pub unsafe fn initWithRTFDFileWrapper_documentAttributes(
            this: Option<Allocated<Self>>,
            wrapper: &NSFileWrapper,
            dict: *mut *mut NSDictionary<NSAttributedStringDocumentAttributeKey, Object>,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSDeprecatedKitAdditions
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:documentAttributes:)]
        pub unsafe fn initWithURL_documentAttributes(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            dict: *mut *mut NSDictionary,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithPath:documentAttributes:)]
        pub unsafe fn initWithPath_documentAttributes(
            this: Option<Allocated<Self>>,
            path: &NSString,
            dict: *mut *mut NSDictionary,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSExtendedAttributedString
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            str: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithString:attributes:)]
        pub unsafe fn initWithString_attributes(
            this: Option<Allocated<Self>>,
            str: &NSString,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Option<Allocated<Self>>,
            attr_str: &NSAttributedString,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSAttributedString`
    ///
    /// NSAttributedStringCreateFromMarkdown
    #[cfg(feature = "AppKit_NSTextStorage")]
    unsafe impl NSTextStorage {
        #[cfg(all(
            feature = "Foundation_NSAttributedStringMarkdownParsingOptions",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfMarkdownFileAtURL:options:baseURL:error:_)]
        pub unsafe fn initWithContentsOfMarkdownFileAtURL_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdown_file: &NSURL,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSAttributedStringMarkdownParsingOptions",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithMarkdown:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdown_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdown: &NSData,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSAttributedStringMarkdownParsingOptions",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithMarkdownString:options:baseURL:error:_)]
        pub unsafe fn initWithMarkdownString_options_baseURL_error(
            this: Option<Allocated<Self>>,
            markdown_string: &NSString,
            options: Option<&NSAttributedStringMarkdownParsingOptions>,
            base_url: Option<&NSURL>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);
