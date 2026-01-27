#ifndef WXD_IPC_H
#define WXD_IPC_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- Opaque Types ---
typedef struct wxd_IPCConnection_t wxd_IPCConnection_t;
typedef struct wxd_IPCServer_t wxd_IPCServer_t;
typedef struct wxd_IPCClient_t wxd_IPCClient_t;

// --- IPC Format ---
typedef enum {
    WXD_IPC_TEXT = 1,           // CF_TEXT
    WXD_IPC_BITMAP = 2,         // CF_BITMAP
    WXD_IPC_METAFILE = 3,       // CF_METAFILEPICT
    WXD_IPC_UNICODETEXT = 13,   // CF_UNICODETEXT
    WXD_IPC_UTF8TEXT = 14,      // UTF-8 text
    WXD_IPC_PRIVATE = 20        // Private/binary data
} wxd_IPCFormat;

// --- Connection Callbacks (Server-side: called when client sends data) ---

// Called when client executes a command via Execute()
// Return true if handled successfully
typedef bool (*wxd_IPC_OnExecute_Callback)(
    void* user_data,
    const char* topic,
    const void* data,
    size_t size,
    wxd_IPCFormat format
);

// Called when client requests data via Request()
// Return pointer to data (must remain valid until next call), set out_size
// Return NULL if request cannot be fulfilled
typedef const void* (*wxd_IPC_OnRequest_Callback)(
    void* user_data,
    const char* topic,
    const char* item,
    size_t* out_size,
    wxd_IPCFormat format
);

// Called when client pokes data via Poke()
// Return true if handled successfully
typedef bool (*wxd_IPC_OnPoke_Callback)(
    void* user_data,
    const char* topic,
    const char* item,
    const void* data,
    size_t size,
    wxd_IPCFormat format
);

// Called when client starts an advise loop
// Return true to accept
typedef bool (*wxd_IPC_OnStartAdvise_Callback)(
    void* user_data,
    const char* topic,
    const char* item
);

// Called when client stops an advise loop
// Return true if stopped successfully
typedef bool (*wxd_IPC_OnStopAdvise_Callback)(
    void* user_data,
    const char* topic,
    const char* item
);

// --- Connection Callbacks (Client-side: called when server sends data) ---

// Called when server sends advised data
// Return true if handled successfully
typedef bool (*wxd_IPC_OnAdvise_Callback)(
    void* user_data,
    const char* topic,
    const char* item,
    const void* data,
    size_t size,
    wxd_IPCFormat format
);

// --- Connection Callbacks (Both sides) ---

// Called when connection is terminated
// Return true to allow default cleanup (delete connection)
typedef bool (*wxd_IPC_OnDisconnect_Callback)(void* user_data);

// Cleanup callback to free user data
typedef void (*wxd_IPC_FreeUserData_Callback)(void* user_data);

// --- Server Callbacks ---

// Called when a new client connects
// Should return a new connection object, or NULL to reject
// The topic parameter indicates what topic the client wants to connect to
typedef wxd_IPCConnection_t* (*wxd_IPC_OnAcceptConnection_Callback)(
    void* user_data,
    const char* topic
);

// --- Connection Functions ---

// Create a new connection with callbacks (typically called from OnAcceptConnection or OnMakeConnection)
WXD_EXPORTED wxd_IPCConnection_t*
wxd_IPCConnection_Create(
    void* user_data,
    wxd_IPC_OnExecute_Callback on_execute,
    wxd_IPC_OnRequest_Callback on_request,
    wxd_IPC_OnPoke_Callback on_poke,
    wxd_IPC_OnStartAdvise_Callback on_start_advise,
    wxd_IPC_OnStopAdvise_Callback on_stop_advise,
    wxd_IPC_OnAdvise_Callback on_advise,
    wxd_IPC_OnDisconnect_Callback on_disconnect,
    wxd_IPC_FreeUserData_Callback free_user_data
);

// Destroy a connection
WXD_EXPORTED void
wxd_IPCConnection_Destroy(wxd_IPCConnection_t* conn);

