//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSXMLNodeKind {
        NSXMLInvalidKind = 0,
        NSXMLDocumentKind = 1,
        NSXMLElementKind = 2,
        NSXMLAttributeKind = 3,
        NSXMLNamespaceKind = 4,
        NSXMLProcessingInstructionKind = 5,
        NSXMLCommentKind = 6,
        NSXMLTextKind = 7,
        NSXMLDTDKind = 8,
        NSXMLEntityDeclarationKind = 9,
        NSXMLAttributeDeclarationKind = 10,
        NSXMLElementDeclarationKind = 11,
        NSXMLNotationDeclarationKind = 12,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSXMLNode;

    unsafe impl ClassType for NSXMLNode {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSXMLNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document() -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other documentWithRootElement:)]
        pub unsafe fn documentWithRootElement(element: &NSXMLElement) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other elementWithName:)]
        pub unsafe fn elementWithName(name: &NSString) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other elementWithName:URI:)]
        pub unsafe fn elementWithName_URI(name: &NSString, URI: &NSString) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other elementWithName:stringValue:)]
        pub unsafe fn elementWithName_stringValue(
            name: &NSString,
            string: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other elementWithName:children:attributes:)]
        pub unsafe fn elementWithName_children_attributes(
            name: &NSString,
            children: Option<&NSArray<NSXMLNode>>,
            attributes: Option<&NSArray<NSXMLNode>>,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other attributeWithName:stringValue:)]
        pub unsafe fn attributeWithName_stringValue(
            name: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other attributeWithName:URI:stringValue:)]
        pub unsafe fn attributeWithName_URI_stringValue(
            name: &NSString,
            URI: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other namespaceWithName:stringValue:)]
        pub unsafe fn namespaceWithName_stringValue(
            name: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other processingInstructionWithName:stringValue:)]
        pub unsafe fn processingInstructionWithName_stringValue(
            name: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other commentWithStringValue:)]
        pub unsafe fn commentWithStringValue(stringValue: &NSString) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other textWithStringValue:)]
        pub unsafe fn textWithStringValue(stringValue: &NSString) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other DTDNodeWithXMLString:)]
        pub unsafe fn DTDNodeWithXMLString(string: &NSString) -> Option<Id<Object, Shared>>;

        #[method(kind)]
        pub unsafe fn kind(&self) -> NSXMLNodeKind;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>);

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Id<NSString, Shared>>;

        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, stringValue: Option<&NSString>);

        #[method(setStringValue:resolvingEntities:)]
        pub unsafe fn setStringValue_resolvingEntities(&self, string: &NSString, resolve: bool);

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(level)]
        pub unsafe fn level(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other rootDocument)]
        pub unsafe fn rootDocument(&self) -> Option<Id<NSXMLDocument, Shared>>;

        #[method_id(@__retain_semantics Other parent)]
        pub unsafe fn parent(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method(childCount)]
        pub unsafe fn childCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other children)]
        pub unsafe fn children(&self) -> Option<Id<NSArray<NSXMLNode>, Shared>>;

        #[method_id(@__retain_semantics Other childAtIndex:)]
        pub unsafe fn childAtIndex(&self, index: NSUInteger) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(@__retain_semantics Other previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(@__retain_semantics Other nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(@__retain_semantics Other previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(@__retain_semantics Other nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method(detach)]
        pub unsafe fn detach(&self);

        #[method_id(@__retain_semantics Other XPath)]
        pub unsafe fn XPath(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localName)]
        pub unsafe fn localName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other prefix)]
        pub unsafe fn prefix(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other URI)]
        pub unsafe fn URI(&self) -> Option<Id<NSString, Shared>>;

        #[method(setURI:)]
        pub unsafe fn setURI(&self, URI: Option<&NSString>);

        #[method_id(@__retain_semantics Other localNameForName:)]
        pub unsafe fn localNameForName(name: &NSString) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other prefixForName:)]
        pub unsafe fn prefixForName(name: &NSString) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other predefinedNamespaceForPrefix:)]
        pub unsafe fn predefinedNamespaceForPrefix(
            name: &NSString,
        ) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other XMLString)]
        pub unsafe fn XMLString(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other XMLStringWithOptions:)]
        pub unsafe fn XMLStringWithOptions(
            &self,
            options: NSXMLNodeOptions,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other canonicalXMLStringPreservingComments:)]
        pub unsafe fn canonicalXMLStringPreservingComments(
            &self,
            comments: bool,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other nodesForXPath:error:_)]
        pub unsafe fn nodesForXPath_error(
            &self,
            xpath: &NSString,
        ) -> Result<Id<NSArray<NSXMLNode>, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other objectsForXQuery:constants:error:_)]
        pub unsafe fn objectsForXQuery_constants_error(
            &self,
            xquery: &NSString,
            constants: Option<&NSDictionary<NSString, Object>>,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other objectsForXQuery:error:_)]
        pub unsafe fn objectsForXQuery_error(
            &self,
            xquery: &NSString,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;
    }
);
