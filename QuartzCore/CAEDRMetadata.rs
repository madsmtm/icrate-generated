//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAEDRMetadata;

    unsafe impl ClassType for CAEDRMetadata {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl CAEDRMetadata {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other HDR10MetadataWithDisplayInfo:contentInfo:opticalOutputScale:)]
        pub unsafe fn HDR10MetadataWithDisplayInfo_contentInfo_opticalOutputScale(
            displayData: Option<&NSData>,
            contentData: Option<&NSData>,
            scale: c_float,
        ) -> Id<CAEDRMetadata, Shared>;

        #[method_id(@__retain_semantics Other HDR10MetadataWithMinLuminance:maxLuminance:opticalOutputScale:)]
        pub unsafe fn HDR10MetadataWithMinLuminance_maxLuminance_opticalOutputScale(
            minNits: c_float,
            maxNits: c_float,
            scale: c_float,
        ) -> Id<CAEDRMetadata, Shared>;

        #[method_id(@__retain_semantics Other HLGMetadata)]
        pub unsafe fn HLGMetadata() -> Id<CAEDRMetadata, Shared>;
    }
);
