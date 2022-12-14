//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMigrationManager;

    unsafe impl ClassType for NSMigrationManager {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMigrationManager {
        #[method_id(@__retain_semantics Init initWithSourceModel:destinationModel:)]
        pub unsafe fn initWithSourceModel_destinationModel(
            this: Option<Allocated<Self>>,
            sourceModel: &NSManagedObjectModel,
            destinationModel: &NSManagedObjectModel,
        ) -> Id<Self, Shared>;

        #[method(migrateStoreFromURL:type:options:withMappingModel:toDestinationURL:destinationType:destinationOptions:error:_)]
        pub unsafe fn migrateStoreFromURL_type_options_withMappingModel_toDestinationURL_destinationType_destinationOptions_error(
            &self,
            sourceURL: &NSURL,
            sStoreType: &NSString,
            sOptions: Option<&NSDictionary>,
            mappings: Option<&NSMappingModel>,
            dURL: &NSURL,
            dStoreType: &NSString,
            dOptions: Option<&NSDictionary>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(usesStoreSpecificMigrationManager)]
        pub unsafe fn usesStoreSpecificMigrationManager(&self) -> bool;

        #[method(setUsesStoreSpecificMigrationManager:)]
        pub unsafe fn setUsesStoreSpecificMigrationManager(
            &self,
            usesStoreSpecificMigrationManager: bool,
        );

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[method_id(@__retain_semantics Other mappingModel)]
        pub unsafe fn mappingModel(&self) -> Id<NSMappingModel, Shared>;

        #[method_id(@__retain_semantics Other sourceModel)]
        pub unsafe fn sourceModel(&self) -> Id<NSManagedObjectModel, Shared>;

        #[method_id(@__retain_semantics Other destinationModel)]
        pub unsafe fn destinationModel(&self) -> Id<NSManagedObjectModel, Shared>;

        #[method_id(@__retain_semantics Other sourceContext)]
        pub unsafe fn sourceContext(&self) -> Id<NSManagedObjectContext, Shared>;

        #[method_id(@__retain_semantics Other destinationContext)]
        pub unsafe fn destinationContext(&self) -> Id<NSManagedObjectContext, Shared>;

        #[method_id(@__retain_semantics Other sourceEntityForEntityMapping:)]
        pub unsafe fn sourceEntityForEntityMapping(
            &self,
            mEntity: &NSEntityMapping,
        ) -> Option<Id<NSEntityDescription, Shared>>;

        #[method_id(@__retain_semantics Other destinationEntityForEntityMapping:)]
        pub unsafe fn destinationEntityForEntityMapping(
            &self,
            mEntity: &NSEntityMapping,
        ) -> Option<Id<NSEntityDescription, Shared>>;

        #[method(associateSourceInstance:withDestinationInstance:forEntityMapping:)]
        pub unsafe fn associateSourceInstance_withDestinationInstance_forEntityMapping(
            &self,
            sourceInstance: &NSManagedObject,
            destinationInstance: &NSManagedObject,
            entityMapping: &NSEntityMapping,
        );

        #[method_id(@__retain_semantics Other destinationInstancesForEntityMappingNamed:sourceInstances:)]
        pub unsafe fn destinationInstancesForEntityMappingNamed_sourceInstances(
            &self,
            mappingName: &NSString,
            sourceInstances: Option<&NSArray<NSManagedObject>>,
        ) -> Id<NSArray<NSManagedObject>, Shared>;

        #[method_id(@__retain_semantics Other sourceInstancesForEntityMappingNamed:destinationInstances:)]
        pub unsafe fn sourceInstancesForEntityMappingNamed_destinationInstances(
            &self,
            mappingName: &NSString,
            destinationInstances: Option<&NSArray<NSManagedObject>>,
        ) -> Id<NSArray<NSManagedObject>, Shared>;

        #[method_id(@__retain_semantics Other currentEntityMapping)]
        pub unsafe fn currentEntityMapping(&self) -> Id<NSEntityMapping, Shared>;

        #[method(migrationProgress)]
        pub unsafe fn migrationProgress(&self) -> c_float;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);

        #[method(cancelMigrationWithError:)]
        pub unsafe fn cancelMigrationWithError(&self, error: &NSError);
    }
);
