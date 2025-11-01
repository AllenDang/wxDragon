#include <wx/wxprec.h>
#include <wx/wx.h>

#include <wx/bitmap.h>
#include <wx/longlong.h>
#include <wx/string.h>
#include <wx/variant.h>
#include <wx/arrstr.h>
#if wxUSE_DATETIME
#include <wx/datetime.h>
#endif
#include <cstring>
#include <new>

#include "../include/wxdragon.h"
#include "../include/wxd_variant.h"

// Opaque handle is actually a wxVariant
static inline const wxVariant*
as_wx_const(const wxd_Variant_t* v)
{
    return reinterpret_cast<const wxVariant*>(v);
}

static inline const wxVariant*
as_wx_const(wxd_Variant_t* v)
{
    return reinterpret_cast<const wxVariant*>(v);
}

static inline wxVariant*
as_wx_mut(wxd_Variant_t* v)
{
    return reinterpret_cast<wxVariant*>(v);
}

static inline wxVariant*
as_wx_mut(const wxd_Variant_t* v)
{
    return const_cast<wxVariant*>(reinterpret_cast<const wxVariant*>(v));
}

// Local variant data for wxBitmap stored by value inside wxVariant
namespace {
class wxd_BitmapVariantData : public wxVariantData {
public:
    wxd_BitmapVariantData() = default;
    explicit wxd_BitmapVariantData(const wxBitmap& bmp) : m_value(bmp)
    {
    }

    wxBitmap&
    GetValue()
    {
        return m_value;
    }
    const wxBitmap&
    GetValue() const
    {
        return m_value;
    }

    bool
    Eq(wxVariantData& data) const override
    {
        wxd_BitmapVariantData* other = static_cast<wxd_BitmapVariantData*>(&data);
        if (!other)
            return false;
        if (!m_value.IsOk() || !other->m_value.IsOk())
            return m_value.IsOk() == other->m_value.IsOk();
        return m_value.GetWidth() == other->m_value.GetWidth() &&
               m_value.GetHeight() == other->m_value.GetHeight();
    }

    wxString
    GetType() const override
    {
        return wxString("wxBitmap");
    }

    wxVariantData*
    Clone() const override
    {
        return new wxd_BitmapVariantData(m_value);
    }

    wxClassInfo*
    GetValueClassInfo() override
    {
        return nullptr;
    }

private:
    wxBitmap m_value;
};

static void
wxd_SetBitmapVariant(wxVariant& v, const wxBitmap& bmp)
{
    v.SetData(new wxd_BitmapVariantData(bmp));
}

static const wxBitmap*
wxd_GetBitmapFromVariant(const wxVariant& v)
{
    if (v.IsNull())
        return nullptr;
    if (v.GetType() != wxString("wxBitmap"))
        return nullptr;
    wxVariantData* data = v.GetData();
    auto* bd = static_cast<wxd_BitmapVariantData*>(data);
    if (!bd)
        return nullptr;
    return &bd->GetValue();
}
} // namespace

extern "C" WXD_EXPORTED wxd_Variant_t*
wxd_Variant_CreateEmpty(void)
{
    wxVariant* v = new (std::nothrow) wxVariant();
    return reinterpret_cast<wxd_Variant_t*>(v);
}

// Clone the variant. Returns nullptr if input is nullptr.
// If not nullptr, the caller is responsible for destroying the returned variant.
extern "C" WXD_EXPORTED wxd_Variant_t*
wxd_Variant_Clone(const wxd_Variant_t* variant)
{
    if (!variant) {
        return nullptr;
    }
    const wxVariant* src = as_wx_const(variant);
    wxVariant* v = new (std::nothrow) wxVariant(*src);
    return reinterpret_cast<wxd_Variant_t*>(v);
}

extern "C" WXD_EXPORTED void
wxd_Variant_Destroy(wxd_Variant_t* variant)
{
    wxVariant* v = as_wx_mut(variant);
    delete v;
}

