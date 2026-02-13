#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "../include/core/wxd_ipc.h"
#include <wx/ipc.h>
#include <unordered_set>

// Global registries of live IPC objects.
// These are used to ensure all DDE objects are destroyed before wxDDECleanUp()
// runs during app shutdown (Windows DDE asserts that all objects are gone).
static std::unordered_set<void*> g_liveServers;
static std::unordered_set<void*> g_liveClients;

// --- WxdConnection: Custom connection class that wraps callbacks ---

class WxdConnection : public wxConnection {
public:
    WxdConnection()
        : m_userData(nullptr),
          m_onExecute(nullptr),
          m_onRequest(nullptr),
          m_onPoke(nullptr),
          m_onStartAdvise(nullptr),
          m_onStopAdvise(nullptr),
          m_onAdvise(nullptr),
          m_onDisconnect(nullptr),
          m_freeUserData(nullptr)
    {
        WXD_LOG_TRACE("WxdConnection created (default)");
    }

    WxdConnection(
        void* user_data,
        wxd_IPC_OnExecute_Callback on_execute,
        wxd_IPC_OnRequest_Callback on_request,
        wxd_IPC_OnPoke_Callback on_poke,
        wxd_IPC_OnStartAdvise_Callback on_start_advise,
        wxd_IPC_OnStopAdvise_Callback on_stop_advise,
        wxd_IPC_OnAdvise_Callback on_advise,
        wxd_IPC_OnDisconnect_Callback on_disconnect,
        wxd_IPC_FreeUserData_Callback free_user_data
    ) : m_userData(user_data),
        m_onExecute(on_execute),
        m_onRequest(on_request),
        m_onPoke(on_poke),
        m_onStartAdvise(on_start_advise),
        m_onStopAdvise(on_stop_advise),
        m_onAdvise(on_advise),
        m_onDisconnect(on_disconnect),
        m_freeUserData(free_user_data)
    {
        WXD_LOG_TRACE("WxdConnection created with callbacks");
    }

    virtual ~WxdConnection() {
        WXD_LOG_TRACE("WxdConnection destroyed");
        if (m_userData && m_freeUserData) {
            m_freeUserData(m_userData);
            m_userData = nullptr;
        }
    }

    // Set callbacks after construction (needed for OnMakeConnection pattern)
    void SetCallbacks(
        void* user_data,
        wxd_IPC_OnExecute_Callback on_execute,
        wxd_IPC_OnRequest_Callback on_request,
        wxd_IPC_OnPoke_Callback on_poke,
        wxd_IPC_OnStartAdvise_Callback on_start_advise,
        wxd_IPC_OnStopAdvise_Callback on_stop_advise,
        wxd_IPC_OnAdvise_Callback on_advise,
        wxd_IPC_OnDisconnect_Callback on_disconnect,
        wxd_IPC_FreeUserData_Callback free_user_data
    ) {
        m_userData = user_data;
        m_onExecute = on_execute;
        m_onRequest = on_request;
        m_onPoke = on_poke;
        m_onStartAdvise = on_start_advise;
        m_onStopAdvise = on_stop_advise;
        m_onAdvise = on_advise;
        m_onDisconnect = on_disconnect;
        m_freeUserData = free_user_data;
    }

    // Server-side callbacks
    virtual bool OnExecute(const wxString& topic, const void* data, size_t size, wxIPCFormat format) override {
        if (m_onExecute) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            return m_onExecute(m_userData, topicUtf8.data(), data, size, static_cast<wxd_IPCFormat>(format));
        }
        return false;
    }

    virtual const void* OnRequest(const wxString& topic, const wxString& item, size_t* size, wxIPCFormat format) override {
        if (m_onRequest) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            wxScopedCharBuffer itemUtf8 = item.utf8_str();
            return m_onRequest(m_userData, topicUtf8.data(), itemUtf8.data(), size, static_cast<wxd_IPCFormat>(format));
        }
        return nullptr;
    }

    virtual bool OnPoke(const wxString& topic, const wxString& item, const void* data, size_t size, wxIPCFormat format) override {
        if (m_onPoke) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            wxScopedCharBuffer itemUtf8 = item.utf8_str();
            return m_onPoke(m_userData, topicUtf8.data(), itemUtf8.data(), data, size, static_cast<wxd_IPCFormat>(format));
        }
        return false;
    }

    virtual bool OnStartAdvise(const wxString& topic, const wxString& item) override {
        if (m_onStartAdvise) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            wxScopedCharBuffer itemUtf8 = item.utf8_str();
            return m_onStartAdvise(m_userData, topicUtf8.data(), itemUtf8.data());
        }
        return false;
    }

    virtual bool OnStopAdvise(const wxString& topic, const wxString& item) override {
        if (m_onStopAdvise) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            wxScopedCharBuffer itemUtf8 = item.utf8_str();
            return m_onStopAdvise(m_userData, topicUtf8.data(), itemUtf8.data());
        }
        return false;
    }

    // Client-side callback
    virtual bool OnAdvise(const wxString& topic, const wxString& item, const void* data, size_t size, wxIPCFormat format) override {
        if (m_onAdvise) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            wxScopedCharBuffer itemUtf8 = item.utf8_str();
            return m_onAdvise(m_userData, topicUtf8.data(), itemUtf8.data(), data, size, static_cast<wxd_IPCFormat>(format));
        }
        return false;
    }

    // Both-side callback
    virtual bool OnDisconnect() override {
        if (m_onDisconnect) {
            return m_onDisconnect(m_userData);
        }
        // Default behavior: allow deletion
        return true;
    }

