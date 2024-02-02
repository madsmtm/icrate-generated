//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::Photos::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHAssetResource")]
    pub struct PHAssetResource;

    #[cfg(feature = "Photos_PHAssetResource")]
    unsafe impl ClassType for PHAssetResource {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHAssetResource")]
unsafe impl NSObjectProtocol for PHAssetResource {}

extern_methods!(
    #[cfg(feature = "Photos_PHAssetResource")]
    unsafe impl PHAssetResource {
        #[method(type)]
        pub unsafe fn r#type(&self) -> PHAssetResourceType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other assetLocalIdentifier)]
        pub unsafe fn assetLocalIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other uniformTypeIdentifier)]
        pub unsafe fn uniformTypeIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other originalFilename)]
        pub unsafe fn originalFilename(&self) -> Id<NSString>;

        #[method(pixelWidth)]
        pub unsafe fn pixelWidth(&self) -> NSInteger;

        #[method(pixelHeight)]
        pub unsafe fn pixelHeight(&self) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Photos_PHAsset"))]
        #[method_id(@__retain_semantics Other assetResourcesForAsset:)]
        pub unsafe fn assetResourcesForAsset(asset: &PHAsset) -> Id<NSArray<PHAssetResource>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Photos_PHLivePhoto"))]
        #[method_id(@__retain_semantics Other assetResourcesForLivePhoto:)]
        pub unsafe fn assetResourcesForLivePhoto(
            live_photo: &PHLivePhoto,
        ) -> Id<NSArray<PHAssetResource>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHAssetResource")]
    unsafe impl PHAssetResource {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
