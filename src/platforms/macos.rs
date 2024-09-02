#![allow(deprecated)]

use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSPasteboard, NSSpeechSynthesizer};
use objc2_foundation::{NSArray, NSString};

pub fn play_text(text: &str, mut onend: impl FnMut() + 'static) {
    let synth = unsafe { NSSpeechSynthesizer::new() };
    let text = NSString::from_str(text);

    unsafe { synth.startSpeakingString(&text) };

    onend();
}

/// Copies the given text to the clipboard.
///
/// # Panics
///
/// This function may panic if:
/// - The `NSString::from_str(text)` fails to create a new `NSString` object. This is unlikely but could happen if the underlying Objective-C API fails unexpectedly.
/// - `NSPasteboard::generalPasteboard()` fails to retrieve the general pasteboard instance. This is also unlikely but can occur if the macOS API is not available.
/// - `pasteboard.clearContents()` fails to clear the pasteboard contents. This might happen if the pasteboard API fails.
/// - `ProtocolObject::from_retained(text)` fails to create a `ProtocolObject` from the `NSString`. This could occur if the Objective-C runtime has issues.
/// - `NSArray::from_vec(vec![obj])` fails to create an `NSArray` containing the `ProtocolObject`. This might happen if there's an issue with the Objective-C array API.
/// - `pasteboard.writeObjects(&objects)` returns `false`, indicating that writing to the pasteboard failed.
pub fn copy_to_clipboard(text: &str) {
    unsafe {
        let text = NSString::from_str(text);
        let pasteboard = NSPasteboard::generalPasteboard();
        let _ = pasteboard.clearContents();
        let obj = ProtocolObject::from_retained(text);
        let objects = NSArray::from_vec(vec![obj]);
        let res = pasteboard.writeObjects(&objects);
        assert!(res, "Failed writing to pasteboard");
    };
}