extern "C" WXD_EXPORTED bool
wxd_Variant_IsNull(const wxd_Variant_t* variant)
{
    const wxVariant* v = as_wx_const(variant);
    if (!v)
        return true;
    return v->IsNull();
}

extern "C" WXD_EXPORTED void
wxd_Variant_MakeNull(wxd_Variant_t* variant)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
    v->MakeNull();
}

// Returned buffer contains a string representing the type of the variant, e.g. "string", "bool", "list", "double", "long".
// The returned value is required UTF-8 byte length (excluding NUL). If out==NULL or out_len==0, just return length.
// Otherwise, copies up to out_len-1 bytes and NUL-terminates. Always returns required length.
extern "C" WXD_EXPORTED int
wxd_Variant_GetTypeName_Utf8(const wxd_Variant_t* variant, char* out, size_t out_len)
{
    if (!variant)
        return -1;
    const wxVariant* v = as_wx_const(variant);
    wxString t = v->GetType();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(t, out, out_len);
}

// Setters
extern "C" WXD_EXPORTED void
wxd_Variant_SetBool(wxd_Variant_t* variant, bool value)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
    *v = value;
}

extern "C" WXD_EXPORTED void
wxd_Variant_SetInt32(wxd_Variant_t* variant, int32_t value)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
    // Store as long. Note: long width varies by platform, but 32-bit values fit
    // in either.
    *v = static_cast<long>(value);
}

extern "C" WXD_EXPORTED void
wxd_Variant_SetInt64(wxd_Variant_t* variant, int64_t value)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
    *v = wxLongLong(value);
}

extern "C" WXD_EXPORTED void
wxd_Variant_SetDouble(wxd_Variant_t* variant, double value)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
    *v = value;
}

// Set a UTF-8 string, s may be null-terminated (if len < 0) or length-specified (if len >= 0).
extern "C" WXD_EXPORTED void
wxd_Variant_SetString_Utf8(wxd_Variant_t* variant, const char* s, int len)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
    if (!s) {
        *v = wxString();
        return;
    }
    // If len < 0, treat s as null-terminated
    size_t n = len >= 0 ? static_cast<size_t>(len) : std::strlen(s);
    if (n > static_cast<size_t>(INT_MAX)) {
        // Truncate to INT_MAX to avoid overflow
        n = static_cast<size_t>(INT_MAX);
    }
    wxString ws = wxString::FromUTF8(s, static_cast<int>(n));
    *v = ws;
}

extern "C" WXD_EXPORTED void
wxd_Variant_SetDateTime(wxd_Variant_t* variant, const wxd_DateTime_t* value)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v)
        return;
#if wxUSE_DATETIME
    const wxDateTime* dt = reinterpret_cast<const wxDateTime*>(value);
    if (dt) {
        *v = *dt;
    }
    else {
        v->MakeNull();
    }
#else
    (void)value;
    v->MakeNull();
#endif
}

extern "C" WXD_EXPORTED void
wxd_Variant_SetBitmap(wxd_Variant_t* variant, const wxd_Bitmap_t* bmp)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v) {
        return;
    }
    const wxBitmap* wb = reinterpret_cast<const wxBitmap*>(bmp);
    if (!wb || !wb->IsOk()) {
        v->MakeNull();
        return;
    }
    // Store by value (RC+COW). This does not take ownership of wb.
    wxd_SetBitmapVariant(*v, *wb);
}

