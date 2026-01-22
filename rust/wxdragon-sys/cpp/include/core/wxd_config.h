#ifndef WXD_CONFIG_H
#define WXD_CONFIG_H

#include "../wxd_types.h"

// --- ConfigBase Creation/Destruction ---

/**
 * Creates a new config object with the specified parameters.
 * Uses the platform-appropriate config implementation (wxFileConfig or wxRegConfig).
 * @param app_name Application name (can be NULL to use wxApp::GetAppName()).
 * @param vendor_name Vendor name (can be NULL).
 * @param local_filename Local config filename (can be NULL).
 * @param global_filename Global config filename (can be NULL).
 * @param style Combination of wxd_ConfigStyle flags.
 * @return Pointer to the new config object, or NULL on failure.
 */
WXD_EXPORTED wxd_ConfigBase_t*
wxd_Config_Create(const char* app_name,
                  const char* vendor_name,
                  const char* local_filename,
                  const char* global_filename,
                  long style);

/**
 * Destroys a config object.
 * @param config Pointer to the config object to destroy.
 */
WXD_EXPORTED void
wxd_Config_Destroy(wxd_ConfigBase_t* config);

// --- Static Functions ---

/**
 * Gets the current global config object.
 * If there is no current object and create_on_demand is true, creates one.
 * @param create_on_demand If true, creates a config object if none exists.
 * @return Pointer to the current config object, or NULL.
 */
WXD_EXPORTED wxd_ConfigBase_t*
wxd_Config_Get(bool create_on_demand);

/**
 * Sets the global config object.
 * @param config The config object to set as current (can be NULL).
 * @return Pointer to the previous config object (can be NULL).
 */
WXD_EXPORTED wxd_ConfigBase_t*
wxd_Config_Set(wxd_ConfigBase_t* config);

// --- Path Management ---

/**
 * Gets the current path.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the path.
 * @param buffer_len Length of the buffer.
 * @return Length of the path, or -1 on error.
 */
WXD_EXPORTED int
wxd_Config_GetPath(const wxd_ConfigBase_t* config, char* buffer, size_t buffer_len);

/**
 * Sets the current path.
 * @param config Pointer to the config object.
 * @param path The path to set.
 */
WXD_EXPORTED void
wxd_Config_SetPath(wxd_ConfigBase_t* config, const char* path);

// --- Read Operations ---

/**
 * Reads a string value.
 * @param config Pointer to the config object.
 * @param key The key to read.
 * @param buffer Buffer to store the value.
 * @param buffer_len Length of the buffer.
 * @param default_val Default value if key not found.
 * @return Length of the value read, or -1 on error.
 */
WXD_EXPORTED int
wxd_Config_ReadString(const wxd_ConfigBase_t* config,
                      const char* key,
                      char* buffer,
                      size_t buffer_len,
                      const char* default_val);

/**
 * Reads a long integer value.
 * @param config Pointer to the config object.
 * @param key The key to read.
 * @param value Pointer to store the value.
 * @param default_val Default value if key not found.
 * @return true if value was found, false if default was used.
 */
WXD_EXPORTED bool
wxd_Config_ReadLong(const wxd_ConfigBase_t* config,
                    const char* key,
                    long* value,
                    long default_val);

/**
 * Reads a double value.
 * @param config Pointer to the config object.
 * @param key The key to read.
 * @param value Pointer to store the value.
 * @param default_val Default value if key not found.
 * @return true if value was found, false if default was used.
 */
WXD_EXPORTED bool
wxd_Config_ReadDouble(const wxd_ConfigBase_t* config,
                      const char* key,
                      double* value,
                      double default_val);

/**
 * Reads a boolean value.
 * @param config Pointer to the config object.
 * @param key The key to read.
 * @param value Pointer to store the value.
 * @param default_val Default value if key not found.
 * @return true if value was found, false if default was used.
 */
WXD_EXPORTED bool
wxd_Config_ReadBool(const wxd_ConfigBase_t* config,
                    const char* key,
                    bool* value,
                    bool default_val);

// --- Write Operations ---

/**
 * Writes a string value.
 * @param config Pointer to the config object.
 * @param key The key to write.
 * @param value The value to write.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_WriteString(wxd_ConfigBase_t* config,
                       const char* key,
                       const char* value);

/**
 * Writes a long integer value.
 * @param config Pointer to the config object.
 * @param key The key to write.
 * @param value The value to write.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_WriteLong(wxd_ConfigBase_t* config,
                     const char* key,
                     long value);

/**
 * Writes a double value.
 * @param config Pointer to the config object.
 * @param key The key to write.
 * @param value The value to write.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_WriteDouble(wxd_ConfigBase_t* config,
                       const char* key,
                       double value);

/**
 * Writes a boolean value.
 * @param config Pointer to the config object.
 * @param key The key to write.
 * @param value The value to write.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_WriteBool(wxd_ConfigBase_t* config,
                     const char* key,
                     bool value);

// --- Existence Tests ---

/**
 * Checks if an entry or group exists.
 * @param config Pointer to the config object.
 * @param name The name to check.
 * @return true if exists, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_Exists(const wxd_ConfigBase_t* config, const char* name);

/**
 * Checks if an entry exists.
 * @param config Pointer to the config object.
 * @param name The entry name to check.
 * @return true if exists, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_HasEntry(const wxd_ConfigBase_t* config, const char* name);

/**
 * Checks if a group exists.
 * @param config Pointer to the config object.
 * @param name The group name to check.
 * @return true if exists, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_HasGroup(const wxd_ConfigBase_t* config, const char* name);

/**
 * Gets the type of an entry.
 * @param config Pointer to the config object.
 * @param name The entry name.
 * @return The entry type (wxd_ConfigEntryType).
 */
