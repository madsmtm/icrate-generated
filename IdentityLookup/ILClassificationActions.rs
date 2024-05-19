//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ILClassificationAction(pub NSInteger);
impl ILClassificationAction {
    #[doc(alias = "ILClassificationActionNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "ILClassificationActionReportNotJunk")]
    pub const ReportNotJunk: Self = Self(1);
    #[doc(alias = "ILClassificationActionReportJunk")]
    pub const ReportJunk: Self = Self(2);
    #[doc(alias = "ILClassificationActionReportJunkAndBlockSender")]
    pub const ReportJunkAndBlockSender: Self = Self(3);
}

unsafe impl Encode for ILClassificationAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for ILClassificationAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
