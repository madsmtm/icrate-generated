//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSResponder;

    unsafe impl ClassType for NSResponder {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other nextResponder)]
        pub unsafe fn nextResponder(&self) -> Option<Id<NSResponder, Shared>>;

        #[method(setNextResponder:)]
        pub unsafe fn setNextResponder(&self, nextResponder: Option<&NSResponder>);

        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&Object>) -> bool;

        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[method_id(@__retain_semantics Other validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            sendType: Option<&NSPasteboardType>,
            returnType: Option<&NSPasteboardType>,
        ) -> Option<Id<Object, Shared>>;

        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[method(rightMouseDown:)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);

        #[method(otherMouseDown:)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);

        #[method(mouseUp:)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);

        #[method(rightMouseUp:)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);

        #[method(otherMouseUp:)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);

        #[method(mouseMoved:)]
        pub unsafe fn mouseMoved(&self, event: &NSEvent);

        #[method(mouseDragged:)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);

        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[method(rightMouseDragged:)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);

        #[method(otherMouseDragged:)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);

        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[method(keyDown:)]
        pub unsafe fn keyDown(&self, event: &NSEvent);

        #[method(keyUp:)]
        pub unsafe fn keyUp(&self, event: &NSEvent);

        #[method(flagsChanged:)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);

        #[method(tabletPoint:)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);

        #[method(tabletProximity:)]
        pub unsafe fn tabletProximity(&self, event: &NSEvent);

        #[method(cursorUpdate:)]
        pub unsafe fn cursorUpdate(&self, event: &NSEvent);

        #[method(magnifyWithEvent:)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);

        #[method(rotateWithEvent:)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);

        #[method(swipeWithEvent:)]
        pub unsafe fn swipeWithEvent(&self, event: &NSEvent);

        #[method(beginGestureWithEvent:)]
        pub unsafe fn beginGestureWithEvent(&self, event: &NSEvent);

        #[method(endGestureWithEvent:)]
        pub unsafe fn endGestureWithEvent(&self, event: &NSEvent);

        #[method(smartMagnifyWithEvent:)]
        pub unsafe fn smartMagnifyWithEvent(&self, event: &NSEvent);

        #[method(changeModeWithEvent:)]
        pub unsafe fn changeModeWithEvent(&self, event: &NSEvent);

        #[method(touchesBeganWithEvent:)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);

        #[method(touchesMovedWithEvent:)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);

        #[method(touchesEndedWithEvent:)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);

        #[method(touchesCancelledWithEvent:)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);

        #[method(quickLookWithEvent:)]
        pub unsafe fn quickLookWithEvent(&self, event: &NSEvent);

        #[method(pressureChangeWithEvent:)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);

        #[method(noResponderFor:)]
        pub unsafe fn noResponderFor(&self, eventSelector: Sel);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(becomeFirstResponder)]
        pub unsafe fn becomeFirstResponder(&self) -> bool;

        #[method(resignFirstResponder)]
        pub unsafe fn resignFirstResponder(&self) -> bool;

        #[method(interpretKeyEvents:)]
        pub unsafe fn interpretKeyEvents(&self, eventArray: &NSArray<NSEvent>);

        #[method(flushBufferedKeyEvents)]
        pub unsafe fn flushBufferedKeyEvents(&self);

        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;

        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(showContextHelp:)]
        pub unsafe fn showContextHelp(&self, sender: Option<&Object>);

        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, eventPtr: &NSEvent);

        #[method(shouldBeTreatedAsInkEvent:)]
        pub unsafe fn shouldBeTreatedAsInkEvent(&self, event: &NSEvent) -> bool;

        #[method(wantsScrollEventsForSwipeTrackingOnAxis:)]
        pub unsafe fn wantsScrollEventsForSwipeTrackingOnAxis(
            &self,
            axis: NSEventGestureAxis,
        ) -> bool;

        #[method(wantsForwardedScrollEventsForAxis:)]
        pub unsafe fn wantsForwardedScrollEventsForAxis(&self, axis: NSEventGestureAxis) -> bool;

        #[method_id(@__retain_semantics Other supplementalTargetForAction:sender:)]
        pub unsafe fn supplementalTargetForAction_sender(
            &self,
            action: Sel,
            sender: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
    }
);

