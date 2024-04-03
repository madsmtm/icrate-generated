//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub const NSOutlineViewDropOnItemIndex: c_int = -1;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSTableView",
        feature = "AppKit_NSView"
    ))]
    pub struct NSOutlineView;

    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSTableView",
        feature = "AppKit_NSView"
    ))]
    unsafe impl ClassType for NSOutlineView {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTableView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityGroup for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityOutline for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityTable for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSCoding for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingSource for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSView"
))]
unsafe impl NSObjectProtocol for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSText",
    feature = "AppKit_NSView"
))]
unsafe impl NSTextDelegate for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSText",
    feature = "AppKit_NSTextView",
    feature = "AppKit_NSView"
))]
unsafe impl NSTextViewDelegate for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSOutlineView {}

#[cfg(all(
    feature = "AppKit_NSControl",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSTableView",
    feature = "AppKit_NSUserInterfaceValidation",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceValidations for NSOutlineView {}

extern_methods!(
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSTableView",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSOutlineViewDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSOutlineViewDelegate>>,
        );

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<ProtocolObject<dyn NSOutlineViewDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSOutlineViewDataSource>>,
        );

        #[cfg(feature = "AppKit_NSTableColumn")]
        #[method_id(@__retain_semantics Other outlineTableColumn)]
        pub unsafe fn outlineTableColumn(&self) -> Option<Id<NSTableColumn>>;

        #[cfg(feature = "AppKit_NSTableColumn")]
        #[method(setOutlineTableColumn:)]
        pub unsafe fn setOutlineTableColumn(&self, outline_table_column: Option<&NSTableColumn>);

        #[method(isExpandable:)]
        pub unsafe fn isExpandable(&self, item: Option<&AnyObject>) -> bool;

        #[method(numberOfChildrenOfItem:)]
        pub unsafe fn numberOfChildrenOfItem(&self, item: Option<&AnyObject>) -> NSInteger;

        #[method_id(@__retain_semantics Other child:ofItem:)]
        pub unsafe fn child_ofItem(
            &self,
            index: NSInteger,
            item: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[method(expandItem:expandChildren:)]
        pub unsafe fn expandItem_expandChildren(
            &self,
            item: Option<&AnyObject>,
            expand_children: bool,
        );

        #[method(expandItem:)]
        pub unsafe fn expandItem(&self, item: Option<&AnyObject>);

        #[method(collapseItem:collapseChildren:)]
        pub unsafe fn collapseItem_collapseChildren(
            &self,
            item: Option<&AnyObject>,
            collapse_children: bool,
        );

        #[method(collapseItem:)]
        pub unsafe fn collapseItem(&self, item: Option<&AnyObject>);

        #[method(reloadItem:reloadChildren:)]
        pub unsafe fn reloadItem_reloadChildren(
            &self,
            item: Option<&AnyObject>,
            reload_children: bool,
        );

        #[method(reloadItem:)]
        pub unsafe fn reloadItem(&self, item: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other parentForItem:)]
        pub unsafe fn parentForItem(&self, item: Option<&AnyObject>) -> Option<Id<AnyObject>>;

        #[method(childIndexForItem:)]
        pub unsafe fn childIndexForItem(&self, item: &AnyObject) -> NSInteger;

        #[method_id(@__retain_semantics Other itemAtRow:)]
        pub unsafe fn itemAtRow(&self, row: NSInteger) -> Option<Id<AnyObject>>;

        #[method(rowForItem:)]
        pub unsafe fn rowForItem(&self, item: Option<&AnyObject>) -> NSInteger;

        #[method(levelForItem:)]
        pub unsafe fn levelForItem(&self, item: Option<&AnyObject>) -> NSInteger;

        #[method(levelForRow:)]
        pub unsafe fn levelForRow(&self, row: NSInteger) -> NSInteger;

        #[method(isItemExpanded:)]
        pub unsafe fn isItemExpanded(&self, item: Option<&AnyObject>) -> bool;

        #[method(indentationPerLevel)]
        pub unsafe fn indentationPerLevel(&self) -> CGFloat;

        #[method(setIndentationPerLevel:)]
        pub unsafe fn setIndentationPerLevel(&self, indentation_per_level: CGFloat);

        #[method(indentationMarkerFollowsCell)]
        pub unsafe fn indentationMarkerFollowsCell(&self) -> bool;

        #[method(setIndentationMarkerFollowsCell:)]
        pub unsafe fn setIndentationMarkerFollowsCell(&self, indentation_marker_follows_cell: bool);

        #[method(autoresizesOutlineColumn)]
        pub unsafe fn autoresizesOutlineColumn(&self) -> bool;

        #[method(setAutoresizesOutlineColumn:)]
        pub unsafe fn setAutoresizesOutlineColumn(&self, autoresizes_outline_column: bool);

        #[method(frameOfOutlineCellAtRow:)]
        pub unsafe fn frameOfOutlineCellAtRow(&self, row: NSInteger) -> NSRect;

        #[method(setDropItem:dropChildIndex:)]
        pub unsafe fn setDropItem_dropChildIndex(&self, item: Option<&AnyObject>, index: NSInteger);

        #[method(shouldCollapseAutoExpandedItemsForDeposited:)]
        pub unsafe fn shouldCollapseAutoExpandedItemsForDeposited(&self, deposited: bool) -> bool;

        #[method(autosaveExpandedItems)]
        pub unsafe fn autosaveExpandedItems(&self) -> bool;

        #[method(setAutosaveExpandedItems:)]
        pub unsafe fn setAutosaveExpandedItems(&self, autosave_expanded_items: bool);

        #[method(insertItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn insertItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &NSIndexSet,
            parent: Option<&AnyObject>,
            animation_options: NSTableViewAnimationOptions,
        );

        #[method(removeItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn removeItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &NSIndexSet,
            parent: Option<&AnyObject>,
            animation_options: NSTableViewAnimationOptions,
        );

        #[method(moveItemAtIndex:inParent:toIndex:inParent:)]
        pub unsafe fn moveItemAtIndex_inParent_toIndex_inParent(
            &self,
            from_index: NSInteger,
            old_parent: Option<&AnyObject>,
            to_index: NSInteger,
            new_parent: Option<&AnyObject>,
        );

        #[method(insertRowsAtIndexes:withAnimation:)]
        pub unsafe fn insertRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            animation_options: NSTableViewAnimationOptions,
        );

        #[method(removeRowsAtIndexes:withAnimation:)]
        pub unsafe fn removeRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            animation_options: NSTableViewAnimationOptions,
        );

        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, old_index: NSInteger, new_index: NSInteger);

        #[cfg(feature = "AppKit_NSUserInterfaceLayout")]
        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[cfg(feature = "AppKit_NSUserInterfaceLayout")]
        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            user_interface_layout_direction: NSUserInterfaceLayoutDirection,
        );

        #[method(stronglyReferencesItems)]
        pub unsafe fn stronglyReferencesItems(&self) -> bool;

        #[method(setStronglyReferencesItems:)]
        pub unsafe fn setStronglyReferencesItems(&self, strongly_references_items: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTableView`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSTableView",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSTableView",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "AppKit_NSControl",
        feature = "AppKit_NSResponder",
        feature = "AppKit_NSTableView",
        feature = "AppKit_NSView"
    ))]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSOutlineViewDataSource: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:numberOfChildrenOfItem:)]
        unsafe fn outlineView_numberOfChildrenOfItem(
            &self,
            outline_view: &NSOutlineView,
            item: Option<&AnyObject>,
        ) -> NSInteger;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:child:ofItem:)]
        unsafe fn outlineView_child_ofItem(
            &self,
            outline_view: &NSOutlineView,
            index: NSInteger,
            item: Option<&AnyObject>,
        ) -> Id<AnyObject>;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:isItemExpandable:)]
        unsafe fn outlineView_isItemExpandable(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:objectValueForTableColumn:byItem:)]
        unsafe fn outlineView_objectValueForTableColumn_byItem(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:setObjectValue:forTableColumn:byItem:)]
        unsafe fn outlineView_setObjectValue_forTableColumn_byItem(
            &self,
            outline_view: &NSOutlineView,
            object: Option<&AnyObject>,
            table_column: Option<&NSTableColumn>,
            item: Option<&AnyObject>,
        );

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:itemForPersistentObject:)]
        unsafe fn outlineView_itemForPersistentObject(
            &self,
            outline_view: &NSOutlineView,
            object: &AnyObject,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:persistentObjectForItem:)]
        unsafe fn outlineView_persistentObjectForItem(
            &self,
            outline_view: &NSOutlineView,
            item: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:sortDescriptorsDidChange:)]
        unsafe fn outlineView_sortDescriptorsDidChange(
            &self,
            outline_view: &NSOutlineView,
            old_descriptors: &NSArray<NSSortDescriptor>,
        );

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:pasteboardWriterForItem:)]
        unsafe fn outlineView_pasteboardWriterForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> Option<Id<ProtocolObject<dyn NSPasteboardWriting>>>;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSDraggingSession",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:draggingSession:willBeginAtPoint:forItems:)]
        unsafe fn outlineView_draggingSession_willBeginAtPoint_forItems(
            &self,
            outline_view: &NSOutlineView,
            session: &NSDraggingSession,
            screen_point: NSPoint,
            dragged_items: &NSArray,
        );

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSDraggingSession",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:draggingSession:endedAtPoint:operation:)]
        unsafe fn outlineView_draggingSession_endedAtPoint_operation(
            &self,
            outline_view: &NSOutlineView,
            session: &NSDraggingSession,
            screen_point: NSPoint,
            operation: NSDragOperation,
        );

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[deprecated = "Use -outlineView:pasteboardWriterForItem: instead"]
        #[optional]
        #[method(outlineView:writeItems:toPasteboard:)]
        unsafe fn outlineView_writeItems_toPasteboard(
            &self,
            outline_view: &NSOutlineView,
            items: &NSArray,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:updateDraggingItemsForDrag:)]
        unsafe fn outlineView_updateDraggingItemsForDrag(
            &self,
            outline_view: &NSOutlineView,
            dragging_info: &ProtocolObject<dyn NSDraggingInfo>,
        );

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:validateDrop:proposedItem:proposedChildIndex:)]
        unsafe fn outlineView_validateDrop_proposedItem_proposedChildIndex(
            &self,
            outline_view: &NSOutlineView,
            info: &ProtocolObject<dyn NSDraggingInfo>,
            item: Option<&AnyObject>,
            index: NSInteger,
        ) -> NSDragOperation;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSDragging",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:acceptDrop:item:childIndex:)]
        unsafe fn outlineView_acceptDrop_item_childIndex(
            &self,
            outline_view: &NSOutlineView,
            info: &ProtocolObject<dyn NSDraggingInfo>,
            item: Option<&AnyObject>,
            index: NSInteger,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[deprecated = "Use NSFilePromiseReceiver objects instead"]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:namesOfPromisedFilesDroppedAtDestination:forDraggedItems:)]
        unsafe fn outlineView_namesOfPromisedFilesDroppedAtDestination_forDraggedItems(
            &self,
            outline_view: &NSOutlineView,
            drop_destination: &NSURL,
            items: &NSArray,
        ) -> Id<NSArray<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSOutlineViewDataSource {}
);

extern_protocol!(
    #[cfg(feature = "AppKit_NSControl")]
    pub unsafe trait NSOutlineViewDelegate: NSControlTextEditingDelegate {
        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:viewForTableColumn:item:)]
        unsafe fn outlineView_viewForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> Option<Id<NSView>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableRowView",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:rowViewForItem:)]
        unsafe fn outlineView_rowViewForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> Option<Id<NSTableRowView>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableRowView",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:didAddRowView:forRow:)]
        unsafe fn outlineView_didAddRowView_forRow(
            &self,
            outline_view: &NSOutlineView,
            row_view: &NSTableRowView,
            row: NSInteger,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableRowView",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:didRemoveRowView:forRow:)]
        unsafe fn outlineView_didRemoveRowView_forRow(
            &self,
            outline_view: &NSOutlineView,
            row_view: &NSTableRowView,
            row: NSInteger,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:willDisplayCell:forTableColumn:item:)]
        unsafe fn outlineView_willDisplayCell_forTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            cell: &AnyObject,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldEditTableColumn:item:)]
        unsafe fn outlineView_shouldEditTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(selectionShouldChangeInOutlineView:)]
        unsafe fn selectionShouldChangeInOutlineView(&self, outline_view: &NSOutlineView) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldSelectItem:)]
        unsafe fn outlineView_shouldSelectItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:selectionIndexesForProposedSelection:)]
        unsafe fn outlineView_selectionIndexesForProposedSelection(
            &self,
            outline_view: &NSOutlineView,
            proposed_selection_indexes: &NSIndexSet,
        ) -> Id<NSIndexSet>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldSelectTableColumn:)]
        unsafe fn outlineView_shouldSelectTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:mouseDownInHeaderOfTableColumn:)]
        unsafe fn outlineView_mouseDownInHeaderOfTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: &NSTableColumn,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:didClickTableColumn:)]
        unsafe fn outlineView_didClickTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: &NSTableColumn,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:didDragTableColumn:)]
        unsafe fn outlineView_didDragTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: &NSTableColumn,
        );

        #[cfg(all(
            feature = "AppKit_NSCell",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:toolTipForCell:rect:tableColumn:item:mouseLocation:)]
        unsafe fn outlineView_toolTipForCell_rect_tableColumn_item_mouseLocation(
            &self,
            outline_view: &NSOutlineView,
            cell: &NSCell,
            rect: NSRectPointer,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
            mouse_location: NSPoint,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:heightOfRowByItem:)]
        unsafe fn outlineView_heightOfRowByItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSTintConfiguration",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:tintConfigurationForItem:)]
        unsafe fn outlineView_tintConfigurationForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> Option<Id<NSTintConfiguration>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:typeSelectStringForTableColumn:item:)]
        unsafe fn outlineView_typeSelectStringForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:nextTypeSelectMatchFromItem:toItem:forString:)]
        unsafe fn outlineView_nextTypeSelectMatchFromItem_toItem_forString(
            &self,
            outline_view: &NSOutlineView,
            start_item: &AnyObject,
            end_item: &AnyObject,
            search_string: &NSString,
        ) -> Option<Id<AnyObject>>;

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldTypeSelectForEvent:withCurrentSearchString:)]
        unsafe fn outlineView_shouldTypeSelectForEvent_withCurrentSearchString(
            &self,
            outline_view: &NSOutlineView,
            event: &NSEvent,
            search_string: Option<&NSString>,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldShowCellExpansionForTableColumn:item:)]
        unsafe fn outlineView_shouldShowCellExpansionForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSCell",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldTrackCell:forTableColumn:item:)]
        unsafe fn outlineView_shouldTrackCell_forTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            cell: &NSCell,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSCell",
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:dataCellForTableColumn:item:)]
        unsafe fn outlineView_dataCellForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        ) -> Option<Id<NSCell>>;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:isGroupItem:)]
        unsafe fn outlineView_isGroupItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldExpandItem:)]
        unsafe fn outlineView_shouldExpandItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldCollapseItem:)]
        unsafe fn outlineView_shouldCollapseItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:willDisplayOutlineCell:forTableColumn:item:)]
        unsafe fn outlineView_willDisplayOutlineCell_forTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            cell: &AnyObject,
            table_column: Option<&NSTableColumn>,
            item: &AnyObject,
        );

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:sizeToFitWidthOfColumn:)]
        unsafe fn outlineView_sizeToFitWidthOfColumn(
            &self,
            outline_view: &NSOutlineView,
            column: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldReorderColumn:toColumn:)]
        unsafe fn outlineView_shouldReorderColumn_toColumn(
            &self,
            outline_view: &NSOutlineView,
            column_index: NSInteger,
            new_column_index: NSInteger,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:shouldShowOutlineCellForItem:)]
        unsafe fn outlineView_shouldShowOutlineCellForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &AnyObject,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:userCanChangeVisibilityOfTableColumn:)]
        unsafe fn outlineView_userCanChangeVisibilityOfTableColumn(
            &self,
            outline_view: &NSOutlineView,
            column: &NSTableColumn,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSResponder",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSTableView",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method(outlineView:userDidChangeVisibilityOfTableColumns:)]
        unsafe fn outlineView_userDidChangeVisibilityOfTableColumns(
            &self,
            outline_view: &NSOutlineView,
            columns: &NSArray<NSTableColumn>,
        );

        #[optional]
        #[method(outlineViewSelectionDidChange:)]
        unsafe fn outlineViewSelectionDidChange(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewColumnDidMove:)]
        unsafe fn outlineViewColumnDidMove(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewColumnDidResize:)]
        unsafe fn outlineViewColumnDidResize(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewSelectionIsChanging:)]
        unsafe fn outlineViewSelectionIsChanging(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewItemWillExpand:)]
        unsafe fn outlineViewItemWillExpand(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewItemDidExpand:)]
        unsafe fn outlineViewItemDidExpand(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewItemWillCollapse:)]
        unsafe fn outlineViewItemWillCollapse(&self, notification: &NSNotification);

        #[optional]
        #[method(outlineViewItemDidCollapse:)]
        unsafe fn outlineViewItemDidCollapse(&self, notification: &NSNotification);
    }

    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl ProtocolType for dyn NSOutlineViewDelegate {}
);

extern "C" {
    #[cfg(feature = "AppKit_NSUserInterfaceItemIdentification")]
    pub static NSOutlineViewDisclosureButtonKey: &'static NSUserInterfaceItemIdentifier;
}

extern "C" {
    #[cfg(feature = "AppKit_NSUserInterfaceItemIdentification")]
    pub static NSOutlineViewShowHideButtonKey: &'static NSUserInterfaceItemIdentifier;
}

extern "C" {
    pub static NSOutlineViewSelectionDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewColumnDidMoveNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewColumnDidResizeNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewSelectionIsChangingNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewItemWillExpandNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewItemDidExpandNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewItemWillCollapseNotification: &'static NSNotificationName;
}

extern "C" {
    pub static NSOutlineViewItemDidCollapseNotification: &'static NSNotificationName;
}
