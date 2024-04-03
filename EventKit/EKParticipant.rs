//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKObject")]
    pub struct EKParticipant;

    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl ClassType for EKParticipant {
        #[inherits(NSObject)]
        type Super = EKObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "EventKit_EKObject")]
unsafe impl NSCopying for EKParticipant {}

#[cfg(feature = "EventKit_EKObject")]
unsafe impl NSObjectProtocol for EKParticipant {}

extern_methods!(
    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl EKParticipant {
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "EventKit_EKTypes")]
        #[method(participantStatus)]
        pub unsafe fn participantStatus(&self) -> EKParticipantStatus;

        #[cfg(feature = "EventKit_EKTypes")]
        #[method(participantRole)]
        pub unsafe fn participantRole(&self) -> EKParticipantRole;

        #[cfg(feature = "EventKit_EKTypes")]
        #[method(participantType)]
        pub unsafe fn participantType(&self) -> EKParticipantType;

        #[method(isCurrentUser)]
        pub unsafe fn isCurrentUser(&self) -> bool;

        #[method_id(@__retain_semantics Other contactPredicate)]
        pub unsafe fn contactPredicate(&self) -> Id<NSPredicate>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "EventKit_EKObject")]
    unsafe impl EKParticipant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