private:
    void* m_userData;
    wxd_IPC_OnExecute_Callback m_onExecute;
    wxd_IPC_OnRequest_Callback m_onRequest;
    wxd_IPC_OnPoke_Callback m_onPoke;
    wxd_IPC_OnStartAdvise_Callback m_onStartAdvise;
    wxd_IPC_OnStopAdvise_Callback m_onStopAdvise;
    wxd_IPC_OnAdvise_Callback m_onAdvise;
    wxd_IPC_OnDisconnect_Callback m_onDisconnect;
    wxd_IPC_FreeUserData_Callback m_freeUserData;
};

// --- WxdServer: Custom server class that wraps callbacks ---

class WxdServer : public wxServer {
public:
    WxdServer(
        void* user_data,
        wxd_IPC_OnAcceptConnection_Callback on_accept_connection,
        wxd_IPC_FreeUserData_Callback free_user_data
    ) : m_userData(user_data),
        m_onAcceptConnection(on_accept_connection),
        m_freeUserData(free_user_data)
    {
        WXD_LOG_TRACE("WxdServer created");
    }

    virtual ~WxdServer() {
        WXD_LOG_TRACE("WxdServer destroyed");
        if (m_userData && m_freeUserData) {
            m_freeUserData(m_userData);
            m_userData = nullptr;
        }
    }

    virtual wxConnectionBase* OnAcceptConnection(const wxString& topic) override {
        if (m_onAcceptConnection) {
            wxScopedCharBuffer topicUtf8 = topic.utf8_str();
            wxd_IPCConnection_t* conn = m_onAcceptConnection(m_userData, topicUtf8.data());
            return reinterpret_cast<wxConnectionBase*>(conn);
        }
        return nullptr;
    }

private:
    void* m_userData;
    wxd_IPC_OnAcceptConnection_Callback m_onAcceptConnection;
    wxd_IPC_FreeUserData_Callback m_freeUserData;
};

// --- WxdClient: Custom client class that stores pending callbacks ---

class WxdClient : public wxClient {
public:
    WxdClient()
        : m_pendingUserData(nullptr),
          m_pendingOnExecute(nullptr),
          m_pendingOnRequest(nullptr),
          m_pendingOnPoke(nullptr),
          m_pendingOnStartAdvise(nullptr),
          m_pendingOnStopAdvise(nullptr),
          m_pendingOnAdvise(nullptr),
          m_pendingOnDisconnect(nullptr),
          m_pendingFreeUserData(nullptr)
    {
        WXD_LOG_TRACE("WxdClient created");
    }

    virtual ~WxdClient() {
        WXD_LOG_TRACE("WxdClient destroyed");
    }

    // Set the callbacks to be used for the next connection
    void SetPendingCallbacks(
        void* user_data,
        wxd_IPC_OnExecute_Callback on_execute,
        wxd_IPC_OnRequest_Callback on_request,
        wxd_IPC_OnPoke_Callback on_poke,
        wxd_IPC_OnStartAdvise_Callback on_start_advise,
        wxd_IPC_OnStopAdvise_Callback on_stop_advise,
        wxd_IPC_OnAdvise_Callback on_advise,
        wxd_IPC_OnDisconnect_Callback on_disconnect,
        wxd_IPC_FreeUserData_Callback free_user_data
    ) {
        m_pendingUserData = user_data;
        m_pendingOnExecute = on_execute;
        m_pendingOnRequest = on_request;
        m_pendingOnPoke = on_poke;
        m_pendingOnStartAdvise = on_start_advise;
        m_pendingOnStopAdvise = on_stop_advise;
        m_pendingOnAdvise = on_advise;
        m_pendingOnDisconnect = on_disconnect;
        m_pendingFreeUserData = free_user_data;
    }

