//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::InputMethodKit::*;

pub const kIMKSingleColumnScrollingCandidatePanel: c_uint = 1;
pub const kIMKScrollingGridCandidatePanel: c_uint = 2;
pub const kIMKSingleRowSteppingCandidatePanel: c_uint = 3;

pub type IMKCandidatePanelType = NSUInteger;

pub const kIMKMain: c_uint = 0;
pub const kIMKAnnotation: c_uint = 1;
pub const kIMKSubList: c_uint = 2;

pub type IMKStyleType = NSUInteger;

pub const kIMKLocateCandidatesAboveHint: c_uint = 1;
pub const kIMKLocateCandidatesBelowHint: c_uint = 2;
pub const kIMKLocateCandidatesLeftHint: c_uint = 3;
pub const kIMKLocateCandidatesRightHint: c_uint = 4;

pub type IMKCandidatesLocationHint = NSUInteger;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSResponder")]
    pub struct IMKCandidates;

    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl ClassType for IMKCandidates {
        #[inherits(NSObject)]
        type Super = NSResponder;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "AppKit_NSResponder", feature = "Foundation_NSObject"))]
unsafe impl NSCoding for IMKCandidates {}

#[cfg(feature = "AppKit_NSResponder")]
unsafe impl NSObjectProtocol for IMKCandidates {}

extern_methods!(
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl IMKCandidates {
        #[cfg(feature = "InputMethodKit_IMKServer")]
        #[method_id(@__retain_semantics Init initWithServer:panelType:)]
        pub unsafe fn initWithServer_panelType(
            this: Allocated<Self>,
            server: Option<&IMKServer>,
            panel_type: IMKCandidatePanelType,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "InputMethodKit_IMKServer")]
        #[method_id(@__retain_semantics Init initWithServer:panelType:styleType:)]
        pub unsafe fn initWithServer_panelType_styleType(
            this: Allocated<Self>,
            server: Option<&IMKServer>,
            panel_type: IMKCandidatePanelType,
            style: IMKStyleType,
        ) -> Option<Id<Self>>;

        #[method(panelType)]
        pub unsafe fn panelType(&self) -> IMKCandidatePanelType;

        #[method(setPanelType:)]
        pub unsafe fn setPanelType(&self, panel_type: IMKCandidatePanelType);

        #[method(show:)]
        pub unsafe fn show(&self, location_hint: IMKCandidatesLocationHint);

        #[method(hide)]
        pub unsafe fn hide(&self);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(showAnnotation:)]
        pub unsafe fn showAnnotation(&self, annotation_string: Option<&NSAttributedString>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(showSublist:subListDelegate:)]
        pub unsafe fn showSublist_subListDelegate(
            &self,
            candidates: Option<&NSArray>,
            delegate: Option<&AnyObject>,
        );

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(candidateFrame)]
        pub unsafe fn candidateFrame(&self) -> NSRect;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSelectionKeys:)]
        pub unsafe fn setSelectionKeys(&self, key_codes: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other selectionKeys)]
        pub unsafe fn selectionKeys(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: Option<&NSDictionary>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Option<Id<NSDictionary>>;

        #[method(setDismissesAutomatically:)]
        pub unsafe fn setDismissesAutomatically(&self, flag: bool);

        #[method(dismissesAutomatically)]
        pub unsafe fn dismissesAutomatically(&self) -> bool;

        #[method(selectedCandidate)]
        pub unsafe fn selectedCandidate(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSGeometry")]
        #[method(setCandidateFrameTopLeft:)]
        pub unsafe fn setCandidateFrameTopLeft(&self, point: NSPoint);

        #[method(showChild)]
        pub unsafe fn showChild(&self);

        #[method(hideChild)]
        pub unsafe fn hideChild(&self);

        #[method(attachChild:toCandidate:type:)]
        pub unsafe fn attachChild_toCandidate_type(
            &self,
            child: Option<&IMKCandidates>,
            candidate_identifier: NSInteger,
            the_type: IMKStyleType,
        );

        #[method(detachChild:)]
        pub unsafe fn detachChild(&self, candidate_identifier: NSInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCandidateData:)]
        pub unsafe fn setCandidateData(&self, candidates_array: Option<&NSArray>);

        #[method(selectCandidateWithIdentifier:)]
        pub unsafe fn selectCandidateWithIdentifier(&self, candidate_identifier: NSInteger)
            -> bool;

        #[method(selectCandidate:)]
        pub unsafe fn selectCandidate(&self, candidate_identifier: NSInteger);

        #[method(showCandidates)]
        pub unsafe fn showCandidates(&self);

        #[method(candidateStringIdentifier:)]
        pub unsafe fn candidateStringIdentifier(
            &self,
            candidate_string: Option<&AnyObject>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other selectedCandidateString)]
        pub unsafe fn selectedCandidateString(&self) -> Option<Id<NSAttributedString>>;

        #[method(candidateIdentifierAtLineNumber:)]
        pub unsafe fn candidateIdentifierAtLineNumber(&self, line_number: NSInteger) -> NSInteger;

        #[method(lineNumberForCandidateWithIdentifier:)]
        pub unsafe fn lineNumberForCandidateWithIdentifier(
            &self,
            candidate_identifier: NSInteger,
        ) -> NSInteger;

        #[method(clearSelection)]
        pub unsafe fn clearSelection(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl IMKCandidates {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSResponder")]
    unsafe impl IMKCandidates {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
