//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct LARightStore;

    unsafe impl ClassType for LARightStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for LARightStore {}

extern_methods!(
    unsafe impl LARightStore {
        #[method_id(@__retain_semantics Other sharedStore)]
        pub unsafe fn sharedStore() -> Id<LARightStore>;

        #[cfg(all(
            feature = "LocalAuthentication_LAPersistedRight",
            feature = "LocalAuthentication_LARight",
            feature = "block2"
        ))]
        #[method(rightForIdentifier:completion:)]
        pub unsafe fn rightForIdentifier_completion(
            &self,
            identifier: &NSString,
            handler: &Block<dyn Fn(*mut LAPersistedRight, *mut NSError)>,
        );

        #[cfg(all(
            feature = "LocalAuthentication_LAPersistedRight",
            feature = "LocalAuthentication_LARight",
            feature = "block2"
        ))]
        #[method(saveRight:identifier:completion:)]
        pub unsafe fn saveRight_identifier_completion(
            &self,
            right: &LARight,
            identifier: &NSString,
            handler: &Block<dyn Fn(*mut LAPersistedRight, *mut NSError)>,
        );

        #[cfg(all(
            feature = "LocalAuthentication_LAPersistedRight",
            feature = "LocalAuthentication_LARight",
            feature = "block2"
        ))]
        #[method(saveRight:identifier:secret:completion:)]
        pub unsafe fn saveRight_identifier_secret_completion(
            &self,
            right: &LARight,
            identifier: &NSString,
            secret: &NSData,
            handler: &Block<dyn Fn(*mut LAPersistedRight, *mut NSError)>,
        );

        #[cfg(all(
            feature = "LocalAuthentication_LAPersistedRight",
            feature = "LocalAuthentication_LARight",
            feature = "block2"
        ))]
        #[method(removeRight:completion:)]
        pub unsafe fn removeRight_completion(
            &self,
            right: &LAPersistedRight,
            handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(removeRightForIdentifier:completion:)]
        pub unsafe fn removeRightForIdentifier_completion(
            &self,
            identifier: &NSString,
            handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(removeAllRightsWithCompletion:)]
        pub unsafe fn removeAllRightsWithCompletion(&self, handler: &Block<dyn Fn(*mut NSError)>);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