    // Clear pending callbacks (called after connection is made)
    void ClearPendingCallbacks() {
        m_pendingUserData = nullptr;
        m_pendingOnExecute = nullptr;
        m_pendingOnRequest = nullptr;
        m_pendingOnPoke = nullptr;
        m_pendingOnStartAdvise = nullptr;
        m_pendingOnStopAdvise = nullptr;
        m_pendingOnAdvise = nullptr;
        m_pendingOnDisconnect = nullptr;
        m_pendingFreeUserData = nullptr;
    }

    // Override OnMakeConnection to return our custom connection type with callbacks
    virtual wxConnectionBase* OnMakeConnection() override {
        WxdConnection* conn = new WxdConnection(
            m_pendingUserData,
            m_pendingOnExecute,
            m_pendingOnRequest,
            m_pendingOnPoke,
            m_pendingOnStartAdvise,
            m_pendingOnStopAdvise,
            m_pendingOnAdvise,
            m_pendingOnDisconnect,
            m_pendingFreeUserData
        );
        // Clear pending callbacks after use (ownership transferred to connection)
        ClearPendingCallbacks();
        return conn;
    }

private:
    void* m_pendingUserData;
    wxd_IPC_OnExecute_Callback m_pendingOnExecute;
    wxd_IPC_OnRequest_Callback m_pendingOnRequest;
    wxd_IPC_OnPoke_Callback m_pendingOnPoke;
    wxd_IPC_OnStartAdvise_Callback m_pendingOnStartAdvise;
    wxd_IPC_OnStopAdvise_Callback m_pendingOnStopAdvise;
    wxd_IPC_OnAdvise_Callback m_pendingOnAdvise;
    wxd_IPC_OnDisconnect_Callback m_pendingOnDisconnect;
    wxd_IPC_FreeUserData_Callback m_pendingFreeUserData;
};

// --- C API Implementation ---

extern "C" {

// --- Connection Functions ---

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
    wxd_IPC_FreeUserData_Callback free_user_data)
{
    WxdConnection* conn = new WxdConnection(
        user_data,
        on_execute,
        on_request,
        on_poke,
        on_start_advise,
        on_stop_advise,
        on_advise,
        on_disconnect,
        free_user_data
    );
    return reinterpret_cast<wxd_IPCConnection_t*>(conn);
}

WXD_EXPORTED void
wxd_IPCConnection_Destroy(wxd_IPCConnection_t* conn)
{
    if (!conn) return;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    delete wx_conn;
}

WXD_EXPORTED bool
wxd_IPCConnection_Execute(
    wxd_IPCConnection_t* conn,
    const void* data,
    size_t size,
    wxd_IPCFormat format)
{
    if (!conn) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    return wx_conn->Execute(data, size, static_cast<wxIPCFormat>(format));
}

WXD_EXPORTED bool
wxd_IPCConnection_ExecuteString(
    wxd_IPCConnection_t* conn,
    const char* data)
{
    if (!conn || !data) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    wxString str = wxString::FromUTF8(data);
    return wx_conn->Execute(str);
}

WXD_EXPORTED const void*
wxd_IPCConnection_Request(
    wxd_IPCConnection_t* conn,
    const char* item,
    size_t* out_size,
    wxd_IPCFormat format)
{
    if (!conn || !item) return nullptr;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    wxString itemStr = wxString::FromUTF8(item);
    return wx_conn->Request(itemStr, out_size, static_cast<wxIPCFormat>(format));
}

WXD_EXPORTED bool
wxd_IPCConnection_Poke(
    wxd_IPCConnection_t* conn,
    const char* item,
    const void* data,
    size_t size,
    wxd_IPCFormat format)
{
    if (!conn || !item) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    wxString itemStr = wxString::FromUTF8(item);
    return wx_conn->Poke(itemStr, data, size, static_cast<wxIPCFormat>(format));
}

WXD_EXPORTED bool
wxd_IPCConnection_StartAdvise(wxd_IPCConnection_t* conn, const char* item)
{
    if (!conn || !item) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    wxString itemStr = wxString::FromUTF8(item);
    return wx_conn->StartAdvise(itemStr);
}

WXD_EXPORTED bool
wxd_IPCConnection_StopAdvise(wxd_IPCConnection_t* conn, const char* item)
{
    if (!conn || !item) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    wxString itemStr = wxString::FromUTF8(item);
    return wx_conn->StopAdvise(itemStr);
}

WXD_EXPORTED bool
wxd_IPCConnection_Advise(
    wxd_IPCConnection_t* conn,
    const char* item,
    const void* data,
    size_t size,
    wxd_IPCFormat format)
{
    if (!conn || !item) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    wxString itemStr = wxString::FromUTF8(item);
    return wx_conn->Advise(itemStr, data, size, static_cast<wxIPCFormat>(format));
}

