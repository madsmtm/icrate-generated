//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
use objc2_quartz_core::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDatePickerMode(pub NSInteger);
impl UIDatePickerMode {
    #[doc(alias = "UIDatePickerModeTime")]
    pub const Time: Self = Self(0);
    #[doc(alias = "UIDatePickerModeDate")]
    pub const Date: Self = Self(1);
    #[doc(alias = "UIDatePickerModeDateAndTime")]
    pub const DateAndTime: Self = Self(2);
    #[doc(alias = "UIDatePickerModeCountDownTimer")]
    pub const CountDownTimer: Self = Self(3);
    #[doc(alias = "UIDatePickerModeYearAndMonth")]
    pub const YearAndMonth: Self = Self(4);
}

unsafe impl Encode for UIDatePickerMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDatePickerMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDatePickerStyle(pub NSInteger);
impl UIDatePickerStyle {
    #[doc(alias = "UIDatePickerStyleAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UIDatePickerStyleWheels")]
    pub const Wheels: Self = Self(1);
    #[doc(alias = "UIDatePickerStyleCompact")]
    pub const Compact: Self = Self(2);
    #[doc(alias = "UIDatePickerStyleInline")]
    pub const Inline: Self = Self(3);
}

unsafe impl Encode for UIDatePickerStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDatePickerStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    pub struct UIDatePicker;

    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl ClassType for UIDatePicker {
        #[inherits(UIView, UIResponder, NSObject)]
        type Super = UIControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
unsafe impl CALayerDelegate for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIDatePicker {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearance for UIDatePicker {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIAppearanceContainer for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusEnvironment for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItem for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIFocusItemContainer for UIDatePicker {}

#[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIDatePicker {}

#[cfg(all(
    feature = "UIControl",
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIDatePicker {}

extern_methods!(
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIDatePicker {
        #[method(datePickerMode)]
        pub unsafe fn datePickerMode(&self) -> UIDatePickerMode;

        #[method(setDatePickerMode:)]
        pub unsafe fn setDatePickerMode(&self, date_picker_mode: UIDatePickerMode);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Id<NSLocale>>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, time_zone: Option<&NSTimeZone>);

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Id<NSDate>;

        #[method(setDate:)]
        pub unsafe fn setDate(&self, date: &NSDate);

        #[method_id(@__retain_semantics Other minimumDate)]
        pub unsafe fn minimumDate(&self) -> Option<Id<NSDate>>;

        #[method(setMinimumDate:)]
        pub unsafe fn setMinimumDate(&self, minimum_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Other maximumDate)]
        pub unsafe fn maximumDate(&self) -> Option<Id<NSDate>>;

        #[method(setMaximumDate:)]
        pub unsafe fn setMaximumDate(&self, maximum_date: Option<&NSDate>);

        #[method(countDownDuration)]
        pub unsafe fn countDownDuration(&self) -> NSTimeInterval;

        #[method(setCountDownDuration:)]
        pub unsafe fn setCountDownDuration(&self, count_down_duration: NSTimeInterval);

        #[method(minuteInterval)]
        pub unsafe fn minuteInterval(&self) -> NSInteger;

        #[method(setMinuteInterval:)]
        pub unsafe fn setMinuteInterval(&self, minute_interval: NSInteger);

        #[method(setDate:animated:)]
        pub unsafe fn setDate_animated(&self, date: &NSDate, animated: bool);

        #[method(preferredDatePickerStyle)]
        pub unsafe fn preferredDatePickerStyle(&self) -> UIDatePickerStyle;

        #[method(setPreferredDatePickerStyle:)]
        pub unsafe fn setPreferredDatePickerStyle(
            &self,
            preferred_date_picker_style: UIDatePickerStyle,
        );

        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> UIDatePickerStyle;

        #[method(roundsToMinuteInterval)]
        pub unsafe fn roundsToMinuteInterval(&self) -> bool;

        #[method(setRoundsToMinuteInterval:)]
        pub unsafe fn setRoundsToMinuteInterval(&self, rounds_to_minute_interval: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIControl`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIDatePicker {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(all(feature = "UIAction", feature = "UIMenuElement"))]
        #[method_id(@__retain_semantics Init initWithFrame:primaryAction:)]
        pub unsafe fn initWithFrame_primaryAction(
            this: Allocated<Self>,
            frame: CGRect,
            primary_action: Option<&UIAction>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIControl", feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIDatePicker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);