#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/config.h>
#include <wx/fileconf.h>

extern "C" {

// Helper to get wxConfigBase pointer
static wxConfigBase* get_config(wxd_ConfigBase_t* config) {
    return reinterpret_cast<wxConfigBase*>(config);
}

static const wxConfigBase* get_config_const(const wxd_ConfigBase_t* config) {
    return reinterpret_cast<const wxConfigBase*>(config);
}

// --- Creation/Destruction ---

wxd_ConfigBase_t*
wxd_Config_Create(const char* app_name,
                  const char* vendor_name,
                  const char* local_filename,
                  const char* global_filename,
                  long style)
{
    wxString appName;
    if (app_name) appName = wxString::FromUTF8(app_name);

    wxString vendorName;
    if (vendor_name) vendorName = wxString::FromUTF8(vendor_name);

    wxString localFilename;
    if (local_filename) localFilename = wxString::FromUTF8(local_filename);

    wxString globalFilename;
    if (global_filename) globalFilename = wxString::FromUTF8(global_filename);

    // Use wxFileConfig for cross-platform consistency
    wxConfigBase* config = new wxFileConfig(appName, vendorName, localFilename, globalFilename, style);
    return reinterpret_cast<wxd_ConfigBase_t*>(config);
}

void
wxd_Config_Destroy(wxd_ConfigBase_t* config)
{
    if (config) {
        wxConfigBase* cfg = get_config(config);
        // If this is the global config, clear the global pointer first
        if (wxConfigBase::Get(false) == cfg) {
            wxConfigBase::Set(nullptr);
        }
        delete cfg;
    }
}

// --- Static Functions ---

wxd_ConfigBase_t*
wxd_Config_Get(bool create_on_demand)
{
    return reinterpret_cast<wxd_ConfigBase_t*>(wxConfigBase::Get(create_on_demand));
}

wxd_ConfigBase_t*
wxd_Config_Set(wxd_ConfigBase_t* config)
{
    wxConfigBase* prev = wxConfigBase::Set(get_config(config));
    return reinterpret_cast<wxd_ConfigBase_t*>(prev);
}

// --- Path Management ---

int
wxd_Config_GetPath(const wxd_ConfigBase_t* config, char* buffer, size_t buffer_len)
{
    if (!config)
        return -1;
    const wxConfigBase* cfg = get_config_const(config);
    wxString path = cfg->GetPath();
    return wxd_cpp_utils::copy_wxstring_to_buffer(path, buffer, buffer_len);
}

void
wxd_Config_SetPath(wxd_ConfigBase_t* config, const char* path)
{
    if (!config || !path)
        return;
    wxConfigBase* cfg = get_config(config);
    cfg->SetPath(wxString::FromUTF8(path));
}

// --- Read Operations ---

int
wxd_Config_ReadString(const wxd_ConfigBase_t* config,
                      const char* key,
                      char* buffer,
                      size_t buffer_len,
                      const char* default_val)
{
    if (!config || !key)
        return -1;
    const wxConfigBase* cfg = get_config_const(config);
    wxString defVal;
    if (default_val) defVal = wxString::FromUTF8(default_val);
    wxString value = cfg->Read(wxString::FromUTF8(key), defVal);
    return wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len);
}

bool
wxd_Config_ReadLong(const wxd_ConfigBase_t* config,
                    const char* key,
                    long* value,
                    long default_val)
{
    if (!config || !key || !value)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->Read(wxString::FromUTF8(key), value, default_val);
}

bool
wxd_Config_ReadDouble(const wxd_ConfigBase_t* config,
                      const char* key,
                      double* value,
                      double default_val)
{
    if (!config || !key || !value)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->Read(wxString::FromUTF8(key), value, default_val);
}

bool
wxd_Config_ReadBool(const wxd_ConfigBase_t* config,
                    const char* key,
                    bool* value,
                    bool default_val)
{
    if (!config || !key || !value)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->Read(wxString::FromUTF8(key), value, default_val);
}

