//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDraggingSession;

    unsafe impl ClassType for NSDraggingSession {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSDraggingSession {}

extern_methods!(
    unsafe impl NSDraggingSession {
        #[cfg(feature = "AppKit_NSDragging")]
        #[method(draggingFormation)]
        pub unsafe fn draggingFormation(&self) -> NSDraggingFormation;

        #[cfg(feature = "AppKit_NSDragging")]
        #[method(setDraggingFormation:)]
        pub unsafe fn setDraggingFormation(&self, dragging_formation: NSDraggingFormation);

        #[method(animatesToStartingPositionsOnCancelOrFail)]
        pub unsafe fn animatesToStartingPositionsOnCancelOrFail(&self) -> bool;

        #[method(setAnimatesToStartingPositionsOnCancelOrFail:)]
        pub unsafe fn setAnimatesToStartingPositionsOnCancelOrFail(
            &self,
            animates_to_starting_positions_on_cancel_or_fail: bool,
        );

        #[method(draggingLeaderIndex)]
        pub unsafe fn draggingLeaderIndex(&self) -> NSInteger;

        #[method(setDraggingLeaderIndex:)]
        pub unsafe fn setDraggingLeaderIndex(&self, dragging_leader_index: NSInteger);

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other draggingPasteboard)]
        pub unsafe fn draggingPasteboard(&self) -> Id<NSPasteboard>;

        #[method(draggingSequenceNumber)]
        pub unsafe fn draggingSequenceNumber(&self) -> NSInteger;

        #[method(draggingLocation)]
        pub unsafe fn draggingLocation(&self) -> NSPoint;

        #[cfg(all(
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSDraggingItem",
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSView",
            feature = "block2"
        ))]
        #[method(enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:)]
        pub unsafe fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock(
            &self,
            enum_opts: NSDraggingItemEnumerationOptions,
            view: Option<&NSView>,
            class_array: &NSArray<TodoClass>,
            search_options: &NSDictionary<NSPasteboardReadingOptionKey, AnyObject>,
            block: &Block<dyn Fn(NonNull<NSDraggingItem>, NSInteger, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDraggingSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
