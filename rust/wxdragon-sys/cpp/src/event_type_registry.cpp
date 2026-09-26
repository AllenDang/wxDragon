#include <wx/wxprec.h>
#include <wx/wx.h>

#include <vector>

#include "../include/wxdragon.h"
#include "../include/events/wxd_event_type_registry.h"

namespace
{

std::vector<WxdEventTypeMapper>&
GetMappers()
{
    static std::vector<WxdEventTypeMapper> mappers;
    return mappers;
}

} // anonymous namespace

void
wxd_RegisterEventTypeMapper(WxdEventTypeMapper mapper)
{
    GetMappers().push_back(mapper);
}

wxEventType
wxd_LookupRegisteredEventType(WXDEventTypeCEnum c_enum_val)
{
    for (WxdEventTypeMapper mapper : GetMappers()) {
        const wxEventType eventType = mapper(c_enum_val);
        if (eventType != wxEVT_NULL) {
            return eventType;
        }
    }

    return wxEVT_NULL;
}
