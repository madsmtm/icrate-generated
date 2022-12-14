//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(u32)]
    pub enum NSOpenGLGlobalOption {
        NSOpenGLGOFormatCacheSize = 501,
        NSOpenGLGOClearFormatCache = 502,
        NSOpenGLGORetainRenderers = 503,
        NSOpenGLGOUseBuildCache = 506,
        NSOpenGLGOResetLibrary = 504,
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSOpenGLPFAAllRenderers = 1,
        NSOpenGLPFATripleBuffer = 3,
        NSOpenGLPFADoubleBuffer = 5,
        NSOpenGLPFAAuxBuffers = 7,
        NSOpenGLPFAColorSize = 8,
        NSOpenGLPFAAlphaSize = 11,
        NSOpenGLPFADepthSize = 12,
        NSOpenGLPFAStencilSize = 13,
        NSOpenGLPFAAccumSize = 14,
        NSOpenGLPFAMinimumPolicy = 51,
        NSOpenGLPFAMaximumPolicy = 52,
        NSOpenGLPFASampleBuffers = 55,
        NSOpenGLPFASamples = 56,
        NSOpenGLPFAAuxDepthStencil = 57,
        NSOpenGLPFAColorFloat = 58,
        NSOpenGLPFAMultisample = 59,
        NSOpenGLPFASupersample = 60,
        NSOpenGLPFASampleAlpha = 61,
        NSOpenGLPFARendererID = 70,
        NSOpenGLPFANoRecovery = 72,
        NSOpenGLPFAAccelerated = 73,
        NSOpenGLPFAClosestPolicy = 74,
        NSOpenGLPFABackingStore = 76,
        NSOpenGLPFAScreenMask = 84,
        NSOpenGLPFAAllowOfflineRenderers = 96,
        NSOpenGLPFAAcceleratedCompute = 97,
        NSOpenGLPFAOpenGLProfile = 99,
        NSOpenGLPFAVirtualScreenCount = 128,
        NSOpenGLPFAStereo = 6,
        NSOpenGLPFAOffScreen = 53,
        NSOpenGLPFAFullScreen = 54,
        NSOpenGLPFASingleRenderer = 71,
        NSOpenGLPFARobust = 75,
        NSOpenGLPFAMPSafe = 78,
        NSOpenGLPFAWindow = 80,
        NSOpenGLPFAMultiScreen = 81,
        NSOpenGLPFACompliant = 83,
        NSOpenGLPFAPixelBuffer = 90,
        NSOpenGLPFARemotePixelBuffer = 91,
    }
);

pub type NSOpenGLPixelFormatAttribute = u32;

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSOpenGLProfileVersionLegacy = 0x1000,
        NSOpenGLProfileVersion3_2Core = 0x3200,
        NSOpenGLProfileVersion4_1Core = 0x4100,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSOpenGLContextParameter {
        NSOpenGLContextParameterSwapInterval = 222,
        NSOpenGLContextParameterSurfaceOrder = 235,
        NSOpenGLContextParameterSurfaceOpacity = 236,
        NSOpenGLContextParameterSurfaceBackingSize = 304,
        NSOpenGLContextParameterReclaimResources = 308,
        NSOpenGLContextParameterCurrentRendererID = 309,
        NSOpenGLContextParameterGPUVertexProcessing = 310,
        NSOpenGLContextParameterGPUFragmentProcessing = 311,
        NSOpenGLContextParameterHasDrawable = 314,
        NSOpenGLContextParameterMPSwapsInFlight = 315,
        NSOpenGLContextParameterSwapRectangle = 200,
        NSOpenGLContextParameterSwapRectangleEnable = 201,
        NSOpenGLContextParameterRasterizationEnable = 221,
        NSOpenGLContextParameterStateValidation = 301,
        NSOpenGLContextParameterSurfaceSurfaceVolatile = 306,
    }
);

extern_static!(
    NSOpenGLCPSwapInterval: NSOpenGLContextParameter = NSOpenGLContextParameterSwapInterval
);

extern_static!(
    NSOpenGLCPSurfaceOrder: NSOpenGLContextParameter = NSOpenGLContextParameterSurfaceOrder
);

extern_static!(
    NSOpenGLCPSurfaceOpacity: NSOpenGLContextParameter = NSOpenGLContextParameterSurfaceOpacity
);

extern_static!(
    NSOpenGLCPSurfaceBackingSize: NSOpenGLContextParameter =
        NSOpenGLContextParameterSurfaceBackingSize
);

extern_static!(
    NSOpenGLCPReclaimResources: NSOpenGLContextParameter = NSOpenGLContextParameterReclaimResources
);

extern_static!(
    NSOpenGLCPCurrentRendererID: NSOpenGLContextParameter =
        NSOpenGLContextParameterCurrentRendererID
);

extern_static!(
    NSOpenGLCPGPUVertexProcessing: NSOpenGLContextParameter =
        NSOpenGLContextParameterGPUVertexProcessing
);

extern_static!(
    NSOpenGLCPGPUFragmentProcessing: NSOpenGLContextParameter =
        NSOpenGLContextParameterGPUFragmentProcessing
);

extern_static!(
    NSOpenGLCPHasDrawable: NSOpenGLContextParameter = NSOpenGLContextParameterHasDrawable
);

extern_static!(
    NSOpenGLCPMPSwapsInFlight: NSOpenGLContextParameter = NSOpenGLContextParameterMPSwapsInFlight
);

extern_static!(
    NSOpenGLCPSwapRectangle: NSOpenGLContextParameter = NSOpenGLContextParameterSwapRectangle
);

extern_static!(
    NSOpenGLCPSwapRectangleEnable: NSOpenGLContextParameter =
        NSOpenGLContextParameterSwapRectangleEnable
);

extern_static!(
    NSOpenGLCPRasterizationEnable: NSOpenGLContextParameter =
        NSOpenGLContextParameterRasterizationEnable
);

extern_static!(
    NSOpenGLCPStateValidation: NSOpenGLContextParameter = NSOpenGLContextParameterStateValidation
);

extern_static!(
    NSOpenGLCPSurfaceSurfaceVolatile: NSOpenGLContextParameter =
        NSOpenGLContextParameterSurfaceSurfaceVolatile
);
