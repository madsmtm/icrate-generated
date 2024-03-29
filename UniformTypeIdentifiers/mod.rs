// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `UniformTypeIdentifiers` framework
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

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {}

#[cfg(feature = "UniformTypeIdentifiers_NSItemProvider_UTType")]
#[path = "NSItemProvider_UTType.rs"]
mod __NSItemProvider_UTType;
#[cfg(feature = "UniformTypeIdentifiers_UTAdditions")]
#[path = "UTAdditions.rs"]
mod __UTAdditions;
#[cfg(feature = "UniformTypeIdentifiers_UTCoreTypes")]
#[path = "UTCoreTypes.rs"]
mod __UTCoreTypes;
#[cfg(feature = "UniformTypeIdentifiers_UTDefines")]
#[path = "UTDefines.rs"]
mod __UTDefines;
#[cfg(feature = "UniformTypeIdentifiers_UTTagClass")]
#[path = "UTTagClass.rs"]
mod __UTTagClass;
#[cfg(feature = "UniformTypeIdentifiers_UTType")]
#[path = "UTType.rs"]
mod __UTType;

#[cfg(feature = "UniformTypeIdentifiers_NSItemProvider_UTType")]
pub use self::__NSItemProvider_UTType::NSItemProviderUTType;
#[cfg(feature = "UniformTypeIdentifiers_UTAdditions")]
pub use self::__UTAdditions::NSStringUTAdditions;
#[cfg(feature = "UniformTypeIdentifiers_UTAdditions")]
pub use self::__UTAdditions::NSURLUTAdditions;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTType3DContent;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAHAP;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAIFF;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeARReferenceObject;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAVI;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAliasFile;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAppleArchive;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAppleProtectedMPEG4Audio;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAppleProtectedMPEG4Video;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAppleScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeApplication;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeApplicationBundle;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeApplicationExtension;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeArchive;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAssemblyLanguageSource;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAudio;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeAudiovisualContent;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeBMP;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeBZ2;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeBinaryPropertyList;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeBookmark;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeBundle;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCHeader;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCPlusPlusHeader;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCPlusPlusSource;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCSource;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCalendarEvent;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCommaSeparatedText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeCompositeContent;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeContact;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeContent;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeData;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeDatabase;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeDelimitedText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeDirectory;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeDiskImage;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeEPUB;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeEXE;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeEmailMessage;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeExecutable;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeFileURL;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeFlatRTFD;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeFolder;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeFont;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeFramework;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeGIF;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeGZIP;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeHEIC;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeHEIF;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeHTML;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeICNS;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeICO;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeImage;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeInternetLocation;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeInternetShortcut;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeItem;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeJPEG;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeJSON;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeJavaScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeLivePhoto;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeLog;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeM3UPlaylist;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMIDI;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMP3;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMPEG;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMPEG2TransportStream;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMPEG2Video;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMPEG4Audio;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMPEG4Movie;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMakefile;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMessage;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMountPoint;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeMovie;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeOSAScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeOSAScriptBundle;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeObjectiveCPlusPlusSource;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeObjectiveCSource;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePDF;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePHPScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePKCS12;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePNG;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePackage;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePerlScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePlainText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePlaylist;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePluginBundle;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePresentation;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePropertyList;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypePythonScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeQuickLookGenerator;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeQuickTimeMovie;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeRAWImage;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeRTF;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeRTFD;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeRealityFile;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeResolvable;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeRubyScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSVG;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSceneKitScene;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeShellScript;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSourceCode;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSpotlightImporter;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSpreadsheet;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSwiftSource;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSymbolicLink;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeSystemPreferencesPane;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeTIFF;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeTabSeparatedText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeToDoItem;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeURL;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeURLBookmarkData;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUSD;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUSDZ;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUTF16ExternalPlainText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUTF16PlainText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUTF8PlainText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUTF8TabSeparatedText;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeUnixExecutable;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeVCard;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeVideo;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeVolume;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeWAV;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeWebArchive;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeWebP;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeX509Certificate;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeXML;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeXMLPropertyList;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeXPCService;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeYAML;
#[cfg(all(
    feature = "UniformTypeIdentifiers_UTCoreTypes",
    feature = "UniformTypeIdentifiers_UTType"
))]
pub use self::__UTCoreTypes::UTTypeZIP;
#[cfg(all(
    feature = "Foundation_NSString",
    feature = "UniformTypeIdentifiers_UTTagClass"
))]
pub use self::__UTTagClass::UTTagClassFilenameExtension;
#[cfg(all(
    feature = "Foundation_NSString",
    feature = "UniformTypeIdentifiers_UTTagClass"
))]
pub use self::__UTTagClass::UTTagClassMIMEType;
#[cfg(feature = "UniformTypeIdentifiers_UTType")]
pub use self::__UTType::UTType;
