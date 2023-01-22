//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::EventKit::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EKEventAvailability {
        EKEventAvailabilityNotSupported = -1,
        EKEventAvailabilityBusy = 0,
        EKEventAvailabilityFree = 1,
        EKEventAvailabilityTentative = 2,
        EKEventAvailabilityUnavailable = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EKEventStatus {
        EKEventStatusNone = 0,
        EKEventStatusConfirmed = 1,
        EKEventStatusTentative = 2,
        EKEventStatusCanceled = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKEvent")]
    pub struct EKEvent;

    #[cfg(feature = "EventKit_EKEvent")]
    unsafe impl ClassType for EKEvent {
        #[inherits(EKObject, NSObject)]
        type Super = EKCalendarItem;
    }
);

extern_methods!(
    #[cfg(feature = "EventKit_EKEvent")]
    unsafe impl EKEvent {
        #[cfg(feature = "EventKit_EKEventStore")]
        #[method_id(@__retain_semantics Other eventWithEventStore:)]
        pub unsafe fn eventWithEventStore(event_store: &EKEventStore) -> Id<EKEvent, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other eventIdentifier)]
        pub unsafe fn eventIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(isAllDay)]
        pub unsafe fn isAllDay(&self) -> bool;

        #[method(setAllDay:)]
        pub unsafe fn setAllDay(&self, all_day: bool);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setStartDate:)]
        pub unsafe fn setStartDate(&self, start_date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Id<NSDate, Shared>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setEndDate:)]
        pub unsafe fn setEndDate(&self, end_date: Option<&NSDate>);

        #[cfg(feature = "EventKit_EKStructuredLocation")]
        #[method_id(@__retain_semantics Other structuredLocation)]
        pub unsafe fn structuredLocation(&self) -> Option<Id<EKStructuredLocation, Shared>>;

        #[cfg(feature = "EventKit_EKStructuredLocation")]
        #[method(setStructuredLocation:)]
        pub unsafe fn setStructuredLocation(
            &self,
            structured_location: Option<&EKStructuredLocation>,
        );

        #[method(compareStartDateWithEvent:)]
        pub unsafe fn compareStartDateWithEvent(&self, other: &EKEvent) -> NSComparisonResult;

        #[cfg(feature = "EventKit_EKParticipant")]
        #[method_id(@__retain_semantics Other organizer)]
        pub unsafe fn organizer(&self) -> Option<Id<EKParticipant, Shared>>;

        #[method(availability)]
        pub unsafe fn availability(&self) -> EKEventAvailability;

        #[method(setAvailability:)]
        pub unsafe fn setAvailability(&self, availability: EKEventAvailability);

        #[method(status)]
        pub unsafe fn status(&self) -> EKEventStatus;

        #[method(isDetached)]
        pub unsafe fn isDetached(&self) -> bool;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other occurrenceDate)]
        pub unsafe fn occurrenceDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(refresh)]
        pub unsafe fn refresh(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other birthdayContactIdentifier)]
        pub unsafe fn birthdayContactIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(birthdayPersonID)]
        pub unsafe fn birthdayPersonID(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use birthdayContactIdentifier instead"]
        #[method_id(@__retain_semantics Other birthdayPersonUniqueID)]
        pub unsafe fn birthdayPersonUniqueID(&self) -> Option<Id<NSString, Shared>>;
    }
);
