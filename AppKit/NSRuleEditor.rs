//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_enum!(
    pub type NSRuleEditorPredicatePartKey = NSString;
);

extern_static!(NSRuleEditorPredicateLeftExpression: &'static NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateRightExpression: &'static NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateComparisonModifier: &'static NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateOptions: &'static NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateOperatorType: &'static NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateCustomSelector: &'static NSRuleEditorPredicatePartKey);

extern_static!(NSRuleEditorPredicateCompoundType: &'static NSRuleEditorPredicatePartKey);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRuleEditorNestingMode {
        NSRuleEditorNestingModeSingle = 0,
        NSRuleEditorNestingModeList = 1,
        NSRuleEditorNestingModeCompound = 2,
        NSRuleEditorNestingModeSimple = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSRuleEditorRowType {
        NSRuleEditorRowTypeSimple = 0,
        NSRuleEditorRowTypeCompound = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSRuleEditor;

    unsafe impl ClassType for NSRuleEditor {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

extern_methods!(
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSRuleEditorDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSRuleEditorDelegate>);

        #[method_id(@__retain_semantics Other formattingStringsFilename)]
        pub unsafe fn formattingStringsFilename(&self) -> Option<Id<NSString, Shared>>;

        #[method(setFormattingStringsFilename:)]
        pub unsafe fn setFormattingStringsFilename(
            &self,
            formattingStringsFilename: Option<&NSString>,
        );

        #[method_id(@__retain_semantics Other formattingDictionary)]
        pub unsafe fn formattingDictionary(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;

        #[method(setFormattingDictionary:)]
        pub unsafe fn setFormattingDictionary(
            &self,
            formattingDictionary: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method(reloadCriteria)]
        pub unsafe fn reloadCriteria(&self);

        #[method(nestingMode)]
        pub unsafe fn nestingMode(&self) -> NSRuleEditorNestingMode;

        #[method(setNestingMode:)]
        pub unsafe fn setNestingMode(&self, nestingMode: NSRuleEditorNestingMode);

        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> CGFloat;

        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, rowHeight: CGFloat);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(canRemoveAllRows)]
        pub unsafe fn canRemoveAllRows(&self) -> bool;

        #[method(setCanRemoveAllRows:)]
        pub unsafe fn setCanRemoveAllRows(&self, canRemoveAllRows: bool);

        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate, Shared>>;

        #[method(reloadPredicate)]
        pub unsafe fn reloadPredicate(&self);

        #[method_id(@__retain_semantics Other predicateForRow:)]
        pub unsafe fn predicateForRow(&self, row: NSInteger) -> Option<Id<NSPredicate, Shared>>;

        #[method(numberOfRows)]
        pub unsafe fn numberOfRows(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other subrowIndexesForRow:)]
        pub unsafe fn subrowIndexesForRow(&self, rowIndex: NSInteger) -> Id<NSIndexSet, Shared>;

        #[method_id(@__retain_semantics Other criteriaForRow:)]
        pub unsafe fn criteriaForRow(&self, row: NSInteger) -> Id<NSArray, Shared>;

        #[method_id(@__retain_semantics Other displayValuesForRow:)]
        pub unsafe fn displayValuesForRow(&self, row: NSInteger) -> Id<NSArray, Shared>;

        #[method(rowForDisplayValue:)]
        pub unsafe fn rowForDisplayValue(&self, displayValue: &Object) -> NSInteger;

        #[method(rowTypeForRow:)]
        pub unsafe fn rowTypeForRow(&self, rowIndex: NSInteger) -> NSRuleEditorRowType;

        #[method(parentRowForRow:)]
        pub unsafe fn parentRowForRow(&self, rowIndex: NSInteger) -> NSInteger;

        #[method(addRow:)]
        pub unsafe fn addRow(&self, sender: Option<&Object>);

        #[method(insertRowAtIndex:withType:asSubrowOfRow:animate:)]
        pub unsafe fn insertRowAtIndex_withType_asSubrowOfRow_animate(
            &self,
            rowIndex: NSInteger,
            rowType: NSRuleEditorRowType,
            parentRow: NSInteger,
            shouldAnimate: bool,
        );

        #[method(setCriteria:andDisplayValues:forRowAtIndex:)]
        pub unsafe fn setCriteria_andDisplayValues_forRowAtIndex(
            &self,
            criteria: &NSArray,
            values: &NSArray,
            rowIndex: NSInteger,
        );

        #[method(removeRowAtIndex:)]
        pub unsafe fn removeRowAtIndex(&self, rowIndex: NSInteger);

        #[method(removeRowsAtIndexes:includeSubrows:)]
        pub unsafe fn removeRowsAtIndexes_includeSubrows(
            &self,
            rowIndexes: &NSIndexSet,
            includeSubrows: bool,
        );

        #[method_id(@__retain_semantics Other selectedRowIndexes)]
        pub unsafe fn selectedRowIndexes(&self) -> Id<NSIndexSet, Shared>;

        #[method(selectRowIndexes:byExtendingSelection:)]
        pub unsafe fn selectRowIndexes_byExtendingSelection(
            &self,
            indexes: &NSIndexSet,
            extend: bool,
        );

        #[method(rowClass)]
        pub unsafe fn rowClass(&self) -> &'static Class;

        #[method(setRowClass:)]
        pub unsafe fn setRowClass(&self, rowClass: &Class);

        #[method_id(@__retain_semantics Other rowTypeKeyPath)]
        pub unsafe fn rowTypeKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setRowTypeKeyPath:)]
        pub unsafe fn setRowTypeKeyPath(&self, rowTypeKeyPath: &NSString);

        #[method_id(@__retain_semantics Other subrowsKeyPath)]
        pub unsafe fn subrowsKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setSubrowsKeyPath:)]
        pub unsafe fn setSubrowsKeyPath(&self, subrowsKeyPath: &NSString);

        #[method_id(@__retain_semantics Other criteriaKeyPath)]
        pub unsafe fn criteriaKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setCriteriaKeyPath:)]
        pub unsafe fn setCriteriaKeyPath(&self, criteriaKeyPath: &NSString);

        #[method_id(@__retain_semantics Other displayValuesKeyPath)]
        pub unsafe fn displayValuesKeyPath(&self) -> Id<NSString, Shared>;

        #[method(setDisplayValuesKeyPath:)]
        pub unsafe fn setDisplayValuesKeyPath(&self, displayValuesKeyPath: &NSString);
    }
);

