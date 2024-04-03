// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "MediaPlayer", kind = "framework")]
extern "C" {}

#[cfg(feature = "MediaPlayer_AVFoundation_MPNowPlayingInfoLanguageOptionAdditions")]
#[path = "AVFoundation_MPNowPlayingInfoLanguageOptionAdditions.rs"]
mod __AVFoundation_MPNowPlayingInfoLanguageOptionAdditions;
#[cfg(feature = "MediaPlayer_AVPlayerItem_MediaPlayerAdditions")]
#[path = "AVPlayerItem_MediaPlayerAdditions.rs"]
mod __AVPlayerItem_MediaPlayerAdditions;
#[cfg(feature = "MediaPlayer_MPContentItem")]
#[path = "MPContentItem.rs"]
mod __MPContentItem;
#[cfg(feature = "MediaPlayer_MPError")]
#[path = "MPError.rs"]
mod __MPError;
#[cfg(feature = "MediaPlayer_MPMediaEntity")]
#[path = "MPMediaEntity.rs"]
mod __MPMediaEntity;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
#[path = "MPMediaItem.rs"]
mod __MPMediaItem;
#[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
#[path = "MPMediaItemCollection.rs"]
mod __MPMediaItemCollection;
#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
#[path = "MPMediaLibrary.rs"]
mod __MPMediaLibrary;
#[cfg(feature = "MediaPlayer_MPMediaPickerController")]
#[path = "MPMediaPickerController.rs"]
mod __MPMediaPickerController;
#[cfg(feature = "MediaPlayer_MPMediaPlayback")]
#[path = "MPMediaPlayback.rs"]
mod __MPMediaPlayback;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
#[path = "MPMediaPlaylist.rs"]
mod __MPMediaPlaylist;
#[cfg(feature = "MediaPlayer_MPMediaQuery")]
#[path = "MPMediaQuery.rs"]
mod __MPMediaQuery;
#[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
#[path = "MPMediaQuerySection.rs"]
mod __MPMediaQuerySection;
#[cfg(feature = "MediaPlayer_MPMoviePlayerController")]
#[path = "MPMoviePlayerController.rs"]
mod __MPMoviePlayerController;
#[cfg(feature = "MediaPlayer_MPMoviePlayerViewController")]
#[path = "MPMoviePlayerViewController.rs"]
mod __MPMoviePlayerViewController;
#[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
#[path = "MPMusicPlayerApplicationController.rs"]
mod __MPMusicPlayerApplicationController;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
#[path = "MPMusicPlayerController.rs"]
mod __MPMusicPlayerController;
#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
#[path = "MPMusicPlayerQueueDescriptor.rs"]
mod __MPMusicPlayerQueueDescriptor;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
#[path = "MPNowPlayingInfoCenter.rs"]
mod __MPNowPlayingInfoCenter;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
#[path = "MPNowPlayingInfoLanguageOption.rs"]
mod __MPNowPlayingInfoLanguageOption;
#[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
#[path = "MPNowPlayingSession.rs"]
mod __MPNowPlayingSession;
#[cfg(feature = "MediaPlayer_MPPlayableContentDataSource")]
#[path = "MPPlayableContentDataSource.rs"]
mod __MPPlayableContentDataSource;
#[cfg(feature = "MediaPlayer_MPPlayableContentDelegate")]
#[path = "MPPlayableContentDelegate.rs"]
mod __MPPlayableContentDelegate;
#[cfg(feature = "MediaPlayer_MPPlayableContentManager")]
#[path = "MPPlayableContentManager.rs"]
mod __MPPlayableContentManager;
#[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
#[path = "MPPlayableContentManagerContext.rs"]
mod __MPPlayableContentManagerContext;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
#[path = "MPRemoteCommand.rs"]
mod __MPRemoteCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommandCenter")]
#[path = "MPRemoteCommandCenter.rs"]
mod __MPRemoteCommandCenter;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
#[path = "MPRemoteCommandEvent.rs"]
mod __MPRemoteCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
#[path = "MPRemoteControlTypes.rs"]
mod __MPRemoteControlTypes;
#[cfg(feature = "MediaPlayer_MPVolumeSettings")]
#[path = "MPVolumeSettings.rs"]
mod __MPVolumeSettings;
#[cfg(feature = "MediaPlayer_MPVolumeView")]
#[path = "MPVolumeView.rs"]
mod __MPVolumeView;
#[cfg(feature = "MediaPlayer_MediaPlayerDefines")]
#[path = "MediaPlayerDefines.rs"]
mod __MediaPlayerDefines;
#[cfg(feature = "MediaPlayer_NSUserActivity_MediaPlayerAdditions")]
#[path = "NSUserActivity_MediaPlayerAdditions.rs"]
mod __NSUserActivity_MediaPlayerAdditions;