// --- Client-side Connection Methods (send data to server) ---

// Execute a command on the server
WXD_EXPORTED bool
wxd_IPCConnection_Execute(
    wxd_IPCConnection_t* conn,
    const void* data,
    size_t size,
    wxd_IPCFormat format
);

// Execute a string command (convenience wrapper for text)
WXD_EXPORTED bool
wxd_IPCConnection_ExecuteString(
    wxd_IPCConnection_t* conn,
    const char* data
);

// Request data from the server
// Returns pointer to data, sets out_size. Caller must copy data before next call.
// Returns NULL on failure.
WXD_EXPORTED const void*
wxd_IPCConnection_Request(
    wxd_IPCConnection_t* conn,
    const char* item,
    size_t* out_size,
    wxd_IPCFormat format
);

// Poke data to the server
WXD_EXPORTED bool
wxd_IPCConnection_Poke(
    wxd_IPCConnection_t* conn,
    const char* item,
    const void* data,
    size_t size,
    wxd_IPCFormat format
);

// Start an advise loop for an item
WXD_EXPORTED bool
wxd_IPCConnection_StartAdvise(wxd_IPCConnection_t* conn, const char* item);

// Stop an advise loop for an item
WXD_EXPORTED bool
wxd_IPCConnection_StopAdvise(wxd_IPCConnection_t* conn, const char* item);

// --- Server-side Connection Methods ---

// Send advised data to the client (server calls this)
WXD_EXPORTED bool
wxd_IPCConnection_Advise(
    wxd_IPCConnection_t* conn,
    const char* item,
    const void* data,
    size_t size,
    wxd_IPCFormat format
);

// --- Both-side Connection Methods ---

// Disconnect the connection
WXD_EXPORTED bool
wxd_IPCConnection_Disconnect(wxd_IPCConnection_t* conn);

// Get the topic of the connection
WXD_EXPORTED size_t
wxd_IPCConnection_GetTopic(wxd_IPCConnection_t* conn, char* buffer, size_t buffer_size);

// Check if connection is connected
WXD_EXPORTED bool
wxd_IPCConnection_IsConnected(wxd_IPCConnection_t* conn);

// --- Server Functions ---

// Create a new IPC server
WXD_EXPORTED wxd_IPCServer_t*
wxd_IPCServer_Create(
    void* user_data,
    wxd_IPC_OnAcceptConnection_Callback on_accept_connection,
    wxd_IPC_FreeUserData_Callback free_user_data
);

// Start the server listening on the given service (port number or Unix socket path)
WXD_EXPORTED bool
wxd_IPCServer_Create_Service(wxd_IPCServer_t* server, const char* service);

// Destroy the server
WXD_EXPORTED void
wxd_IPCServer_Destroy(wxd_IPCServer_t* server);

// --- Client Functions ---

// Create a new IPC client
WXD_EXPORTED wxd_IPCClient_t*
wxd_IPCClient_Create(void);

// Connect to a server and return the connection
// Returns NULL if connection failed
WXD_EXPORTED wxd_IPCConnection_t*
wxd_IPCClient_MakeConnection(
    wxd_IPCClient_t* client,
    const char* host,
    const char* service,
    const char* topic,
    void* user_data,
    wxd_IPC_OnExecute_Callback on_execute,
    wxd_IPC_OnRequest_Callback on_request,
    wxd_IPC_OnPoke_Callback on_poke,
    wxd_IPC_OnStartAdvise_Callback on_start_advise,
    wxd_IPC_OnStopAdvise_Callback on_stop_advise,
    wxd_IPC_OnAdvise_Callback on_advise,
    wxd_IPC_OnDisconnect_Callback on_disconnect,
    wxd_IPC_FreeUserData_Callback free_user_data
);

// Destroy the client
WXD_EXPORTED void
wxd_IPCClient_Destroy(wxd_IPCClient_t* client);

#ifdef __cplusplus
}
#endif

#endif // WXD_IPC_H
