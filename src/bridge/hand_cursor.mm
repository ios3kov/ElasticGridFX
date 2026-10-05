#import <AppKit/AppKit.h>

// Called only by AE viewer UI events, never by render workers. No cursor stack:
// every transition replaces the cursor, so cancellation cannot leak a push.
extern "C" bool eg_set_hand_cursor(bool dragging) {
    if (![NSThread isMainThread]) return false;
    [(dragging ? [NSCursor closedHandCursor] : [NSCursor openHandCursor]) set];
    return true;
}

// Read-only UI query for AE's own Point-control drag path. No synthesized input.
extern "C" bool eg_loupe_button_down() {
    return [NSThread isMainThread] && [NSApp isActive] &&
        (([NSEvent pressedMouseButtons] & 1U) != 0);
}
