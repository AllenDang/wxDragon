// Native printing regression for #246. The printout shim must start and end the
// document through wxPrintout, which is what calls StartDoc and EndDoc on the
// printer DC, and must pass on the begin-document callback's answer. The shim is
// driven directly against a DC that records those calls, so no printer is needed.
#include <wx/wx.h>
#include <iostream>
#include <string>
#include <vector>
#include "wxdragon.h"

#if wxUSE_PRINTING_ARCHITECTURE && wxUSE_GRAPHICS_CONTEXT
#include <wx/dcgraph.h>
#include <wx/print.h>
#endif

class PrintoutTestApp : public wxApp {
public:
    bool
    OnInit() override
    { return true; }
};
wxIMPLEMENT_APP_NO_MAIN(PrintoutTestApp);

#if wxUSE_PRINTING_ARCHITECTURE && wxUSE_GRAPHICS_CONTEXT

using Events = std::vector<std::string>;

// What the DC and the shim's callbacks were asked to do, in order.
struct Journal {
    Events events;
    bool startDocSucceeds = true;
    bool beginCallbackAnswer = true;
};

class RecordingDCImpl : public wxGCDCImpl {
public:
    RecordingDCImpl(wxDC* owner, Journal& journal) : wxGCDCImpl(owner), m_journal(journal)
    {
    }

    bool
    StartDoc(const wxString&) override
    {
        m_journal.events.push_back("StartDoc");
        return m_journal.startDocSucceeds;
    }

    void
    EndDoc() override
    { m_journal.events.push_back("EndDoc"); }

private:
    Journal& m_journal;
};

// Stands in for the printer DC that wxPrinter hands the printout.
class RecordingDC : public wxDC {
public:
    explicit RecordingDC(Journal& journal) : wxDC(new RecordingDCImpl(this, journal))
    {
    }
};

static bool
OnBeginDocument(void* userData, int, int)
{
    auto& journal = *static_cast<Journal*>(userData);
    journal.events.push_back("begin callback");
    return journal.beginCallbackAnswer;
}

static void
OnEndDocument(void* userData)
{ static_cast<Journal*>(userData)->events.push_back("end callback"); }

// The shim, created through the C function Printer::print uses, with only the
// document callbacks set and a recording DC attached.
class ShimUnderTest {
public:
    explicit ShimUnderTest(Journal& journal)
        : m_dc(journal),
          m_handle(wxd_Printout_CreateWithCallbacks("printout_document", &journal, nullptr, nullptr,
                                                    nullptr, OnBeginDocument, OnEndDocument,
                                                    nullptr, nullptr, nullptr))
    { Printout().SetDC(&m_dc); }

    ~ShimUnderTest()
    {
        Printout().SetDC(nullptr);
        wxd_Printout_Destroy(m_handle);
    }

    wxPrintout&
    Printout()
    { return *reinterpret_cast<wxPrintout*>(m_handle); }

private:
    RecordingDC m_dc;
    wxd_Printout_t* m_handle;
};

static std::string
DescribeEvents(const Journal& journal)
{
    std::string text = "events [";
    for (size_t i = 0; i < journal.events.size(); ++i)
        text += (i ? ", " : "") + journal.events[i];
    return text + "]";
}

static std::string
DescribeBegin(bool answer, const Journal& journal)
{ return (answer ? "answered true, " : "answered false, ") + DescribeEvents(journal); }

static std::string
BeginDocumentStartsTheDocumentBeforeTheCallback()
{
    Journal journal;
    ShimUnderTest shim(journal);
    bool answer = shim.Printout().OnBeginDocument(1, 1);
    bool expected = answer && journal.events == Events{ "StartDoc", "begin callback" };
    return expected ? "" : DescribeBegin(answer, journal);
}

static std::string
BeginDocumentAnswersWhatTheCallbackAnswers()
{
    Journal journal;
    journal.beginCallbackAnswer = false;
    ShimUnderTest shim(journal);
    bool answer = shim.Printout().OnBeginDocument(1, 1);
    return !answer ? "" : DescribeBegin(answer, journal);
}

static std::string
BeginDocumentStopsWithoutTheCallbackWhenTheDocumentCannotStart()
{
    Journal journal;
    journal.startDocSucceeds = false;
    ShimUnderTest shim(journal);
    bool answer = shim.Printout().OnBeginDocument(1, 1);
    bool expected = !answer && journal.events == Events{ "StartDoc" };
    return expected ? "" : DescribeBegin(answer, journal);
}

static std::string
EndDocumentEndsTheDocumentAfterTheCallback()
{
    Journal journal;
    ShimUnderTest shim(journal);
    shim.Printout().OnEndDocument();
    bool expected = journal.events == Events{ "end callback", "EndDoc" };
    return expected ? "" : DescribeEvents(journal);
}

static int
RunCases()
{
    struct Case {
        const char* name;
        std::string (*run)();
    };
    const Case cases[] = {
        { "begin document starts the document before the callback",
          BeginDocumentStartsTheDocumentBeforeTheCallback },
        { "begin document answers what the callback answers",
          BeginDocumentAnswersWhatTheCallbackAnswers },
        { "begin document stops without the callback when the document cannot start",
          BeginDocumentStopsWithoutTheCallbackWhenTheDocumentCannotStart },
        { "end document ends the document after the callback",
          EndDocumentEndsTheDocumentAfterTheCallback },
    };
    int failures = 0;
    for (const Case& test : cases) {
        std::string failure = test.run();
        if (failure.empty()) {
            std::cout << "ok: " << test.name << '\n';
        }
        else {
            std::cerr << "FAILED: " << test.name << ": " << failure << '\n';
            ++failures;
        }
    }
    return failures;
}

#endif // wxUSE_PRINTING_ARCHITECTURE && wxUSE_GRAPHICS_CONTEXT

int
main(int argc, char** argv)
{
#if wxUSE_PRINTING_ARCHITECTURE && wxUSE_GRAPHICS_CONTEXT
    if (!wxEntryStart(argc, argv))
        return 1;
    if (!wxTheApp->CallOnInit()) {
        wxEntryCleanup();
        return 1;
    }
    int failures = RunCases();
    wxTheApp->OnExit();
    wxEntryCleanup();
    if (failures == 0)
        std::cout << "Printout document regressions passed\n";
    return failures == 0 ? 0 : 1;
#else
    (void)argc;
    (void)argv;
    std::cout << "Printing is disabled in this wxWidgets build; nothing to test\n";
    return 0;
#endif
}
