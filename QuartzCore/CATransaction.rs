//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CATransaction")]
    pub struct CATransaction;

    #[cfg(feature = "CoreAnimation_CATransaction")]
    unsafe impl ClassType for CATransaction {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATransaction")]
    unsafe impl CATransaction {
        #[method(begin)]
        pub fn begin();

        #[method(commit)]
        pub fn commit();

        #[method(flush)]
        pub fn flush();

        #[method(lock)]
        pub unsafe fn lock();

        #[method(unlock)]
        pub unsafe fn unlock();

        #[method(animationDuration)]
        pub fn animationDuration() -> CFTimeInterval;

        #[method(setAnimationDuration:)]
        pub fn setAnimationDuration(dur: CFTimeInterval);

        #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
        #[method_id(@__retain_semantics Other animationTimingFunction)]
        pub fn animationTimingFunction() -> Option<Id<CAMediaTimingFunction, Shared>>;

        #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
        #[method(setAnimationTimingFunction:)]
        pub fn setAnimationTimingFunction(function: Option<&CAMediaTimingFunction>);

        #[method(disableActions)]
        pub fn disableActions() -> bool;

        #[method(setDisableActions:)]
        pub fn setDisableActions(flag: bool);

        #[method(completionBlock)]
        pub unsafe fn completionBlock() -> *mut Block<(), ()>;

        #[method(setCompletionBlock:)]
        pub unsafe fn setCompletionBlock(block: Option<&Block<(), ()>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForKey:)]
        pub unsafe fn valueForKey(key: &NSString) -> Option<Id<Object, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forKey:)]
        pub unsafe fn setValue_forKey(anObject: Option<&Object>, key: &NSString);
    }
);

extern_static!(kCATransactionAnimationDuration: &'static NSString);

extern_static!(kCATransactionDisableActions: &'static NSString);

extern_static!(kCATransactionAnimationTimingFunction: &'static NSString);

extern_static!(kCATransactionCompletionBlock: &'static NSString);
