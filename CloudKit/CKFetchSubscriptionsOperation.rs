//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchSubscriptionsOperation")]
    pub struct CKFetchSubscriptionsOperation;

    #[cfg(feature = "CloudKit_CKFetchSubscriptionsOperation")]
    unsafe impl ClassType for CKFetchSubscriptionsOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchSubscriptionsOperation")]
    unsafe impl CKFetchSubscriptionsOperation {
        #[method_id(@__retain_semantics Other fetchAllSubscriptionsOperation)]
        pub unsafe fn fetchAllSubscriptionsOperation() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithSubscriptionIDs:)]
        pub unsafe fn initWithSubscriptionIDs(
            this: Option<Allocated<Self>>,
            subscription_i_ds: &NSArray<CKSubscriptionID>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subscriptionIDs)]
        pub unsafe fn subscriptionIDs(&self) -> Option<Id<NSArray<CKSubscriptionID>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSubscriptionIDs:)]
        pub unsafe fn setSubscriptionIDs(
            &self,
            subscription_i_ds: Option<&NSArray<CKSubscriptionID>>,
        );

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSError"))]
        #[method(perSubscriptionCompletionBlock)]
        pub unsafe fn perSubscriptionCompletionBlock(
            &self,
        ) -> *mut Block<(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError), ()>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSError"))]
        #[method(setPerSubscriptionCompletionBlock:)]
        pub unsafe fn setPerSubscriptionCompletionBlock(
            &self,
            per_subscription_completion_block: Option<
                &Block<(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError), ()>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchSubscriptionCompletionBlock)]
        pub unsafe fn fetchSubscriptionCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                *mut NSDictionary<CKSubscriptionID, CKSubscription>,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method(setFetchSubscriptionCompletionBlock:)]
        pub unsafe fn setFetchSubscriptionCompletionBlock(
            &self,
            fetch_subscription_completion_block: Option<
                &Block<
                    (
                        *mut NSDictionary<CKSubscriptionID, CKSubscription>,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );
    }
);
