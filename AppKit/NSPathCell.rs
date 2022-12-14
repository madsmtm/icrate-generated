//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPathStyle {
        NSPathStyleStandard = 0,
        NSPathStylePopUp = 2,
        NSPathStyleNavigationBar = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPathCell;

    unsafe impl ClassType for NSPathCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSPathCell {
        #[method(pathStyle)]
        pub unsafe fn pathStyle(&self) -> NSPathStyle;

        #[method(setPathStyle:)]
        pub unsafe fn setPathStyle(&self, pathStyle: NSPathStyle);

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[method(setURL:)]
        pub unsafe fn setURL(&self, URL: Option<&NSURL>);

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, obj: Option<&Object>);

        #[method_id(@__retain_semantics Other allowedTypes)]
        pub unsafe fn allowedTypes(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(setAllowedTypes:)]
        pub unsafe fn setAllowedTypes(&self, allowedTypes: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPathCellDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPathCellDelegate>);

        #[method(pathComponentCellClass)]
        pub unsafe fn pathComponentCellClass() -> &'static Class;

        #[method_id(@__retain_semantics Other pathComponentCells)]
        pub unsafe fn pathComponentCells(&self) -> Id<NSArray<NSPathComponentCell>, Shared>;

        #[method(setPathComponentCells:)]
        pub unsafe fn setPathComponentCells(
            &self,
            pathComponentCells: &NSArray<NSPathComponentCell>,
        );

        #[method(rectOfPathComponentCell:withFrame:inView:)]
        pub unsafe fn rectOfPathComponentCell_withFrame_inView(
            &self,
            cell: &NSPathComponentCell,
            frame: NSRect,
            view: &NSView,
        ) -> NSRect;

        #[method_id(@__retain_semantics Other pathComponentCellAtPoint:withFrame:inView:)]
        pub unsafe fn pathComponentCellAtPoint_withFrame_inView(
            &self,
            point: NSPoint,
            frame: NSRect,
            view: &NSView,
        ) -> Option<Id<NSPathComponentCell, Shared>>;

        #[method_id(@__retain_semantics Other clickedPathComponentCell)]
        pub unsafe fn clickedPathComponentCell(&self) -> Option<Id<NSPathComponentCell, Shared>>;

        #[method(mouseEntered:withFrame:inView:)]
        pub unsafe fn mouseEntered_withFrame_inView(
            &self,
            event: &NSEvent,
            frame: NSRect,
            view: &NSView,
        );

        #[method(mouseExited:withFrame:inView:)]
        pub unsafe fn mouseExited_withFrame_inView(
            &self,
            event: &NSEvent,
            frame: NSRect,
            view: &NSView,
        );

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString, Shared>>;

        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholderString: Option<&NSString>);

        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholderAttributedString: Option<&NSAttributedString>,
        );
    }
);

extern_protocol!(
    pub struct NSPathCellDelegate;

    unsafe impl ProtocolType for NSPathCellDelegate {
        #[optional]
        #[method(pathCell:willDisplayOpenPanel:)]
        pub unsafe fn pathCell_willDisplayOpenPanel(
            &self,
            pathCell: &NSPathCell,
            openPanel: &NSOpenPanel,
        );

        #[optional]
        #[method(pathCell:willPopUpMenu:)]
        pub unsafe fn pathCell_willPopUpMenu(&self, pathCell: &NSPathCell, menu: &NSMenu);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    unsafe impl NSPathCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);
