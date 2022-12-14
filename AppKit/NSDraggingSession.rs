//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDraggingSession;

    unsafe impl ClassType for NSDraggingSession {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSDraggingSession {
        #[method(draggingFormation)]
        pub unsafe fn draggingFormation(&self) -> NSDraggingFormation;

        #[method(setDraggingFormation:)]
        pub unsafe fn setDraggingFormation(&self, draggingFormation: NSDraggingFormation);

        #[method(animatesToStartingPositionsOnCancelOrFail)]
        pub unsafe fn animatesToStartingPositionsOnCancelOrFail(&self) -> bool;

        #[method(setAnimatesToStartingPositionsOnCancelOrFail:)]
        pub unsafe fn setAnimatesToStartingPositionsOnCancelOrFail(
            &self,
            animatesToStartingPositionsOnCancelOrFail: bool,
        );

        #[method(draggingLeaderIndex)]
        pub unsafe fn draggingLeaderIndex(&self) -> NSInteger;

        #[method(setDraggingLeaderIndex:)]
        pub unsafe fn setDraggingLeaderIndex(&self, draggingLeaderIndex: NSInteger);

        #[method_id(@__retain_semantics Other draggingPasteboard)]
        pub unsafe fn draggingPasteboard(&self) -> Id<NSPasteboard, Shared>;

        #[method(draggingSequenceNumber)]
        pub unsafe fn draggingSequenceNumber(&self) -> NSInteger;

        #[method(draggingLocation)]
        pub unsafe fn draggingLocation(&self) -> NSPoint;

        #[method(enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:)]
        pub unsafe fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock(
            &self,
            enumOpts: NSDraggingItemEnumerationOptions,
            view: Option<&NSView>,
            classArray: &NSArray<TodoClass>,
            searchOptions: &NSDictionary<NSPasteboardReadingOptionKey, Object>,
            block: &Block<(NonNull<NSDraggingItem>, NSInteger, NonNull<Bool>), ()>,
        );
    }
);
