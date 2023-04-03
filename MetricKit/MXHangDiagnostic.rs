//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXHangDiagnostic")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MXHangDiagnostic;

    #[cfg(feature = "MetricKit_MXHangDiagnostic")]
    unsafe impl ClassType for MXHangDiagnostic {
        #[inherits(NSObject)]
        type Super = MXDiagnostic;
    }
);

#[cfg(feature = "MetricKit_MXHangDiagnostic")]
unsafe impl NSCoding for MXHangDiagnostic {}

#[cfg(feature = "MetricKit_MXHangDiagnostic")]
unsafe impl NSObjectProtocol for MXHangDiagnostic {}

#[cfg(feature = "MetricKit_MXHangDiagnostic")]
unsafe impl NSSecureCoding for MXHangDiagnostic {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXHangDiagnostic")]
    unsafe impl MXHangDiagnostic {
        #[cfg(feature = "MetricKit_MXCallStackTree")]
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Id<MXCallStackTree>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other hangDuration)]
        pub unsafe fn hangDuration(&self) -> Id<NSMeasurement<NSUnitDuration>>;
    }
);