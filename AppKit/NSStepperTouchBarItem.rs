//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    pub struct NSStepperTouchBarItem;

    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl ClassType for NSStepperTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSTouchBarItem")]
unsafe impl NSCoding for NSStepperTouchBarItem {}

#[cfg(feature = "AppKit_NSTouchBarItem")]
unsafe impl NSObjectProtocol for NSStepperTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl NSStepperTouchBarItem {
        #[method_id(@__retain_semantics Other stepperTouchBarItemWithIdentifier:formatter:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_formatter(
            identifier: &NSTouchBarItemIdentifier,
            formatter: &NSFormatter,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other stepperTouchBarItemWithIdentifier:drawingHandler:)]
        pub unsafe fn stepperTouchBarItemWithIdentifier_drawingHandler(
            identifier: &NSTouchBarItemIdentifier,
            drawing_handler: &Block<dyn Fn(NSRect, c_double)>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[method(increment)]
        pub unsafe fn increment(&self) -> c_double;

        #[method(setIncrement:)]
        pub unsafe fn setIncrement(&self, increment: c_double);

        #[method(value)]
        pub unsafe fn value(&self) -> c_double;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_double);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<AnyObject>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl NSStepperTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTouchBarItem")]
    unsafe impl NSStepperTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
