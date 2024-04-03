//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSOpenGLGlobalOption(pub u32);
impl NSOpenGLGlobalOption {
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub const NSOpenGLGOFormatCacheSize: Self = Self(501);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub const NSOpenGLGOClearFormatCache: Self = Self(502);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub const NSOpenGLGORetainRenderers: Self = Self(503);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub const NSOpenGLGOUseBuildCache: Self = Self(506);
    #[deprecated]
    pub const NSOpenGLGOResetLibrary: Self = Self(504);
}

unsafe impl Encode for NSOpenGLGlobalOption {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for NSOpenGLGlobalOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAllRenderers: c_uint = 1;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFATripleBuffer: c_uint = 3;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFADoubleBuffer: c_uint = 5;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAuxBuffers: c_uint = 7;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAColorSize: c_uint = 8;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAlphaSize: c_uint = 11;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFADepthSize: c_uint = 12;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAStencilSize: c_uint = 13;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAccumSize: c_uint = 14;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAMinimumPolicy: c_uint = 51;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAMaximumPolicy: c_uint = 52;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASampleBuffers: c_uint = 55;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASamples: c_uint = 56;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAuxDepthStencil: c_uint = 57;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAColorFloat: c_uint = 58;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAMultisample: c_uint = 59;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASupersample: c_uint = 60;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASampleAlpha: c_uint = 61;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFARendererID: c_uint = 70;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFANoRecovery: c_uint = 72;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAccelerated: c_uint = 73;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAClosestPolicy: c_uint = 74;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFABackingStore: c_uint = 76;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAScreenMask: c_uint = 84;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAllowOfflineRenderers: c_uint = 96;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAcceleratedCompute: c_uint = 97;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAOpenGLProfile: c_uint = 99;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAVirtualScreenCount: c_uint = 128;
#[deprecated]
pub const NSOpenGLPFAStereo: c_uint = 6;
#[deprecated]
pub const NSOpenGLPFAOffScreen: c_uint = 53;
#[deprecated]
pub const NSOpenGLPFAFullScreen: c_uint = 54;
#[deprecated]
pub const NSOpenGLPFASingleRenderer: c_uint = 71;
#[deprecated]
pub const NSOpenGLPFARobust: c_uint = 75;
#[deprecated]
pub const NSOpenGLPFAMPSafe: c_uint = 78;
#[deprecated]
pub const NSOpenGLPFAWindow: c_uint = 80;
#[deprecated]
pub const NSOpenGLPFAMultiScreen: c_uint = 81;
#[deprecated]
pub const NSOpenGLPFACompliant: c_uint = 83;
#[deprecated]
pub const NSOpenGLPFAPixelBuffer: c_uint = 90;
#[deprecated]
pub const NSOpenGLPFARemotePixelBuffer: c_uint = 91;

pub type NSOpenGLPixelFormatAttribute = u32;

#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLProfileVersionLegacy: c_uint = 0x1000;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLProfileVersion3_2Core: c_uint = 0x3200;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLProfileVersion4_1Core: c_uint = 0x4100;

// NS_ENUM
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSOpenGLContextParameter(pub NSInteger);
impl NSOpenGLContextParameter {
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterSwapInterval")]
    pub const SwapInterval: Self = Self(222);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterSurfaceOrder")]
    pub const SurfaceOrder: Self = Self(235);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterSurfaceOpacity")]
    pub const SurfaceOpacity: Self = Self(236);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterSurfaceBackingSize")]
    pub const SurfaceBackingSize: Self = Self(304);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterReclaimResources")]
    pub const ReclaimResources: Self = Self(308);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterCurrentRendererID")]
    pub const CurrentRendererID: Self = Self(309);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterGPUVertexProcessing")]
    pub const GPUVertexProcessing: Self = Self(310);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterGPUFragmentProcessing")]
    pub const GPUFragmentProcessing: Self = Self(311);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterHasDrawable")]
    pub const HasDrawable: Self = Self(314);
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    #[doc(alias = "NSOpenGLContextParameterMPSwapsInFlight")]
    pub const MPSwapsInFlight: Self = Self(315);
    #[deprecated]
    #[doc(alias = "NSOpenGLContextParameterSwapRectangle")]
    pub const SwapRectangle: Self = Self(200);
    #[deprecated]
    #[doc(alias = "NSOpenGLContextParameterSwapRectangleEnable")]
    pub const SwapRectangleEnable: Self = Self(201);
    #[deprecated]
    #[doc(alias = "NSOpenGLContextParameterRasterizationEnable")]
    pub const RasterizationEnable: Self = Self(221);
    #[deprecated]
    #[doc(alias = "NSOpenGLContextParameterStateValidation")]
    pub const StateValidation: Self = Self(301);
    #[deprecated]
    #[doc(alias = "NSOpenGLContextParameterSurfaceSurfaceVolatile")]
    pub const SurfaceSurfaceVolatile: Self = Self(306);
}

unsafe impl Encode for NSOpenGLContextParameter {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSOpenGLContextParameter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub static NSOpenGLCPSwapInterval: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SwapInterval.0);

pub static NSOpenGLCPSurfaceOrder: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SurfaceOrder.0);

pub static NSOpenGLCPSurfaceOpacity: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SurfaceOpacity.0);

pub static NSOpenGLCPSurfaceBackingSize: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SurfaceBackingSize.0);

pub static NSOpenGLCPReclaimResources: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::ReclaimResources.0);

pub static NSOpenGLCPCurrentRendererID: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::CurrentRendererID.0);

pub static NSOpenGLCPGPUVertexProcessing: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::GPUVertexProcessing.0);

pub static NSOpenGLCPGPUFragmentProcessing: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::GPUFragmentProcessing.0);

pub static NSOpenGLCPHasDrawable: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::HasDrawable.0);

pub static NSOpenGLCPMPSwapsInFlight: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::MPSwapsInFlight.0);

pub static NSOpenGLCPSwapRectangle: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SwapRectangle.0);

pub static NSOpenGLCPSwapRectangleEnable: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SwapRectangleEnable.0);

pub static NSOpenGLCPRasterizationEnable: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::RasterizationEnable.0);

pub static NSOpenGLCPStateValidation: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::StateValidation.0);

pub static NSOpenGLCPSurfaceSurfaceVolatile: NSOpenGLContextParameter =
    NSOpenGLContextParameter(NSOpenGLContextParameter::SurfaceSurfaceVolatile.0);
