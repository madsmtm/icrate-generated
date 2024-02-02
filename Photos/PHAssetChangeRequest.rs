//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::Photos::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHAssetChangeRequest")]
    pub struct PHAssetChangeRequest;

    #[cfg(feature = "Photos_PHAssetChangeRequest")]
    unsafe impl ClassType for PHAssetChangeRequest {
        #[inherits(NSObject)]
        type Super = PHChangeRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHAssetChangeRequest")]
unsafe impl NSObjectProtocol for PHAssetChangeRequest {}

extern_methods!(
    #[cfg(feature = "Photos_PHAssetChangeRequest")]
    unsafe impl PHAssetChangeRequest {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromImage:)]
        pub unsafe fn creationRequestForAssetFromImage(image: &NSImage) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromImageAtFileURL:)]
        pub unsafe fn creationRequestForAssetFromImageAtFileURL(
            file_url: &NSURL,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromVideoAtFileURL:)]
        pub unsafe fn creationRequestForAssetFromVideoAtFileURL(
            file_url: &NSURL,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Photos_PHObjectPlaceholder")]
        #[method_id(@__retain_semantics Other placeholderForCreatedAsset)]
        pub unsafe fn placeholderForCreatedAsset(&self) -> Option<Id<PHObjectPlaceholder>>;

        #[method(deleteAssets:)]
        pub unsafe fn deleteAssets(assets: &ProtocolObject<dyn NSFastEnumeration>);

        #[cfg(feature = "Photos_PHAsset")]
        #[method_id(@__retain_semantics Other changeRequestForAsset:)]
        pub unsafe fn changeRequestForAsset(asset: &PHAsset) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setCreationDate:)]
        pub unsafe fn setCreationDate(&self, creation_date: Option<&NSDate>);

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(setLocation:)]
        pub unsafe fn setLocation(&self, location: Option<&CLLocation>);

        #[method(isFavorite)]
        pub unsafe fn isFavorite(&self) -> bool;

        #[method(setFavorite:)]
        pub unsafe fn setFavorite(&self, favorite: bool);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[cfg(feature = "Photos_PHContentEditingOutput")]
        #[method_id(@__retain_semantics Other contentEditingOutput)]
        pub unsafe fn contentEditingOutput(&self) -> Option<Id<PHContentEditingOutput>>;

        #[cfg(feature = "Photos_PHContentEditingOutput")]
        #[method(setContentEditingOutput:)]
        pub unsafe fn setContentEditingOutput(
            &self,
            content_editing_output: Option<&PHContentEditingOutput>,
        );

        #[method(revertAssetContentToOriginal)]
        pub unsafe fn revertAssetContentToOriginal(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHAssetChangeRequest")]
    unsafe impl PHAssetChangeRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

pub type PHContentEditingInputRequestID = NSUInteger;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Photos_PHContentEditingInputRequestOptions")]
    pub struct PHContentEditingInputRequestOptions;

    #[cfg(feature = "Photos_PHContentEditingInputRequestOptions")]
    unsafe impl ClassType for PHContentEditingInputRequestOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Photos_PHContentEditingInputRequestOptions")]
unsafe impl NSObjectProtocol for PHContentEditingInputRequestOptions {}

extern_methods!(
    #[cfg(feature = "Photos_PHContentEditingInputRequestOptions")]
    unsafe impl PHContentEditingInputRequestOptions {
        #[cfg(feature = "Photos_PHAdjustmentData")]
        #[method(canHandleAdjustmentData)]
        pub unsafe fn canHandleAdjustmentData(
            &self,
        ) -> NonNull<Block<dyn Fn(NonNull<PHAdjustmentData>) -> Bool>>;

        #[cfg(feature = "Photos_PHAdjustmentData")]
        #[method(setCanHandleAdjustmentData:)]
        pub unsafe fn setCanHandleAdjustmentData(
            &self,
            can_handle_adjustment_data: &Block<dyn Fn(NonNull<PHAdjustmentData>) -> Bool>,
        );

        #[method(isNetworkAccessAllowed)]
        pub unsafe fn isNetworkAccessAllowed(&self) -> bool;

        #[method(setNetworkAccessAllowed:)]
        pub unsafe fn setNetworkAccessAllowed(&self, network_access_allowed: bool);

        #[method(progressHandler)]
        pub unsafe fn progressHandler(&self) -> *mut Block<dyn Fn(c_double, NonNull<Bool>)>;

        #[method(setProgressHandler:)]
        pub unsafe fn setProgressHandler(
            &self,
            progress_handler: Option<&Block<dyn Fn(c_double, NonNull<Bool>)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Photos_PHContentEditingInputRequestOptions")]
    unsafe impl PHContentEditingInputRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// PHContentEditingInput
    #[cfg(feature = "Photos_PHAsset")]
    unsafe impl PHAsset {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Photos_PHContentEditingInput",
            feature = "Photos_PHContentEditingInputRequestOptions"
        ))]
        #[method(requestContentEditingInputWithOptions:completionHandler:)]
        pub unsafe fn requestContentEditingInputWithOptions_completionHandler(
            &self,
            options: Option<&PHContentEditingInputRequestOptions>,
            completion_handler: &Block<dyn Fn(*mut PHContentEditingInput, NonNull<NSDictionary>)>,
        ) -> PHContentEditingInputRequestID;

        #[method(cancelContentEditingInputRequest:)]
        pub unsafe fn cancelContentEditingInputRequest(
            &self,
            request_id: PHContentEditingInputRequestID,
        );
    }
);

extern_static!(PHContentEditingInputResultIsInCloudKey: &'static NSString);

extern_static!(PHContentEditingInputCancelledKey: &'static NSString);

extern_static!(PHContentEditingInputErrorKey: &'static NSString);

extern_methods!(
    /// PHAssetChangeRequest
    #[cfg(feature = "Photos_PHContentEditingOutput")]
    unsafe impl PHContentEditingOutput {
        #[cfg(feature = "Photos_PHObjectPlaceholder")]
        #[method_id(@__retain_semantics Init initWithPlaceholderForCreatedAsset:)]
        pub unsafe fn initWithPlaceholderForCreatedAsset(
            this: Allocated<Self>,
            placeholder_for_created_asset: &PHObjectPlaceholder,
        ) -> Id<Self>;
    }
);
