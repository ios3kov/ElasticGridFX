#import <AppKit/AppKit.h>

// Called only by AE viewer UI events, never by render workers. No cursor stack:
// every transition replaces the cursor, so cancellation cannot leak a push.
extern "C" bool eg_set_hand_cursor(bool dragging) {
    if (![NSThread isMainThread]) return false;
    [(dragging ? [NSCursor closedHandCursor] : [NSCursor openHandCursor]) set];
    return true;
}