extern_protocol!(
    pub struct NSRuleEditorDelegate;

    unsafe impl ProtocolType for NSRuleEditorDelegate {
        #[method(ruleEditor:numberOfChildrenForCriterion:withRowType:)]
        pub unsafe fn ruleEditor_numberOfChildrenForCriterion_withRowType(
            &self,
            editor: &NSRuleEditor,
            criterion: Option<&Object>,
            rowType: NSRuleEditorRowType,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other ruleEditor:child:forCriterion:withRowType:)]
        pub unsafe fn ruleEditor_child_forCriterion_withRowType(
            &self,
            editor: &NSRuleEditor,
            index: NSInteger,
            criterion: Option<&Object>,
            rowType: NSRuleEditorRowType,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other ruleEditor:displayValueForCriterion:inRow:)]
        pub unsafe fn ruleEditor_displayValueForCriterion_inRow(
            &self,
            editor: &NSRuleEditor,
            criterion: &Object,
            row: NSInteger,
        ) -> Id<Object, Shared>;

        #[optional]
        #[method_id(@__retain_semantics Other ruleEditor:predicatePartsForCriterion:withDisplayValue:inRow:)]
        pub unsafe fn ruleEditor_predicatePartsForCriterion_withDisplayValue_inRow(
            &self,
            editor: &NSRuleEditor,
            criterion: &Object,
            value: &Object,
            row: NSInteger,
        ) -> Option<Id<NSDictionary<NSRuleEditorPredicatePartKey, Object>, Shared>>;

        #[optional]
        #[method(ruleEditorRowsDidChange:)]
        pub unsafe fn ruleEditorRowsDidChange(&self, notification: &NSNotification);
    }
);

extern_static!(NSRuleEditorRowsDidChangeNotification: &'static NSNotificationName);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    unsafe impl NSRuleEditor {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;
    }
);
