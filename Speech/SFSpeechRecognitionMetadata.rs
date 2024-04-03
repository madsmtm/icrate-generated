//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct SFSpeechRecognitionMetadata;

    unsafe impl ClassType for SFSpeechRecognitionMetadata {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCoding for SFSpeechRecognitionMetadata {}

unsafe impl NSCopying for SFSpeechRecognitionMetadata {}

unsafe impl NSObjectProtocol for SFSpeechRecognitionMetadata {}

unsafe impl NSSecureCoding for SFSpeechRecognitionMetadata {}

extern_methods!(
    unsafe impl SFSpeechRecognitionMetadata {
        #[method(speakingRate)]
        pub unsafe fn speakingRate(&self) -> c_double;

        #[method(averagePauseDuration)]
        pub unsafe fn averagePauseDuration(&self) -> NSTimeInterval;

        #[method(speechStartTimestamp)]
        pub unsafe fn speechStartTimestamp(&self) -> NSTimeInterval;

        #[method(speechDuration)]
        pub unsafe fn speechDuration(&self) -> NSTimeInterval;

        #[cfg(feature = "Speech_SFVoiceAnalytics")]
        #[method_id(@__retain_semantics Other voiceAnalytics)]
        pub unsafe fn voiceAnalytics(&self) -> Option<Id<SFVoiceAnalytics>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SFSpeechRecognitionMetadata {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