WXD_EXPORTED bool
wxd_IPCConnection_Disconnect(wxd_IPCConnection_t* conn)
{
    if (!conn) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    return wx_conn->Disconnect();
}

WXD_EXPORTED size_t
wxd_IPCConnection_GetTopic(wxd_IPCConnection_t* conn, char* buffer, size_t buffer_size)
{
    if (!conn) return 0;
    // wxConnection doesn't expose GetTopic() directly
    // We would need to track it ourselves or access protected members
    (void)buffer;
    (void)buffer_size;
    return 0;
}

WXD_EXPORTED bool
wxd_IPCConnection_IsConnected(wxd_IPCConnection_t* conn)
{
    if (!conn) return false;
    WxdConnection* wx_conn = reinterpret_cast<WxdConnection*>(conn);
    return wx_conn->GetConnected();
}

// --- Server Functions ---

WXD_EXPORTED wxd_IPCServer_t*
wxd_IPCServer_Create(
    void* user_data,
    wxd_IPC_OnAcceptConnection_Callback on_accept_connection,
    wxd_IPC_FreeUserData_Callback free_user_data)
{
    WxdServer* server = new WxdServer(user_data, on_accept_connection, free_user_data);
    g_liveServers.insert(server);
    return reinterpret_cast<wxd_IPCServer_t*>(server);
}

WXD_EXPORTED bool
wxd_IPCServer_Create_Service(wxd_IPCServer_t* server, const char* service)
{
    if (!server || !service) return false;
    WxdServer* wx_server = reinterpret_cast<WxdServer*>(server);
    wxString serviceStr = wxString::FromUTF8(service);
    return wx_server->Create(serviceStr);
}

WXD_EXPORTED void
wxd_IPCServer_Destroy(wxd_IPCServer_t* server)
{
    if (!server) return;
    // Only delete if still tracked (idempotent for double-destroy safety)
    if (g_liveServers.erase(server) == 0) return;
    WxdServer* wx_server = reinterpret_cast<WxdServer*>(server);
    delete wx_server;
}

// --- Client Functions ---

WXD_EXPORTED wxd_IPCClient_t*
wxd_IPCClient_Create(void)
{
    WxdClient* client = new WxdClient();
    g_liveClients.insert(client);
    return reinterpret_cast<wxd_IPCClient_t*>(client);
}

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
    wxd_IPC_FreeUserData_Callback free_user_data)
{
    if (!client || !host || !service || !topic) return nullptr;

    WxdClient* wx_client = reinterpret_cast<WxdClient*>(client);

    // Set the pending callbacks before making the connection
    // These will be used in OnMakeConnection()
    wx_client->SetPendingCallbacks(
        user_data,
        on_execute,
        on_request,
        on_poke,
        on_start_advise,
        on_stop_advise,
        on_advise,
        on_disconnect,
        free_user_data
    );

    wxString hostStr = wxString::FromUTF8(host);
    wxString serviceStr = wxString::FromUTF8(service);
    wxString topicStr = wxString::FromUTF8(topic);

    // MakeConnection will call OnMakeConnection() internally,
    // which creates our WxdConnection with the pending callbacks
    wxConnectionBase* conn = wx_client->MakeConnection(hostStr, serviceStr, topicStr);

    if (!conn) {
        // Connection failed, clear pending callbacks
        // Note: If callbacks were set but connection failed,
        // the user_data won't be freed automatically.
        // We should call free_user_data if it was set.
        if (user_data && free_user_data) {
            free_user_data(user_data);
        }
        wx_client->ClearPendingCallbacks();
        return nullptr;
    }

    return reinterpret_cast<wxd_IPCConnection_t*>(conn);
}

WXD_EXPORTED void
wxd_IPCClient_Destroy(wxd_IPCClient_t* client)
{
    if (!client) return;
    // Only delete if still tracked (idempotent for double-destroy safety)
    if (g_liveClients.erase(client) == 0) return;
    WxdClient* wx_client = reinterpret_cast<WxdClient*>(client);
    delete wx_client;
}

// Destroy all remaining IPC server/client objects.
// Called from WxdApp::OnExit() to ensure DDE objects are cleaned up
// before wxDDECleanUp() asserts they're gone.
WXD_EXPORTED void
wxd_IPC_CleanupAll(void)
{
    // Destroy all live servers
    for (void* ptr : g_liveServers) {
        WxdServer* server = reinterpret_cast<WxdServer*>(ptr);
        delete server;
    }
    g_liveServers.clear();

    // Destroy all live clients
    for (void* ptr : g_liveClients) {
        WxdClient* client = reinterpret_cast<WxdClient*>(ptr);
        delete client;
    }
    g_liveClients.clear();
}

} // extern "C"
