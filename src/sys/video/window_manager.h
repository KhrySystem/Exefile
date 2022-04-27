#ifndef DG_WINDOW_H
#define DG_WINDOW_H



struct window {
    bool fullscreen, borderless;
    int height, width;
    char* name;
}

DGAPI void dgCreateWindow(bool fullscreen, bool borderless) {
    window w = window();
}

extern void

#endif