//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    pub static SFSpeechErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SFSpeechErrorCode(pub NSInteger);
impl SFSpeechErrorCode {
    #[doc(alias = "SFSpeechErrorCodeInternalServiceError")]
    pub const InternalServiceError: Self = Self(1);
    #[doc(alias = "SFSpeechErrorCodeAudioReadFailed")]
    pub const AudioReadFailed: Self = Self(2);
    #[doc(alias = "SFSpeechErrorCodeUndefinedTemplateClassName")]
    pub const UndefinedTemplateClassName: Self = Self(7);
    #[doc(alias = "SFSpeechErrorCodeMalformedSupplementalModel")]
    pub const MalformedSupplementalModel: Self = Self(8);
}

unsafe impl Encode for SFSpeechErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for SFSpeechErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
