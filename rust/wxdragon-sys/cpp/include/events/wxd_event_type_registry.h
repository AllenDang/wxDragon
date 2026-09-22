#ifndef WXD_EVENT_TYPE_REGISTRY_H
#define WXD_EVENT_TYPE_REGISTRY_H

#include <wx/event.h>

#include "../wxd_types.h"

// Widget-specific event types are mapped to the wxWidgets ones by the file
// implementing the widget itself and not by event.cpp, which is always linked
// in: otherwise using any event at all would pull in the code of every widget
// having events of its own, e.g. wxGrid or wxPropertyGrid, when linking
// statically.
//
// The mappers must not be called during static initialization, as the
// wxWidgets event types themselves are initialized then, so the registry
// stores the functions and calls them only when an event is actually bound.
typedef wxEventType (*WxdEventTypeMapper)(WXDEventTypeCEnum);

void
wxd_RegisterEventTypeMapper(WxdEventTypeMapper mapper);

// Returns wxEVT_NULL if none of the registered mappers knows this event.
wxEventType
wxd_LookupRegisteredEventType(WXDEventTypeCEnum c_enum_val);

// Helper registering the mapper of the file defining an object of this type.
struct WxdEventTypeMapperRegistrar
{
    explicit WxdEventTypeMapperRegistrar(WxdEventTypeMapper mapper)
    {
        wxd_RegisterEventTypeMapper(mapper);
    }
};

#endif // WXD_EVENT_TYPE_REGISTRY_H
