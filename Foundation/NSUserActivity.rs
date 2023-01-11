//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSUserActivityPersistentIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUserActivity")]
    pub struct NSUserActivity;

    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl ClassType for NSUserActivity {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSUserActivity")]
    unsafe impl NSUserActivity {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithActivityType:)]
        pub unsafe fn initWithActivityType(
            this: Option<Allocated<Self>>,
            activityType: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other activityType)]
        pub unsafe fn activityType(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(addUserInfoEntriesFromDictionary:)]
        pub unsafe fn addUserInfoEntriesFromDictionary(&self, otherDictionary: &NSDictionary);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other requiredUserInfoKeys)]
        pub unsafe fn requiredUserInfoKeys(&self) -> Option<Id<NSSet<NSString>, Shared>>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(setRequiredUserInfoKeys:)]
        pub unsafe fn setRequiredUserInfoKeys(
            &self,
            requiredUserInfoKeys: Option<&NSSet<NSString>>,
        );

        #[method(needsSave)]
        pub unsafe fn needsSave(&self) -> bool;

        #[method(setNeedsSave:)]
        pub unsafe fn setNeedsSave(&self, needsSave: bool);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other webpageURL)]
        pub unsafe fn webpageURL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setWebpageURL:)]
        pub unsafe fn setWebpageURL(&self, webpageURL: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other referrerURL)]
        pub unsafe fn referrerURL(&self) -> Option<Id<NSURL, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setReferrerURL:)]
        pub unsafe fn setReferrerURL(&self, referrerURL: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other expirationDate)]
        pub unsafe fn expirationDate(&self) -> Option<Id<NSDate, Shared>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setExpirationDate:)]
        pub unsafe fn setExpirationDate(&self, expirationDate: Option<&NSDate>);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other keywords)]
        pub unsafe fn keywords(&self) -> Id<NSSet<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(setKeywords:)]
        pub unsafe fn setKeywords(&self, keywords: &NSSet<NSString>);

        #[method(supportsContinuationStreams)]
        pub unsafe fn supportsContinuationStreams(&self) -> bool;

        #[method(setSupportsContinuationStreams:)]
        pub unsafe fn setSupportsContinuationStreams(&self, supportsContinuationStreams: bool);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSUserActivityDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSUserActivityDelegate>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTargetContentIdentifier:)]
        pub unsafe fn setTargetContentIdentifier(&self, targetContentIdentifier: Option<&NSString>);

        #[method(becomeCurrent)]
        pub unsafe fn becomeCurrent(&self);

        #[method(resignCurrent)]
        pub unsafe fn resignCurrent(&self);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSOutputStream"
        ))]
        #[method(getContinuationStreamsWithCompletionHandler:)]
        pub unsafe fn getContinuationStreamsWithCompletionHandler(
            &self,
            completionHandler: &Block<(*mut NSInputStream, *mut NSOutputStream, *mut NSError), ()>,
        );

        #[method(isEligibleForHandoff)]
        pub unsafe fn isEligibleForHandoff(&self) -> bool;

        #[method(setEligibleForHandoff:)]
        pub unsafe fn setEligibleForHandoff(&self, eligibleForHandoff: bool);

        #[method(isEligibleForSearch)]
        pub unsafe fn isEligibleForSearch(&self) -> bool;

        #[method(setEligibleForSearch:)]
        pub unsafe fn setEligibleForSearch(&self, eligibleForSearch: bool);

        #[method(isEligibleForPublicIndexing)]
        pub unsafe fn isEligibleForPublicIndexing(&self) -> bool;

        #[method(setEligibleForPublicIndexing:)]
        pub unsafe fn setEligibleForPublicIndexing(&self, eligibleForPublicIndexing: bool);

        #[method(isEligibleForPrediction)]
        pub unsafe fn isEligibleForPrediction(&self) -> bool;

        #[method(setEligibleForPrediction:)]
        pub unsafe fn setEligibleForPrediction(&self, eligibleForPrediction: bool);

        #[method_id(@__retain_semantics Other persistentIdentifier)]
        pub unsafe fn persistentIdentifier(
            &self,
        ) -> Option<Id<NSUserActivityPersistentIdentifier, Shared>>;

        #[method(setPersistentIdentifier:)]
        pub unsafe fn setPersistentIdentifier(
            &self,
            persistentIdentifier: Option<&NSUserActivityPersistentIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deleteSavedUserActivitiesWithPersistentIdentifiers:completionHandler:)]
        pub unsafe fn deleteSavedUserActivitiesWithPersistentIdentifiers_completionHandler(
            persistentIdentifiers: &NSArray<NSUserActivityPersistentIdentifier>,
            handler: &Block<(), ()>,
        );

        #[method(deleteAllSavedUserActivitiesWithCompletionHandler:)]
        pub unsafe fn deleteAllSavedUserActivitiesWithCompletionHandler(handler: &Block<(), ()>);
    }
);

extern_static!(NSUserActivityTypeBrowsingWeb: &'static NSString);

extern_protocol!(
    pub struct NSUserActivityDelegate;

    unsafe impl ProtocolType for NSUserActivityDelegate {
        #[optional]
        #[method(userActivityWillSave:)]
        pub unsafe fn userActivityWillSave(&self, userActivity: &NSUserActivity);

        #[optional]
        #[method(userActivityWasContinued:)]
        pub unsafe fn userActivityWasContinued(&self, userActivity: &NSUserActivity);

        #[optional]
        #[method(userActivity:didReceiveInputStream:outputStream:)]
        pub unsafe fn userActivity_didReceiveInputStream_outputStream(
            &self,
            userActivity: &NSUserActivity,
            inputStream: &NSInputStream,
            outputStream: &NSOutputStream,
        );
    }
);
