//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSOpenGLSurfaceResolution
    unsafe impl NSView {
        #[method(wantsBestResolutionOpenGLSurface)]
        pub unsafe fn wantsBestResolutionOpenGLSurface(&self) -> bool;

        #[method(setWantsBestResolutionOpenGLSurface:)]
        pub unsafe fn setWantsBestResolutionOpenGLSurface(
            &self,
            wantsBestResolutionOpenGLSurface: bool,
        );
    }
);

extern_methods!(
    /// NSExtendedDynamicRange
    unsafe impl NSView {
        #[method(wantsExtendedDynamicRangeOpenGLSurface)]
        pub unsafe fn wantsExtendedDynamicRangeOpenGLSurface(&self) -> bool;

        #[method(setWantsExtendedDynamicRangeOpenGLSurface:)]
        pub unsafe fn setWantsExtendedDynamicRangeOpenGLSurface(
            &self,
            wantsExtendedDynamicRangeOpenGLSurface: bool,
        );
    }
);
