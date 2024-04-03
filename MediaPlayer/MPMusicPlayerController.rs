//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMusicPlaybackState(pub NSInteger);
impl MPMusicPlaybackState {
    #[doc(alias = "MPMusicPlaybackStateStopped")]
    pub const Stopped: Self = Self(0);
    #[doc(alias = "MPMusicPlaybackStatePlaying")]
    pub const Playing: Self = Self(1);
    #[doc(alias = "MPMusicPlaybackStatePaused")]
    pub const Paused: Self = Self(2);
    #[doc(alias = "MPMusicPlaybackStateInterrupted")]
    pub const Interrupted: Self = Self(3);
    #[doc(alias = "MPMusicPlaybackStateSeekingForward")]
    pub const SeekingForward: Self = Self(4);
    #[doc(alias = "MPMusicPlaybackStateSeekingBackward")]
    pub const SeekingBackward: Self = Self(5);
}

unsafe impl Encode for MPMusicPlaybackState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMusicPlaybackState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMusicRepeatMode(pub NSInteger);
impl MPMusicRepeatMode {
    #[doc(alias = "MPMusicRepeatModeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MPMusicRepeatModeNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "MPMusicRepeatModeOne")]
    pub const One: Self = Self(2);
    #[doc(alias = "MPMusicRepeatModeAll")]
    pub const All: Self = Self(3);
}

unsafe impl Encode for MPMusicRepeatMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMusicRepeatMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPMusicShuffleMode(pub NSInteger);
impl MPMusicShuffleMode {
    #[doc(alias = "MPMusicShuffleModeDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "MPMusicShuffleModeOff")]
    pub const Off: Self = Self(1);
    #[doc(alias = "MPMusicShuffleModeSongs")]
    pub const Songs: Self = Self(2);
    #[doc(alias = "MPMusicShuffleModeAlbums")]
    pub const Albums: Self = Self(3);
}

unsafe impl Encode for MPMusicShuffleMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MPMusicShuffleMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait MPSystemMusicPlayerController: NSObjectProtocol {
        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(openToPlayQueueDescriptor:)]
        unsafe fn openToPlayQueueDescriptor(&self, queue_descriptor: &MPMusicPlayerQueueDescriptor);
    }

    unsafe impl ProtocolType for dyn MPSystemMusicPlayerController {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MPMusicPlayerController;

    unsafe impl ClassType for MPMusicPlayerController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaPlayback")]
unsafe impl MPMediaPlayback for MPMusicPlayerController {}

unsafe impl NSObjectProtocol for MPMusicPlayerController {}

extern_methods!(
    unsafe impl MPMusicPlayerController {
        #[method_id(@__retain_semantics Other applicationMusicPlayer)]
        pub unsafe fn applicationMusicPlayer() -> Id<MPMusicPlayerController>;

        #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
        #[method_id(@__retain_semantics Other applicationQueuePlayer)]
        pub unsafe fn applicationQueuePlayer() -> Id<MPMusicPlayerApplicationController>;

        #[method_id(@__retain_semantics Other systemMusicPlayer)]
        pub unsafe fn systemMusicPlayer() -> Id<MPMusicPlayerController>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method(playbackState)]
        pub unsafe fn playbackState(&self) -> MPMusicPlaybackState;

        #[method(repeatMode)]
        pub unsafe fn repeatMode(&self) -> MPMusicRepeatMode;

        #[method(setRepeatMode:)]
        pub unsafe fn setRepeatMode(&self, repeat_mode: MPMusicRepeatMode);

        #[method(shuffleMode)]
        pub unsafe fn shuffleMode(&self) -> MPMusicShuffleMode;

        #[method(setShuffleMode:)]
        pub unsafe fn setShuffleMode(&self, shuffle_mode: MPMusicShuffleMode);

        #[deprecated = "Use MPVolumeView for volume control."]
        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[deprecated = "Use MPVolumeView for volume control."]
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method_id(@__retain_semantics Other nowPlayingItem)]
        pub unsafe fn nowPlayingItem(&self) -> Option<Id<MPMediaItem>>;

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method(setNowPlayingItem:)]
        pub unsafe fn setNowPlayingItem(&self, now_playing_item: Option<&MPMediaItem>);

        #[method(indexOfNowPlayingItem)]
        pub unsafe fn indexOfNowPlayingItem(&self) -> NSUInteger;

        #[cfg(feature = "MediaPlayer_MPMediaQuery")]
        #[method(setQueueWithQuery:)]
        pub unsafe fn setQueueWithQuery(&self, query: &MPMediaQuery);

        #[cfg(all(
            feature = "MediaPlayer_MPMediaEntity",
            feature = "MediaPlayer_MPMediaItemCollection"
        ))]
        #[method(setQueueWithItemCollection:)]
        pub unsafe fn setQueueWithItemCollection(&self, item_collection: &MPMediaItemCollection);

        #[method(setQueueWithStoreIDs:)]
        pub unsafe fn setQueueWithStoreIDs(&self, store_i_ds: &NSArray<NSString>);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(setQueueWithDescriptor:)]
        pub unsafe fn setQueueWithDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(prependQueueDescriptor:)]
        pub unsafe fn prependQueueDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(appendQueueDescriptor:)]
        pub unsafe fn appendQueueDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "block2")]
        #[method(prepareToPlayWithCompletionHandler:)]
        pub unsafe fn prepareToPlayWithCompletionHandler(
            &self,
            completion_handler: &Block<dyn Fn(*mut NSError)>,
        );

        #[method(skipToNextItem)]
        pub unsafe fn skipToNextItem(&self);

        #[method(skipToBeginning)]
        pub unsafe fn skipToBeginning(&self);

        #[method(skipToPreviousItem)]
        pub unsafe fn skipToPreviousItem(&self);

        #[method(beginGeneratingPlaybackNotifications)]
        pub unsafe fn beginGeneratingPlaybackNotifications(&self);

        #[method(endGeneratingPlaybackNotifications)]
        pub unsafe fn endGeneratingPlaybackNotifications(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other iPodMusicPlayer)]
        pub unsafe fn iPodMusicPlayer() -> Id<MPMusicPlayerController>;
    }
);

extern "C" {
    pub static MPMusicPlayerControllerPlaybackStateDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub static MPMusicPlayerControllerNowPlayingItemDidChangeNotification:
        &'static NSNotificationName;
}

extern "C" {
    pub static MPMusicPlayerControllerVolumeDidChangeNotification: &'static NSNotificationName;
}
