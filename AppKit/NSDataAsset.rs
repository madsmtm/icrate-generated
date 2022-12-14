//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSDataAssetName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDataAsset;

    unsafe impl ClassType for NSDataAsset {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSDataAsset {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(
            this: Option<Allocated<Self>>,
            name: &NSDataAssetName,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithName:bundle:)]
        pub unsafe fn initWithName_bundle(
            this: Option<Allocated<Self>>,
            name: &NSDataAssetName,
            bundle: &NSBundle,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSDataAssetName, Shared>;

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other typeIdentifier)]
        pub unsafe fn typeIdentifier(&self) -> Id<NSString, Shared>;
    }
);
