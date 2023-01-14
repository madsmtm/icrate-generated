//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileWrapperReadingOptions {
        NSFileWrapperReadingImmediate = 1 << 0,
        NSFileWrapperReadingWithoutMapping = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileWrapperWritingOptions {
        NSFileWrapperWritingAtomic = 1 << 0,
        NSFileWrapperWritingWithNameUpdating = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSFileWrapper")]
    pub struct NSFileWrapper;

    #[cfg(feature = "Foundation_NSFileWrapper")]
    unsafe impl ClassType for NSFileWrapper {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSFileWrapper")]
    unsafe impl NSFileWrapper {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:options:error:_)]
        pub unsafe fn initWithURL_options_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initDirectoryWithFileWrappers:)]
        pub unsafe fn initDirectoryWithFileWrappers(
            this: Option<Allocated<Self>>,
            children_by_preferred_name: &NSDictionary<NSString, NSFileWrapper>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initRegularFileWithContents:)]
        pub unsafe fn initRegularFileWithContents(
            this: Option<Allocated<Self>>,
            contents: &NSData,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initSymbolicLinkWithDestinationURL:)]
        pub unsafe fn initSymbolicLinkWithDestinationURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithSerializedRepresentation:)]
        pub unsafe fn initWithSerializedRepresentation(
            this: Option<Allocated<Self>>,
            serialize_representation: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            in_coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(isDirectory)]
        pub unsafe fn isDirectory(&self) -> bool;

        #[method(isRegularFile)]
        pub unsafe fn isRegularFile(&self) -> bool;

        #[method(isSymbolicLink)]
        pub unsafe fn isSymbolicLink(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other preferredFilename)]
        pub unsafe fn preferredFilename(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPreferredFilename:)]
        pub unsafe fn setPreferredFilename(&self, preferred_filename: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other filename)]
        pub unsafe fn filename(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFilename:)]
        pub unsafe fn setFilename(&self, filename: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileAttributes)]
        pub unsafe fn fileAttributes(&self) -> Id<NSDictionary<NSString, Object>, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setFileAttributes:)]
        pub unsafe fn setFileAttributes(&self, file_attributes: &NSDictionary<NSString, Object>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(matchesContentsOfURL:)]
        pub unsafe fn matchesContentsOfURL(&self, url: &NSURL) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(readFromURL:options:error:_)]
        pub unsafe fn readFromURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:options:originalContentsURL:error:_)]
        pub unsafe fn writeToURL_options_originalContentsURL_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperWritingOptions,
            original_contents_url: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other serializedRepresentation)]
        pub unsafe fn serializedRepresentation(&self) -> Option<Id<NSData, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other addFileWrapper:)]
        pub unsafe fn addFileWrapper(&self, child: &NSFileWrapper) -> Id<NSString, Shared>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addRegularFileWithContents:preferredFilename:)]
        pub unsafe fn addRegularFileWithContents_preferredFilename(
            &self,
            data: &NSData,
            file_name: &NSString,
        ) -> Id<NSString, Shared>;

        #[method(removeFileWrapper:)]
        pub unsafe fn removeFileWrapper(&self, child: &NSFileWrapper);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other fileWrappers)]
        pub unsafe fn fileWrappers(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSFileWrapper>, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other keyForFileWrapper:)]
        pub unsafe fn keyForFileWrapper(
            &self,
            child: &NSFileWrapper,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other regularFileContents)]
        pub unsafe fn regularFileContents(&self) -> Option<Id<NSData, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other symbolicLinkDestinationURL)]
        pub unsafe fn symbolicLinkDestinationURL(&self) -> Option<Id<NSURL, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSFileWrapper")]
    unsafe impl NSFileWrapper {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initSymbolicLinkWithDestination:)]
        pub unsafe fn initSymbolicLinkWithDestination(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(needsToBeUpdatedFromPath:)]
        pub unsafe fn needsToBeUpdatedFromPath(&self, path: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(updateFromPath:)]
        pub unsafe fn updateFromPath(&self, path: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:atomically:updateFilenames:)]
        pub unsafe fn writeToFile_atomically_updateFilenames(
            &self,
            path: &NSString,
            atomic_flag: bool,
            update_filenames_flag: bool,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other addFileWithPath:)]
        pub unsafe fn addFileWithPath(&self, path: &NSString) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other addSymbolicLinkWithDestination:preferredFilename:)]
        pub unsafe fn addSymbolicLinkWithDestination_preferredFilename(
            &self,
            path: &NSString,
            filename: &NSString,
        ) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other symbolicLinkDestination)]
        pub unsafe fn symbolicLinkDestination(&self) -> Id<NSString, Shared>;
    }
);
