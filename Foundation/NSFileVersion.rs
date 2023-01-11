//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileVersionAddingOptions {
        NSFileVersionAddingByMoving = 1 << 0,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileVersionReplacingOptions {
        NSFileVersionReplacingByMoving = 1 << 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFileVersion")]
    pub struct NSFileVersion;

    #[cfg(feature = "Foundation_NSFileVersion")]
    unsafe impl ClassType for NSFileVersion {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSFileVersion")]
    unsafe impl NSFileVersion {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other currentVersionOfItemAtURL:)]
        pub unsafe fn currentVersionOfItemAtURL(url: &NSURL) -> Option<Id<NSFileVersion, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other otherVersionsOfItemAtURL:)]
        pub unsafe fn otherVersionsOfItemAtURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSFileVersion>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other unresolvedConflictVersionsOfItemAtURL:)]
        pub unsafe fn unresolvedConflictVersionsOfItemAtURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSFileVersion>, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(getNonlocalVersionsOfItemAtURL:completionHandler:)]
        pub unsafe fn getNonlocalVersionsOfItemAtURL_completionHandler(
            url: &NSURL,
            completionHandler: &Block<(*mut NSArray<NSFileVersion>, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other versionOfItemAtURL:forPersistentIdentifier:)]
        pub unsafe fn versionOfItemAtURL_forPersistentIdentifier(
            url: &NSURL,
            persistentIdentifier: &Object,
        ) -> Option<Id<NSFileVersion, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other addVersionOfItemAtURL:withContentsOfURL:options:error:_)]
        pub unsafe fn addVersionOfItemAtURL_withContentsOfURL_options_error(
            url: &NSURL,
            contentsURL: &NSURL,
            options: NSFileVersionAddingOptions,
        ) -> Result<Id<NSFileVersion, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other temporaryDirectoryURLForNewVersionOfItemAtURL:)]
        pub unsafe fn temporaryDirectoryURLForNewVersionOfItemAtURL(
            url: &NSURL,
        ) -> Id<NSURL, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedNameOfSavingComputer)]
        pub unsafe fn localizedNameOfSavingComputer(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSPersonNameComponents")]
        #[method_id(@__retain_semantics Other originatorNameComponents)]
        pub unsafe fn originatorNameComponents(&self)
            -> Option<Id<NSPersonNameComponents, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method_id(@__retain_semantics Other persistentIdentifier)]
        pub unsafe fn persistentIdentifier(&self) -> Id<NSCoding, Shared>;

        #[method(isConflict)]
        pub unsafe fn isConflict(&self) -> bool;

        #[method(isResolved)]
        pub unsafe fn isResolved(&self) -> bool;

        #[method(setResolved:)]
        pub unsafe fn setResolved(&self, resolved: bool);

        #[method(isDiscardable)]
        pub unsafe fn isDiscardable(&self) -> bool;

        #[method(setDiscardable:)]
        pub unsafe fn setDiscardable(&self, discardable: bool);

        #[method(hasLocalContents)]
        pub unsafe fn hasLocalContents(&self) -> bool;

        #[method(hasThumbnail)]
        pub unsafe fn hasThumbnail(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other replaceItemAtURL:options:error:_)]
        pub unsafe fn replaceItemAtURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileVersionReplacingOptions,
        ) -> Result<Id<NSURL, Shared>, Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(removeAndReturnError:_)]
        pub unsafe fn removeAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(removeOtherVersionsOfItemAtURL:error:_)]
        pub unsafe fn removeOtherVersionsOfItemAtURL_error(
            url: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);
