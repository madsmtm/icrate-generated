//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSCalendarIdentifier = NSString;
);

extern_static!(NSCalendarIdentifierGregorian: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierBuddhist: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierChinese: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierCoptic: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierEthiopicAmeteMihret: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierEthiopicAmeteAlem: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierHebrew: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierISO8601: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierIndian: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierIslamic: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierIslamicCivil: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierJapanese: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierPersian: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierRepublicOfChina: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierIslamicTabular: &'static NSCalendarIdentifier);

extern_static!(NSCalendarIdentifierIslamicUmmAlQura: &'static NSCalendarIdentifier);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSCalendarUnit {
        NSCalendarUnitEra = 2,
        NSCalendarUnitYear = 4,
        NSCalendarUnitMonth = 8,
        NSCalendarUnitDay = 16,
        NSCalendarUnitHour = 32,
        NSCalendarUnitMinute = 64,
        NSCalendarUnitSecond = 128,
        NSCalendarUnitWeekday = 512,
        NSCalendarUnitWeekdayOrdinal = 1024,
        NSCalendarUnitQuarter = 2048,
        NSCalendarUnitWeekOfMonth = 4096,
        NSCalendarUnitWeekOfYear = 8192,
        NSCalendarUnitYearForWeekOfYear = 16384,
        NSCalendarUnitNanosecond = 32768,
        NSCalendarUnitCalendar = 1048576,
        NSCalendarUnitTimeZone = 2097152,
        NSEraCalendarUnit = 2,
        NSYearCalendarUnit = 4,
        NSMonthCalendarUnit = 8,
        NSDayCalendarUnit = 16,
        NSHourCalendarUnit = 32,
        NSMinuteCalendarUnit = 64,
        NSSecondCalendarUnit = 128,
        NSWeekCalendarUnit = 256,
        NSWeekdayCalendarUnit = 512,
        NSWeekdayOrdinalCalendarUnit = 1024,
        NSQuarterCalendarUnit = 2048,
        NSWeekOfMonthCalendarUnit = 4096,
        NSWeekOfYearCalendarUnit = 8192,
        NSYearForWeekOfYearCalendarUnit = 16384,
        NSCalendarCalendarUnit = 1048576,
        NSTimeZoneCalendarUnit = 2097152,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSCalendarOptions {
        NSCalendarWrapComponents = 1 << 0,
        NSCalendarMatchStrictly = 1 << 1,
        NSCalendarSearchBackwards = 1 << 2,
        NSCalendarMatchPreviousTimePreservingSmallerUnits = 1 << 8,
        NSCalendarMatchNextTimePreservingSmallerUnits = 1 << 9,
        NSCalendarMatchNextTime = 1 << 10,
        NSCalendarMatchFirst = 1 << 12,
        NSCalendarMatchLast = 1 << 13,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSCalendar;

    unsafe impl ClassType for NSCalendar {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSCalendar {
        #[method_id(@__retain_semantics Other currentCalendar)]
        pub unsafe fn currentCalendar() -> Id<NSCalendar, Shared>;

        #[method_id(@__retain_semantics Other autoupdatingCurrentCalendar)]
        pub unsafe fn autoupdatingCurrentCalendar() -> Id<NSCalendar, Shared>;

        #[method_id(@__retain_semantics Other calendarWithIdentifier:)]
        pub unsafe fn calendarWithIdentifier(
            calendarIdentifierConstant: &NSCalendarIdentifier,
        ) -> Option<Id<NSCalendar, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCalendarIdentifier:)]
        pub unsafe fn initWithCalendarIdentifier(
            this: Option<Allocated<Self>>,
            ident: &NSCalendarIdentifier,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Id<NSCalendarIdentifier, Shared>;

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Id<NSLocale, Shared>>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: &NSTimeZone);

        #[method(firstWeekday)]
        pub unsafe fn firstWeekday(&self) -> NSUInteger;

        #[method(setFirstWeekday:)]
        pub unsafe fn setFirstWeekday(&self, firstWeekday: NSUInteger);

        #[method(minimumDaysInFirstWeek)]
        pub unsafe fn minimumDaysInFirstWeek(&self) -> NSUInteger;

        #[method(setMinimumDaysInFirstWeek:)]
        pub unsafe fn setMinimumDaysInFirstWeek(&self, minimumDaysInFirstWeek: NSUInteger);

        #[method_id(@__retain_semantics Other eraSymbols)]
        pub unsafe fn eraSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other longEraSymbols)]
        pub unsafe fn longEraSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other monthSymbols)]
        pub unsafe fn monthSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other shortMonthSymbols)]
        pub unsafe fn shortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other veryShortMonthSymbols)]
        pub unsafe fn veryShortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other standaloneMonthSymbols)]
        pub unsafe fn standaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other shortStandaloneMonthSymbols)]
        pub unsafe fn shortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other veryShortStandaloneMonthSymbols)]
        pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other weekdaySymbols)]
        pub unsafe fn weekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other shortWeekdaySymbols)]
        pub unsafe fn shortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other veryShortWeekdaySymbols)]
        pub unsafe fn veryShortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other standaloneWeekdaySymbols)]
        pub unsafe fn standaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other shortStandaloneWeekdaySymbols)]
        pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other veryShortStandaloneWeekdaySymbols)]
        pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other quarterSymbols)]
        pub unsafe fn quarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other shortQuarterSymbols)]
        pub unsafe fn shortQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other standaloneQuarterSymbols)]
        pub unsafe fn standaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other shortStandaloneQuarterSymbols)]
        pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other AMSymbol)]
        pub unsafe fn AMSymbol(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other PMSymbol)]
        pub unsafe fn PMSymbol(&self) -> Id<NSString, Shared>;

        #[method(minimumRangeOfUnit:)]
        pub unsafe fn minimumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange;

        #[method(maximumRangeOfUnit:)]
        pub unsafe fn maximumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange;

        #[method(rangeOfUnit:inUnit:forDate:)]
        pub unsafe fn rangeOfUnit_inUnit_forDate(
            &self,
            smaller: NSCalendarUnit,
            larger: NSCalendarUnit,
            date: &NSDate,
        ) -> NSRange;

        #[method(ordinalityOfUnit:inUnit:forDate:)]
        pub unsafe fn ordinalityOfUnit_inUnit_forDate(
            &self,
            smaller: NSCalendarUnit,
            larger: NSCalendarUnit,
            date: &NSDate,
        ) -> NSUInteger;

        #[method(rangeOfUnit:startDate:interval:forDate:)]
        pub unsafe fn rangeOfUnit_startDate_interval_forDate(
            &self,
            unit: NSCalendarUnit,
            datep: *mut *mut NSDate,
            tip: *mut NSTimeInterval,
            date: &NSDate,
        ) -> bool;

        #[method_id(@__retain_semantics Other dateFromComponents:)]
        pub unsafe fn dateFromComponents(
            &self,
            comps: &NSDateComponents,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other components:fromDate:)]
        pub unsafe fn components_fromDate(
            &self,
            unitFlags: NSCalendarUnit,
            date: &NSDate,
        ) -> Id<NSDateComponents, Shared>;

        #[method_id(@__retain_semantics Other dateByAddingComponents:toDate:options:)]
        pub unsafe fn dateByAddingComponents_toDate_options(
            &self,
            comps: &NSDateComponents,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other components:fromDate:toDate:options:)]
        pub unsafe fn components_fromDate_toDate_options(
            &self,
            unitFlags: NSCalendarUnit,
            startingDate: &NSDate,
            resultDate: &NSDate,
            opts: NSCalendarOptions,
        ) -> Id<NSDateComponents, Shared>;

        #[method(getEra:year:month:day:fromDate:)]
        pub unsafe fn getEra_year_month_day_fromDate(
            &self,
            eraValuePointer: *mut NSInteger,
            yearValuePointer: *mut NSInteger,
            monthValuePointer: *mut NSInteger,
            dayValuePointer: *mut NSInteger,
            date: &NSDate,
        );

        #[method(getEra:yearForWeekOfYear:weekOfYear:weekday:fromDate:)]
        pub unsafe fn getEra_yearForWeekOfYear_weekOfYear_weekday_fromDate(
            &self,
            eraValuePointer: *mut NSInteger,
            yearValuePointer: *mut NSInteger,
            weekValuePointer: *mut NSInteger,
            weekdayValuePointer: *mut NSInteger,
            date: &NSDate,
        );

        #[method(getHour:minute:second:nanosecond:fromDate:)]
        pub unsafe fn getHour_minute_second_nanosecond_fromDate(
            &self,
            hourValuePointer: *mut NSInteger,
            minuteValuePointer: *mut NSInteger,
            secondValuePointer: *mut NSInteger,
            nanosecondValuePointer: *mut NSInteger,
            date: &NSDate,
        );

        #[method(component:fromDate:)]
        pub unsafe fn component_fromDate(&self, unit: NSCalendarUnit, date: &NSDate) -> NSInteger;

        #[method_id(@__retain_semantics Other dateWithEra:year:month:day:hour:minute:second:nanosecond:)]
        pub unsafe fn dateWithEra_year_month_day_hour_minute_second_nanosecond(
            &self,
            eraValue: NSInteger,
            yearValue: NSInteger,
            monthValue: NSInteger,
            dayValue: NSInteger,
            hourValue: NSInteger,
            minuteValue: NSInteger,
            secondValue: NSInteger,
            nanosecondValue: NSInteger,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other dateWithEra:yearForWeekOfYear:weekOfYear:weekday:hour:minute:second:nanosecond:)]
        pub unsafe fn dateWithEra_yearForWeekOfYear_weekOfYear_weekday_hour_minute_second_nanosecond(
            &self,
            eraValue: NSInteger,
            yearValue: NSInteger,
            weekValue: NSInteger,
            weekdayValue: NSInteger,
            hourValue: NSInteger,
            minuteValue: NSInteger,
            secondValue: NSInteger,
            nanosecondValue: NSInteger,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other startOfDayForDate:)]
        pub unsafe fn startOfDayForDate(&self, date: &NSDate) -> Id<NSDate, Shared>;

        #[method_id(@__retain_semantics Other componentsInTimeZone:fromDate:)]
        pub unsafe fn componentsInTimeZone_fromDate(
            &self,
            timezone: &NSTimeZone,
            date: &NSDate,
        ) -> Id<NSDateComponents, Shared>;

        #[method(compareDate:toDate:toUnitGranularity:)]
        pub unsafe fn compareDate_toDate_toUnitGranularity(
            &self,
            date1: &NSDate,
            date2: &NSDate,
            unit: NSCalendarUnit,
        ) -> NSComparisonResult;

        #[method(isDate:equalToDate:toUnitGranularity:)]
        pub unsafe fn isDate_equalToDate_toUnitGranularity(
            &self,
            date1: &NSDate,
            date2: &NSDate,
            unit: NSCalendarUnit,
        ) -> bool;

        #[method(isDate:inSameDayAsDate:)]
        pub unsafe fn isDate_inSameDayAsDate(&self, date1: &NSDate, date2: &NSDate) -> bool;

        #[method(isDateInToday:)]
        pub unsafe fn isDateInToday(&self, date: &NSDate) -> bool;

        #[method(isDateInYesterday:)]
        pub unsafe fn isDateInYesterday(&self, date: &NSDate) -> bool;

        #[method(isDateInTomorrow:)]
        pub unsafe fn isDateInTomorrow(&self, date: &NSDate) -> bool;

        #[method(isDateInWeekend:)]
        pub unsafe fn isDateInWeekend(&self, date: &NSDate) -> bool;

        #[method(rangeOfWeekendStartDate:interval:containingDate:)]
        pub unsafe fn rangeOfWeekendStartDate_interval_containingDate(
            &self,
            datep: *mut *mut NSDate,
            tip: *mut NSTimeInterval,
            date: &NSDate,
        ) -> bool;

        #[method(nextWeekendStartDate:interval:options:afterDate:)]
        pub unsafe fn nextWeekendStartDate_interval_options_afterDate(
            &self,
            datep: *mut *mut NSDate,
            tip: *mut NSTimeInterval,
            options: NSCalendarOptions,
            date: &NSDate,
        ) -> bool;

        #[method_id(@__retain_semantics Other components:fromDateComponents:toDateComponents:options:)]
        pub unsafe fn components_fromDateComponents_toDateComponents_options(
            &self,
            unitFlags: NSCalendarUnit,
            startingDateComp: &NSDateComponents,
            resultDateComp: &NSDateComponents,
            options: NSCalendarOptions,
        ) -> Id<NSDateComponents, Shared>;

        #[method_id(@__retain_semantics Other dateByAddingUnit:value:toDate:options:)]
        pub unsafe fn dateByAddingUnit_value_toDate_options(
            &self,
            unit: NSCalendarUnit,
            value: NSInteger,
            date: &NSDate,
            options: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method(enumerateDatesStartingAfterDate:matchingComponents:options:usingBlock:)]
        pub unsafe fn enumerateDatesStartingAfterDate_matchingComponents_options_usingBlock(
            &self,
            start: &NSDate,
            comps: &NSDateComponents,
            opts: NSCalendarOptions,
            block: &Block<(*mut NSDate, Bool, NonNull<Bool>), ()>,
        );

        #[method_id(@__retain_semantics Other nextDateAfterDate:matchingComponents:options:)]
        pub unsafe fn nextDateAfterDate_matchingComponents_options(
            &self,
            date: &NSDate,
            comps: &NSDateComponents,
            options: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other nextDateAfterDate:matchingUnit:value:options:)]
        pub unsafe fn nextDateAfterDate_matchingUnit_value_options(
            &self,
            date: &NSDate,
            unit: NSCalendarUnit,
            value: NSInteger,
            options: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other nextDateAfterDate:matchingHour:minute:second:options:)]
        pub unsafe fn nextDateAfterDate_matchingHour_minute_second_options(
            &self,
            date: &NSDate,
            hourValue: NSInteger,
            minuteValue: NSInteger,
            secondValue: NSInteger,
            options: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other dateBySettingUnit:value:ofDate:options:)]
        pub unsafe fn dateBySettingUnit_value_ofDate_options(
            &self,
            unit: NSCalendarUnit,
            v: NSInteger,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other dateBySettingHour:minute:second:ofDate:options:)]
        pub unsafe fn dateBySettingHour_minute_second_ofDate_options(
            &self,
            h: NSInteger,
            m: NSInteger,
            s: NSInteger,
            date: &NSDate,
            opts: NSCalendarOptions,
        ) -> Option<Id<NSDate, Shared>>;

        #[method(date:matchesComponents:)]
        pub unsafe fn date_matchesComponents(
            &self,
            date: &NSDate,
            components: &NSDateComponents,
        ) -> bool;
    }
);

