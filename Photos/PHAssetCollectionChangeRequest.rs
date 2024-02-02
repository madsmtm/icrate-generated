//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::Photos::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHAssetCollectionChangeRequest")]
    pub struct PHAssetCollectionChangeRequest;

    #[cfg(feature = "Photos_PHAssetCollectionChangeRequest")]
    unsafe impl ClassType for PHAssetCollectionChangeRequest {
        #[inherits(NSObject)]
        type Super = PHChangeRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHAssetCollectionChangeRequest")]
unsafe impl NSObjectProtocol for PHAssetCollectionChangeRequest {}

extern_methods!(
    #[cfg(feature = "Photos_PHAssetCollectionChangeRequest")]
    unsafe impl PHAssetCollectionChangeRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other creationRequestForAssetCollectionWithTitle:)]
        pub unsafe fn creationRequestForAssetCollectionWithTitle(title: &NSString) -> Id<Self>;

        #[cfg(feature = "Photos_PHObjectPlaceholder")]
        #[method_id(@__retain_semantics Other placeholderForCreatedAssetCollection)]
        pub unsafe fn placeholderForCreatedAssetCollection(&self) -> Id<PHObjectPlaceholder>;

        #[method(deleteAssetCollections:)]
        pub unsafe fn deleteAssetCollections(
            asset_collections: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(feature = "Photos_PHAssetCollection")]
        #[method_id(@__retain_semantics Other changeRequestForAssetCollection:)]
        pub unsafe fn changeRequestForAssetCollection(
            asset_collection: &PHAssetCollection,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Photos_PHAsset",
            feature = "Photos_PHAssetCollection",
            feature = "Photos_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other changeRequestForAssetCollection:assets:)]
        pub unsafe fn changeRequestForAssetCollection_assets(
            asset_collection: &PHAssetCollection,
            assets: &PHFetchResult<PHAsset>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(addAssets:)]
        pub unsafe fn addAssets(&self, assets: &ProtocolObject<dyn NSFastEnumeration>);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertAssets:atIndexes:)]
        pub unsafe fn insertAssets_atIndexes(
            &self,
            assets: &ProtocolObject<dyn NSFastEnumeration>,
            indexes: &NSIndexSet,
        );

        #[method(removeAssets:)]
        pub unsafe fn removeAssets(&self, assets: &ProtocolObject<dyn NSFastEnumeration>);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeAssetsAtIndexes:)]
        pub unsafe fn removeAssetsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(replaceAssetsAtIndexes:withAssets:)]
        pub unsafe fn replaceAssetsAtIndexes_withAssets(
            &self,
            indexes: &NSIndexSet,
            assets: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(moveAssetsAtIndexes:toIndex:)]
        pub unsafe fn moveAssetsAtIndexes_toIndex(
            &self,
            from_indexes: &NSIndexSet,
            to_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHAssetCollectionChangeRequest")]
    unsafe impl PHAssetCollectionChangeRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
