//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    pub struct CKModifySubscriptionsOperation;

    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    unsafe impl ClassType for CKModifySubscriptionsOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "CloudKit_CKDatabaseOperation",
    feature = "CloudKit_CKOperation",
    feature = "Foundation_NSOperation"
))]
unsafe impl NSObjectProtocol for CKModifySubscriptionsOperation {}

extern_methods!(
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    unsafe impl CKModifySubscriptionsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithSubscriptionsToSave:subscriptionIDsToDelete:)]
        pub unsafe fn initWithSubscriptionsToSave_subscriptionIDsToDelete(
            this: Allocated<Self>,
            subscriptions_to_save: Option<&NSArray<CKSubscription>>,
            subscription_i_ds_to_delete: Option<&NSArray<CKSubscriptionID>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other subscriptionsToSave)]
        pub unsafe fn subscriptionsToSave(&self) -> Option<Id<NSArray<CKSubscription>>>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSArray"))]
        #[method(setSubscriptionsToSave:)]
        pub unsafe fn setSubscriptionsToSave(
            &self,
            subscriptions_to_save: Option<&NSArray<CKSubscription>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other subscriptionIDsToDelete)]
        pub unsafe fn subscriptionIDsToDelete(&self) -> Option<Id<NSArray<CKSubscriptionID>>>;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(setSubscriptionIDsToDelete:)]
        pub unsafe fn setSubscriptionIDsToDelete(
            &self,
            subscription_i_ds_to_delete: Option<&NSArray<CKSubscriptionID>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(perSubscriptionSaveBlock)]
        pub unsafe fn perSubscriptionSaveBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError)>;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setPerSubscriptionSaveBlock:)]
        pub unsafe fn setPerSubscriptionSaveBlock(
            &self,
            per_subscription_save_block: Option<
                &Block<dyn Fn(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(perSubscriptionDeleteBlock)]
        pub unsafe fn perSubscriptionDeleteBlock(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<CKSubscriptionID>, *mut NSError)>;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setPerSubscriptionDeleteBlock:)]
        pub unsafe fn setPerSubscriptionDeleteBlock(
            &self,
            per_subscription_delete_block: Option<
                &Block<dyn Fn(NonNull<CKSubscriptionID>, *mut NSError)>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(modifySubscriptionsCompletionBlock)]
        pub unsafe fn modifySubscriptionsCompletionBlock(
            &self,
        ) -> *mut Block<
            dyn Fn(*mut NSArray<CKSubscription>, *mut NSArray<CKSubscriptionID>, *mut NSError),
        >;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(setModifySubscriptionsCompletionBlock:)]
        pub unsafe fn setModifySubscriptionsCompletionBlock(
            &self,
            modify_subscriptions_completion_block: Option<
                &Block<
                    dyn Fn(
                        *mut NSArray<CKSubscription>,
                        *mut NSArray<CKSubscriptionID>,
                        *mut NSError,
                    ),
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "CloudKit_CKDatabaseOperation",
        feature = "CloudKit_CKOperation",
        feature = "Foundation_NSOperation"
    ))]
    unsafe impl CKModifySubscriptionsOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
