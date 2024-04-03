//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    #[deprecated]
    pub struct DOMNodeIterator;

    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl ClassType for DOMNodeIterator {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSCopying for DOMNodeIterator {}

#[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
unsafe impl NSObjectProtocol for DOMNodeIterator {}

extern_methods!(
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNodeIterator {
        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other root)]
        pub unsafe fn root(&self) -> Option<Id<DOMNode>>;

        #[deprecated]
        #[method(whatToShow)]
        pub unsafe fn whatToShow(&self) -> c_uint;

        #[cfg(feature = "WebKit_DOMNodeFilter")]
        #[deprecated]
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Id<ProtocolObject<dyn DOMNodeFilter>>>;

        #[deprecated]
        #[method(expandEntityReferences)]
        pub unsafe fn expandEntityReferences(&self) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other referenceNode)]
        pub unsafe fn referenceNode(&self) -> Option<Id<DOMNode>>;

        #[method(pointerBeforeReferenceNode)]
        pub unsafe fn pointerBeforeReferenceNode(&self) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Id<DOMNode>>;

        #[deprecated]
        #[method(detach)]
        pub unsafe fn detach(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNodeIterator {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "WebKit_DOMObject", feature = "WebKit_WebScriptObject"))]
    unsafe impl DOMNodeIterator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
