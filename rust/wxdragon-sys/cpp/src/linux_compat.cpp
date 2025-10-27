// Linux compatibility layer for wxDragon
// Provides compatibility shims for missing functions on older Linux systems

#include <cstdio>
#include <cstdlib>
#include <cwchar>
#include <cstdarg>

// Check glibc version to determine if we need compatibility functions
// The __isoc23_ functions are available in glibc 2.38+
// We only provide compatibility if we're on an older system
#ifdef __GLIBC__
#define GLIBC_VERSION (__GLIBC__ * 100 + __GLIBC_MINOR__)
#else
#define GLIBC_VERSION 0
#endif

extern "C" {

// ============================================================================
// GLib Compatibility (GNOME utility library)
// ============================================================================

// Minimal GString structure definition (matches GLib's internal structure)
typedef struct {
    char* str;
    unsigned long len;
    unsigned long allocated_len;
} GString;

// Declare g_string_free as an external function (will be provided by system GLib)
extern char*
g_string_free(GString* string, int free_segment);

// Provide g_string_free_and_steal implementation
// This function is available in GLib 2.76+, but older systems may not have it
char*
g_string_free_and_steal(GString* string)
{
    if (string == nullptr) {
        return nullptr;
    }

    // This is equivalent to g_string_free(string, FALSE)
    // which frees the GString structure but returns the character data
    return g_string_free(string, 0);
}

// ============================================================================
// glibc Compatibility (GNU C Library)
// ============================================================================

// Only provide compatibility functions if we're on glibc < 2.38
// The __isoc23_ symbols are available in glibc 2.38+
#if GLIBC_VERSION > 0 && GLIBC_VERSION < 238

// The __isoc23_ symbols are used in newer glibc versions but may not be available on older systems
// We provide compatibility implementations that call the standard versions

// sscanf family
int
__isoc23_sscanf(const char* str, const char* format, ...)
{
    va_list args;
    va_start(args, format);
    int result = vsscanf(str, format, args);
    va_end(args);
    return result;
}

int
__isoc23_swscanf(const wchar_t* str, const wchar_t* format, ...)
{
    va_list args;
    va_start(args, format);
    int result = vswscanf(str, format, args);
    va_end(args);
    return result;
}

// strtol family
long
__isoc23_strtol(const char* nptr, char** endptr, int base)
{
    return strtol(nptr, endptr, base);
}

long long
__isoc23_strtoll(const char* nptr, char** endptr, int base)
{
    return strtoll(nptr, endptr, base);
}

unsigned long
__isoc23_strtoul(const char* nptr, char** endptr, int base)
{
    return strtoul(nptr, endptr, base);
}

unsigned long long
__isoc23_strtoull(const char* nptr, char** endptr, int base)
{
    return strtoull(nptr, endptr, base);
}

// wcstol family
long
__isoc23_wcstol(const wchar_t* nptr, wchar_t** endptr, int base)
{
    return wcstol(nptr, endptr, base);
}

long long
__isoc23_wcstoll(const wchar_t* nptr, wchar_t** endptr, int base)
{
    return wcstoll(nptr, endptr, base);
}

unsigned long
__isoc23_wcstoul(const wchar_t* nptr, wchar_t** endptr, int base)
{
    return wcstoul(nptr, endptr, base);
}

unsigned long long
__isoc23_wcstoull(const wchar_t* nptr, wchar_t** endptr, int base)
{
    return wcstoull(nptr, endptr, base);
}

// strtod family
double
__isoc23_strtod(const char* nptr, char** endptr)
{
    return strtod(nptr, endptr);
}

float
__isoc23_strtof(const char* nptr, char** endptr)
{
    return strtof(nptr, endptr);
}

long double
__isoc23_strtold(const char* nptr, char** endptr)
{
    return strtold(nptr, endptr);
}

// wcstod family
double
__isoc23_wcstod(const wchar_t* nptr, wchar_t** endptr)
{
    return wcstod(nptr, endptr);
}

float
__isoc23_wcstof(const wchar_t* nptr, wchar_t** endptr)
{
    return wcstof(nptr, endptr);
}

long double
__isoc23_wcstold(const wchar_t* nptr, wchar_t** endptr)
{
    return wcstold(nptr, endptr);
}

#endif // GLIBC_VERSION < 238

} // extern "C"