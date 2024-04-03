//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDragOperation(pub NSUInteger);
impl NSDragOperation {
    #[doc(alias = "NSDragOperationNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSDragOperationCopy")]
    pub const Copy: Self = Self(1);
    #[doc(alias = "NSDragOperationLink")]
    pub const Link: Self = Self(2);
    #[doc(alias = "NSDragOperationGeneric")]
    pub const Generic: Self = Self(4);
    #[doc(alias = "NSDragOperationPrivate")]
    pub const Private: Self = Self(8);
    #[doc(alias = "NSDragOperationMove")]
    pub const Move: Self = Self(16);
    #[doc(alias = "NSDragOperationDelete")]
    pub const Delete: Self = Self(32);
    #[doc(alias = "NSDragOperationEvery")]
    pub const Every: Self = Self(NSUIntegerMax as _);
    #[deprecated]
    #[doc(alias = "NSDragOperationAll_Obsolete")]
    pub const All_Obsolete: Self = Self(15);
    #[deprecated]
    #[doc(alias = "NSDragOperationAll")]
    pub const All: Self = Self(NSDragOperation::All_Obsolete.0);
}

unsafe impl Encode for NSDragOperation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDragOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDraggingFormation(pub NSInteger);
impl NSDraggingFormation {
    #[doc(alias = "NSDraggingFormationDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSDraggingFormationNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "NSDraggingFormationPile")]
    pub const Pile: Self = Self(2);
    #[doc(alias = "NSDraggingFormationList")]
    pub const List: Self = Self(3);
    #[doc(alias = "NSDraggingFormationStack")]
    pub const Stack: Self = Self(4);
}

unsafe impl Encode for NSDraggingFormation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDraggingFormation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDraggingContext(pub NSInteger);
impl NSDraggingContext {
    #[doc(alias = "NSDraggingContextOutsideApplication")]
    pub const OutsideApplication: Self = Self(0);
    #[doc(alias = "NSDraggingContextWithinApplication")]
    pub const WithinApplication: Self = Self(1);
}

unsafe impl Encode for NSDraggingContext {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSDraggingContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSDraggingItemEnumerationOptions(pub NSUInteger);
impl NSDraggingItemEnumerationOptions {
    pub const NSDraggingItemEnumerationConcurrent: Self =
        Self(NSEnumerationOptions::NSEnumerationConcurrent.0);
    pub const NSDraggingItemEnumerationClearNonenumeratedImages: Self = Self(1 << 16);
}

unsafe impl Encode for NSDraggingItemEnumerationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSDraggingItemEnumerationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSpringLoadingHighlight(pub NSInteger);
impl NSSpringLoadingHighlight {
    #[doc(alias = "NSSpringLoadingHighlightNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSSpringLoadingHighlightStandard")]
    pub const Standard: Self = Self(1);
    #[doc(alias = "NSSpringLoadingHighlightEmphasized")]
    pub const Emphasized: Self = Self(2);
}

unsafe impl Encode for NSSpringLoadingHighlight {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSSpringLoadingHighlight {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSDraggingInfo: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSWindow"))]
        #[method_id(@__retain_semantics Other draggingDestinationWindow)]
        unsafe fn draggingDestinationWindow(&self) -> Option<Id<NSWindow>>;

        #[method(draggingSourceOperationMask)]
        unsafe fn draggingSourceOperationMask(&self) -> NSDragOperation;

        #[method(draggingLocation)]
        unsafe fn draggingLocation(&self) -> NSPoint;

        #[method(draggedImageLocation)]
        unsafe fn draggedImageLocation(&self) -> NSPoint;

        #[cfg(feature = "AppKit_NSImage")]
        #[deprecated = "Use NSDraggingItem objects instead"]
        #[method_id(@__retain_semantics Other draggedImage)]
        unsafe fn draggedImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other draggingPasteboard)]
        unsafe fn draggingPasteboard(&self) -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other draggingSource)]
        unsafe fn draggingSource(&self) -> Option<Id<AnyObject>>;

        #[method(draggingSequenceNumber)]
        unsafe fn draggingSequenceNumber(&self) -> NSInteger;

        #[method(slideDraggedImageTo:)]
        unsafe fn slideDraggedImageTo(&self, screen_point: NSPoint);

        #[deprecated = "Use NSFilePromiseReceiver objects instead"]
        #[method_id(@__retain_semantics Other namesOfPromisedFilesDroppedAtDestination:)]
        unsafe fn namesOfPromisedFilesDroppedAtDestination(
            &self,
            drop_destination: &NSURL,
        ) -> Option<Id<NSArray<NSString>>>;

        #[method(draggingFormation)]
        unsafe fn draggingFormation(&self) -> NSDraggingFormation;

        #[method(setDraggingFormation:)]
        unsafe fn setDraggingFormation(&self, dragging_formation: NSDraggingFormation);

        #[method(animatesToDestination)]
        unsafe fn animatesToDestination(&self) -> bool;

        #[method(setAnimatesToDestination:)]
        unsafe fn setAnimatesToDestination(&self, animates_to_destination: bool);

        #[method(numberOfValidItemsForDrop)]
        unsafe fn numberOfValidItemsForDrop(&self) -> NSInteger;

        #[method(setNumberOfValidItemsForDrop:)]
        unsafe fn setNumberOfValidItemsForDrop(&self, number_of_valid_items_for_drop: NSInteger);

        #[cfg(all(
            feature = "AppKit_NSDraggingItem",
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "block2"
        ))]
        #[method(enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:)]
        unsafe fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock(
            &self,
            enum_opts: NSDraggingItemEnumerationOptions,
            view: Option<&NSView>,
            class_array: &NSArray<TodoClass>,
            search_options: &NSDictionary<NSPasteboardReadingOptionKey, AnyObject>,
            block: &Block<dyn Fn(NonNull<NSDraggingItem>, NSInteger, NonNull<Bool>)>,
        );

        #[method(springLoadingHighlight)]
        unsafe fn springLoadingHighlight(&self) -> NSSpringLoadingHighlight;

        #[method(resetSpringLoading)]
        unsafe fn resetSpringLoading(&self);
    }

    unsafe impl ProtocolType for dyn NSDraggingInfo {}
);

