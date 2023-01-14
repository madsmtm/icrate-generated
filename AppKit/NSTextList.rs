//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextListOptions {
        NSTextListPrependEnclosingMarker = 1 << 0,
    }
);

typed_extensible_enum!(
    pub type NSTextListMarkerFormat = NSString;
);

extern_static!(NSTextListMarkerBox: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerCheck: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerCircle: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerDiamond: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerDisc: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerHyphen: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerSquare: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerLowercaseHexadecimal: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerUppercaseHexadecimal: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerOctal: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerLowercaseAlpha: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerUppercaseAlpha: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerLowercaseLatin: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerUppercaseLatin: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerLowercaseRoman: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerUppercaseRoman: &'static NSTextListMarkerFormat);

extern_static!(NSTextListMarkerDecimal: &'static NSTextListMarkerFormat);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextList")]
    pub struct NSTextList;

    #[cfg(feature = "AppKit_NSTextList")]
    unsafe impl ClassType for NSTextList {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTextList")]
    unsafe impl NSTextList {
        #[method_id(@__retain_semantics Init initWithMarkerFormat:options:startingItemNumber:)]
        pub unsafe fn initWithMarkerFormat_options_startingItemNumber(
            this: Option<Allocated<Self>>,
            marker_format: &NSTextListMarkerFormat,
            options: NSTextListOptions,
            starting_item_number: NSInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithMarkerFormat:options:)]
        pub unsafe fn initWithMarkerFormat_options(
            this: Option<Allocated<Self>>,
            marker_format: &NSTextListMarkerFormat,
            options: NSUInteger,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other markerFormat)]
        pub unsafe fn markerFormat(&self) -> Id<NSTextListMarkerFormat, Shared>;

        #[method(listOptions)]
        pub unsafe fn listOptions(&self) -> NSTextListOptions;

        #[method(startingItemNumber)]
        pub unsafe fn startingItemNumber(&self) -> NSInteger;

        #[method(setStartingItemNumber:)]
        pub unsafe fn setStartingItemNumber(&self, starting_item_number: NSInteger);

        #[method(isOrdered)]
        pub unsafe fn isOrdered(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other markerForItemNumber:)]
        pub unsafe fn markerForItemNumber(&self, item_number: NSInteger) -> Id<NSString, Shared>;
    }
);
