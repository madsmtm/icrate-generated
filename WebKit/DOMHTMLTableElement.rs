//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    #[deprecated]
    pub struct DOMHTMLTableElement;

    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl ClassType for DOMHTMLTableElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMEventTarget",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl DOMEventTarget for DOMHTMLTableElement {}

#[cfg(all(
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSCopying for DOMHTMLTableElement {}

#[cfg(all(
    feature = "WebKit_DOMElement",
    feature = "WebKit_DOMHTMLElement",
    feature = "WebKit_DOMNode",
    feature = "WebKit_DOMObject",
    feature = "WebKit_WebScriptObject"
))]
unsafe impl NSObjectProtocol for DOMHTMLTableElement {}

extern_methods!(
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableElement {
        #[cfg(feature = "WebKit_DOMHTMLTableCaptionElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other caption)]
        pub unsafe fn caption(&self) -> Option<Id<DOMHTMLTableCaptionElement>>;

        #[cfg(feature = "WebKit_DOMHTMLTableCaptionElement")]
        #[deprecated]
        #[method(setCaption:)]
        pub unsafe fn setCaption(&self, caption: Option<&DOMHTMLTableCaptionElement>);

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other tHead)]
        pub unsafe fn tHead(&self) -> Option<Id<DOMHTMLTableSectionElement>>;

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[deprecated]
        #[method(setTHead:)]
        pub unsafe fn setTHead(&self, t_head: Option<&DOMHTMLTableSectionElement>);

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other tFoot)]
        pub unsafe fn tFoot(&self) -> Option<Id<DOMHTMLTableSectionElement>>;

        #[cfg(feature = "WebKit_DOMHTMLTableSectionElement")]
        #[deprecated]
        #[method(setTFoot:)]
        pub unsafe fn setTFoot(&self, t_foot: Option<&DOMHTMLTableSectionElement>);

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other rows)]
        pub unsafe fn rows(&self) -> Option<Id<DOMHTMLCollection>>;

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[deprecated]
        #[method_id(@__retain_semantics Other tBodies)]
        pub unsafe fn tBodies(&self) -> Option<Id<DOMHTMLCollection>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other bgColor)]
        pub unsafe fn bgColor(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setBgColor:)]
        pub unsafe fn setBgColor(&self, bg_color: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other border)]
        pub unsafe fn border(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setBorder:)]
        pub unsafe fn setBorder(&self, border: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other cellPadding)]
        pub unsafe fn cellPadding(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setCellPadding:)]
        pub unsafe fn setCellPadding(&self, cell_padding: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other cellSpacing)]
        pub unsafe fn cellSpacing(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setCellSpacing:)]
        pub unsafe fn setCellSpacing(&self, cell_spacing: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other frameBorders)]
        pub unsafe fn frameBorders(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setFrameBorders:)]
        pub unsafe fn setFrameBorders(&self, frame_borders: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other rules)]
        pub unsafe fn rules(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setRules:)]
        pub unsafe fn setRules(&self, rules: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other summary)]
        pub unsafe fn summary(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setSummary:)]
        pub unsafe fn setSummary(&self, summary: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[deprecated]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other createTHead)]
        pub unsafe fn createTHead(&self) -> Option<Id<DOMHTMLElement>>;

        #[deprecated]
        #[method(deleteTHead)]
        pub unsafe fn deleteTHead(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other createTFoot)]
        pub unsafe fn createTFoot(&self) -> Option<Id<DOMHTMLElement>>;

        #[deprecated]
        #[method(deleteTFoot)]
        pub unsafe fn deleteTFoot(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other createCaption)]
        pub unsafe fn createCaption(&self) -> Option<Id<DOMHTMLElement>>;

        #[deprecated]
        #[method(deleteCaption)]
        pub unsafe fn deleteCaption(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other insertRow:)]
        pub unsafe fn insertRow(&self, index: c_int) -> Option<Id<DOMHTMLElement>>;

        #[deprecated]
        #[method(deleteRow:)]
        pub unsafe fn deleteRow(&self, index: c_int);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(
        feature = "WebKit_DOMElement",
        feature = "WebKit_DOMHTMLElement",
        feature = "WebKit_DOMNode",
        feature = "WebKit_DOMObject",
        feature = "WebKit_WebScriptObject"
    ))]
    unsafe impl DOMHTMLTableElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
