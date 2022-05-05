#ifndef RUNTIME_CONFIG_H
    #define RUNTIME_CONFIG_H

    #define DEBUG_ENABLED

    #define Stringize( L )     #L 
    #define MakeString( M, L ) M(L)
    #define $Line MakeString( Stringize, __LINE__ )
    #define Reminder __FILE__ "(" $Line ") : Reminder: "

    #pragma message(Reminder "vulkan-tutorial.com->Drawing a triangle.Overview.Validation Layers")
#endif