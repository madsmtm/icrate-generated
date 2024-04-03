//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSRulerOrientation(pub NSUInteger);
impl NSRulerOrientation {
    pub const NSHorizontalRuler: Self = Self(0);
    pub const NSVerticalRuler: Self = Self(1);
}

unsafe impl Encode for NSRulerOrientation {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSRulerOrientation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSRulerViewUnitName = NSString;

extern "C" {
    pub static NSRulerViewUnitInches: &'static NSRulerViewUnitName;
}

extern "C" {
    pub static NSRulerViewUnitCentimeters: &'static NSRulerViewUnitName;
}

extern "C" {
    pub static NSRulerViewUnitPoints: &'static NSRulerViewUnitName;
}

extern "C" {
    pub static NSRulerViewUnitPicas: &'static NSRulerViewUnitName;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    pub struct NSRulerView;

    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl ClassType for NSRulerView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibility for NSRulerView {}

#[cfg(all(
    feature = "AppKit_NSAccessibilityProtocols",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSRulerView {}

#[cfg(all(
    feature = "AppKit_NSAnimation",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSRulerView {}

#[cfg(all(
    feature = "AppKit_NSAppearance",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSAppearanceCustomization for NSRulerView {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSCoding for NSRulerView {}

#[cfg(all(
    feature = "AppKit_NSDragging",
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSView"
))]
unsafe impl NSDraggingDestination for NSRulerView {}

#[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
unsafe impl NSObjectProtocol for NSRulerView {}

#[cfg(all(
    feature = "AppKit_NSResponder",
    feature = "AppKit_NSUserInterfaceItemIdentification",
    feature = "AppKit_NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSRulerView {}

