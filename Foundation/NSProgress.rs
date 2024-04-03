//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSProgressKind = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSProgressUserInfoKey = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "Foundation_NSString")]
pub type NSProgressFileOperationKind = NSString;

#[cfg(feature = "block2")]
pub type NSProgressUnpublishingHandler = *mut Block<dyn Fn()>;

#[cfg(feature = "block2")]
pub type NSProgressPublishingHandler =
    *mut Block<dyn Fn(NonNull<NSProgress>) -> NSProgressUnpublishingHandler>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSProgress;

    unsafe impl ClassType for NSProgress {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl Send for NSProgress {}

unsafe impl Sync for NSProgress {}

unsafe impl NSObjectProtocol for NSProgress {}

extern_methods!(
    unsafe impl NSProgress {
        #[method_id(@__retain_semantics Other currentProgress)]
        pub unsafe fn currentProgress() -> Option<Id<NSProgress>>;

        #[method_id(@__retain_semantics Other progressWithTotalUnitCount:)]
        pub unsafe fn progressWithTotalUnitCount(unit_count: i64) -> Id<NSProgress>;

        #[method_id(@__retain_semantics Other discreteProgressWithTotalUnitCount:)]
        pub unsafe fn discreteProgressWithTotalUnitCount(unit_count: i64) -> Id<NSProgress>;

        #[method_id(@__retain_semantics Other progressWithTotalUnitCount:parent:pendingUnitCount:)]
        pub unsafe fn progressWithTotalUnitCount_parent_pendingUnitCount(
            unit_count: i64,
            parent: &NSProgress,
            portion_of_parent_total_unit_count: i64,
        ) -> Id<NSProgress>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithParent:userInfo:)]
        pub unsafe fn initWithParent_userInfo(
            this: Allocated<Self>,
            parent_progress_or_nil: Option<&NSProgress>,
            user_info_or_nil: Option<&NSDictionary<NSProgressUserInfoKey, AnyObject>>,
        ) -> Id<Self>;

        #[method(becomeCurrentWithPendingUnitCount:)]
        pub unsafe fn becomeCurrentWithPendingUnitCount(&self, unit_count: i64);

        #[cfg(feature = "block2")]
        #[method(performAsCurrentWithPendingUnitCount:usingBlock:)]
        pub unsafe fn performAsCurrentWithPendingUnitCount_usingBlock(
            &self,
            unit_count: i64,
            work: &Block<dyn Fn() + '_>,
        );

        #[method(resignCurrent)]
        pub unsafe fn resignCurrent(&self);

        #[method(addChild:withPendingUnitCount:)]
        pub unsafe fn addChild_withPendingUnitCount(&self, child: &NSProgress, in_unit_count: i64);

        #[method(totalUnitCount)]
        pub unsafe fn totalUnitCount(&self) -> i64;

        #[method(setTotalUnitCount:)]
        pub unsafe fn setTotalUnitCount(&self, total_unit_count: i64);

        #[method(completedUnitCount)]
        pub unsafe fn completedUnitCount(&self) -> i64;

        #[method(setCompletedUnitCount:)]
        pub unsafe fn setCompletedUnitCount(&self, completed_unit_count: i64);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedDescription:)]
        pub unsafe fn setLocalizedDescription(&self, localized_description: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedAdditionalDescription)]
        pub unsafe fn localizedAdditionalDescription(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedAdditionalDescription:)]
        pub unsafe fn setLocalizedAdditionalDescription(
            &self,
            localized_additional_description: Option<&NSString>,
        );

        #[method(isCancellable)]
        pub unsafe fn isCancellable(&self) -> bool;

        #[method(setCancellable:)]
        pub unsafe fn setCancellable(&self, cancellable: bool);

        #[method(isPausable)]
        pub unsafe fn isPausable(&self) -> bool;

        #[method(setPausable:)]
        pub unsafe fn setPausable(&self, pausable: bool);

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(cancellationHandler)]
        pub unsafe fn cancellationHandler(&self) -> *mut Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setCancellationHandler:)]
        pub unsafe fn setCancellationHandler(&self, cancellation_handler: Option<&Block<dyn Fn()>>);

        #[cfg(feature = "block2")]
        #[method(pausingHandler)]
        pub unsafe fn pausingHandler(&self) -> *mut Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setPausingHandler:)]
        pub unsafe fn setPausingHandler(&self, pausing_handler: Option<&Block<dyn Fn()>>);

        #[cfg(feature = "block2")]
        #[method(resumingHandler)]
        pub unsafe fn resumingHandler(&self) -> *mut Block<dyn Fn()>;

        #[cfg(feature = "block2")]
        #[method(setResumingHandler:)]
        pub unsafe fn setResumingHandler(&self, resuming_handler: Option<&Block<dyn Fn()>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUserInfoObject:forKey:)]
        pub unsafe fn setUserInfoObject_forKey(
            &self,
            object_or_nil: Option<&AnyObject>,
            key: &NSProgressUserInfoKey,
        );

        #[method(isIndeterminate)]
        pub unsafe fn isIndeterminate(&self) -> bool;

        #[method(fractionCompleted)]
        pub unsafe fn fractionCompleted(&self) -> c_double;

        #[method(isFinished)]
        pub unsafe fn isFinished(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(pause)]
        pub unsafe fn pause(&self);

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary<NSProgressUserInfoKey, AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other kind)]
        pub unsafe fn kind(&self) -> Option<Id<NSProgressKind>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKind:)]
        pub unsafe fn setKind(&self, kind: Option<&NSProgressKind>);

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other estimatedTimeRemaining)]
        pub unsafe fn estimatedTimeRemaining(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method(setEstimatedTimeRemaining:)]
        pub unsafe fn setEstimatedTimeRemaining(&self, estimated_time_remaining: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other throughput)]
        pub unsafe fn throughput(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method(setThroughput:)]
        pub unsafe fn setThroughput(&self, throughput: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileOperationKind)]
        pub unsafe fn fileOperationKind(&self) -> Option<Id<NSProgressFileOperationKind>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileOperationKind:)]
        pub unsafe fn setFileOperationKind(
            &self,
            file_operation_kind: Option<&NSProgressFileOperationKind>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setFileURL:)]
        pub unsafe fn setFileURL(&self, file_url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other fileTotalCount)]
        pub unsafe fn fileTotalCount(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method(setFileTotalCount:)]
        pub unsafe fn setFileTotalCount(&self, file_total_count: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSValue")]
        #[method_id(@__retain_semantics Other fileCompletedCount)]
        pub unsafe fn fileCompletedCount(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSValue")]
        #[method(setFileCompletedCount:)]
        pub unsafe fn setFileCompletedCount(&self, file_completed_count: Option<&NSNumber>);

        #[method(publish)]
        pub unsafe fn publish(&self);

        #[method(unpublish)]
        pub unsafe fn unpublish(&self);

        #[cfg(all(feature = "Foundation_NSURL", feature = "block2"))]
        #[method_id(@__retain_semantics Other addSubscriberForFileURL:withPublishingHandler:)]
        pub unsafe fn addSubscriberForFileURL_withPublishingHandler(
            url: &NSURL,
            publishing_handler: NSProgressPublishingHandler,
        ) -> Id<AnyObject>;

        #[method(removeSubscriber:)]
        pub unsafe fn removeSubscriber(subscriber: &AnyObject);

        #[method(isOld)]
        pub unsafe fn isOld(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSProgress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSProgressReporting: NSObjectProtocol {
        #[method_id(@__retain_semantics Other progress)]
        unsafe fn progress(&self) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSProgressReporting {}
);

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressEstimatedTimeRemainingKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressThroughputKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressKindFile: &'static NSProgressKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindDownloading: &'static NSProgressFileOperationKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindDecompressingAfterDownloading:
        &'static NSProgressFileOperationKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindReceiving: &'static NSProgressFileOperationKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindCopying: &'static NSProgressFileOperationKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindUploading: &'static NSProgressFileOperationKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileOperationKindDuplicating: &'static NSProgressFileOperationKind;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileURLKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileTotalCountKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileCompletedCountKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileAnimationImageKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileAnimationImageOriginalRectKey: &'static NSProgressUserInfoKey;
}

extern "C" {
    #[cfg(feature = "Foundation_NSString")]
    pub static NSProgressFileIconKey: &'static NSProgressUserInfoKey;
}