extern_protocol!(
    pub struct NSStandardKeyBindingResponding;

    unsafe impl ProtocolType for NSStandardKeyBindingResponding {
        #[optional]
        #[method(insertText:)]
        pub unsafe fn insertText(&self, insertString: &Object);

        #[optional]
        #[method(doCommandBySelector:)]
        pub unsafe fn doCommandBySelector(&self, selector: Sel);

        #[optional]
        #[method(moveForward:)]
        pub unsafe fn moveForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveRight:)]
        pub unsafe fn moveRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveBackward:)]
        pub unsafe fn moveBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveLeft:)]
        pub unsafe fn moveLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveUp:)]
        pub unsafe fn moveUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveDown:)]
        pub unsafe fn moveDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordForward:)]
        pub unsafe fn moveWordForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordBackward:)]
        pub unsafe fn moveWordBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfLine:)]
        pub unsafe fn moveToBeginningOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfLine:)]
        pub unsafe fn moveToEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfParagraph:)]
        pub unsafe fn moveToBeginningOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfParagraph:)]
        pub unsafe fn moveToEndOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfDocument:)]
        pub unsafe fn moveToEndOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfDocument:)]
        pub unsafe fn moveToBeginningOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageDown:)]
        pub unsafe fn pageDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageUp:)]
        pub unsafe fn pageUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(centerSelectionInVisibleArea:)]
        pub unsafe fn centerSelectionInVisibleArea(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveBackwardAndModifySelection:)]
        pub unsafe fn moveBackwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveForwardAndModifySelection:)]
        pub unsafe fn moveForwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordForwardAndModifySelection:)]
        pub unsafe fn moveWordForwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordBackwardAndModifySelection:)]
        pub unsafe fn moveWordBackwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveUpAndModifySelection:)]
        pub unsafe fn moveUpAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveDownAndModifySelection:)]
        pub unsafe fn moveDownAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfLineAndModifySelection:)]
        pub unsafe fn moveToBeginningOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfLineAndModifySelection:)]
        pub unsafe fn moveToEndOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfParagraphAndModifySelection:)]
        pub unsafe fn moveToBeginningOfParagraphAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfParagraphAndModifySelection:)]
        pub unsafe fn moveToEndOfParagraphAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToEndOfDocumentAndModifySelection:)]
        pub unsafe fn moveToEndOfDocumentAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToBeginningOfDocumentAndModifySelection:)]
        pub unsafe fn moveToBeginningOfDocumentAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageDownAndModifySelection:)]
        pub unsafe fn pageDownAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(pageUpAndModifySelection:)]
        pub unsafe fn pageUpAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveParagraphForwardAndModifySelection:)]
        pub unsafe fn moveParagraphForwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveParagraphBackwardAndModifySelection:)]
        pub unsafe fn moveParagraphBackwardAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordRight:)]
        pub unsafe fn moveWordRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordLeft:)]
        pub unsafe fn moveWordLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveRightAndModifySelection:)]
        pub unsafe fn moveRightAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveLeftAndModifySelection:)]
        pub unsafe fn moveLeftAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordRightAndModifySelection:)]
        pub unsafe fn moveWordRightAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveWordLeftAndModifySelection:)]
        pub unsafe fn moveWordLeftAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToLeftEndOfLine:)]
        pub unsafe fn moveToLeftEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToRightEndOfLine:)]
        pub unsafe fn moveToRightEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToLeftEndOfLineAndModifySelection:)]
        pub unsafe fn moveToLeftEndOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(moveToRightEndOfLineAndModifySelection:)]
        pub unsafe fn moveToRightEndOfLineAndModifySelection(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollPageUp:)]
        pub unsafe fn scrollPageUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollPageDown:)]
        pub unsafe fn scrollPageDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollLineUp:)]
        pub unsafe fn scrollLineUp(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollLineDown:)]
        pub unsafe fn scrollLineDown(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollToBeginningOfDocument:)]
        pub unsafe fn scrollToBeginningOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(scrollToEndOfDocument:)]
        pub unsafe fn scrollToEndOfDocument(&self, sender: Option<&Object>);

        #[optional]
        #[method(transpose:)]
        pub unsafe fn transpose(&self, sender: Option<&Object>);

        #[optional]
        #[method(transposeWords:)]
        pub unsafe fn transposeWords(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectParagraph:)]
        pub unsafe fn selectParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectLine:)]
        pub unsafe fn selectLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectWord:)]
        pub unsafe fn selectWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(indent:)]
        pub unsafe fn indent(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertTab:)]
        pub unsafe fn insertTab(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertBacktab:)]
        pub unsafe fn insertBacktab(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertNewline:)]
        pub unsafe fn insertNewline(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertParagraphSeparator:)]
        pub unsafe fn insertParagraphSeparator(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertNewlineIgnoringFieldEditor:)]
        pub unsafe fn insertNewlineIgnoringFieldEditor(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertTabIgnoringFieldEditor:)]
        pub unsafe fn insertTabIgnoringFieldEditor(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertLineBreak:)]
        pub unsafe fn insertLineBreak(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertContainerBreak:)]
        pub unsafe fn insertContainerBreak(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertSingleQuoteIgnoringSubstitution:)]
        pub unsafe fn insertSingleQuoteIgnoringSubstitution(&self, sender: Option<&Object>);

        #[optional]
        #[method(insertDoubleQuoteIgnoringSubstitution:)]
        pub unsafe fn insertDoubleQuoteIgnoringSubstitution(&self, sender: Option<&Object>);

        #[optional]
        #[method(changeCaseOfLetter:)]
        pub unsafe fn changeCaseOfLetter(&self, sender: Option<&Object>);

        #[optional]
        #[method(uppercaseWord:)]
        pub unsafe fn uppercaseWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(lowercaseWord:)]
        pub unsafe fn lowercaseWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(capitalizeWord:)]
        pub unsafe fn capitalizeWord(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteForward:)]
        pub unsafe fn deleteForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteBackward:)]
        pub unsafe fn deleteBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteBackwardByDecomposingPreviousCharacter:)]
        pub unsafe fn deleteBackwardByDecomposingPreviousCharacter(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteWordForward:)]
        pub unsafe fn deleteWordForward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteWordBackward:)]
        pub unsafe fn deleteWordBackward(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToBeginningOfLine:)]
        pub unsafe fn deleteToBeginningOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToEndOfLine:)]
        pub unsafe fn deleteToEndOfLine(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToBeginningOfParagraph:)]
        pub unsafe fn deleteToBeginningOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToEndOfParagraph:)]
        pub unsafe fn deleteToEndOfParagraph(&self, sender: Option<&Object>);

        #[optional]
        #[method(yank:)]
        pub unsafe fn yank(&self, sender: Option<&Object>);

        #[optional]
        #[method(complete:)]
        pub unsafe fn complete(&self, sender: Option<&Object>);

        #[optional]
        #[method(setMark:)]
        pub unsafe fn setMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(deleteToMark:)]
        pub unsafe fn deleteToMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(selectToMark:)]
        pub unsafe fn selectToMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(swapWithMark:)]
        pub unsafe fn swapWithMark(&self, sender: Option<&Object>);

        #[optional]
        #[method(cancelOperation:)]
        pub unsafe fn cancelOperation(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeBaseWritingDirectionNatural:)]
        pub unsafe fn makeBaseWritingDirectionNatural(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeBaseWritingDirectionLeftToRight:)]
        pub unsafe fn makeBaseWritingDirectionLeftToRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeBaseWritingDirectionRightToLeft:)]
        pub unsafe fn makeBaseWritingDirectionRightToLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeTextWritingDirectionNatural:)]
        pub unsafe fn makeTextWritingDirectionNatural(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeTextWritingDirectionLeftToRight:)]
        pub unsafe fn makeTextWritingDirectionLeftToRight(&self, sender: Option<&Object>);

        #[optional]
        #[method(makeTextWritingDirectionRightToLeft:)]
        pub unsafe fn makeTextWritingDirectionRightToLeft(&self, sender: Option<&Object>);

        #[optional]
        #[method(quickLookPreviewItems:)]
        pub unsafe fn quickLookPreviewItems(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSStandardKeyBindingMethods
    unsafe impl NSResponder {}
);

extern_methods!(
    /// NSUndoSupport
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager, Shared>>;
    }
);

extern_methods!(
    /// NSControlEditingSupport
    unsafe impl NSResponder {
        #[method(validateProposedFirstResponder:forEvent:)]
        pub unsafe fn validateProposedFirstResponder_forEvent(
            &self,
            responder: &NSResponder,
            event: Option<&NSEvent>,
        ) -> bool;
    }
);

extern_methods!(
    /// NSErrorPresentation
    unsafe impl NSResponder {
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&Object>,
            didPresentSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );

        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[method_id(@__retain_semantics Other willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Id<NSError, Shared>;
    }
);

extern_methods!(
    /// NSTextFinderSupport
    unsafe impl NSResponder {
        #[method(performTextFinderAction:)]
        pub unsafe fn performTextFinderAction(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSWindowTabbing
    unsafe impl NSResponder {
        #[method(newWindowForTab:)]
        pub unsafe fn newWindowForTab(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSResponder {
        #[method(performMnemonic:)]
        pub unsafe fn performMnemonic(&self, string: &NSString) -> bool;
    }
);
