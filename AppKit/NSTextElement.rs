//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextElement;

    unsafe impl ClassType for NSTextElement {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextElement {
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Option<Allocated<Self>>,
            textContentManager: Option<&NSTextContentManager>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager, Shared>>;

        #[method(setTextContentManager:)]
        pub unsafe fn setTextContentManager(
            &self,
            textContentManager: Option<&NSTextContentManager>,
        );

        #[method_id(@__retain_semantics Other elementRange)]
        pub unsafe fn elementRange(&self) -> Option<Id<NSTextRange, Shared>>;

        #[method(setElementRange:)]
        pub unsafe fn setElementRange(&self, elementRange: Option<&NSTextRange>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextParagraph;

    unsafe impl ClassType for NSTextParagraph {
        #[inherits(NSObject)]
        type Super = NSTextElement;
    }
);

extern_methods!(
    unsafe impl NSTextParagraph {
        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Option<Allocated<Self>>,
            attributedString: Option<&NSAttributedString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString, Shared>;

        #[method_id(@__retain_semantics Other paragraphContentRange)]
        pub unsafe fn paragraphContentRange(&self) -> Option<Id<NSTextRange, Shared>>;

        #[method_id(@__retain_semantics Other paragraphSeparatorRange)]
        pub unsafe fn paragraphSeparatorRange(&self) -> Option<Id<NSTextRange, Shared>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextElement`
    unsafe impl NSTextParagraph {
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Option<Allocated<Self>>,
            textContentManager: Option<&NSTextContentManager>,
        ) -> Id<Self, Shared>;
    }
);
