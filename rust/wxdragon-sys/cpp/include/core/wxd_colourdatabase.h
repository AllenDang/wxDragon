#ifndef WXD_COLOURDATABASE_H
#define WXD_COLOURDATABASE_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Colour scheme used by the global colour database. CSS is the wxWidgets 3.3.0+ default
// (matching CSS/web colour names); Traditional is the legacy scheme used before that.
typedef enum {
    WXD_COLOUR_DATABASE_SCHEME_CSS = 0,
    WXD_COLOUR_DATABASE_SCHEME_TRADITIONAL = 1
} wxd_ColourDatabaseScheme;

// Selects which colour scheme the database's built-in names use.
WXD_EXPORTED void
wxd_ColourDatabase_UseScheme(wxd_ColourDatabaseScheme scheme);

// Looks up a colour by name (e.g. "MEDIUM FOREST GREEN"), case-insensitive.
// Returns true and fills `out_colour` if found, false (leaving `out_colour` untouched) otherwise.
WXD_EXPORTED bool
wxd_ColourDatabase_Find(const char* name, wxd_Colour_t* out_colour);

// Looks up the name for a colour. Returns the required buffer length in bytes (excluding
// the null terminator), or 0 if the colour has no name in the database. Follows the same
// call-twice pattern as the other wxd string getters.
WXD_EXPORTED int
wxd_ColourDatabase_FindName(wxd_Colour_t colour, char* buffer, size_t buffer_len);

// Adds (or overwrites) a named colour in the database.
WXD_EXPORTED void
wxd_ColourDatabase_AddColour(const char* name, wxd_Colour_t colour);

// Returns all colour names currently known to the database, as a newly allocated
// wxd_ArrayString_t* owned by the caller (free with wxd_ArrayString_Free).
WXD_EXPORTED wxd_ArrayString_t*
wxd_ColourDatabase_GetAllNames(void);

#ifdef __cplusplus
}
#endif

#endif // WXD_COLOURDATABASE_H
