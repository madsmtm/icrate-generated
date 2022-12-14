//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSMatrixMode {
        NSRadioModeMatrix = 0,
        NSHighlightModeMatrix = 1,
        NSListModeMatrix = 2,
        NSTrackModeMatrix = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMatrix;

    unsafe impl ClassType for NSMatrix {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

extern_methods!(
    unsafe impl NSMatrix {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

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

        #[method(cellClass)]
        pub unsafe fn cellClass(&self) -> &'static Class;

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(&self, cellClass: &Class);

        #[method_id(@__retain_semantics Other prototype)]
        pub unsafe fn prototype(&self) -> Option<Id<NSCell, Shared>>;

        #[method(setPrototype:)]
        pub unsafe fn setPrototype(&self, prototype: Option<&NSCell>);

        #[method_id(@__retain_semantics Other makeCellAtRow:column:)]
        pub unsafe fn makeCellAtRow_column(
            &self,
            row: NSInteger,
            col: NSInteger,
        ) -> Id<NSCell, Shared>;

        #[method(mode)]
        pub unsafe fn mode(&self) -> NSMatrixMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSMatrixMode);

        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;

        #[method(setAllowsEmptySelection:)]
        pub unsafe fn setAllowsEmptySelection(&self, allowsEmptySelection: bool);

        #[method(sendAction:to:forAllCells:)]
        pub unsafe fn sendAction_to_forAllCells(&self, selector: Sel, object: &Object, flag: bool);

        #[method_id(@__retain_semantics Other cells)]
        pub unsafe fn cells(&self) -> Id<NSArray<NSCell>, Shared>;

        #[method(sortUsingSelector:)]
        pub unsafe fn sortUsingSelector(&self, comparator: Sel);

        #[method(sortUsingFunction:context:)]
        pub unsafe fn sortUsingFunction_context(
            &self,
            compare: unsafe extern "C" fn(
                NonNull<Object>,
                NonNull<Object>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        );

        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Id<NSCell, Shared>>;

        #[method_id(@__retain_semantics Other selectedCells)]
        pub unsafe fn selectedCells(&self) -> Id<NSArray<NSCell>, Shared>;

        #[method(selectedRow)]
        pub unsafe fn selectedRow(&self) -> NSInteger;

        #[method(selectedColumn)]
        pub unsafe fn selectedColumn(&self) -> NSInteger;

        #[method(isSelectionByRect)]
        pub unsafe fn isSelectionByRect(&self) -> bool;

        #[method(setSelectionByRect:)]
        pub unsafe fn setSelectionByRect(&self, selectionByRect: bool);

        #[method(setSelectionFrom:to:anchor:highlight:)]
        pub unsafe fn setSelectionFrom_to_anchor_highlight(
            &self,
            startPos: NSInteger,
            endPos: NSInteger,
            anchorPos: NSInteger,
            lit: bool,
        );

        #[method(deselectSelectedCell)]
        pub unsafe fn deselectSelectedCell(&self);

        #[method(deselectAllCells)]
        pub unsafe fn deselectAllCells(&self);

        #[method(selectCellAtRow:column:)]
        pub unsafe fn selectCellAtRow_column(&self, row: NSInteger, col: NSInteger);

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);

        #[method(selectCellWithTag:)]
        pub unsafe fn selectCellWithTag(&self, tag: NSInteger) -> bool;

        #[method(cellSize)]
        pub unsafe fn cellSize(&self) -> NSSize;

        #[method(setCellSize:)]
        pub unsafe fn setCellSize(&self, cellSize: NSSize);

        #[method(intercellSpacing)]
        pub unsafe fn intercellSpacing(&self) -> NSSize;

        #[method(setIntercellSpacing:)]
        pub unsafe fn setIntercellSpacing(&self, intercellSpacing: NSSize);

        #[method(setScrollable:)]
        pub unsafe fn setScrollable(&self, flag: bool);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);

        #[method_id(@__retain_semantics Other cellBackgroundColor)]
        pub unsafe fn cellBackgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setCellBackgroundColor:)]
        pub unsafe fn setCellBackgroundColor(&self, cellBackgroundColor: Option<&NSColor>);

        #[method(drawsCellBackground)]
        pub unsafe fn drawsCellBackground(&self) -> bool;

        #[method(setDrawsCellBackground:)]
        pub unsafe fn setDrawsCellBackground(&self, drawsCellBackground: bool);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method(setState:atRow:column:)]
        pub unsafe fn setState_atRow_column(
            &self,
            value: NSInteger,
            row: NSInteger,
            col: NSInteger,
        );

        #[method(getNumberOfRows:columns:)]
        pub unsafe fn getNumberOfRows_columns(
            &self,
            rowCount: *mut NSInteger,
            colCount: *mut NSInteger,
        );

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other cellAtRow:column:)]
        pub unsafe fn cellAtRow_column(
            &self,
            row: NSInteger,
            col: NSInteger,
        ) -> Option<Id<NSCell, Shared>>;

        #[method(cellFrameAtRow:column:)]
        pub unsafe fn cellFrameAtRow_column(&self, row: NSInteger, col: NSInteger) -> NSRect;

        #[method(getRow:column:ofCell:)]
        pub unsafe fn getRow_column_ofCell(
            &self,
            row: NonNull<NSInteger>,
            col: NonNull<NSInteger>,
            cell: &NSCell,
        ) -> bool;

        #[method(getRow:column:forPoint:)]
        pub unsafe fn getRow_column_forPoint(
            &self,
            row: NonNull<NSInteger>,
            col: NonNull<NSInteger>,
            point: NSPoint,
        ) -> bool;

        #[method(renewRows:columns:)]
        pub unsafe fn renewRows_columns(&self, newRows: NSInteger, newCols: NSInteger);

        #[method(putCell:atRow:column:)]
        pub unsafe fn putCell_atRow_column(&self, newCell: &NSCell, row: NSInteger, col: NSInteger);

        #[method(addRow)]
        pub unsafe fn addRow(&self);

        #[method(addRowWithCells:)]
        pub unsafe fn addRowWithCells(&self, newCells: &NSArray<NSCell>);

        #[method(insertRow:)]
        pub unsafe fn insertRow(&self, row: NSInteger);

        #[method(insertRow:withCells:)]
        pub unsafe fn insertRow_withCells(
            &self,
            row: NSInteger,
            newCells: Option<&NSArray<NSCell>>,
        );

        #[method(removeRow:)]
        pub unsafe fn removeRow(&self, row: NSInteger);

        #[method(addColumn)]
        pub unsafe fn addColumn(&self);

        #[method(addColumnWithCells:)]
        pub unsafe fn addColumnWithCells(&self, newCells: &NSArray<NSCell>);

        #[method(insertColumn:)]
        pub unsafe fn insertColumn(&self, column: NSInteger);

        #[method(insertColumn:withCells:)]
        pub unsafe fn insertColumn_withCells(
            &self,
            column: NSInteger,
            newCells: Option<&NSArray<NSCell>>,
        );

        #[method(removeColumn:)]
        pub unsafe fn removeColumn(&self, col: NSInteger);

        #[method_id(@__retain_semantics Other cellWithTag:)]
        pub unsafe fn cellWithTag(&self, tag: NSInteger) -> Option<Id<NSCell, Shared>>;

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, doubleAction: Option<Sel>);

        #[method(autosizesCells)]
        pub unsafe fn autosizesCells(&self) -> bool;

        #[method(setAutosizesCells:)]
        pub unsafe fn setAutosizesCells(&self, autosizesCells: bool);

        #[method(sizeToCells)]
        pub unsafe fn sizeToCells(&self);

        #[method(setValidateSize:)]
        pub unsafe fn setValidateSize(&self, flag: bool);

        #[method(drawCellAtRow:column:)]
        pub unsafe fn drawCellAtRow_column(&self, row: NSInteger, col: NSInteger);

        #[method(highlightCell:atRow:column:)]
        pub unsafe fn highlightCell_atRow_column(&self, flag: bool, row: NSInteger, col: NSInteger);

        #[method(isAutoscroll)]
        pub unsafe fn isAutoscroll(&self) -> bool;

        #[method(setAutoscroll:)]
        pub unsafe fn setAutoscroll(&self, autoscroll: bool);

        #[method(scrollCellToVisibleAtRow:column:)]
        pub unsafe fn scrollCellToVisibleAtRow_column(&self, row: NSInteger, col: NSInteger);

        #[method(mouseDownFlags)]
        pub unsafe fn mouseDownFlags(&self) -> NSInteger;

        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;

        #[method(sendDoubleAction)]
        pub unsafe fn sendDoubleAction(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSMatrixDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSMatrixDelegate>);

        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, textObject: &NSText) -> bool;

        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, textObject: &NSText) -> bool;

        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);

        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);

        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);

        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&Object>);

        #[method_id(@__retain_semantics Other selectTextAtRow:column:)]
        pub unsafe fn selectTextAtRow_column(
            &self,
            row: NSInteger,
            col: NSInteger,
        ) -> Option<Id<NSCell, Shared>>;

        #[method(acceptsFirstMouse:)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&NSEvent>) -> bool;

        #[method(resetCursorRects)]
        pub unsafe fn resetCursorRects(&self);

        #[method(setToolTip:forCell:)]
        pub unsafe fn setToolTip_forCell(&self, toolTipString: Option<&NSString>, cell: &NSCell);

        #[method_id(@__retain_semantics Other toolTipForCell:)]
        pub unsafe fn toolTipForCell(&self, cell: &NSCell) -> Option<Id<NSString, Shared>>;

        #[method(autorecalculatesCellSize)]
        pub unsafe fn autorecalculatesCellSize(&self) -> bool;

        #[method(setAutorecalculatesCellSize:)]
        pub unsafe fn setAutorecalculatesCellSize(&self, autorecalculatesCellSize: bool);
    }
);

extern_methods!(
    /// NSKeyboardUI
    unsafe impl NSMatrix {
        #[method(tabKeyTraversesCells)]
        pub unsafe fn tabKeyTraversesCells(&self) -> bool;

        #[method(setTabKeyTraversesCells:)]
        pub unsafe fn setTabKeyTraversesCells(&self, tabKeyTraversesCells: bool);

        #[method_id(@__retain_semantics Other keyCell)]
        pub unsafe fn keyCell(&self) -> Option<Id<NSCell, Shared>>;

        #[method(setKeyCell:)]
        pub unsafe fn setKeyCell(&self, keyCell: Option<&NSCell>);
    }
);

extern_protocol!(
    pub struct NSMatrixDelegate;

    unsafe impl ProtocolType for NSMatrixDelegate {}
);
