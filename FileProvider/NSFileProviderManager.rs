//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderDomainRemovalMode(pub NSInteger);
impl NSFileProviderDomainRemovalMode {
    #[doc(alias = "NSFileProviderDomainRemovalModeRemoveAll")]
    pub const RemoveAll: Self = Self(0);
    #[doc(alias = "NSFileProviderDomainRemovalModePreserveDirtyUserData")]
    pub const PreserveDirtyUserData: Self = Self(1);
    #[doc(alias = "NSFileProviderDomainRemovalModePreserveDownloadedUserData")]
    pub const PreserveDownloadedUserData: Self = Self(2);
}

unsafe impl Encode for NSFileProviderDomainRemovalMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderDomainRemovalMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFileProviderManager;

    unsafe impl ClassType for NSFileProviderManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSFileProviderManager {}

extern_methods!(
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other defaultManager)]
        pub unsafe fn defaultManager() -> Retained<NSFileProviderManager>;

        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other managerForDomain:)]
        pub unsafe fn managerForDomain(domain: &NSFileProviderDomain) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(signalEnumeratorForContainerItemIdentifier:completionHandler:)]
        pub unsafe fn signalEnumeratorForContainerItemIdentifier_completionHandler(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(getUserVisibleURLForItemIdentifier:completionHandler:)]
        pub unsafe fn getUserVisibleURLForItemIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &block2::Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );

        #[cfg(all(
            feature = "NSFileProviderDomain",
            feature = "NSFileProviderItem",
            feature = "block2"
        ))]
        #[method(getIdentifierForUserVisibleFileAtURL:completionHandler:)]
        pub unsafe fn getIdentifierForUserVisibleFileAtURL_completionHandler(
            url: &NSURL,
            completion_handler: &block2::Block<
                dyn Fn(
                    *mut NSFileProviderItemIdentifier,
                    *mut NSFileProviderDomainIdentifier,
                    *mut NSError,
                ),
            >,
        );

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(registerURLSessionTask:forItemWithIdentifier:completionHandler:)]
        pub unsafe fn registerURLSessionTask_forItemWithIdentifier_completionHandler(
            &self,
            task: &NSURLSessionTask,
            identifier: &NSFileProviderItemIdentifier,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other providerIdentifier)]
        pub unsafe fn providerIdentifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other documentStorageURL)]
        pub unsafe fn documentStorageURL(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other temporaryDirectoryURLWithError:_)]
        pub unsafe fn temporaryDirectoryURLWithError(
            &self,
        ) -> Result<Retained<NSURL>, Retained<NSError>>;

        #[cfg(feature = "NSFileProviderItem")]
        #[method(writePlaceholderAtURL:withMetadata:error:_)]
        pub unsafe fn writePlaceholderAtURL_withMetadata_error(
            placeholder_url: &NSURL,
            metadata: &NSFileProviderItem,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other placeholderURLForURL:)]
        pub unsafe fn placeholderURLForURL(url: &NSURL) -> Retained<NSURL>;

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(addDomain:completionHandler:)]
        pub unsafe fn addDomain_completionHandler(
            domain: &NSFileProviderDomain,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(removeDomain:completionHandler:)]
        pub unsafe fn removeDomain_completionHandler(
            domain: &NSFileProviderDomain,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(removeDomain:mode:completionHandler:)]
        pub unsafe fn removeDomain_mode_completionHandler(
            domain: &NSFileProviderDomain,
            mode: NSFileProviderDomainRemovalMode,
            completion_handler: &block2::Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(getDomainsWithCompletionHandler:)]
        pub unsafe fn getDomainsWithCompletionHandler(
            completion_handler: &block2::Block<
                dyn Fn(NonNull<NSArray<NSFileProviderDomain>>, *mut NSError),
            >,
        );

        #[cfg(feature = "block2")]
        #[method(removeAllDomainsWithCompletionHandler:)]
        pub unsafe fn removeAllDomainsWithCompletionHandler(
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(signalErrorResolved:completionHandler:)]
        pub unsafe fn signalErrorResolved_completionHandler(
            &self,
            error: &NSError,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[method_id(@__retain_semantics Other globalProgressForKind:)]
        pub unsafe fn globalProgressForKind(
            &self,
            kind: &NSProgressFileOperationKind,
        ) -> Retained<NSProgress>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    pub static NSFileProviderMaterializedSetDidChange: &'static NSNotificationName;
}

extern_methods!(
    /// MaterializedSet
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "NSFileProviderEnumerating")]
        #[method_id(@__retain_semantics Other enumeratorForMaterializedItems)]
        pub unsafe fn enumeratorForMaterializedItems(
            &self,
        ) -> Retained<ProtocolObject<dyn NSFileProviderEnumerator>>;
    }
);

extern "C" {
    pub static NSFileProviderPendingSetDidChange: &'static NSNotificationName;
}

