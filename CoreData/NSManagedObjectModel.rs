//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSManagedObjectModel;

    unsafe impl ClassType for NSManagedObjectModel {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for NSManagedObjectModel {}

unsafe impl NSCopying for NSManagedObjectModel {}

unsafe impl NSFastEnumeration for NSManagedObjectModel {}

unsafe impl NSObjectProtocol for NSManagedObjectModel {}

extern_methods!(
    unsafe impl NSManagedObjectModel {
        #[method_id(@__retain_semantics Other mergedModelFromBundles:)]
        pub unsafe fn mergedModelFromBundles(
            bundles: Option<&NSArray<NSBundle>>,
        ) -> Option<Id<NSManagedObjectModel>>;

        #[method_id(@__retain_semantics Other modelByMergingModels:)]
        pub unsafe fn modelByMergingModels(
            models: Option<&NSArray<NSManagedObjectModel>>,
        ) -> Option<Id<NSManagedObjectModel>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL)
            -> Option<Id<Self>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entitiesByName)]
        pub unsafe fn entitiesByName(&self) -> Id<NSDictionary<NSString, NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entities)]
        pub unsafe fn entities(&self) -> Id<NSArray<NSEntityDescription>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method(setEntities:)]
        pub unsafe fn setEntities(&self, entities: &NSArray<NSEntityDescription>);

        #[method_id(@__retain_semantics Other configurations)]
        pub unsafe fn configurations(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entitiesForConfiguration:)]
        pub unsafe fn entitiesForConfiguration(
            &self,
            configuration: Option<&NSString>,
        ) -> Option<Id<NSArray<NSEntityDescription>>>;

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method(setEntities:forConfiguration:)]
        pub unsafe fn setEntities_forConfiguration(
            &self,
            entities: &NSArray<NSEntityDescription>,
            configuration: &NSString,
        );

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSPersistentStoreRequest"
        ))]
        #[method(setFetchRequestTemplate:forName:)]
        pub unsafe fn setFetchRequestTemplate_forName(
            &self,
            fetch_request_template: Option<&NSFetchRequest>,
            name: &NSString,
        );

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other fetchRequestTemplateForName:)]
        pub unsafe fn fetchRequestTemplateForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSFetchRequest>>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other fetchRequestFromTemplateWithName:substitutionVariables:)]
        pub unsafe fn fetchRequestFromTemplateWithName_substitutionVariables(
            &self,
            name: &NSString,
            variables: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Id<NSFetchRequest>>;

        #[method_id(@__retain_semantics Other localizationDictionary)]
        pub unsafe fn localizationDictionary(&self)
            -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[method(setLocalizationDictionary:)]
        pub unsafe fn setLocalizationDictionary(
            &self,
            localization_dictionary: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method_id(@__retain_semantics Other mergedModelFromBundles:forStoreMetadata:)]
        pub unsafe fn mergedModelFromBundles_forStoreMetadata(
            bundles: Option<&NSArray<NSBundle>>,
            metadata: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Id<NSManagedObjectModel>>;

        #[method_id(@__retain_semantics Other modelByMergingModels:forStoreMetadata:)]
        pub unsafe fn modelByMergingModels_forStoreMetadata(
            models: &NSArray<NSManagedObjectModel>,
            metadata: &NSDictionary<NSString, AnyObject>,
        ) -> Option<Id<NSManagedObjectModel>>;

        #[cfg(all(
            feature = "CoreData_NSFetchRequest",
            feature = "CoreData_NSPersistentStoreRequest"
        ))]
        #[method_id(@__retain_semantics Other fetchRequestTemplatesByName)]
        pub unsafe fn fetchRequestTemplatesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSFetchRequest>>;

        #[method_id(@__retain_semantics Other versionIdentifiers)]
        pub unsafe fn versionIdentifiers(&self) -> Id<NSSet>;

        #[method(setVersionIdentifiers:)]
        pub unsafe fn setVersionIdentifiers(&self, version_identifiers: &NSSet);

        #[method(isConfiguration:compatibleWithStoreMetadata:)]
        pub unsafe fn isConfiguration_compatibleWithStoreMetadata(
            &self,
            configuration: Option<&NSString>,
            metadata: &NSDictionary<NSString, AnyObject>,
        ) -> bool;

        #[method_id(@__retain_semantics Other entityVersionHashesByName)]
        pub unsafe fn entityVersionHashesByName(&self) -> Id<NSDictionary<NSString, NSData>>;

        #[method_id(@__retain_semantics Other versionChecksum)]
        pub unsafe fn versionChecksum(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics Other checksumsForVersionedModelAtURL:error:_)]
        pub unsafe fn checksumsForVersionedModelAtURL_error(
            model_url: &NSURL,
        ) -> Result<Id<NSDictionary<NSString, NSString>>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSManagedObjectModel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
