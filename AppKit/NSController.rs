//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSController;

    unsafe impl ClassType for NSController {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl NSCoding for NSController {}

#[cfg(feature = "AppKit_NSKeyValueBinding")]
unsafe impl NSEditor for NSController {}

#[cfg(feature = "AppKit_NSKeyValueBinding")]
unsafe impl NSEditorRegistration for NSController {}

unsafe impl NSObjectProtocol for NSController {}

extern_methods!(
    unsafe impl NSController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[cfg(feature = "AppKit_NSKeyValueBinding")]
        #[method(objectDidBeginEditing:)]
        pub unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[cfg(feature = "AppKit_NSKeyValueBinding")]
        #[method(objectDidEndEditing:)]
        pub unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[method(discardEditing)]
        pub unsafe fn discardEditing(&self);

        #[method(commitEditing)]
        pub unsafe fn commitEditing(&self) -> bool;

        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        pub unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&AnyObject>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(isEditing)]
        pub unsafe fn isEditing(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
