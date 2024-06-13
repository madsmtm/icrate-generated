//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResidencySetDescriptor;

    unsafe impl ClassType for MTLResidencySetDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MTLResidencySetDescriptor {}

unsafe impl NSObjectProtocol for MTLResidencySetDescriptor {}

extern_methods!(
    unsafe impl MTLResidencySetDescriptor {
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(initialCapacity)]
        pub unsafe fn initialCapacity(&self) -> NSUInteger;

        #[method(setInitialCapacity:)]
        pub unsafe fn setInitialCapacity(&self, initial_capacity: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLResidencySetDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLResidencySet: NSObjectProtocol + IsRetainable {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(allocatedSize)]
        unsafe fn allocatedSize(&self) -> u64;

        #[method(requestResidency)]
        unsafe fn requestResidency(&self);

        #[method(endResidency)]
        unsafe fn endResidency(&self);

        #[cfg(feature = "MTLAllocation")]
        #[method(addAllocation:)]
        unsafe fn addAllocation(&self, allocation: &ProtocolObject<dyn MTLAllocation>);

        #[cfg(feature = "MTLAllocation")]
        #[method(addAllocations:count:)]
        unsafe fn addAllocations_count(
            &self,
            allocations: NonNull<NonNull<ProtocolObject<dyn MTLAllocation>>>,
            count: NSUInteger,
        );

        #[cfg(feature = "MTLAllocation")]
        #[method(removeAllocation:)]
        unsafe fn removeAllocation(&self, allocation: &ProtocolObject<dyn MTLAllocation>);

        #[cfg(feature = "MTLAllocation")]
        #[method(removeAllocations:count:)]
        unsafe fn removeAllocations_count(
            &self,
            allocations: NonNull<NonNull<ProtocolObject<dyn MTLAllocation>>>,
            count: NSUInteger,
        );

        #[method(removeAllAllocations)]
        unsafe fn removeAllAllocations(&self);

        #[cfg(feature = "MTLAllocation")]
        #[method(containsAllocation:)]
        unsafe fn containsAllocation(
            &self,
            an_allocation: &ProtocolObject<dyn MTLAllocation>,
        ) -> bool;

        #[cfg(feature = "MTLAllocation")]
        #[method_id(@__retain_semantics Other allAllocations)]
        unsafe fn allAllocations(&self) -> Retained<NSArray<ProtocolObject<dyn MTLAllocation>>>;

        #[method(allocationCount)]
        unsafe fn allocationCount(&self) -> NSUInteger;

        #[method(commit)]
        unsafe fn commit(&self);
    }

    unsafe impl ProtocolType for dyn MTLResidencySet {}
);
