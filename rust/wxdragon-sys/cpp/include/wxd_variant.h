#ifndef WXD_VARIANT_H
#define WXD_VARIANT_H 1

#include "wxd_types.h"

// C-compatible variant type, it represents wxVariant in C
typedef struct wxd_Variant_t wxd_Variant_t;

// clang-format off
#ifdef __cplusplus
extern "C" {
#endif

// New typed API (preferred)
WXD_EXPORTED wxd_Variant_t*
wxd_Variant_CreateEmpty(void);

/**
 * Clone the variant. Returns nullptr if input is nullptr.
 * If not nullptr, the caller is responsible for destroying the returned variant.
 */
WXD_EXPORTED wxd_Variant_t*
wxd_Variant_Clone(const wxd_Variant_t* variant);

WXD_EXPORTED void
wxd_Variant_Destroy(wxd_Variant_t* variant);

WXD_EXPORTED bool
wxd_Variant_IsNull(const wxd_Variant_t* variant);

WXD_EXPORTED void
wxd_Variant_MakeNull(wxd_Variant_t* variant);

/**
 * Returned buffer contains a string representing the type of the variant, e.g. "string", "bool", "list", "double", "long".
 * The returned value is required UTF-8 byte length (excluding NUL). If out==NULL or out_len==0, just return length.
 * Otherwise, copies up to out_len-1 bytes and NUL-terminates. Always returns required length.
 */
WXD_EXPORTED int
wxd_Variant_GetTypeName_Utf8(const wxd_Variant_t* variant, char* out, size_t out_len);

// Setters
WXD_EXPORTED void
wxd_Variant_SetBool(wxd_Variant_t* variant, bool value);

WXD_EXPORTED void
wxd_Variant_SetInt32(wxd_Variant_t* variant, int32_t value);

WXD_EXPORTED void
wxd_Variant_SetInt64(wxd_Variant_t* variant, int64_t value);

WXD_EXPORTED void
wxd_Variant_SetDouble(wxd_Variant_t* variant, double value);

/**
 * Set a UTF-8 string, s may be null-terminated (if len < 0) or length-specified (if len >= 0).
 */
WXD_EXPORTED void
wxd_Variant_SetString_Utf8(wxd_Variant_t* variant, const char* s, int len);

WXD_EXPORTED void
wxd_Variant_SetDateTime(wxd_Variant_t* variant, const wxd_DateTime_t* value);

// Bitmap: store by value inside wxVariant (RC+COW). If bmp is null or invalid, makes variant null.
WXD_EXPORTED void
wxd_Variant_SetBitmap(wxd_Variant_t* variant, const wxd_Bitmap_t* bmp);

// Getters (return false if cannot convert)
WXD_EXPORTED bool
wxd_Variant_GetBool(const wxd_Variant_t* variant, bool* out_value);

WXD_EXPORTED bool
wxd_Variant_GetInt32(const wxd_Variant_t* variant, int32_t* out_value);

WXD_EXPORTED bool
wxd_Variant_GetInt64(const wxd_Variant_t* variant, int64_t* out_value);

WXD_EXPORTED bool
wxd_Variant_GetDouble(const wxd_Variant_t* variant, double* out_value);

/**
 * Returns the required UTF-8 byte length (excluding NUL). If out==NULL or out_len==0, just return length.
 * Otherwise, copies up to out_len-1 bytes and NUL-terminates. Always returns required length.
 */
WXD_EXPORTED int
wxd_Variant_GetString_Utf8(const wxd_Variant_t* variant, char* out, size_t out_len);

WXD_EXPORTED wxd_DateTime_t*
wxd_Variant_GetDateTime(const wxd_Variant_t* variant);

// Bitmap: returns a new heap-allocated clone on success; caller must destroy with wxd_Bitmap_Destroy.
WXD_EXPORTED wxd_Bitmap_t*
wxd_Variant_GetBitmapClone(const wxd_Variant_t* variant);

// ArrayString: store by value inside wxVariant using custom VariantData. If arr is null, makes variant null.
WXD_EXPORTED void
wxd_Variant_SetArrayString(wxd_Variant_t* variant, const wxd_ArrayString_t* arr);

// ArrayString: returns a new heap-allocated clone on success; caller must destroy with wxd_ArrayString_Free.
WXD_EXPORTED wxd_ArrayString_t*
wxd_Variant_GetArrayStringClone(const wxd_Variant_t* variant);

#ifdef __cplusplus
}
#endif
// clang-format on

#endif // WXD_VARIANT_H
