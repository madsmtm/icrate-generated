//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGridCellPlacement {
        NSGridCellPlacementInherited = 0,
        NSGridCellPlacementNone = 1,
        NSGridCellPlacementLeading = 2,
        NSGridCellPlacementTop = NSGridCellPlacementLeading,
        NSGridCellPlacementTrailing = 3,
        NSGridCellPlacementBottom = NSGridCellPlacementTrailing,
        NSGridCellPlacementCenter = 4,
        NSGridCellPlacementFill = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSGridRowAlignment {
        NSGridRowAlignmentInherited = 0,
        NSGridRowAlignmentNone = 1,
        NSGridRowAlignmentFirstBaseline = 2,
        NSGridRowAlignmentLastBaseline = 3,
    }
);

extern_static!(NSGridViewSizeForContent: CGFloat);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGridView")]
    pub struct NSGridView;

    #[cfg(feature = "AppKit_NSGridView")]
    unsafe impl ClassType for NSGridView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGridView")]
    unsafe impl NSGridView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other gridViewWithNumberOfColumns:rows:)]
        pub unsafe fn gridViewWithNumberOfColumns_rows(
            columnCount: NSInteger,
            rowCount: NSInteger,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other gridViewWithViews:)]
        pub unsafe fn gridViewWithViews(rows: &NSArray<NSArray<NSView>>) -> Id<Self, Shared>;

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSGridRow")]
        #[method_id(@__retain_semantics Other rowAtIndex:)]
        pub unsafe fn rowAtIndex(&self, index: NSInteger) -> Id<NSGridRow, Shared>;

        #[cfg(feature = "AppKit_NSGridRow")]
        #[method(indexOfRow:)]
        pub unsafe fn indexOfRow(&self, row: &NSGridRow) -> NSInteger;

        #[cfg(feature = "AppKit_NSGridColumn")]
        #[method_id(@__retain_semantics Other columnAtIndex:)]
        pub unsafe fn columnAtIndex(&self, index: NSInteger) -> Id<NSGridColumn, Shared>;

        #[cfg(feature = "AppKit_NSGridColumn")]
        #[method(indexOfColumn:)]
        pub unsafe fn indexOfColumn(&self, column: &NSGridColumn) -> NSInteger;

        #[cfg(feature = "AppKit_NSGridCell")]
        #[method_id(@__retain_semantics Other cellAtColumnIndex:rowIndex:)]
        pub unsafe fn cellAtColumnIndex_rowIndex(
            &self,
            columnIndex: NSInteger,
            rowIndex: NSInteger,
        ) -> Id<NSGridCell, Shared>;

        #[cfg(feature = "AppKit_NSGridCell")]
        #[method_id(@__retain_semantics Other cellForView:)]
        pub unsafe fn cellForView(&self, view: &NSView) -> Option<Id<NSGridCell, Shared>>;

        #[cfg(all(feature = "AppKit_NSGridRow", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other addRowWithViews:)]
        pub unsafe fn addRowWithViews(&self, views: &NSArray<NSView>) -> Id<NSGridRow, Shared>;

        #[cfg(all(feature = "AppKit_NSGridRow", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other insertRowAtIndex:withViews:)]
        pub unsafe fn insertRowAtIndex_withViews(
            &self,
            index: NSInteger,
            views: &NSArray<NSView>,
        ) -> Id<NSGridRow, Shared>;

        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, fromIndex: NSInteger, toIndex: NSInteger);

        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, index: NSInteger);

        #[cfg(all(feature = "AppKit_NSGridColumn", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other addColumnWithViews:)]
        pub unsafe fn addColumnWithViews(
            &self,
            views: &NSArray<NSView>,
        ) -> Id<NSGridColumn, Shared>;

        #[cfg(all(feature = "AppKit_NSGridColumn", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other insertColumnAtIndex:withViews:)]
        pub unsafe fn insertColumnAtIndex_withViews(
            &self,
            index: NSInteger,
            views: &NSArray<NSView>,
        ) -> Id<NSGridColumn, Shared>;

        #[method(moveColumnAtIndex:toIndex:)]
        pub unsafe fn moveColumnAtIndex_toIndex(&self, fromIndex: NSInteger, toIndex: NSInteger);

        #[method(removeColumnAtIndex:)]
        pub unsafe fn removeColumnAtIndex(&self, index: NSInteger);

        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;

        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, xPlacement: NSGridCellPlacement);

        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;

        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, yPlacement: NSGridCellPlacement);

        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;

        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, rowAlignment: NSGridRowAlignment);

        #[method(rowSpacing)]
        pub unsafe fn rowSpacing(&self) -> CGFloat;

        #[method(setRowSpacing:)]
        pub unsafe fn setRowSpacing(&self, rowSpacing: CGFloat);

        #[method(columnSpacing)]
        pub unsafe fn columnSpacing(&self) -> CGFloat;

        #[method(setColumnSpacing:)]
        pub unsafe fn setColumnSpacing(&self, columnSpacing: CGFloat);

        #[method(mergeCellsInHorizontalRange:verticalRange:)]
        pub unsafe fn mergeCellsInHorizontalRange_verticalRange(
            &self,
            hRange: NSRange,
            vRange: NSRange,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGridRow")]
    pub struct NSGridRow;

    #[cfg(feature = "AppKit_NSGridRow")]
    unsafe impl ClassType for NSGridRow {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGridRow")]
    unsafe impl NSGridRow {
        #[cfg(feature = "AppKit_NSGridView")]
        #[method_id(@__retain_semantics Other gridView)]
        pub unsafe fn gridView(&self) -> Option<Id<NSGridView, Shared>>;

        #[method(numberOfCells)]
        pub unsafe fn numberOfCells(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSGridCell")]
        #[method_id(@__retain_semantics Other cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Id<NSGridCell, Shared>;

        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;

        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, yPlacement: NSGridCellPlacement);

        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;

        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, rowAlignment: NSGridRowAlignment);

        #[method(height)]
        pub unsafe fn height(&self) -> CGFloat;

        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: CGFloat);

        #[method(topPadding)]
        pub unsafe fn topPadding(&self) -> CGFloat;

        #[method(setTopPadding:)]
        pub unsafe fn setTopPadding(&self, topPadding: CGFloat);

        #[method(bottomPadding)]
        pub unsafe fn bottomPadding(&self) -> CGFloat;

        #[method(setBottomPadding:)]
        pub unsafe fn setBottomPadding(&self, bottomPadding: CGFloat);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(mergeCellsInRange:)]
        pub unsafe fn mergeCellsInRange(&self, range: NSRange);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGridColumn")]
    pub struct NSGridColumn;

    #[cfg(feature = "AppKit_NSGridColumn")]
    unsafe impl ClassType for NSGridColumn {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGridColumn")]
    unsafe impl NSGridColumn {
        #[cfg(feature = "AppKit_NSGridView")]
        #[method_id(@__retain_semantics Other gridView)]
        pub unsafe fn gridView(&self) -> Option<Id<NSGridView, Shared>>;

        #[method(numberOfCells)]
        pub unsafe fn numberOfCells(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSGridCell")]
        #[method_id(@__retain_semantics Other cellAtIndex:)]
        pub unsafe fn cellAtIndex(&self, index: NSInteger) -> Id<NSGridCell, Shared>;

        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;

        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, xPlacement: NSGridCellPlacement);

        #[method(width)]
        pub unsafe fn width(&self) -> CGFloat;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: CGFloat);

        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[method(setLeadingPadding:)]
        pub unsafe fn setLeadingPadding(&self, leadingPadding: CGFloat);

        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[method(setTrailingPadding:)]
        pub unsafe fn setTrailingPadding(&self, trailingPadding: CGFloat);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(mergeCellsInRange:)]
        pub unsafe fn mergeCellsInRange(&self, range: NSRange);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGridCell")]
    pub struct NSGridCell;

    #[cfg(feature = "AppKit_NSGridCell")]
    unsafe impl ClassType for NSGridCell {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSGridCell")]
    unsafe impl NSGridCell {
        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other emptyContentView)]
        pub unsafe fn emptyContentView() -> Id<NSView, Shared>;

        #[cfg(feature = "AppKit_NSGridRow")]
        #[method_id(@__retain_semantics Other row)]
        pub unsafe fn row(&self) -> Option<Id<NSGridRow, Shared>>;

        #[cfg(feature = "AppKit_NSGridColumn")]
        #[method_id(@__retain_semantics Other column)]
        pub unsafe fn column(&self) -> Option<Id<NSGridColumn, Shared>>;

        #[method(xPlacement)]
        pub unsafe fn xPlacement(&self) -> NSGridCellPlacement;

        #[method(setXPlacement:)]
        pub unsafe fn setXPlacement(&self, xPlacement: NSGridCellPlacement);

        #[method(yPlacement)]
        pub unsafe fn yPlacement(&self) -> NSGridCellPlacement;

        #[method(setYPlacement:)]
        pub unsafe fn setYPlacement(&self, yPlacement: NSGridCellPlacement);

        #[method(rowAlignment)]
        pub unsafe fn rowAlignment(&self) -> NSGridRowAlignment;

        #[method(setRowAlignment:)]
        pub unsafe fn setRowAlignment(&self, rowAlignment: NSGridRowAlignment);

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other customPlacementConstraints)]
        pub unsafe fn customPlacementConstraints(&self) -> Id<NSArray<NSLayoutConstraint>, Shared>;

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(setCustomPlacementConstraints:)]
        pub unsafe fn setCustomPlacementConstraints(
            &self,
            customPlacementConstraints: &NSArray<NSLayoutConstraint>,
        );
    }
);
