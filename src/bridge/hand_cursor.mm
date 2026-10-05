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

extern "C" int eg_loupe_button_probe() {
    return ([NSThread isMainThread] ? 1 : 0) |
           ([NSApp isActive] ? 2 : 0) |
           (([NSEvent pressedMouseButtons] & 1U) != 0 ? 4 : 0);
}

// Replace the cursor image locally; no hide/unhide stack or global visibility counter.
extern "C" bool eg_set_loupe_cursor(bool hidden) {
    if (![NSThread isMainThread]) return false;
    if (!hidden) { [[NSCursor arrowCursor] set]; return true; }
    if (![NSApp isActive]) return false;
    static NSCursor *blank = [[NSCursor alloc] initWithImage:
        [[NSImage alloc] initWithSize:NSMakeSize(1, 1)] hotSpot:NSZeroPoint];
    [blank set];
    return true;
}

// Read-only pointer position; used only to qualify a previously hit-tested corner.
extern "C" bool eg_loupe_pointer(double *x, double *y) {
    if (![NSThread isMainThread] || ![NSApp isActive] || !x || !y) return false;
    const NSPoint p = [NSEvent mouseLocation];
    *x = p.x; *y = p.y; return true;
}
