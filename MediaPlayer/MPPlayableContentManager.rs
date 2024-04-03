//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use CarPlay framework"]
    pub struct MPPlayableContentManager;

    unsafe impl ClassType for MPPlayableContentManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MPPlayableContentManager {}

extern_methods!(
    unsafe impl MPPlayableContentManager {
        #[cfg(feature = "MediaPlayer_MPPlayableContentDataSource")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MPPlayableContentDataSource>>>;

        #[cfg(feature = "MediaPlayer_MPPlayableContentDataSource")]
        #[deprecated = "Use CarPlay framework"]
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn MPPlayableContentDataSource>>,
        );

        #[cfg(feature = "MediaPlayer_MPPlayableContentDelegate")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn MPPlayableContentDelegate>>>;

        #[cfg(feature = "MediaPlayer_MPPlayableContentDelegate")]
        #[deprecated = "Use CarPlay framework"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MPPlayableContentDelegate>>,
        );

        #[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<MPPlayableContentManagerContext>;

        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other nowPlayingIdentifiers)]
        pub unsafe fn nowPlayingIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[deprecated = "Use CarPlay framework"]
        #[method(setNowPlayingIdentifiers:)]
        pub unsafe fn setNowPlayingIdentifiers(&self, now_playing_identifiers: &NSArray<NSString>);

        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other sharedContentManager)]
        pub unsafe fn sharedContentManager() -> Id<Self>;

        #[deprecated = "Use CarPlay framework"]
        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[deprecated = "Use CarPlay framework"]
        #[method(beginUpdates)]
        pub unsafe fn beginUpdates(&self);

        #[deprecated = "Use CarPlay framework"]
        #[method(endUpdates)]
        pub unsafe fn endUpdates(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPPlayableContentManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