WXD_EXPORTED int
wxd_Config_GetEntryType(const wxd_ConfigBase_t* config, const char* name);

// --- Delete Operations ---

/**
 * Deletes an entry.
 * @param config Pointer to the config object.
 * @param key The key to delete.
 * @param delete_group_if_empty If true, delete group if it becomes empty.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_DeleteEntry(wxd_ConfigBase_t* config,
                       const char* key,
                       bool delete_group_if_empty);

/**
 * Deletes a group and all its contents.
 * @param config Pointer to the config object.
 * @param key The group to delete.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_DeleteGroup(wxd_ConfigBase_t* config, const char* key);

/**
 * Deletes all entries and groups.
 * @param config Pointer to the config object.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_DeleteAll(wxd_ConfigBase_t* config);

// --- Enumeration ---

/**
 * Gets the first entry in the current group.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the entry name.
 * @param buffer_len Length of the buffer.
 * @param index Pointer to store the enumeration cookie.
 * @return true if an entry was found, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_GetFirstEntry(const wxd_ConfigBase_t* config,
                         char* buffer,
                         size_t buffer_len,
                         long* index);

/**
 * Gets the next entry in the current group.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the entry name.
 * @param buffer_len Length of the buffer.
 * @param index Pointer to the enumeration cookie.
 * @return true if an entry was found, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_GetNextEntry(const wxd_ConfigBase_t* config,
                        char* buffer,
                        size_t buffer_len,
                        long* index);

/**
 * Gets the first group in the current group.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the group name.
 * @param buffer_len Length of the buffer.
 * @param index Pointer to store the enumeration cookie.
 * @return true if a group was found, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_GetFirstGroup(const wxd_ConfigBase_t* config,
                         char* buffer,
                         size_t buffer_len,
                         long* index);

/**
 * Gets the next group in the current group.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the group name.
 * @param buffer_len Length of the buffer.
 * @param index Pointer to the enumeration cookie.
 * @return true if a group was found, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_GetNextGroup(const wxd_ConfigBase_t* config,
                        char* buffer,
                        size_t buffer_len,
                        long* index);

/**
 * Gets the number of entries in the current group.
 * @param config Pointer to the config object.
 * @param recursive If true, count entries in subgroups too.
 * @return Number of entries.
 */
WXD_EXPORTED size_t
wxd_Config_GetNumberOfEntries(const wxd_ConfigBase_t* config, bool recursive);

/**
 * Gets the number of groups in the current group.
 * @param config Pointer to the config object.
 * @param recursive If true, count groups in subgroups too.
 * @return Number of groups.
 */
WXD_EXPORTED size_t
wxd_Config_GetNumberOfGroups(const wxd_ConfigBase_t* config, bool recursive);

// --- Rename Operations ---

/**
 * Renames an entry.
 * @param config Pointer to the config object.
 * @param old_name The current name.
 * @param new_name The new name.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_RenameEntry(wxd_ConfigBase_t* config,
                       const char* old_name,
                       const char* new_name);

/**
 * Renames a group.
 * @param config Pointer to the config object.
 * @param old_name The current name.
 * @param new_name The new name.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_RenameGroup(wxd_ConfigBase_t* config,
                       const char* old_name,
                       const char* new_name);

// --- Miscellaneous ---

/**
 * Flushes all changes to storage.
 * @param config Pointer to the config object.
 * @param current_only If true, only flush current group.
 * @return true on success, false on failure.
 */
WXD_EXPORTED bool
wxd_Config_Flush(wxd_ConfigBase_t* config, bool current_only);

/**
 * Gets the application name.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the name.
 * @param buffer_len Length of the buffer.
 * @return Length of the name, or -1 on error.
 */
WXD_EXPORTED int
wxd_Config_GetAppName(const wxd_ConfigBase_t* config, char* buffer, size_t buffer_len);

/**
 * Gets the vendor name.
 * @param config Pointer to the config object.
 * @param buffer Buffer to store the name.
 * @param buffer_len Length of the buffer.
 * @return Length of the name, or -1 on error.
 */
WXD_EXPORTED int
wxd_Config_GetVendorName(const wxd_ConfigBase_t* config, char* buffer, size_t buffer_len);

/**
 * Checks if environment variable expansion is enabled.
 * @param config Pointer to the config object.
 * @return true if enabled, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_IsExpandingEnvVars(const wxd_ConfigBase_t* config);

/**
 * Sets whether to expand environment variables.
 * @param config Pointer to the config object.
 * @param do_it true to enable, false to disable.
 */
WXD_EXPORTED void
wxd_Config_SetExpandEnvVars(wxd_ConfigBase_t* config, bool do_it);

/**
 * Checks if recording defaults is enabled.
 * @param config Pointer to the config object.
 * @return true if enabled, false otherwise.
 */
WXD_EXPORTED bool
wxd_Config_IsRecordingDefaults(const wxd_ConfigBase_t* config);

/**
 * Sets whether to record defaults.
 * @param config Pointer to the config object.
 * @param do_it true to enable, false to disable.
 */
WXD_EXPORTED void
wxd_Config_SetRecordDefaults(wxd_ConfigBase_t* config, bool do_it);

#endif // WXD_CONFIG_H
