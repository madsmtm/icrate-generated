//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCalendarDate;

    unsafe impl ClassType for NSCalendarDate {
        #[inherits(NSObject)]
        type Super = NSDate;
    }
);

extern_methods!(
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics Other calendarDate)]
        pub unsafe fn calendarDate() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other dateWithString:calendarFormat:locale:)]
        pub unsafe fn dateWithString_calendarFormat_locale(
            description: &NSString,
            format: &NSString,
            locale: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other dateWithString:calendarFormat:)]
        pub unsafe fn dateWithString_calendarFormat(
            description: &NSString,
            format: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other dateWithYear:month:day:hour:minute:second:timeZone:)]
        pub unsafe fn dateWithYear_month_day_hour_minute_second_timeZone(
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            aTimeZone: Option<&NSTimeZone>,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other dateByAddingYears:months:days:hours:minutes:seconds:)]
        pub unsafe fn dateByAddingYears_months_days_hours_minutes_seconds(
            &self,
            year: NSInteger,
            month: NSInteger,
            day: NSInteger,
            hour: NSInteger,
            minute: NSInteger,
            second: NSInteger,
        ) -> Id<NSCalendarDate, Shared>;

        #[method(dayOfCommonEra)]
        pub unsafe fn dayOfCommonEra(&self) -> NSInteger;

        #[method(dayOfMonth)]
        pub unsafe fn dayOfMonth(&self) -> NSInteger;

        #[method(dayOfWeek)]
        pub unsafe fn dayOfWeek(&self) -> NSInteger;

        #[method(dayOfYear)]
        pub unsafe fn dayOfYear(&self) -> NSInteger;

        #[method(hourOfDay)]
        pub unsafe fn hourOfDay(&self) -> NSInteger;

        #[method(minuteOfHour)]
        pub unsafe fn minuteOfHour(&self) -> NSInteger;

        #[method(monthOfYear)]
        pub unsafe fn monthOfYear(&self) -> NSInteger;

        #[method(secondOfMinute)]
        pub unsafe fn secondOfMinute(&self) -> NSInteger;

        #[method(yearOfCommonEra)]
        pub unsafe fn yearOfCommonEra(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other calendarFormat)]
        pub unsafe fn calendarFormat(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:locale:)]
        pub unsafe fn descriptionWithCalendarFormat_locale(
            &self,
            format: &NSString,
            locale: Option<&Object>,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:)]
        pub unsafe fn descriptionWithCalendarFormat(
            &self,
            format: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;

        #[method_id(@__retain_semantics Init initWithString:calendarFormat:locale:)]
        pub unsafe fn initWithString_calendarFormat_locale(
            this: Option<Allocated<Self>>,
            description: &NSString,
            format: &NSString,
            locale: Option<&Object>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithString:calendarFormat:)]
        pub unsafe fn initWithString_calendarFormat(
            this: Option<Allocated<Self>>,
            description: &NSString,
            format: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            description: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithYear:month:day:hour:minute:second:timeZone:)]
        pub unsafe fn initWithYear_month_day_hour_minute_second_timeZone(
            this: Option<Allocated<Self>>,
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            aTimeZone: Option<&NSTimeZone>,
        ) -> Id<Self, Shared>;

        #[method(setCalendarFormat:)]
        pub unsafe fn setCalendarFormat(&self, format: Option<&NSString>);

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, aTimeZone: Option<&NSTimeZone>);

        #[method(years:months:days:hours:minutes:seconds:sinceDate:)]
        pub unsafe fn years_months_days_hours_minutes_seconds_sinceDate(
            &self,
            yp: *mut NSInteger,
            mop: *mut NSInteger,
            dp: *mut NSInteger,
            hp: *mut NSInteger,
            mip: *mut NSInteger,
            sp: *mut NSInteger,
            date: &NSCalendarDate,
        );

        #[method_id(@__retain_semantics Other distantFuture)]
        pub unsafe fn distantFuture() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other distantPast)]
        pub unsafe fn distantPast() -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSCalendarDateExtras
    unsafe impl NSDate {
        #[method_id(@__retain_semantics Other dateWithNaturalLanguageString:locale:)]
        pub unsafe fn dateWithNaturalLanguageString_locale(
            string: &NSString,
            locale: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other dateWithNaturalLanguageString:)]
        pub unsafe fn dateWithNaturalLanguageString(
            string: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other dateWithString:)]
        pub unsafe fn dateWithString(aString: &NSString) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other dateWithCalendarFormat:timeZone:)]
        pub unsafe fn dateWithCalendarFormat_timeZone(
            &self,
            format: Option<&NSString>,
            aTimeZone: Option<&NSTimeZone>,
        ) -> Id<NSCalendarDate, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithCalendarFormat:timeZone:locale:)]
        pub unsafe fn descriptionWithCalendarFormat_timeZone_locale(
            &self,
            format: Option<&NSString>,
            aTimeZone: Option<&NSTimeZone>,
            locale: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            description: &NSString,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDate`
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            this: Option<Allocated<Self>>,
            ti: NSTimeInterval,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDate`
    ///
    /// NSDateCreation
    unsafe impl NSCalendarDate {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSinceNow:)]
        pub unsafe fn dateWithTimeIntervalSinceNow(secs: NSTimeInterval) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn dateWithTimeIntervalSinceReferenceDate(
            ti: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSince1970:)]
        pub unsafe fn dateWithTimeIntervalSince1970(secs: NSTimeInterval) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeInterval:sinceDate:)]
        pub unsafe fn dateWithTimeInterval_sinceDate(
            secsToBeAdded: NSTimeInterval,
            date: &NSDate,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceNow:)]
        pub unsafe fn initWithTimeIntervalSinceNow(
            this: Option<Allocated<Self>>,
            secs: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSince1970:)]
        pub unsafe fn initWithTimeIntervalSince1970(
            this: Option<Allocated<Self>>,
            secs: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeInterval:sinceDate:)]
        pub unsafe fn initWithTimeInterval_sinceDate(
            this: Option<Allocated<Self>>,
            secsToBeAdded: NSTimeInterval,
            date: &NSDate,
        ) -> Id<Self, Shared>;
    }
);
