//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPropertyDescription")]
    pub struct NSPropertyDescription;

    #[cfg(feature = "CoreData_NSPropertyDescription")]
    unsafe impl ClassType for NSPropertyDescription {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreData_NSPropertyDescription")]
    unsafe impl NSPropertyDescription {
        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Id<NSEntityDescription, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[method(isOptional)]
        pub unsafe fn isOptional(&self) -> bool;

        #[method(setOptional:)]
        pub unsafe fn setOptional(&self, optional: bool);

        #[method(isTransient)]
        pub unsafe fn isTransient(&self) -> bool;

        #[method(setTransient:)]
        pub unsafe fn setTransient(&self, transient: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPredicate"))]
        #[method_id(@__retain_semantics Other validationPredicates)]
        pub unsafe fn validationPredicates(&self) -> Id<NSArray<NSPredicate>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other validationWarnings)]
        pub unsafe fn validationWarnings(&self) -> Id<NSArray, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSString"
        ))]
        #[method(setValidationPredicates:withValidationWarnings:)]
        pub unsafe fn setValidationPredicates_withValidationWarnings(
            &self,
            validationPredicates: Option<&NSArray<NSPredicate>>,
            validationWarnings: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);

        #[method(isIndexed)]
        pub unsafe fn isIndexed(&self) -> bool;

        #[method(setIndexed:)]
        pub unsafe fn setIndexed(&self, indexed: bool);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other versionHash)]
        pub unsafe fn versionHash(&self) -> Id<NSData, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other versionHashModifier)]
        pub unsafe fn versionHashModifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVersionHashModifier:)]
        pub unsafe fn setVersionHashModifier(&self, versionHashModifier: Option<&NSString>);

        #[method(isIndexedBySpotlight)]
        pub unsafe fn isIndexedBySpotlight(&self) -> bool;

        #[method(setIndexedBySpotlight:)]
        pub unsafe fn setIndexedBySpotlight(&self, indexedBySpotlight: bool);

        #[method(isStoredInExternalRecord)]
        pub unsafe fn isStoredInExternalRecord(&self) -> bool;

        #[method(setStoredInExternalRecord:)]
        pub unsafe fn setStoredInExternalRecord(&self, storedInExternalRecord: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other renamingIdentifier)]
        pub unsafe fn renamingIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setRenamingIdentifier:)]
        pub unsafe fn setRenamingIdentifier(&self, renamingIdentifier: Option<&NSString>);
    }
);
