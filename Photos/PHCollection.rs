//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-location")]
use objc2_core_location::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHObject")]
    pub struct PHCollection;

    #[cfg(feature = "Photos_PHObject")]
    unsafe impl ClassType for PHCollection {
        #[inherits(NSObject)]
        type Super = PHObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHObject")]
unsafe impl Send for PHCollection {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl Sync for PHCollection {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl NSCopying for PHCollection {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl NSObjectProtocol for PHCollection {}

extern_methods!(
    #[cfg(feature = "Photos_PHObject")]
    unsafe impl PHCollection {
        #[method(canContainAssets)]
        pub unsafe fn canContainAssets(&self) -> bool;

        #[method(canContainCollections)]
        pub unsafe fn canContainCollections(&self) -> bool;

        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Photos_PhotosTypes")]
        #[method(canPerformEditOperation:)]
        pub unsafe fn canPerformEditOperation(
            &self,
            an_operation: PHCollectionEditOperation,
        ) -> bool;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other fetchCollectionsInCollectionList:options:)]
        pub unsafe fn fetchCollectionsInCollectionList_options(
            collection_list: &PHCollectionList,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollection>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other fetchTopLevelUserCollectionsWithOptions:)]
        pub unsafe fn fetchTopLevelUserCollectionsWithOptions(
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollection>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHObject")]
    unsafe impl PHCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHObject")]
    pub struct PHAssetCollection;

    #[cfg(feature = "Photos_PHObject")]
    unsafe impl ClassType for PHAssetCollection {
        #[inherits(PHObject, NSObject)]
        type Super = PHCollection;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHObject")]
unsafe impl Send for PHAssetCollection {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl Sync for PHAssetCollection {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl NSCopying for PHAssetCollection {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl NSObjectProtocol for PHAssetCollection {}

extern_methods!(
    #[cfg(feature = "Photos_PHObject")]
    unsafe impl PHAssetCollection {
        #[cfg(feature = "Photos_PhotosTypes")]
        #[method(assetCollectionType)]
        pub unsafe fn assetCollectionType(&self) -> PHAssetCollectionType;

        #[cfg(feature = "Photos_PhotosTypes")]
        #[method(assetCollectionSubtype)]
        pub unsafe fn assetCollectionSubtype(&self) -> PHAssetCollectionSubtype;

        #[method(estimatedAssetCount)]
        pub unsafe fn estimatedAssetCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Id<NSDate>>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "objc2-core-location")]
        #[method_id(@__retain_semantics Other approximateLocation)]
        pub unsafe fn approximateLocation(&self) -> Option<Id<CLLocation>>;

        #[method_id(@__retain_semantics Other localizedLocationNames)]
        pub unsafe fn localizedLocationNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other fetchAssetCollectionsWithLocalIdentifiers:options:)]
        pub unsafe fn fetchAssetCollectionsWithLocalIdentifiers_options(
            identifiers: &NSArray<NSString>,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAssetCollection>>;

        #[cfg(all(
            feature = "Photos_PHFetchOptions",
            feature = "Photos_PHFetchResult",
            feature = "Photos_PhotosTypes"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetCollectionsWithType:subtype:options:)]
        pub unsafe fn fetchAssetCollectionsWithType_subtype_options(
            r#type: PHAssetCollectionType,
            subtype: PHAssetCollectionSubtype,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAssetCollection>>;

        #[cfg(all(
            feature = "Photos_PHAsset",
            feature = "Photos_PHFetchOptions",
            feature = "Photos_PHFetchResult",
            feature = "Photos_PhotosTypes"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetCollectionsContainingAsset:withType:options:)]
        pub unsafe fn fetchAssetCollectionsContainingAsset_withType_options(
            asset: &PHAsset,
            r#type: PHAssetCollectionType,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAssetCollection>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[deprecated = "Will be removed in a future release"]
        #[method_id(@__retain_semantics Other fetchAssetCollectionsWithALAssetGroupURLs:options:)]
        pub unsafe fn fetchAssetCollectionsWithALAssetGroupURLs_options(
            asset_group_ur_ls: &NSArray<NSURL>,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAssetCollection>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[deprecated = "Will be removed in a future release"]
        #[method_id(@__retain_semantics Other fetchMomentsInMomentList:options:)]
        pub unsafe fn fetchMomentsInMomentList_options(
            moment_list: &PHCollectionList,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAssetCollection>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[deprecated = "Will be removed in a future release"]
        #[method_id(@__retain_semantics Other fetchMomentsWithOptions:)]
        pub unsafe fn fetchMomentsWithOptions(
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAssetCollection>>;

        #[cfg(feature = "Photos_PHAsset")]
        #[method_id(@__retain_semantics Other transientAssetCollectionWithAssets:title:)]
        pub unsafe fn transientAssetCollectionWithAssets_title(
            assets: &NSArray<PHAsset>,
            title: Option<&NSString>,
        ) -> Id<PHAssetCollection>;

        #[cfg(all(feature = "Photos_PHAsset", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other transientAssetCollectionWithAssetFetchResult:title:)]
        pub unsafe fn transientAssetCollectionWithAssetFetchResult_title(
            fetch_result: &PHFetchResult<PHAsset>,
            title: Option<&NSString>,
        ) -> Id<PHAssetCollection>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHObject")]
    unsafe impl PHAssetCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHObject")]
    pub struct PHCollectionList;

    #[cfg(feature = "Photos_PHObject")]
    unsafe impl ClassType for PHCollectionList {
        #[inherits(PHObject, NSObject)]
        type Super = PHCollection;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHObject")]
unsafe impl Send for PHCollectionList {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl Sync for PHCollectionList {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl NSCopying for PHCollectionList {}

#[cfg(feature = "Photos_PHObject")]
unsafe impl NSObjectProtocol for PHCollectionList {}

extern_methods!(
    #[cfg(feature = "Photos_PHObject")]
    unsafe impl PHCollectionList {
        #[cfg(feature = "Photos_PhotosTypes")]
        #[method(collectionListType)]
        pub unsafe fn collectionListType(&self) -> PHCollectionListType;

        #[cfg(feature = "Photos_PhotosTypes")]
        #[method(collectionListSubtype)]
        pub unsafe fn collectionListSubtype(&self) -> PHCollectionListSubtype;

        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Id<NSDate>>;

        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[method_id(@__retain_semantics Other localizedLocationNames)]
        pub unsafe fn localizedLocationNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other fetchCollectionListsContainingCollection:options:)]
        pub unsafe fn fetchCollectionListsContainingCollection_options(
            collection: &PHCollection,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollectionList>>;

        #[cfg(all(feature = "Photos_PHFetchOptions", feature = "Photos_PHFetchResult"))]
        #[method_id(@__retain_semantics Other fetchCollectionListsWithLocalIdentifiers:options:)]
        pub unsafe fn fetchCollectionListsWithLocalIdentifiers_options(
            identifiers: &NSArray<NSString>,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollectionList>>;

        #[cfg(all(
            feature = "Photos_PHFetchOptions",
            feature = "Photos_PHFetchResult",
            feature = "Photos_PhotosTypes"
        ))]
        #[method_id(@__retain_semantics Other fetchCollectionListsWithType:subtype:options:)]
        pub unsafe fn fetchCollectionListsWithType_subtype_options(
            collection_list_type: PHCollectionListType,
            subtype: PHCollectionListSubtype,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollectionList>>;

        #[cfg(all(
            feature = "Photos_PHFetchOptions",
            feature = "Photos_PHFetchResult",
            feature = "Photos_PhotosTypes"
        ))]
        #[deprecated = "Will be removed in a future release"]
        #[method_id(@__retain_semantics Other fetchMomentListsWithSubtype:containingMoment:options:)]
        pub unsafe fn fetchMomentListsWithSubtype_containingMoment_options(
            moment_list_subtype: PHCollectionListSubtype,
            moment: &PHAssetCollection,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollectionList>>;

        #[cfg(all(
            feature = "Photos_PHFetchOptions",
            feature = "Photos_PHFetchResult",
            feature = "Photos_PhotosTypes"
        ))]
        #[deprecated = "Will be removed in a future release"]
        #[method_id(@__retain_semantics Other fetchMomentListsWithSubtype:options:)]
        pub unsafe fn fetchMomentListsWithSubtype_options(
            moment_list_subtype: PHCollectionListSubtype,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHCollectionList>>;

        #[method_id(@__retain_semantics Other transientCollectionListWithCollections:title:)]
        pub unsafe fn transientCollectionListWithCollections_title(
            collections: &NSArray<PHCollection>,
            title: Option<&NSString>,
        ) -> Id<PHCollectionList>;

        #[cfg(feature = "Photos_PHFetchResult")]
        #[method_id(@__retain_semantics Other transientCollectionListWithCollectionsFetchResult:title:)]
        pub unsafe fn transientCollectionListWithCollectionsFetchResult_title(
            fetch_result: &PHFetchResult<PHCollection>,
            title: Option<&NSString>,
        ) -> Id<PHCollectionList>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHObject")]
    unsafe impl PHCollectionList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
