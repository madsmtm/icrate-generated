//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDragPreview;

    unsafe impl ClassType for UIDragPreview {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCopying for UIDragPreview {}

unsafe impl NSObjectProtocol for UIDragPreview {}

extern_methods!(
    unsafe impl UIDragPreview {
        #[cfg(all(
            feature = "UIDragPreviewParameters",
            feature = "UIPreviewParameters",
            feature = "UIResponder",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Init initWithView:parameters:)]
        pub unsafe fn initWithView_parameters(
            this: Allocated<Self>,
            view: &UIView,
            parameters: &UIDragPreviewParameters,
        ) -> Id<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithView:)]
        pub unsafe fn initWithView(this: Allocated<Self>, view: &UIView) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Id<UIView>;

        #[cfg(all(feature = "UIDragPreviewParameters", feature = "UIPreviewParameters"))]
        #[method_id(@__retain_semantics Other parameters)]
        pub unsafe fn parameters(&self) -> Id<UIDragPreviewParameters>;
    }
);