extern_methods!(
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSRulerView {
        #[method(registerUnitWithName:abbreviation:unitToPointsConversionFactor:stepUpCycle:stepDownCycle:)]
        pub unsafe fn registerUnitWithName_abbreviation_unitToPointsConversionFactor_stepUpCycle_stepDownCycle(
            unit_name: &NSRulerViewUnitName,
            abbreviation: &NSString,
            conversion_factor: CGFloat,
            step_up_cycle: &NSArray<NSNumber>,
            step_down_cycle: &NSArray<NSNumber>,
            mtm: MainThreadMarker,
        );

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScrollView")]
        #[method_id(@__retain_semantics Init initWithScrollView:orientation:)]
        pub unsafe fn initWithScrollView_orientation(
            this: Allocated<Self>,
            scroll_view: Option<&NSScrollView>,
            orientation: NSRulerOrientation,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScrollView")]
        #[method_id(@__retain_semantics Other scrollView)]
        pub unsafe fn scrollView(&self) -> Option<Id<NSScrollView>>;

        #[cfg(feature = "AppKit_NSScrollView")]
        #[method(setScrollView:)]
        pub unsafe fn setScrollView(&self, scroll_view: Option<&NSScrollView>);

        #[method(orientation)]
        pub unsafe fn orientation(&self) -> NSRulerOrientation;

        #[method(setOrientation:)]
        pub unsafe fn setOrientation(&self, orientation: NSRulerOrientation);

        #[method(baselineLocation)]
        pub unsafe fn baselineLocation(&self) -> CGFloat;

        #[method(requiredThickness)]
        pub unsafe fn requiredThickness(&self) -> CGFloat;

        #[method(ruleThickness)]
        pub unsafe fn ruleThickness(&self) -> CGFloat;

        #[method(setRuleThickness:)]
        pub unsafe fn setRuleThickness(&self, rule_thickness: CGFloat);

        #[method(reservedThicknessForMarkers)]
        pub unsafe fn reservedThicknessForMarkers(&self) -> CGFloat;

        #[method(setReservedThicknessForMarkers:)]
        pub unsafe fn setReservedThicknessForMarkers(
            &self,
            reserved_thickness_for_markers: CGFloat,
        );

        #[method(reservedThicknessForAccessoryView)]
        pub unsafe fn reservedThicknessForAccessoryView(&self) -> CGFloat;

        #[method(setReservedThicknessForAccessoryView:)]
        pub unsafe fn setReservedThicknessForAccessoryView(
            &self,
            reserved_thickness_for_accessory_view: CGFloat,
        );

        #[method_id(@__retain_semantics Other measurementUnits)]
        pub unsafe fn measurementUnits(&self) -> Id<NSRulerViewUnitName>;

        #[method(setMeasurementUnits:)]
        pub unsafe fn setMeasurementUnits(&self, measurement_units: &NSRulerViewUnitName);

        #[method(originOffset)]
        pub unsafe fn originOffset(&self) -> CGFloat;

        #[method(setOriginOffset:)]
        pub unsafe fn setOriginOffset(&self, origin_offset: CGFloat);

        #[method_id(@__retain_semantics Other clientView)]
        pub unsafe fn clientView(&self) -> Option<Id<NSView>>;

        #[method(setClientView:)]
        pub unsafe fn setClientView(&self, client_view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(addMarker:)]
        pub unsafe fn addMarker(&self, marker: &NSRulerMarker);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(removeMarker:)]
        pub unsafe fn removeMarker(&self, marker: &NSRulerMarker);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method_id(@__retain_semantics Other markers)]
        pub unsafe fn markers(&self) -> Option<Id<NSArray<NSRulerMarker>>>;

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(setMarkers:)]
        pub unsafe fn setMarkers(&self, markers: Option<&NSArray<NSRulerMarker>>);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSRulerMarker"))]
        #[method(trackMarker:withMouseEvent:)]
        pub unsafe fn trackMarker_withMouseEvent(
            &self,
            marker: &NSRulerMarker,
            event: &NSEvent,
        ) -> bool;

        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[method(moveRulerlineFromLocation:toLocation:)]
        pub unsafe fn moveRulerlineFromLocation_toLocation(
            &self,
            old_location: CGFloat,
            new_location: CGFloat,
        );

        #[method(invalidateHashMarks)]
        pub unsafe fn invalidateHashMarks(&self);

        #[method(drawHashMarksAndLabelsInRect:)]
        pub unsafe fn drawHashMarksAndLabelsInRect(&self, rect: NSRect);

        #[method(drawMarkersInRect:)]
        pub unsafe fn drawMarkersInRect(&self, rect: NSRect);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSRulerView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSRulerView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSRulerView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSRulerMarkerClientViewDelegation
    #[cfg(all(feature = "AppKit_NSResponder", feature = "AppKit_NSView"))]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:shouldMoveMarker:)]
        pub unsafe fn rulerView_shouldMoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:willMoveMarker:toLocation:)]
        pub unsafe fn rulerView_willMoveMarker_toLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:didMoveMarker:)]
        pub unsafe fn rulerView_didMoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:shouldRemoveMarker:)]
        pub unsafe fn rulerView_shouldRemoveMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:didRemoveMarker:)]
        pub unsafe fn rulerView_didRemoveMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:shouldAddMarker:)]
        pub unsafe fn rulerView_shouldAddMarker(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
        ) -> bool;

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:willAddMarker:atLocation:)]
        pub unsafe fn rulerView_willAddMarker_atLocation(
            &self,
            ruler: &NSRulerView,
            marker: &NSRulerMarker,
            location: CGFloat,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSRulerMarker")]
        #[method(rulerView:didAddMarker:)]
        pub unsafe fn rulerView_didAddMarker(&self, ruler: &NSRulerView, marker: &NSRulerMarker);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(rulerView:handleMouseDown:)]
        pub unsafe fn rulerView_handleMouseDown(&self, ruler: &NSRulerView, event: &NSEvent);

        #[method(rulerView:willSetClientView:)]
        pub unsafe fn rulerView_willSetClientView(&self, ruler: &NSRulerView, new_client: &NSView);

        #[method(rulerView:locationForPoint:)]
        pub unsafe fn rulerView_locationForPoint(
            &self,
            ruler: &NSRulerView,
            point: NSPoint,
        ) -> CGFloat;

        #[method(rulerView:pointForLocation:)]
        pub unsafe fn rulerView_pointForLocation(
            &self,
            ruler: &NSRulerView,
            point: CGFloat,
        ) -> NSPoint;
    }
);
