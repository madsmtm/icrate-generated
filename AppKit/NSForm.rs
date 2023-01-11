//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSForm")]
    pub struct NSForm;

    #[cfg(feature = "AppKit_NSForm")]
    unsafe impl ClassType for NSForm {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSMatrix;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSForm")]
    unsafe impl NSForm {
        #[method(indexOfSelectedItem)]
        pub unsafe fn indexOfSelectedItem(&self) -> NSInteger;

        #[method(setEntryWidth:)]
        pub unsafe fn setEntryWidth(&self, width: CGFloat);

        #[method(setInterlineSpacing:)]
        pub unsafe fn setInterlineSpacing(&self, spacing: CGFloat);

        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, flag: bool);

        #[method(setBezeled:)]
        pub unsafe fn setBezeled(&self, flag: bool);

        #[method(setTitleAlignment:)]
        pub unsafe fn setTitleAlignment(&self, mode: NSTextAlignment);

        #[method(setTextAlignment:)]
        pub unsafe fn setTextAlignment(&self, mode: NSTextAlignment);

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, fontObj: &NSFont);

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setTextFont:)]
        pub unsafe fn setTextFont(&self, fontObj: &NSFont);

        #[method_id(@__retain_semantics Other cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Option<Id<Object, Shared>>;

        #[method(drawCellAtIndex:)]
        pub unsafe fn drawCellAtIndex(&self, index: NSInteger);

        #[cfg(all(feature = "AppKit_NSFormCell", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addEntry:)]
        pub unsafe fn addEntry(&self, title: &NSString) -> Id<NSFormCell, Shared>;

        #[cfg(all(feature = "AppKit_NSFormCell", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other insertEntry:atIndex:)]
        pub unsafe fn insertEntry_atIndex(
            &self,
            title: &NSString,
            index: NSInteger,
        ) -> Option<Id<NSFormCell, Shared>>;

        #[method(removeEntryAtIndex:)]
        pub unsafe fn removeEntryAtIndex(&self, index: NSInteger);

        #[method(indexOfCellWithTag:)]
        pub unsafe fn indexOfCellWithTag(&self, tag: NSInteger) -> NSInteger;

        #[method(selectTextAtIndex:)]
        pub unsafe fn selectTextAtIndex(&self, index: NSInteger);

        #[method(setFrameSize:)]
        pub unsafe fn setFrameSize(&self, newSize: NSSize);

        #[method(setTitleBaseWritingDirection:)]
        pub unsafe fn setTitleBaseWritingDirection(&self, writingDirection: NSWritingDirection);

        #[method(setTextBaseWritingDirection:)]
        pub unsafe fn setTextBaseWritingDirection(&self, writingDirection: NSWritingDirection);

        #[method(setPreferredTextFieldWidth:)]
        pub unsafe fn setPreferredTextFieldWidth(&self, preferredWidth: CGFloat);

        #[method(preferredTextFieldWidth)]
        pub unsafe fn preferredTextFieldWidth(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSMatrix`
    #[cfg(feature = "AppKit_NSForm")]
    unsafe impl NSForm {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Init initWithFrame:mode:prototype:numberOfRows:numberOfColumns:)]
        pub unsafe fn initWithFrame_mode_prototype_numberOfRows_numberOfColumns(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
            mode: NSMatrixMode,
            cell: &NSCell,
            rowsHigh: NSInteger,
            colsWide: NSInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithFrame:mode:cellClass:numberOfRows:numberOfColumns:)]
        pub unsafe fn initWithFrame_mode_cellClass_numberOfRows_numberOfColumns(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
            mode: NSMatrixMode,
            factoryId: Option<&Class>,
            rowsHigh: NSInteger,
            colsWide: NSInteger,
        ) -> Id<Self, Shared>;
    }
);
