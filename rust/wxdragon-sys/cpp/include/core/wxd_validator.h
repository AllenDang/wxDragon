#ifndef WXD_VALIDATOR_H
#define WXD_VALIDATOR_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- wxTextValidator ---

/// Creates a text validator using the given wxFILTER_XXX style flags.
WXD_EXPORTED wxd_TextValidator_t*
wxd_TextValidator_Create(int64_t style);

/// Destroys a validator created by wxd_TextValidator_Create().
WXD_EXPORTED void
wxd_TextValidator_Destroy(wxd_TextValidator_t* self);

/// Sets the characters which are allowed, used with wxFILTER_INCLUDE_CHAR_LIST.
WXD_EXPORTED void
wxd_TextValidator_SetCharIncludes(wxd_TextValidator_t* self, const char* chars);

/// Sets the characters which are not allowed, used with wxFILTER_EXCLUDE_CHAR_LIST.
WXD_EXPORTED void
wxd_TextValidator_SetCharExcludes(wxd_TextValidator_t* self, const char* chars);

/// Sets the strings which are accepted, used with wxFILTER_INCLUDE_LIST.
WXD_EXPORTED void
wxd_TextValidator_SetIncludes(wxd_TextValidator_t* self, const char* const* items, size_t count);

/// Sets the strings which are rejected, used with wxFILTER_EXCLUDE_LIST.
WXD_EXPORTED void
wxd_TextValidator_SetExcludes(wxd_TextValidator_t* self, const char* const* items, size_t count);

/// Associates a copy of the validator with the window, the caller keeps the original.
WXD_EXPORTED void
wxd_Window_SetTextValidator(wxd_Window_t* self, wxd_TextValidator_t* validator);

/// Validates the contents of the window and of all its children.
WXD_EXPORTED bool
wxd_Window_Validate(wxd_Window_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_VALIDATOR_H
