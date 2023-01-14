//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSNibLoading
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {}
);

extern_methods!(
    /// NSNibLoadingDeprecated
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(feature = "Foundation_NSString")]
        #[method(loadNibNamed:owner:)]
        pub unsafe fn loadNibNamed_owner(
            nib_name: Option<&NSString>,
            owner: Option<&Object>,
        ) -> bool;
    }
);
