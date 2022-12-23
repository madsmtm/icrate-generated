//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSNotificationName = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotification;

    unsafe impl ClassType for NSNotification {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSNotification {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSNotificationName, Shared>;

        #[method_id(@__retain_semantics Other object)]
        pub unsafe fn object(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;

        #[method_id(@__retain_semantics Init initWithName:object:userInfo:)]
        pub unsafe fn initWithName_object_userInfo(
            this: Option<Allocated<Self>>,
            name: &NSNotificationName,
            object: Option<&Object>,
            userInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSNotificationCreation
    unsafe impl NSNotification {
        #[method_id(@__retain_semantics Other notificationWithName:object:)]
        pub unsafe fn notificationWithName_object(
            aName: &NSNotificationName,
            anObject: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other notificationWithName:object:userInfo:)]
        pub unsafe fn notificationWithName_object_userInfo(
            aName: &NSNotificationName,
            anObject: Option<&Object>,
            aUserInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSNotificationCenter;

    unsafe impl ClassType for NSNotificationCenter {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSNotificationCenter {
        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<NSNotificationCenter, Shared>;

        #[method(addObserver:selector:name:object:)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &Object,
            aSelector: Sel,
            aName: Option<&NSNotificationName>,
            anObject: Option<&Object>,
        );

        #[method(postNotification:)]
        pub unsafe fn postNotification(&self, notification: &NSNotification);

        #[method(postNotificationName:object:)]
        pub unsafe fn postNotificationName_object(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&Object>,
        );

        #[method(postNotificationName:object:userInfo:)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&Object>,
            aUserInfo: Option<&NSDictionary>,
        );

        #[method(removeObserver:)]
        pub unsafe fn removeObserver(&self, observer: &Object);

        #[method(removeObserver:name:object:)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &Object,
            aName: Option<&NSNotificationName>,
            anObject: Option<&Object>,
        );

        #[method_id(@__retain_semantics Other addObserverForName:object:queue:usingBlock:)]
        pub unsafe fn addObserverForName_object_queue_usingBlock(
            &self,
            name: Option<&NSNotificationName>,
            obj: Option<&Object>,
            queue: Option<&NSOperationQueue>,
            block: &Block<(NonNull<NSNotification>,), ()>,
        ) -> Id<NSObject, Shared>;
    }
);
