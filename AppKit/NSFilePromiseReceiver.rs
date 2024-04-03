//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFilePromiseReceiver;

    unsafe impl ClassType for NSFilePromiseReceiver {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSFilePromiseReceiver {}

#[cfg(feature = "AppKit_NSPasteboard")]
unsafe impl NSPasteboardReading for NSFilePromiseReceiver {}

extern_methods!(
    unsafe impl NSFilePromiseReceiver {
        #[method_id(@__retain_semantics Other readableDraggedTypes)]
        pub unsafe fn readableDraggedTypes() -> Id<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other fileTypes)]
        pub unsafe fn fileTypes(&self) -> Id<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other fileNames)]
        pub unsafe fn fileNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "block2")]
        #[method(receivePromisedFilesAtDestination:options:operationQueue:reader:)]
        pub unsafe fn receivePromisedFilesAtDestination_options_operationQueue_reader(
            &self,
            destination_dir: &NSURL,
            options: &NSDictionary,
            operation_queue: &NSOperationQueue,
            reader: &Block<dyn Fn(NonNull<NSURL>, *mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSFilePromiseReceiver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