extern_protocol!(
    #[cfg(feature = "NSFileProviderEnumerating")]
    pub unsafe trait NSFileProviderPendingSetEnumerator: NSFileProviderEnumerator {
        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Option<Retained<NSFileProviderDomainVersion>>;

        #[method(refreshInterval)]
        unsafe fn refreshInterval(&self) -> NSTimeInterval;

        #[method(isMaximumSizeReached)]
        unsafe fn isMaximumSizeReached(&self) -> bool;
    }

    #[cfg(feature = "NSFileProviderEnumerating")]
    unsafe impl ProtocolType for dyn NSFileProviderPendingSetEnumerator {}
);

extern_methods!(
    /// PendingSet
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "NSFileProviderEnumerating")]
        #[method_id(@__retain_semantics Other enumeratorForPendingItems)]
        pub unsafe fn enumeratorForPendingItems(
            &self,
        ) -> Retained<ProtocolObject<dyn NSFileProviderPendingSetEnumerator>>;
    }
);

extern_methods!(
    /// Import
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderDomain", feature = "block2"))]
        #[method(importDomain:fromDirectoryAtURL:completionHandler:)]
        pub unsafe fn importDomain_fromDirectoryAtURL_completionHandler(
            domain: &NSFileProviderDomain,
            url: &NSURL,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(reimportItemsBelowItemWithIdentifier:completionHandler:)]
        pub unsafe fn reimportItemsBelowItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderModifyItemOptions",
            feature = "block2"
        ))]
        #[method(requestModificationOfFields:forItemWithIdentifier:options:completionHandler:)]
        pub unsafe fn requestModificationOfFields_forItemWithIdentifier_options_completionHandler(
            &self,
            fields: NSFileProviderItemFields,
            item_identifier: &NSFileProviderItemIdentifier,
            options: NSFileProviderModifyItemOptions,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Eviction
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(evictItemWithIdentifier:completionHandler:)]
        pub unsafe fn evictItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Barrier
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(waitForChangesOnItemsBelowItemWithIdentifier:completionHandler:)]
        pub unsafe fn waitForChangesOnItemsBelowItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Stabilization
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "block2")]
        #[method(waitForStabilizationWithCompletionHandler:)]
        pub unsafe fn waitForStabilizationWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderManagerDisconnectionOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileProviderManagerDisconnectionOptions: NSUInteger {
        #[doc(alias = "NSFileProviderManagerDisconnectionOptionsTemporary")]
        const Temporary = 1<<0;
    }
}

unsafe impl Encode for NSFileProviderManagerDisconnectionOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderManagerDisconnectionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// Disconnection
    unsafe impl NSFileProviderManager {
        #[cfg(feature = "block2")]
        #[method(disconnectWithReason:options:completionHandler:)]
        pub unsafe fn disconnectWithReason_options_completionHandler(
            &self,
            localized_reason: &NSString,
            options: NSFileProviderManagerDisconnectionOptions,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(reconnectWithCompletionHandler:)]
        pub unsafe fn reconnectWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Materialize
    unsafe impl NSFileProviderManager {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method(requestDownloadForItemWithIdentifier:requestedRange:completionHandler:)]
        pub unsafe fn requestDownloadForItemWithIdentifier_requestedRange_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            range_to_materialize: NSRange,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// StateDirectory
    unsafe impl NSFileProviderManager {
        #[method_id(@__retain_semantics Other stateDirectoryURLWithError:_)]
        pub unsafe fn stateDirectoryURLWithError(
            &self,
        ) -> Result<Retained<NSURL>, Retained<NSError>>;
    }
);

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderVolumeUnsupportedReason(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileProviderVolumeUnsupportedReason: NSUInteger {
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonNone")]
        const None = 0;
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonUnknown")]
        const Unknown = 1<<0;
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonNonAPFS")]
        const NonAPFS = 1<<1;
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonNonEncrypted")]
        const NonEncrypted = 1<<2;
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonReadOnly")]
        const ReadOnly = 1<<3;
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonNetwork")]
        const Network = 1<<4;
        #[doc(alias = "NSFileProviderVolumeUnsupportedReasonQuarantined")]
        const Quarantined = 1<<5;
    }
}

unsafe impl Encode for NSFileProviderVolumeUnsupportedReason {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderVolumeUnsupportedReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// ExternalDomain
    unsafe impl NSFileProviderManager {
        #[method(checkDomainsCanBeStored:onVolumeAtURL:unsupportedReason:error:_)]
        pub unsafe fn checkDomainsCanBeStored_onVolumeAtURL_unsupportedReason_error(
            eligible: NonNull<Bool>,
            url: &NSURL,
            unsupported_reason: *mut NSFileProviderVolumeUnsupportedReason,
        ) -> Result<(), Retained<NSError>>;
    }
);