// --- Write Operations ---

bool
wxd_Config_WriteString(wxd_ConfigBase_t* config,
                       const char* key,
                       const char* value)
{
    if (!config || !key)
        return false;
    wxConfigBase* cfg = get_config(config);
    wxString val;
    if (value) val = wxString::FromUTF8(value);
    return cfg->Write(wxString::FromUTF8(key), val);
}

bool
wxd_Config_WriteLong(wxd_ConfigBase_t* config,
                     const char* key,
                     long value)
{
    if (!config || !key)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->Write(wxString::FromUTF8(key), value);
}

bool
wxd_Config_WriteDouble(wxd_ConfigBase_t* config,
                       const char* key,
                       double value)
{
    if (!config || !key)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->Write(wxString::FromUTF8(key), value);
}

bool
wxd_Config_WriteBool(wxd_ConfigBase_t* config,
                     const char* key,
                     bool value)
{
    if (!config || !key)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->Write(wxString::FromUTF8(key), value);
}

// --- Existence Tests ---

bool
wxd_Config_Exists(const wxd_ConfigBase_t* config, const char* name)
{
    if (!config || !name)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->Exists(wxString::FromUTF8(name));
}

bool
wxd_Config_HasEntry(const wxd_ConfigBase_t* config, const char* name)
{
    if (!config || !name)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->HasEntry(wxString::FromUTF8(name));
}

bool
wxd_Config_HasGroup(const wxd_ConfigBase_t* config, const char* name)
{
    if (!config || !name)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->HasGroup(wxString::FromUTF8(name));
}

int
wxd_Config_GetEntryType(const wxd_ConfigBase_t* config, const char* name)
{
    if (!config || !name)
        return WXD_CONFIG_TYPE_UNKNOWN;
    const wxConfigBase* cfg = get_config_const(config);
    wxConfigBase::EntryType type = cfg->GetEntryType(wxString::FromUTF8(name));
    switch (type) {
        case wxConfigBase::Type_String:
            return WXD_CONFIG_TYPE_STRING;
        case wxConfigBase::Type_Boolean:
            return WXD_CONFIG_TYPE_BOOLEAN;
        case wxConfigBase::Type_Integer:
            return WXD_CONFIG_TYPE_INTEGER;
        case wxConfigBase::Type_Float:
            return WXD_CONFIG_TYPE_FLOAT;
        default:
            return WXD_CONFIG_TYPE_UNKNOWN;
    }
}

// --- Delete Operations ---

bool
wxd_Config_DeleteEntry(wxd_ConfigBase_t* config,
                       const char* key,
                       bool delete_group_if_empty)
{
    if (!config || !key)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->DeleteEntry(wxString::FromUTF8(key), delete_group_if_empty);
}

bool
wxd_Config_DeleteGroup(wxd_ConfigBase_t* config, const char* key)
{
    if (!config || !key)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->DeleteGroup(wxString::FromUTF8(key));
}

bool
wxd_Config_DeleteAll(wxd_ConfigBase_t* config)
{
    if (!config)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->DeleteAll();
}

// --- Enumeration ---

bool
wxd_Config_GetFirstEntry(const wxd_ConfigBase_t* config,
                         char* buffer,
                         size_t buffer_len,
                         long* index)
{
    if (!config || !index)
        return false;
    // Need to cast away const because wxWidgets enumeration modifies internal state
    wxConfigBase* cfg = const_cast<wxConfigBase*>(get_config_const(config));
    wxString str;
    bool result = cfg->GetFirstEntry(str, *index);
    if (result && buffer && buffer_len > 0) {
        wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, buffer_len);
    }
    return result;
}

bool
wxd_Config_GetNextEntry(const wxd_ConfigBase_t* config,
                        char* buffer,
                        size_t buffer_len,
                        long* index)
{
    if (!config || !index)
        return false;
    wxConfigBase* cfg = const_cast<wxConfigBase*>(get_config_const(config));
    wxString str;
    bool result = cfg->GetNextEntry(str, *index);
    if (result && buffer && buffer_len > 0) {
        wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, buffer_len);
    }
    return result;
}

