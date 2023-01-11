//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextContentManagerEnumerationOptions {
        NSTextContentManagerEnumerationOptionsNone = 0,
        NSTextContentManagerEnumerationOptionsReverse = 1 << 0,
    }
);

extern_protocol!(
    pub struct NSTextElementProvider;

    unsafe impl ProtocolType for NSTextElementProvider {
        #[method_id(@__retain_semantics Other documentRange)]
        pub unsafe fn documentRange(&self) -> Id<NSTextRange, Shared>;

        #[method_id(@__retain_semantics Other enumerateTextElementsFromLocation:options:usingBlock:)]
        pub unsafe fn enumerateTextElementsFromLocation_options_usingBlock(
            &self,
            textLocation: Option<&NSTextLocation>,
            options: NSTextContentManagerEnumerationOptions,
            block: &Block<(NonNull<NSTextElement>,), Bool>,
        ) -> Option<Id<NSTextLocation, Shared>>;

        #[method(replaceContentsInRange:withTextElements:)]
        pub unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            textElements: Option<&NSArray<NSTextElement>>,
        );

        #[method(synchronizeToBackingStore:)]
        pub unsafe fn synchronizeToBackingStore(
            &self,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
        pub unsafe fn locationFromLocation_withOffset(
            &self,
            location: &NSTextLocation,
            offset: NSInteger,
        ) -> Option<Id<NSTextLocation, Shared>>;

        #[optional]
        #[method(offsetFromLocation:toLocation:)]
        pub unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &NSTextLocation,
            to: &NSTextLocation,
        ) -> NSInteger;

        #[optional]
        #[method_id(@__retain_semantics Other adjustedRangeFromRange:forEditingTextSelection:)]
        pub unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            textRange: &NSTextRange,
            forEditingTextSelection: bool,
        ) -> Option<Id<NSTextRange, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextContentManager")]
    pub struct NSTextContentManager;

    #[cfg(feature = "AppKit_NSTextContentManager")]
    unsafe impl ClassType for NSTextContentManager {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextContentManager")]
    unsafe impl NSTextContentManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextContentManagerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextContentManagerDelegate>);

        #[cfg(all(feature = "AppKit_NSTextLayoutManager", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLayoutManagers)]
        pub unsafe fn textLayoutManagers(&self) -> Id<NSArray<NSTextLayoutManager>, Shared>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method(addTextLayoutManager:)]
        pub unsafe fn addTextLayoutManager(&self, textLayoutManager: &NSTextLayoutManager);

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method(removeTextLayoutManager:)]
        pub unsafe fn removeTextLayoutManager(&self, textLayoutManager: &NSTextLayoutManager);

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other primaryTextLayoutManager)]
        pub unsafe fn primaryTextLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method(setPrimaryTextLayoutManager:)]
        pub unsafe fn setPrimaryTextLayoutManager(
            &self,
            primaryTextLayoutManager: Option<&NSTextLayoutManager>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(synchronizeTextLayoutManagers:)]
        pub unsafe fn synchronizeTextLayoutManagers(
            &self,
            completionHandler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other textElementsForRange:)]
        pub unsafe fn textElementsForRange(
            &self,
            range: &NSTextRange,
        ) -> Id<NSArray<NSTextElement>, Shared>;

        #[method(hasEditingTransaction)]
        pub unsafe fn hasEditingTransaction(&self) -> bool;

        #[method(performEditingTransactionUsingBlock:)]
        pub unsafe fn performEditingTransactionUsingBlock(&self, transaction: &Block<(), ()>);

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(recordEditActionInRange:newTextRange:)]
        pub unsafe fn recordEditActionInRange_newTextRange(
            &self,
            originalTextRange: &NSTextRange,
            newTextRange: &NSTextRange,
        );

        #[method(automaticallySynchronizesTextLayoutManagers)]
        pub unsafe fn automaticallySynchronizesTextLayoutManagers(&self) -> bool;

        #[method(setAutomaticallySynchronizesTextLayoutManagers:)]
        pub unsafe fn setAutomaticallySynchronizesTextLayoutManagers(
            &self,
            automaticallySynchronizesTextLayoutManagers: bool,
        );

        #[method(automaticallySynchronizesToBackingStore)]
        pub unsafe fn automaticallySynchronizesToBackingStore(&self) -> bool;

        #[method(setAutomaticallySynchronizesToBackingStore:)]
        pub unsafe fn setAutomaticallySynchronizesToBackingStore(
            &self,
            automaticallySynchronizesToBackingStore: bool,
        );
    }
);

extern_protocol!(
    pub struct NSTextContentManagerDelegate;

    unsafe impl ProtocolType for NSTextContentManagerDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other textContentManager:textElementAtLocation:)]
        pub unsafe fn textContentManager_textElementAtLocation(
            &self,
            textContentManager: &NSTextContentManager,
            location: &NSTextLocation,
        ) -> Option<Id<NSTextElement, Shared>>;

        #[optional]
        #[method(textContentManager:shouldEnumerateTextElement:options:)]
        pub unsafe fn textContentManager_shouldEnumerateTextElement_options(
            &self,
            textContentManager: &NSTextContentManager,
            textElement: &NSTextElement,
            options: NSTextContentManagerEnumerationOptions,
        ) -> bool;
    }
);

extern_protocol!(
    pub struct NSTextContentStorageDelegate;

    unsafe impl ProtocolType for NSTextContentStorageDelegate {
        #[optional]
        #[method_id(@__retain_semantics Other textContentStorage:textParagraphWithRange:)]
        pub unsafe fn textContentStorage_textParagraphWithRange(
            &self,
            textContentStorage: &NSTextContentStorage,
            range: NSRange,
        ) -> Option<Id<NSTextParagraph, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextContentStorage")]
    pub struct NSTextContentStorage;

    #[cfg(feature = "AppKit_NSTextContentStorage")]
    unsafe impl ClassType for NSTextContentStorage {
        #[inherits(NSObject)]
        type Super = NSTextContentManager;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextContentStorage")]
    unsafe impl NSTextContentStorage {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextContentStorageDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextContentStorageDelegate>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributedString: Option<&NSAttributedString>);

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Other attributedStringForTextElement:)]
        pub unsafe fn attributedStringForTextElement(
            &self,
            textElement: &NSTextElement,
        ) -> Option<Id<NSAttributedString, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Other textElementForAttributedString:)]
        pub unsafe fn textElementForAttributedString(
            &self,
            attributedString: &NSAttributedString,
        ) -> Option<Id<NSTextElement, Shared>>;

        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
        pub unsafe fn locationFromLocation_withOffset(
            &self,
            location: &NSTextLocation,
            offset: NSInteger,
        ) -> Option<Id<NSTextLocation, Shared>>;

        #[method(offsetFromLocation:toLocation:)]
        pub unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &NSTextLocation,
            to: &NSTextLocation,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other adjustedRangeFromRange:forEditingTextSelection:)]
        pub unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            textRange: &NSTextRange,
            forEditingTextSelection: bool,
        ) -> Option<Id<NSTextRange, Shared>>;
    }
);

extern_static!(
    NSTextContentStorageUnsupportedAttributeAddedNotification: &'static NSNotificationName
);