// Getters
extern "C" WXD_EXPORTED bool
wxd_Variant_GetBool(const wxd_Variant_t* variant, bool* out_value)
{
    if (!variant || !out_value)
        return false;
    const wxVariant* v = as_wx_const(variant);
    bool tmp = false;
    if (!v->Convert(&tmp))
        return false;
    *out_value = tmp;
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_Variant_GetInt32(const wxd_Variant_t* variant, int32_t* out_value)
{
    if (!variant || !out_value)
        return false;
    const wxVariant* v = as_wx_const(variant);
    long lv = 0;
    if (!v->Convert(&lv))
        return false;
    // Range-check for 32-bit
    if (lv < INT32_MIN || lv > INT32_MAX)
        return false;
    *out_value = static_cast<int32_t>(lv);
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_Variant_GetInt64(const wxd_Variant_t* variant, int64_t* out_value)
{
    if (!variant || !out_value)
        return false;
    const wxVariant* v = as_wx_const(variant);
    wxLongLong ll;
    if (!v->Convert(&ll))
        return false;
    *out_value = static_cast<int64_t>(ll.GetValue());
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_Variant_GetDouble(const wxd_Variant_t* variant, double* out_value)
{
    if (!variant || !out_value)
        return false;
    const wxVariant* v = as_wx_const(variant);
    double dv = 0.0;
    if (!v->Convert(&dv)) {
        return false;
    }
    *out_value = dv;
    return true;
}

extern "C" WXD_EXPORTED int
wxd_Variant_GetString_Utf8(const wxd_Variant_t* variant, char* out, size_t out_len)
{
    if (!variant)
        return -1;
    const wxVariant* v = as_wx_const(variant);
    wxString s;
    if (!v->Convert(&s)) {
        // Not convertible to string
        if (out && out_len > 0)
            out[0] = '\0';
        return -1;
    }
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(s, out, out_len);
}

extern "C" WXD_EXPORTED wxd_DateTime_t*
wxd_Variant_GetDateTime(const wxd_Variant_t* variant)
{
    if (!variant)
        return nullptr;
#if wxUSE_DATETIME
    const wxVariant* v = as_wx_const(variant);
    wxDateTime dt;
    if (!v->Convert(&dt))
        return nullptr;
    wxDateTime* cloned = new (std::nothrow) wxDateTime(dt);
    if (!cloned)
        return nullptr;
    return reinterpret_cast<wxd_DateTime_t*>(cloned);
#else
    (void)variant;
    return nullptr;
#endif
}

extern "C" WXD_EXPORTED wxd_Bitmap_t*
wxd_Variant_GetBitmapClone(const wxd_Variant_t* variant)
{
    if (!variant)
        return nullptr;
    const wxVariant* v = as_wx_const(variant);
    const wxBitmap* bmp = wxd_GetBitmapFromVariant(*v);
    if (!bmp || !bmp->IsOk())
        return nullptr;
    wxBitmap* cloned = new (std::nothrow) wxBitmap(*bmp);
    if (!cloned || !cloned->IsOk()) {
        delete cloned;
        return nullptr;
    }
    return reinterpret_cast<wxd_Bitmap_t*>(cloned);
}

extern "C" WXD_EXPORTED void
wxd_Variant_SetArrayString(wxd_Variant_t* variant, const wxd_ArrayString_t* arr)
{
    wxVariant* v = as_wx_mut(variant);
    if (!v) {
        return;
    }
    if (!arr) {
        v->MakeNull();
        return;
    }
    const wxArrayString* wa = reinterpret_cast<const wxArrayString*>(arr);
    *v = *wa;
}

extern "C" WXD_EXPORTED wxd_ArrayString_t*
wxd_Variant_GetArrayStringClone(const wxd_Variant_t* variant)
{
    if (!variant)
        return nullptr;
    const wxVariant* v = as_wx_const(variant);
    // Prefer robust type checks; wxVariant commonly uses "arrstring" as the type name.
    if (!(v->IsType("arrstring") || v->IsType("stringlist") || v->IsType("wxArrayString"))) {
        return nullptr;
    }
    wxArrayString arr = v->GetArrayString();
    wxArrayString* cloned = new (std::nothrow) wxArrayString(arr);
    if (!cloned)
        return nullptr;
    return reinterpret_cast<wxd_ArrayString_t*>(cloned);
}
