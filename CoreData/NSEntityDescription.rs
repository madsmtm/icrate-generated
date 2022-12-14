//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSEntityDescription;

    unsafe impl ClassType for NSEntityDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSEntityDescription {
        #[method_id(@__retain_semantics Other entityForName:inManagedObjectContext:)]
        pub unsafe fn entityForName_inManagedObjectContext(
            entityName: &NSString,
            context: &NSManagedObjectContext,
        ) -> Option<Id<NSEntityDescription, Shared>>;

        #[method_id(@__retain_semantics Other insertNewObjectForEntityForName:inManagedObjectContext:)]
        pub unsafe fn insertNewObjectForEntityForName_inManagedObjectContext(
            entityName: &NSString,
            context: &NSManagedObjectContext,
        ) -> Id<NSManagedObject, Shared>;

        #[method_id(@__retain_semantics Other managedObjectModel)]
        pub unsafe fn managedObjectModel(&self) -> Id<NSManagedObjectModel, Shared>;

        #[method_id(@__retain_semantics Other managedObjectClassName)]
        pub unsafe fn managedObjectClassName(&self) -> Id<NSString, Shared>;

        #[method(setManagedObjectClassName:)]
        pub unsafe fn setManagedObjectClassName(&self, managedObjectClassName: Option<&NSString>);

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method(isAbstract)]
        pub unsafe fn isAbstract(&self) -> bool;

        #[method(setAbstract:)]
        pub unsafe fn setAbstract(&self, abstract_: bool);

        #[method_id(@__retain_semantics Other subentitiesByName)]
        pub unsafe fn subentitiesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSEntityDescription>, Shared>;

        #[method_id(@__retain_semantics Other subentities)]
        pub unsafe fn subentities(&self) -> Id<NSArray<NSEntityDescription>, Shared>;

        #[method(setSubentities:)]
        pub unsafe fn setSubentities(&self, subentities: &NSArray<NSEntityDescription>);

        #[method_id(@__retain_semantics Other superentity)]
        pub unsafe fn superentity(&self) -> Option<Id<NSEntityDescription, Shared>>;

        #[method_id(@__retain_semantics Other propertiesByName)]
        pub unsafe fn propertiesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSPropertyDescription>, Shared>;

        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Id<NSArray<NSPropertyDescription>, Shared>;

        #[method(setProperties:)]
        pub unsafe fn setProperties(&self, properties: &NSArray<NSPropertyDescription>);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);

        #[method_id(@__retain_semantics Other attributesByName)]
        pub unsafe fn attributesByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSAttributeDescription>, Shared>;

        #[method_id(@__retain_semantics Other relationshipsByName)]
        pub unsafe fn relationshipsByName(
            &self,
        ) -> Id<NSDictionary<NSString, NSRelationshipDescription>, Shared>;

        #[method_id(@__retain_semantics Other relationshipsWithDestinationEntity:)]
        pub unsafe fn relationshipsWithDestinationEntity(
            &self,
            entity: &NSEntityDescription,
        ) -> Id<NSArray<NSRelationshipDescription>, Shared>;

        #[method(isKindOfEntity:)]
        pub unsafe fn isKindOfEntity(&self, entity: &NSEntityDescription) -> bool;

        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other versionHashModifier)]
        pub unsafe fn versionHashModifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(setVersionHashModifier:)]
        pub unsafe fn setVersionHashModifier(&self, versionHashModifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other renamingIdentifier)]
        pub unsafe fn renamingIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(setRenamingIdentifier:)]
        pub unsafe fn setRenamingIdentifier(&self, renamingIdentifier: Option<&NSString>);

        #[method_id(@__retain_semantics Other indexes)]
        pub unsafe fn indexes(&self) -> Id<NSArray<NSFetchIndexDescription>, Shared>;

        #[method(setIndexes:)]
        pub unsafe fn setIndexes(&self, indexes: &NSArray<NSFetchIndexDescription>);

        #[method_id(@__retain_semantics Other uniquenessConstraints)]
        pub unsafe fn uniquenessConstraints(&self) -> Id<NSArray<NSArray<Object>>, Shared>;

        #[method(setUniquenessConstraints:)]
        pub unsafe fn setUniquenessConstraints(
            &self,
            uniquenessConstraints: &NSArray<NSArray<Object>>,
        );

        #[method_id(@__retain_semantics Other compoundIndexes)]
        pub unsafe fn compoundIndexes(&self) -> Id<NSArray<NSArray<Object>>, Shared>;

        #[method(setCompoundIndexes:)]
        pub unsafe fn setCompoundIndexes(&self, compoundIndexes: &NSArray<NSArray<Object>>);

        #[method_id(@__retain_semantics Other coreSpotlightDisplayNameExpression)]
        pub unsafe fn coreSpotlightDisplayNameExpression(&self) -> Id<NSExpression, Shared>;

        #[method(setCoreSpotlightDisplayNameExpression:)]
        pub unsafe fn setCoreSpotlightDisplayNameExpression(
            &self,
            coreSpotlightDisplayNameExpression: &NSExpression,
        );
    }
);
