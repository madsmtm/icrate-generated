//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    pub struct CKDiscoverAllUserIdentitiesOperation;

    #[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
    unsafe impl ClassType for CKDiscoverAllUserIdentitiesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
unsafe impl NSObjectProtocol for CKDiscoverAllUserIdentitiesOperation {}

extern_methods!(
    #[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
    unsafe impl CKDiscoverAllUserIdentitiesOperation {
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(userIdentityDiscoveredBlock)]
        pub unsafe fn userIdentityDiscoveredBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKUserIdentity>)>;

        #[cfg(feature = "CloudKit_CKUserIdentity")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setUserIdentityDiscoveredBlock:)]
        pub unsafe fn setUserIdentityDiscoveredBlock(
            &self,
            user_identity_discovered_block: Option<&Block<dyn Fn(NonNull<CKUserIdentity>)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverAllUserIdentitiesCompletionBlock)]
        pub unsafe fn discoverAllUserIdentitiesCompletionBlock(
            &self,
        ) -> *mut Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setDiscoverAllUserIdentitiesCompletionBlock:)]
        pub unsafe fn setDiscoverAllUserIdentitiesCompletionBlock(
            &self,
            discover_all_user_identities_completion_block: Option<&Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSOperation"))]
    unsafe impl CKDiscoverAllUserIdentitiesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