extern_static!(NSCalendarDayChangedNotification: &'static NSNotificationName);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum {
        NSDateComponentUndefined = NSIntegerMax as _,
        NSUndefinedDateComponent = NSDateComponentUndefined,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDateComponents;

    unsafe impl ClassType for NSDateComponents {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSDateComponents {
        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<NSCalendar, Shared>>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);

        #[method(era)]
        pub unsafe fn era(&self) -> NSInteger;

        #[method(setEra:)]
        pub unsafe fn setEra(&self, era: NSInteger);

        #[method(year)]
        pub unsafe fn year(&self) -> NSInteger;

        #[method(setYear:)]
        pub unsafe fn setYear(&self, year: NSInteger);

        #[method(month)]
        pub unsafe fn month(&self) -> NSInteger;

        #[method(setMonth:)]
        pub unsafe fn setMonth(&self, month: NSInteger);

        #[method(day)]
        pub unsafe fn day(&self) -> NSInteger;

        #[method(setDay:)]
        pub unsafe fn setDay(&self, day: NSInteger);

        #[method(hour)]
        pub unsafe fn hour(&self) -> NSInteger;

        #[method(setHour:)]
        pub unsafe fn setHour(&self, hour: NSInteger);

        #[method(minute)]
        pub unsafe fn minute(&self) -> NSInteger;

        #[method(setMinute:)]
        pub unsafe fn setMinute(&self, minute: NSInteger);

        #[method(second)]
        pub unsafe fn second(&self) -> NSInteger;

        #[method(setSecond:)]
        pub unsafe fn setSecond(&self, second: NSInteger);

        #[method(nanosecond)]
        pub unsafe fn nanosecond(&self) -> NSInteger;

        #[method(setNanosecond:)]
        pub unsafe fn setNanosecond(&self, nanosecond: NSInteger);

        #[method(weekday)]
        pub unsafe fn weekday(&self) -> NSInteger;

        #[method(setWeekday:)]
        pub unsafe fn setWeekday(&self, weekday: NSInteger);

        #[method(weekdayOrdinal)]
        pub unsafe fn weekdayOrdinal(&self) -> NSInteger;

        #[method(setWeekdayOrdinal:)]
        pub unsafe fn setWeekdayOrdinal(&self, weekdayOrdinal: NSInteger);

        #[method(quarter)]
        pub unsafe fn quarter(&self) -> NSInteger;

        #[method(setQuarter:)]
        pub unsafe fn setQuarter(&self, quarter: NSInteger);

        #[method(weekOfMonth)]
        pub unsafe fn weekOfMonth(&self) -> NSInteger;

        #[method(setWeekOfMonth:)]
        pub unsafe fn setWeekOfMonth(&self, weekOfMonth: NSInteger);

        #[method(weekOfYear)]
        pub unsafe fn weekOfYear(&self) -> NSInteger;

        #[method(setWeekOfYear:)]
        pub unsafe fn setWeekOfYear(&self, weekOfYear: NSInteger);

        #[method(yearForWeekOfYear)]
        pub unsafe fn yearForWeekOfYear(&self) -> NSInteger;

        #[method(setYearForWeekOfYear:)]
        pub unsafe fn setYearForWeekOfYear(&self, yearForWeekOfYear: NSInteger);

        #[method(isLeapMonth)]
        pub unsafe fn isLeapMonth(&self) -> bool;

        #[method(setLeapMonth:)]
        pub unsafe fn setLeapMonth(&self, leapMonth: bool);

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Option<Id<NSDate, Shared>>;

        #[method(week)]
        pub unsafe fn week(&self) -> NSInteger;

        #[method(setWeek:)]
        pub unsafe fn setWeek(&self, v: NSInteger);

        #[method(setValue:forComponent:)]
        pub unsafe fn setValue_forComponent(&self, value: NSInteger, unit: NSCalendarUnit);

        #[method(valueForComponent:)]
        pub unsafe fn valueForComponent(&self, unit: NSCalendarUnit) -> NSInteger;

        #[method(isValidDate)]
        pub unsafe fn isValidDate(&self) -> bool;

        #[method(isValidDateInCalendar:)]
        pub unsafe fn isValidDateInCalendar(&self, calendar: &NSCalendar) -> bool;
    }
);
