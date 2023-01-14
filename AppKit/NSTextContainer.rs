//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextContainer")]
    pub struct NSTextContainer;

    #[cfg(feature = "AppKit_NSTextContainer")]
    unsafe impl ClassType for NSTextContainer {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextContainer")]
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Option<Allocated<Self>>, size: NSSize)
            -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method_id(@__retain_semantics Other layoutManager)]
        pub unsafe fn layoutManager(&self) -> Option<Id<NSLayoutManager, Shared>>;

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(setLayoutManager:)]
        pub unsafe fn setLayoutManager(&self, layout_manager: Option<&NSLayoutManager>);

        #[cfg(feature = "AppKit_NSLayoutManager")]
        #[method(replaceLayoutManager:)]
        pub unsafe fn replaceLayoutManager(&self, new_layout_manager: &NSLayoutManager);

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[cfg(all(feature = "AppKit_NSBezierPath", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other exclusionPaths)]
        pub unsafe fn exclusionPaths(&self) -> Id<NSArray<NSBezierPath>, Shared>;

        #[cfg(all(feature = "AppKit_NSBezierPath", feature = "Foundation_NSArray"))]
        #[method(setExclusionPaths:)]
        pub unsafe fn setExclusionPaths(&self, exclusion_paths: &NSArray<NSBezierPath>);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(lineFragmentPadding)]
        pub unsafe fn lineFragmentPadding(&self) -> CGFloat;

        #[method(setLineFragmentPadding:)]
        pub unsafe fn setLineFragmentPadding(&self, line_fragment_padding: CGFloat);

        #[method(maximumNumberOfLines)]
        pub unsafe fn maximumNumberOfLines(&self) -> NSUInteger;

        #[method(setMaximumNumberOfLines:)]
        pub unsafe fn setMaximumNumberOfLines(&self, maximum_number_of_lines: NSUInteger);

        #[method(lineFragmentRectForProposedRect:atIndex:writingDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_atIndex_writingDirection_remainingRect(
            &self,
            proposed_rect: NSRect,
            character_index: NSUInteger,
            base_writing_direction: NSWritingDirection,
            remaining_rect: *mut NSRect,
        ) -> NSRect;

        #[method(isSimpleRectangularTextContainer)]
        pub unsafe fn isSimpleRectangularTextContainer(&self) -> bool;

        #[method(widthTracksTextView)]
        pub unsafe fn widthTracksTextView(&self) -> bool;

        #[method(setWidthTracksTextView:)]
        pub unsafe fn setWidthTracksTextView(&self, width_tracks_text_view: bool);

        #[method(heightTracksTextView)]
        pub unsafe fn heightTracksTextView(&self) -> bool;

        #[method(setHeightTracksTextView:)]
        pub unsafe fn setHeightTracksTextView(&self, height_tracks_text_view: bool);

        #[cfg(feature = "AppKit_NSTextView")]
        #[method_id(@__retain_semantics Other textView)]
        pub unsafe fn textView(&self) -> Option<Id<NSTextView, Shared>>;

        #[cfg(feature = "AppKit_NSTextView")]
        #[method(setTextView:)]
        pub unsafe fn setTextView(&self, text_view: Option<&NSTextView>);
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineSweepDirection {
        NSLineSweepLeft = 0,
        NSLineSweepRight = 1,
        NSLineSweepDown = 2,
        NSLineSweepUp = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLineMovementDirection {
        NSLineDoesntMove = 0,
        NSLineMovesLeft = 1,
        NSLineMovesRight = 2,
        NSLineMovesDown = 3,
        NSLineMovesUp = 4,
    }
);

extern_methods!(
    /// NSTextContainerDeprecated
    #[cfg(feature = "AppKit_NSTextContainer")]
    unsafe impl NSTextContainer {
        #[method_id(@__retain_semantics Init initWithContainerSize:)]
        pub unsafe fn initWithContainerSize(
            this: Option<Allocated<Self>>,
            a_container_size: NSSize,
        ) -> Id<Self, Shared>;

        #[method(containerSize)]
        pub unsafe fn containerSize(&self) -> NSSize;

        #[method(setContainerSize:)]
        pub unsafe fn setContainerSize(&self, container_size: NSSize);

        #[method(lineFragmentRectForProposedRect:sweepDirection:movementDirection:remainingRect:)]
        pub unsafe fn lineFragmentRectForProposedRect_sweepDirection_movementDirection_remainingRect(
            &self,
            proposed_rect: NSRect,
            sweep_direction: NSLineSweepDirection,
            movement_direction: NSLineMovementDirection,
            remaining_rect: NSRectPointer,
        ) -> NSRect;

        #[method(containsPoint:)]
        pub unsafe fn containsPoint(&self, point: NSPoint) -> bool;
    }
);
