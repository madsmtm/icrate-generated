//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static NSMigrationManagerKey: &'static NSString;
}

extern "C" {
    pub static NSMigrationSourceObjectKey: &'static NSString;
}

extern "C" {
    pub static NSMigrationDestinationObjectKey: &'static NSString;
}

extern "C" {
    pub static NSMigrationEntityMappingKey: &'static NSString;
}

extern "C" {
    pub static NSMigrationPropertyMappingKey: &'static NSString;
}

extern "C" {
    pub static NSMigrationEntityPolicyKey: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEntityMigrationPolicy;

    unsafe impl ClassType for NSEntityMigrationPolicy {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSEntityMigrationPolicy {}

extern_methods!(
    unsafe impl NSEntityMigrationPolicy {
        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(beginEntityMapping:manager:error:_)]
        pub unsafe fn beginEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(createDestinationInstancesForSourceInstance:entityMapping:manager:error:_)]
        pub unsafe fn createDestinationInstancesForSourceInstance_entityMapping_manager_error(
            &self,
            s_instance: &NSManagedObject,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(endInstanceCreationForEntityMapping:manager:error:_)]
        pub unsafe fn endInstanceCreationForEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSManagedObject",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(createRelationshipsForDestinationInstance:entityMapping:manager:error:_)]
        pub unsafe fn createRelationshipsForDestinationInstance_entityMapping_manager_error(
            &self,
            d_instance: &NSManagedObject,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(endRelationshipCreationForEntityMapping:manager:error:_)]
        pub unsafe fn endRelationshipCreationForEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(performCustomValidationForEntityMapping:manager:error:_)]
        pub unsafe fn performCustomValidationForEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "CoreData_NSMigrationManager"
        ))]
        #[method(endEntityMapping:manager:error:_)]
        pub unsafe fn endEntityMapping_manager_error(
            &self,
            mapping: &NSEntityMapping,
            manager: &NSMigrationManager,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSEntityMigrationPolicy {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
