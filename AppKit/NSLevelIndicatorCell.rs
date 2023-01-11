//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSLevelIndicatorStyle {
        NSLevelIndicatorStyleRelevancy = 0,
        NSLevelIndicatorStyleContinuousCapacity = 1,
        NSLevelIndicatorStyleDiscreteCapacity = 2,
        NSLevelIndicatorStyleRating = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    pub struct NSLevelIndicatorCell;

    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl ClassType for NSLevelIndicatorCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics Init initWithLevelIndicatorStyle:)]
        pub unsafe fn initWithLevelIndicatorStyle(
            this: Option<Allocated<Self>>,
            levelIndicatorStyle: NSLevelIndicatorStyle,
        ) -> Id<Self, Shared>;

        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;

        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, levelIndicatorStyle: NSLevelIndicatorStyle);

        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);

        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);

        #[method(warningValue)]
        pub unsafe fn warningValue(&self) -> c_double;

        #[method(setWarningValue:)]
        pub unsafe fn setWarningValue(&self, warningValue: c_double);

        #[method(criticalValue)]
        pub unsafe fn criticalValue(&self) -> c_double;

        #[method(setCriticalValue:)]
        pub unsafe fn setCriticalValue(&self, criticalValue: c_double);

        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tickMarkPosition: NSTickMarkPosition);

        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, numberOfTickMarks: NSInteger);

        #[method(numberOfMajorTickMarks)]
        pub unsafe fn numberOfMajorTickMarks(&self) -> NSInteger;

        #[method(setNumberOfMajorTickMarks:)]
        pub unsafe fn setNumberOfMajorTickMarks(&self, numberOfMajorTickMarks: NSInteger);

        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
    }
);

extern_static!(
    NSRelevancyLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRelevancy
);

extern_static!(
    NSContinuousCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
        NSLevelIndicatorStyleContinuousCapacity
);

extern_static!(
    NSDiscreteCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
        NSLevelIndicatorStyleDiscreteCapacity
);

extern_static!(NSRatingLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRating);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);