extern_protocol!(
    pub unsafe trait NSDraggingDestination: NSObjectProtocol + IsMainThreadOnly {
        #[optional]
        #[method(draggingEntered:)]
        unsafe fn draggingEntered(
            &self,
            sender: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> NSDragOperation;

        #[optional]
        #[method(draggingUpdated:)]
        unsafe fn draggingUpdated(
            &self,
            sender: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> NSDragOperation;

        #[optional]
        #[method(draggingExited:)]
        unsafe fn draggingExited(&self, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>);

        #[optional]
        #[method(prepareForDragOperation:)]
        unsafe fn prepareForDragOperation(
            &self,
            sender: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> bool;

        #[optional]
        #[method(performDragOperation:)]
        unsafe fn performDragOperation(&self, sender: &ProtocolObject<dyn NSDraggingInfo>) -> bool;

        #[optional]
        #[method(concludeDragOperation:)]
        unsafe fn concludeDragOperation(&self, sender: Option<&ProtocolObject<dyn NSDraggingInfo>>);

        #[optional]
        #[method(draggingEnded:)]
        unsafe fn draggingEnded(&self, sender: &ProtocolObject<dyn NSDraggingInfo>);

        #[optional]
        #[method(wantsPeriodicDraggingUpdates)]
        unsafe fn wantsPeriodicDraggingUpdates(&self) -> bool;

        #[optional]
        #[method(updateDraggingItemsForDrag:)]
        unsafe fn updateDraggingItemsForDrag(
            &self,
            sender: Option<&ProtocolObject<dyn NSDraggingInfo>>,
        );
    }

    unsafe impl ProtocolType for dyn NSDraggingDestination {}
);

extern_protocol!(
    pub unsafe trait NSDraggingSource: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[method(draggingSession:sourceOperationMaskForDraggingContext:)]
        unsafe fn draggingSession_sourceOperationMaskForDraggingContext(
            &self,
            session: &NSDraggingSession,
            context: NSDraggingContext,
        ) -> NSDragOperation;

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(draggingSession:willBeginAtPoint:)]
        unsafe fn draggingSession_willBeginAtPoint(
            &self,
            session: &NSDraggingSession,
            screen_point: NSPoint,
        );

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(draggingSession:movedToPoint:)]
        unsafe fn draggingSession_movedToPoint(
            &self,
            session: &NSDraggingSession,
            screen_point: NSPoint,
        );

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(draggingSession:endedAtPoint:operation:)]
        unsafe fn draggingSession_endedAtPoint_operation(
            &self,
            session: &NSDraggingSession,
            screen_point: NSPoint,
            operation: NSDragOperation,
        );

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(ignoreModifierKeysForDraggingSession:)]
        unsafe fn ignoreModifierKeysForDraggingSession(&self, session: &NSDraggingSession) -> bool;
    }

    unsafe impl ProtocolType for dyn NSDraggingSource {}
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSSpringLoadingOptions(pub NSUInteger);
impl NSSpringLoadingOptions {
    pub const NSSpringLoadingDisabled: Self = Self(0);
    pub const NSSpringLoadingEnabled: Self = Self(1 << 0);
    pub const NSSpringLoadingContinuousActivation: Self = Self(1 << 1);
    pub const NSSpringLoadingNoHover: Self = Self(1 << 3);
}

unsafe impl Encode for NSSpringLoadingOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSSpringLoadingOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSSpringLoadingDestination:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[method(springLoadingActivated:draggingInfo:)]
        unsafe fn springLoadingActivated_draggingInfo(
            &self,
            activated: bool,
            dragging_info: &ProtocolObject<dyn NSDraggingInfo>,
        );

        #[method(springLoadingHighlightChanged:)]
        unsafe fn springLoadingHighlightChanged(
            &self,
            dragging_info: &ProtocolObject<dyn NSDraggingInfo>,
        );

        #[optional]
        #[method(springLoadingEntered:)]
        unsafe fn springLoadingEntered(
            &self,
            dragging_info: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> NSSpringLoadingOptions;

        #[optional]
        #[method(springLoadingUpdated:)]
        unsafe fn springLoadingUpdated(
            &self,
            dragging_info: &ProtocolObject<dyn NSDraggingInfo>,
        ) -> NSSpringLoadingOptions;

        #[optional]
        #[method(springLoadingExited:)]
        unsafe fn springLoadingExited(&self, dragging_info: &ProtocolObject<dyn NSDraggingInfo>);

        #[optional]
        #[method(draggingEnded:)]
        unsafe fn draggingEnded(&self, dragging_info: &ProtocolObject<dyn NSDraggingInfo>);
    }

    unsafe impl ProtocolType for dyn NSSpringLoadingDestination {}
);