bool
wxd_Config_GetFirstGroup(const wxd_ConfigBase_t* config,
                         char* buffer,
                         size_t buffer_len,
                         long* index)
{
    if (!config || !index)
        return false;
    wxConfigBase* cfg = const_cast<wxConfigBase*>(get_config_const(config));
    wxString str;
    bool result = cfg->GetFirstGroup(str, *index);
    if (result && buffer && buffer_len > 0) {
        wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, buffer_len);
    }
    return result;
}

bool
wxd_Config_GetNextGroup(const wxd_ConfigBase_t* config,
                        char* buffer,
                        size_t buffer_len,
                        long* index)
{
    if (!config || !index)
        return false;
    wxConfigBase* cfg = const_cast<wxConfigBase*>(get_config_const(config));
    wxString str;
    bool result = cfg->GetNextGroup(str, *index);
    if (result && buffer && buffer_len > 0) {
        wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, buffer_len);
    }
    return result;
}

size_t
wxd_Config_GetNumberOfEntries(const wxd_ConfigBase_t* config, bool recursive)
{
    if (!config)
        return 0;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->GetNumberOfEntries(recursive);
}

size_t
wxd_Config_GetNumberOfGroups(const wxd_ConfigBase_t* config, bool recursive)
{
    if (!config)
        return 0;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->GetNumberOfGroups(recursive);
}

// --- Rename Operations ---

bool
wxd_Config_RenameEntry(wxd_ConfigBase_t* config,
                       const char* old_name,
                       const char* new_name)
{
    if (!config || !old_name || !new_name)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->RenameEntry(wxString::FromUTF8(old_name), wxString::FromUTF8(new_name));
}

bool
wxd_Config_RenameGroup(wxd_ConfigBase_t* config,
                       const char* old_name,
                       const char* new_name)
{
    if (!config || !old_name || !new_name)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->RenameGroup(wxString::FromUTF8(old_name), wxString::FromUTF8(new_name));
}

// --- Miscellaneous ---

bool
wxd_Config_Flush(wxd_ConfigBase_t* config, bool current_only)
{
    if (!config)
        return false;
    wxConfigBase* cfg = get_config(config);
    return cfg->Flush(current_only);
}

int
wxd_Config_GetAppName(const wxd_ConfigBase_t* config, char* buffer, size_t buffer_len)
{
    if (!config)
        return -1;
    const wxConfigBase* cfg = get_config_const(config);
    wxString name = cfg->GetAppName();
    return wxd_cpp_utils::copy_wxstring_to_buffer(name, buffer, buffer_len);
}

int
wxd_Config_GetVendorName(const wxd_ConfigBase_t* config, char* buffer, size_t buffer_len)
{
    if (!config)
        return -1;
    const wxConfigBase* cfg = get_config_const(config);
    wxString name = cfg->GetVendorName();
    return wxd_cpp_utils::copy_wxstring_to_buffer(name, buffer, buffer_len);
}

bool
wxd_Config_IsExpandingEnvVars(const wxd_ConfigBase_t* config)
{
    if (!config)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->IsExpandingEnvVars();
}

void
wxd_Config_SetExpandEnvVars(wxd_ConfigBase_t* config, bool do_it)
{
    if (!config)
        return;
    wxConfigBase* cfg = get_config(config);
    cfg->SetExpandEnvVars(do_it);
}

bool
wxd_Config_IsRecordingDefaults(const wxd_ConfigBase_t* config)
{
    if (!config)
        return false;
    const wxConfigBase* cfg = get_config_const(config);
    return cfg->IsRecordingDefaults();
}

void
wxd_Config_SetRecordDefaults(wxd_ConfigBase_t* config, bool do_it)
{
    if (!config)
        return;
    wxConfigBase* cfg = get_config(config);
    cfg->SetRecordDefaults(do_it);
}

} // extern "C"