#[cfg(feature = "MediaPlayer_MPContentItem")]
pub use self::__MPContentItem::MPContentItem;
#[cfg(feature = "MediaPlayer_MPError")]
pub use self::__MPError::MPErrorCode;
#[cfg(feature = "MediaPlayer_MPError")]
pub use self::__MPError::MPErrorDomain;
#[cfg(feature = "MediaPlayer_MPMediaEntity")]
pub use self::__MPMediaEntity::MPMediaEntity;
#[cfg(feature = "MediaPlayer_MPMediaEntity")]
pub use self::__MPMediaEntity::MPMediaEntityPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaEntity")]
pub use self::__MPMediaEntity::MPMediaEntityPropertyPersistentID;
#[cfg(all(
    feature = "MediaPlayer_MPMediaEntity",
    feature = "MediaPlayer_MPMediaItem"
))]
pub use self::__MPMediaItem::MPMediaItem;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemArtwork;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumArtist;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumArtistPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumTitle;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumTrackCount;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAlbumTrackNumber;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyArtist;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyArtistPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyArtwork;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyAssetURL;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyBeatsPerMinute;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyBookmarkTime;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyComments;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyComposer;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyComposerPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyDateAdded;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyDiscCount;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyDiscNumber;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyGenre;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyGenrePersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyHasProtectedAsset;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsCloudItem;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsCompilation;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsExplicit;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyIsPreorder;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyLastPlayedDate;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyLyrics;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyMediaType;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPlayCount;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPlaybackDuration;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPlaybackStoreID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPodcastPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyPodcastTitle;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyRating;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyReleaseDate;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertySkipCount;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyTitle;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaItemPropertyUserGrouping;
#[cfg(feature = "MediaPlayer_MPMediaItem")]
pub use self::__MPMediaItem::MPMediaType;
#[cfg(all(
    feature = "MediaPlayer_MPMediaEntity",
    feature = "MediaPlayer_MPMediaItemCollection"
))]
pub use self::__MPMediaItemCollection::MPMediaItemCollection;
#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
pub use self::__MPMediaLibrary::MPMediaLibrary;
#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
pub use self::__MPMediaLibrary::MPMediaLibraryAuthorizationStatus;
#[cfg(feature = "MediaPlayer_MPMediaLibrary")]
pub use self::__MPMediaLibrary::MPMediaLibraryDidChangeNotification;
#[cfg(feature = "MediaPlayer_MPMediaPlayback")]
pub use self::__MPMediaPlayback::MPMediaPlayback;
#[cfg(feature = "MediaPlayer_MPMediaPlayback")]
pub use self::__MPMediaPlayback::MPMediaPlaybackIsPreparedToPlayDidChangeNotification;
#[cfg(all(
    feature = "MediaPlayer_MPMediaEntity",
    feature = "MediaPlayer_MPMediaItemCollection",
    feature = "MediaPlayer_MPMediaPlaylist"
))]
pub use self::__MPMediaPlaylist::MPMediaPlaylist;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistAttribute;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistCreationMetadata;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyAuthorDisplayName;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyCloudGlobalID;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyDescriptionText;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyName;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyPersistentID;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertyPlaylistAttributes;
#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
pub use self::__MPMediaPlaylist::MPMediaPlaylistPropertySeedItems;
#[cfg(feature = "MediaPlayer_MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaGrouping;
#[cfg(feature = "MediaPlayer_MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaPredicate;
#[cfg(feature = "MediaPlayer_MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaPredicateComparison;
#[cfg(feature = "MediaPlayer_MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaPropertyPredicate;
#[cfg(feature = "MediaPlayer_MPMediaQuery")]
pub use self::__MPMediaQuery::MPMediaQuery;
#[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
pub use self::__MPMediaQuerySection::MPMediaQuerySection;
#[cfg(all(
    feature = "MediaPlayer_MPMusicPlayerApplicationController",
    feature = "MediaPlayer_MPMusicPlayerController"
))]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerApplicationController;
#[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerControllerMutableQueue;
#[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerControllerQueue;
#[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
pub use self::__MPMusicPlayerApplicationController::MPMusicPlayerControllerQueueDidChangeNotification;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlaybackState;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerController;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerControllerNowPlayingItemDidChangeNotification;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerControllerPlaybackStateDidChangeNotification;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicPlayerControllerVolumeDidChangeNotification;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicRepeatMode;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPMusicShuffleMode;
#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
pub use self::__MPMusicPlayerController::MPSystemMusicPlayerController;
#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerMediaItemQueueDescriptor;
#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerPlayParameters;
#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerPlayParametersQueueDescriptor;
#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerQueueDescriptor;
#[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
pub use self::__MPMusicPlayerQueueDescriptor::MPMusicPlayerStoreQueueDescriptor;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoCenter;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoCollectionIdentifier;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoMediaType;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyAdTimeRanges;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyAssetURL;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyAvailableLanguageOptions;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyChapterCount;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyChapterNumber;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyCreditsStartTime;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyCurrentLanguageOptions;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyCurrentPlaybackDate;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyDefaultPlaybackRate;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyElapsedPlaybackTime;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyExternalContentIdentifier;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyExternalUserProfileIdentifier;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyIsLiveStream;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyMediaType;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackProgress;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackQueueCount;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackQueueIndex;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyPlaybackRate;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingInfoPropertyServiceIdentifier;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
pub use self::__MPNowPlayingInfoCenter::MPNowPlayingPlaybackState;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicContainsOnlyForcedSubtitles;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicDescribesMusicAndSound;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicDescribesVideo;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicDubbedTranslation;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicEasyToRead;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicIsAuxiliaryContent;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicIsMainProgramContent;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicLanguageTranslation;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicTranscribesSpokenDialog;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPLanguageOptionCharacteristicVoiceOverTranslation;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPNowPlayingInfoLanguageOption;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPNowPlayingInfoLanguageOptionGroup;
#[cfg(feature = "MediaPlayer_MPNowPlayingInfoLanguageOption")]
pub use self::__MPNowPlayingInfoLanguageOption::MPNowPlayingInfoLanguageOptionType;
#[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
pub use self::__MPNowPlayingSession::MPAdTimeRange;
#[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
pub use self::__MPNowPlayingSession::MPNowPlayingSession;
#[cfg(feature = "MediaPlayer_MPNowPlayingSession")]
pub use self::__MPNowPlayingSession::MPNowPlayingSessionDelegate;
#[cfg(feature = "MediaPlayer_MPPlayableContentDataSource")]
pub use self::__MPPlayableContentDataSource::MPPlayableContentDataSource;
#[cfg(feature = "MediaPlayer_MPPlayableContentDelegate")]
pub use self::__MPPlayableContentDelegate::MPPlayableContentDelegate;
#[cfg(feature = "MediaPlayer_MPPlayableContentManager")]
pub use self::__MPPlayableContentManager::MPPlayableContentManager;
#[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
pub use self::__MPPlayableContentManagerContext::MPPlayableContentManagerContext;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangePlaybackPositionCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangePlaybackRateCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangeRepeatModeCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPChangeShuffleModeCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPFeedbackCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPRatingCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPRemoteCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPRemoteCommandHandlerStatus;
#[cfg(feature = "MediaPlayer_MPRemoteCommand")]
pub use self::__MPRemoteCommand::MPSkipIntervalCommand;
#[cfg(feature = "MediaPlayer_MPRemoteCommandCenter")]
pub use self::__MPRemoteCommandCenter::MPRemoteCommandCenter;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangeLanguageOptionCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangePlaybackPositionCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangePlaybackRateCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangeRepeatModeCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPChangeShuffleModeCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPFeedbackCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPRatingCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPRemoteCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPSeekCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPSeekCommandEventType;
#[cfg(feature = "MediaPlayer_MPRemoteCommandEvent")]
pub use self::__MPRemoteCommandEvent::MPSkipIntervalCommandEvent;
#[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
pub use self::__MPRemoteControlTypes::MPChangeLanguageOptionSetting;
#[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
pub use self::__MPRemoteControlTypes::MPRepeatType;
#[cfg(feature = "MediaPlayer_MPRemoteControlTypes")]
pub use self::__MPRemoteControlTypes::MPShuffleType;
#[cfg(feature = "MediaPlayer_MPVolumeSettings")]
pub use self::__MPVolumeSettings::MPVolumeSettingsAlertHide;
#[cfg(feature = "MediaPlayer_MPVolumeSettings")]
pub use self::__MPVolumeSettings::MPVolumeSettingsAlertIsVisible;
#[cfg(feature = "MediaPlayer_MPVolumeSettings")]
pub use self::__MPVolumeSettings::MPVolumeSettingsAlertShow;
#[cfg(feature = "MediaPlayer_NSUserActivity_MediaPlayerAdditions")]
pub use self::__NSUserActivity_MediaPlayerAdditions::NSUserActivityMediaPlayerAdditions;
