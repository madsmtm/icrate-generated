//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechRecognitionResult;

    unsafe impl ClassType for SFSpeechRecognitionResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for SFSpeechRecognitionResult {}

unsafe impl NSCopying for SFSpeechRecognitionResult {}

unsafe impl NSObjectProtocol for SFSpeechRecognitionResult {}

unsafe impl NSSecureCoding for SFSpeechRecognitionResult {}

extern_methods!(
    unsafe impl SFSpeechRecognitionResult {
        #[cfg(feature = "Speech_SFTranscription")]
        #[method_id(@__retain_semantics Other bestTranscription)]
        pub unsafe fn bestTranscription(&self) -> Id<SFTranscription>;

        #[cfg(feature = "Speech_SFTranscription")]
        #[method_id(@__retain_semantics Other transcriptions)]
        pub unsafe fn transcriptions(&self) -> Id<NSArray<SFTranscription>>;

        #[method(isFinal)]
        pub unsafe fn isFinal(&self) -> bool;

        #[cfg(feature = "Speech_SFSpeechRecognitionMetadata")]
        #[method_id(@__retain_semantics Other speechRecognitionMetadata)]
        pub unsafe fn speechRecognitionMetadata(&self) -> Option<Id<SFSpeechRecognitionMetadata>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechRecognitionResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
