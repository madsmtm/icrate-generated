//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGTaskRequest;

    unsafe impl ClassType for BGTaskRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for BGTaskRequest {}

unsafe impl NSObjectProtocol for BGTaskRequest {}

extern_methods!(
    unsafe impl BGTaskRequest {
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other earliestBeginDate)]
        pub unsafe fn earliestBeginDate(&self) -> Option<Retained<NSDate>>;

        #[method(setEarliestBeginDate:)]
        pub unsafe fn setEarliestBeginDate(&self, earliest_begin_date: Option<&NSDate>);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGAppRefreshTaskRequest;

    unsafe impl ClassType for BGAppRefreshTaskRequest {
        #[inherits(NSObject)]
        type Super = BGTaskRequest;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for BGAppRefreshTaskRequest {}

unsafe impl NSObjectProtocol for BGAppRefreshTaskRequest {}

extern_methods!(
    unsafe impl BGAppRefreshTaskRequest {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `BGTaskRequest`
    unsafe impl BGAppRefreshTaskRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGProcessingTaskRequest;

    unsafe impl ClassType for BGProcessingTaskRequest {
        #[inherits(NSObject)]
        type Super = BGTaskRequest;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for BGProcessingTaskRequest {}

unsafe impl NSObjectProtocol for BGProcessingTaskRequest {}

extern_methods!(
    unsafe impl BGProcessingTaskRequest {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[method(requiresNetworkConnectivity)]
        pub unsafe fn requiresNetworkConnectivity(&self) -> bool;

        #[method(setRequiresNetworkConnectivity:)]
        pub unsafe fn setRequiresNetworkConnectivity(&self, requires_network_connectivity: bool);

        #[method(requiresExternalPower)]
        pub unsafe fn requiresExternalPower(&self) -> bool;

        #[method(setRequiresExternalPower:)]
        pub unsafe fn setRequiresExternalPower(&self, requires_external_power: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `BGTaskRequest`
    unsafe impl BGProcessingTaskRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGHealthResearchTaskRequest;

    unsafe impl ClassType for BGHealthResearchTaskRequest {
        #[inherits(BGTaskRequest, NSObject)]
        type Super = BGProcessingTaskRequest;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for BGHealthResearchTaskRequest {}

unsafe impl NSObjectProtocol for BGHealthResearchTaskRequest {}

extern_methods!(
    unsafe impl BGHealthResearchTaskRequest {
        #[method_id(@__retain_semantics Other protectionTypeOfRequiredData)]
        pub unsafe fn protectionTypeOfRequiredData(&self) -> Retained<NSFileProtectionType>;

        #[method(setProtectionTypeOfRequiredData:)]
        pub unsafe fn setProtectionTypeOfRequiredData(
            &self,
            protection_type_of_required_data: &NSFileProtectionType,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `BGProcessingTaskRequest`
    unsafe impl BGHealthResearchTaskRequest {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `BGTaskRequest`
    unsafe impl BGHealthResearchTaskRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct BGContinuedProcessingTaskRequest;

    unsafe impl ClassType for BGContinuedProcessingTaskRequest {
        #[inherits(NSObject)]
        type Super = BGTaskRequest;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for BGContinuedProcessingTaskRequest {}

unsafe impl NSObjectProtocol for BGContinuedProcessingTaskRequest {}

extern_methods!(
    unsafe impl BGContinuedProcessingTaskRequest {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other reason)]
        pub unsafe fn reason(&self) -> Retained<NSString>;

        #[method(setReason:)]
        pub unsafe fn setReason(&self, reason: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `BGTaskRequest`
    unsafe impl BGContinuedProcessingTaskRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
