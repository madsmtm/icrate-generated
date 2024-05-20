//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

#[cfg(all(feature = "block2", feature = "objc2-core-image"))]
#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos"
))]
pub type PHLivePhotoFrameProcessingBlock = *mut block2::Block<
    dyn Fn(NonNull<ProtocolObject<dyn PHLivePhotoFrame>>, NonNull<*mut NSError>) -> *mut CIImage,
>;

// NS_TYPED_ENUM
pub type PHLivePhotoEditingOption = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHLivePhotoEditingContext;

    unsafe impl ClassType for PHLivePhotoEditingContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for PHLivePhotoEditingContext {}

extern_methods!(
    unsafe impl PHLivePhotoEditingContext {
        #[cfg(feature = "PHContentEditingInput")]
        #[method_id(@__retain_semantics Init initWithLivePhotoEditingInput:)]
        pub unsafe fn initWithLivePhotoEditingInput(
            this: Allocated<Self>,
            live_photo_input: &PHContentEditingInput,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "objc2-core-image")]
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        #[method_id(@__retain_semantics Other fullSizeImage)]
        pub unsafe fn fullSizeImage(&self) -> Id<CIImage>;

        #[cfg(all(feature = "block2", feature = "objc2-core-image"))]
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        #[method(frameProcessor)]
        pub unsafe fn frameProcessor(&self) -> PHLivePhotoFrameProcessingBlock;

        #[cfg(all(feature = "block2", feature = "objc2-core-image"))]
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        #[method(setFrameProcessor:)]
        pub unsafe fn setFrameProcessor(&self, frame_processor: PHLivePhotoFrameProcessingBlock);

        #[method(audioVolume)]
        pub unsafe fn audioVolume(&self) -> c_float;

        #[method(setAudioVolume:)]
        pub unsafe fn setAudioVolume(&self, audio_volume: c_float);

        #[cfg(all(feature = "PHLivePhoto", feature = "block2"))]
        #[method(prepareLivePhotoForPlaybackWithTargetSize:options:completionHandler:)]
        pub unsafe fn prepareLivePhotoForPlaybackWithTargetSize_options_completionHandler(
            &self,
            target_size: CGSize,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            handler: &block2::Block<dyn Fn(*mut PHLivePhoto, *mut NSError)>,
        );

        #[cfg(all(feature = "PHContentEditingOutput", feature = "block2"))]
        #[method(saveLivePhotoToOutput:options:completionHandler:)]
        pub unsafe fn saveLivePhotoToOutput_options_completionHandler(
            &self,
            output: &PHContentEditingOutput,
            options: Option<&NSDictionary<NSString, AnyObject>>,
            handler: &block2::Block<dyn Fn(Bool, *mut NSError)>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHLivePhotoEditingContext {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHLivePhotoFrameType(pub NSInteger);
impl PHLivePhotoFrameType {
    #[doc(alias = "PHLivePhotoFrameTypePhoto")]
    pub const Photo: Self = Self(0);
    #[doc(alias = "PHLivePhotoFrameTypeVideo")]
    pub const Video: Self = Self(1);
}

unsafe impl Encode for PHLivePhotoFrameType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHLivePhotoFrameType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait PHLivePhotoFrame {
        #[cfg(feature = "objc2-core-image")]
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "visionos"
        ))]
        #[method_id(@__retain_semantics Other image)]
        unsafe fn image(&self) -> Id<CIImage>;

        #[method(type)]
        unsafe fn r#type(&self) -> PHLivePhotoFrameType;

        #[method(renderScale)]
        unsafe fn renderScale(&self) -> CGFloat;
    }

    unsafe impl ProtocolType for dyn PHLivePhotoFrame {}
);

extern "C" {
    pub static PHLivePhotoShouldRenderAtPlaybackTime: &'static PHLivePhotoEditingOption;
}

extern "C" {
    pub static PHLivePhotoEditingErrorDomain: &'static NSString;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHLivePhotoEditingErrorCode(pub NSInteger);
impl PHLivePhotoEditingErrorCode {
    #[deprecated]
    #[doc(alias = "PHLivePhotoEditingErrorCodeUnknown")]
    pub const Unknown: Self = Self(0);
    #[deprecated]
    #[doc(alias = "PHLivePhotoEditingErrorCodeAborted")]
    pub const Aborted: Self = Self(1);
}

unsafe impl Encode for PHLivePhotoEditingErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHLivePhotoEditingErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
