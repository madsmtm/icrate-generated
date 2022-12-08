//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSManagedObjectModel;

    unsafe impl ClassType for NSManagedObjectModel {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSManagedObjectModel {
        #[method_id(@__retain_semantics Other mergedModelFromBundles:)]
        pub unsafe fn mergedModelFromBundles(
            bundles: Option<&NSArray<NSBundle>>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[method_id(@__retain_semantics Other modelByMergingModels:)]
        pub unsafe fn modelByMergingModels(
            models: Option<&NSArray<NSManagedObjectModel>>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other entitiesByName)]
        pub unsafe fn entitiesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSEntityDescription>, Shared>;

        #[method_id(@__retain_semantics Other entities)]
        pub unsafe fn entities(&self) -> Id<NSArray<NSEntityDescription>, Shared>;

        #[method(setEntities:)]
        pub unsafe fn setEntities(&self, entities: &NSArray<NSEntityDescription>);

        #[method_id(@__retain_semantics Other configurations)]
        pub unsafe fn configurations(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(@__retain_semantics Other entitiesForConfiguration:)]
        pub unsafe fn entitiesForConfiguration(
            &self,
            configuration: Option<&NSString>,
        ) -> Option<Id<NSArray<NSEntityDescription>, Shared>>;

        #[method(setEntities:forConfiguration:)]
        pub unsafe fn setEntities_forConfiguration(
            &self,
            entities: &NSArray<NSEntityDescription>,
            configuration: &NSString,
        );

        #[method(setFetchRequestTemplate:forName:)]
        pub unsafe fn setFetchRequestTemplate_forName(
            &self,
            fetchRequestTemplate: Option<&NSFetchRequest>,
            name: &NSString,
        );

        #[method_id(@__retain_semantics Other fetchRequestTemplateForName:)]
        pub unsafe fn fetchRequestTemplateForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSFetchRequest, Shared>>;

        #[method_id(@__retain_semantics Other fetchRequestFromTemplateWithName:substitutionVariables:)]
        pub unsafe fn fetchRequestFromTemplateWithName_substitutionVariables(
            &self,
            name: &NSString,
            variables: &NSDictionary<NSString, Object>,
        ) -> Option<Id<NSFetchRequest, Shared>>;

        #[method_id(@__retain_semantics Other localizationDictionary)]
        pub unsafe fn localizationDictionary(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSString>, Shared>>;

        #[method(setLocalizationDictionary:)]
        pub unsafe fn setLocalizationDictionary(
            &self,
            localizationDictionary: Option<&NSDictionary<NSString, NSString>>,
        );

        #[method_id(@__retain_semantics Other mergedModelFromBundles:forStoreMetadata:)]
        pub unsafe fn mergedModelFromBundles_forStoreMetadata(
            bundles: Option<&NSArray<NSBundle>>,
            metadata: &NSDictionary<NSString, Object>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[method_id(@__retain_semantics Other modelByMergingModels:forStoreMetadata:)]
        pub unsafe fn modelByMergingModels_forStoreMetadata(
            models: &NSArray<NSManagedObjectModel>,
            metadata: &NSDictionary<NSString, Object>,
        ) -> Option<Id<NSManagedObjectModel, Shared>>;

        #[method_id(@__retain_semantics Other fetchRequestTemplatesByName)]
        pub unsafe fn fetchRequestTemplatesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSFetchRequest>, Shared>;

        #[method_id(@__retain_semantics Other versionIdentifiers)]
        pub unsafe fn versionIdentifiers(&self) -> Id<NSSet, Shared>;

        #[method(setVersionIdentifiers:)]
        pub unsafe fn setVersionIdentifiers(&self, versionIdentifiers: &NSSet);

        #[method(isConfiguration:compatibleWithStoreMetadata:)]
        pub unsafe fn isConfiguration_compatibleWithStoreMetadata(
            &self,
            configuration: Option<&NSString>,
            metadata: &NSDictionary<NSString, Object>,
        ) -> bool;

        #[method_id(@__retain_semantics Other entityVersionHashesByName)]
        pub unsafe fn entityVersionHashesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSData>, Shared>;
    }
);
