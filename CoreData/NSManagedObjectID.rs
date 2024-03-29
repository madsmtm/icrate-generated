//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObjectID;

    unsafe impl ClassType for NSManagedObjectID {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSManagedObjectID {}

unsafe impl Sync for NSManagedObjectID {}

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCopying for NSManagedObjectID {}

unsafe impl NSObjectProtocol for NSManagedObjectID {}

extern_methods!(
    unsafe impl NSManagedObjectID {
        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Id<NSEntityDescription>;

        #[cfg(feature = "CoreData_NSPersistentStore")]
        #[method_id(@__retain_semantics Other persistentStore)]
        pub unsafe fn persistentStore(&self) -> Option<Id<NSPersistentStore>>;

        #[method(isTemporaryID)]
        pub unsafe fn isTemporaryID(&self) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URIRepresentation)]
        pub unsafe fn URIRepresentation(&self) -> Id<NSURL>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSManagedObjectID {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
