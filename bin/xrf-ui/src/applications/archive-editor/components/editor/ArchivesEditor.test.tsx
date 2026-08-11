import { beforeEach, describe, expect, it } from "@jest/globals";
import { fireEvent, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { Route, Routes } from "react-router-dom";

import { ArchivesEditorPage } from "@/applications/archive-editor/pages/ArchivesEditorPage";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { ApplicationShellFrame } from "@/core/components/shell/ApplicationShellFrame";
import { ApplicationStatusBar } from "@/core/components/shell/ApplicationStatusBar";
import { EditorBusyProvider } from "@/core/components/shell/EditorBusyContext";
import { EditorToolsProvider } from "@/core/components/shell/EditorToolsContext";
import { ProjectService } from "@/core/store/project";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/archive.mocks";
import { renderWithProviders } from "@/fixtures/render";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/tauri.mocks";
import { IArchivesProject } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";

const TEXT_FILE = mockArchiveFileDescriptor({ name: "readme.ltx", sizeReal: 1024, sizeCompressed: 1024 });

const BINARY_FILE = mockArchiveFileDescriptor({
  extension: "dds",
  name: "texture.dds",
  sizeReal: 2048,
  sizeCompressed: 2048,
});

const PROJECT: IArchivesProject = mockArchivesProject([TEXT_FILE, BINARY_FILE]);

describe("opened archives editor", () => {
  beforeEach(() => {
    window.localStorage.clear();

    setMockInvokeResponses({
      [EArchivesEditorCommand.GET_ARCHIVES_PROJECT]: PROJECT,
      [EArchivesEditorCommand.READ_ARCHIVE_FILE]: {
        name: TEXT_FILE.name,
        content: "line one\nline two",
        size: TEXT_FILE.sizeReal,
      },
      [EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT]: undefined,
    });
  });

  function renderEditor() {
    return renderWithProviders(
      <>
        <ArchivesEditorPage />
        <ApplicationStatusBar />
      </>,
      { route: "/archives-editor/editor", bindings: [ProjectService, ArchivesService] }
    );
  }

  it("presents archive context, aggregate status, and a guided empty state", async () => {
    const { findByText, getByText } = renderEditor();

    expect(await findByText("Select a file to preview")).toBeInTheDocument();
    expect(getByText("Archives editor")).toBeInTheDocument();
    expect(getByText("C:\\game\\database")).toBeInTheDocument();
    expect(getByText("1 archives")).toBeInTheDocument();
    expect(getByText("2 files")).toBeInTheDocument();
    expect(getByText("3 KB")).toBeInTheDocument();
  });

  it("selects and renders readable files as code with line numbers", async () => {
    const { findByLabelText, findByText } = renderEditor();

    await userEvent.click(await findByText("readme.ltx"));

    const viewer: HTMLElement = await findByLabelText("Contents of readme.ltx");
    const [lineNumbers, contents] = Array.from(viewer.querySelectorAll("pre"));

    expect(lineNumbers).toHaveTextContent("1 2");
    expect(contents).toHaveTextContent("line one line two");
  });

  it("selects unsupported files without asking the backend to read them", async () => {
    const { findByText, getByText } = renderEditor();

    await userEvent.click(await findByText("texture.dds"));

    expect(getByText("Preview unavailable")).toBeInTheDocument();
    expect(getByText(/only LTX and script files have text previews/)).toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, {
      path: BINARY_FILE.name,
    });
  });

  it("filters files and clears the filter", async () => {
    const { findByRole, findByText, getByLabelText, getByText, queryByText } = renderEditor();
    const search: HTMLElement = await findByRole("textbox", { name: "Filter archive files" });

    fireEvent.change(search, { target: { value: "readme" } });

    expect(search).toHaveValue("readme");
    expect(getByText("texture.dds")).toBeInTheDocument();
    await waitFor(() => expect(queryByText("texture.dds")).not.toBeInTheDocument());
    expect(getByText("readme.ltx")).toBeInTheDocument();

    await userEvent.click(getByLabelText("Clear file filter"));

    expect(await findByText("texture.dds")).toBeInTheDocument();
  });

  it("expands folders without reading and restores expansion after filtering", async () => {
    const nestedFile = mockArchiveFileDescriptor({ name: "configs\\system.ltx", sizeReal: 512, sizeCompressed: 512 });

    setMockInvokeResponses({
      [EArchivesEditorCommand.GET_ARCHIVES_PROJECT]: mockArchivesProject([nestedFile, BINARY_FILE]),
    });

    const { findByLabelText, findByRole, findByText, getByLabelText, queryByText } = renderEditor();

    await userEvent.click(await findByText("configs"));

    expect(await findByText("system.ltx")).toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, expect.anything());

    const search: HTMLElement = await findByRole("textbox", { name: "Filter archive files" });

    fireEvent.change(search, { target: { value: "texture" } });
    await waitFor(() => expect(queryByText("system.ltx")).not.toBeInTheDocument());

    await userEvent.click(getByLabelText("Clear file filter"));
    expect(await findByText("system.ltx")).toBeInTheDocument();
    expect(await findByLabelText("Filter archive files")).toHaveValue("");
  });

  it("keeps the selection and retries a failed file read", async () => {
    let readCount: number = 0;

    setMockInvokeResponses({
      [EArchivesEditorCommand.GET_ARCHIVES_PROJECT]: PROJECT,
      [EArchivesEditorCommand.READ_ARCHIVE_FILE]: () => {
        readCount += 1;

        if (readCount === 1) {
          throw new Error("temporary read failure");
        }

        return { name: TEXT_FILE.name, content: "recovered", size: TEXT_FILE.sizeReal };
      },
    });

    const { findByLabelText, findByRole, findByText } = renderEditor();

    await userEvent.click(await findByText("readme.ltx"));

    expect(await findByText("Could not read this file")).toBeInTheDocument();
    expect(await findByText("temporary read failure")).toBeInTheDocument();

    await userEvent.click(await findByRole("button", { name: "Retry" }));

    expect(await findByLabelText("Contents of readme.ltx")).toHaveTextContent("recovered");
    expect(readCount).toBe(2);
  });

  it("keeps file details collapsed until its tool button is used", async () => {
    const { findByLabelText, findByText, queryByText } = renderWithProviders(
      <EditorBusyProvider>
        <EditorToolsProvider>
          <ApplicationShellFrame>
            <ArchivesEditorPage />
          </ApplicationShellFrame>
        </EditorToolsProvider>
      </EditorBusyProvider>,
      { route: "/archives-editor/editor", bindings: [ProjectService, ArchivesService] }
    );

    const detailsButton: HTMLElement = await findByLabelText("File details");

    expect(queryByText("Select a file to inspect its archive metadata.")).not.toBeInTheDocument();

    await userEvent.click(detailsButton);

    expect(await findByText("Select a file to inspect its archive metadata.")).toBeInTheDocument();
  });

  it("renders the selected file metadata in Details", async () => {
    const { findByLabelText, findByText } = renderWithProviders(
      <EditorBusyProvider>
        <EditorToolsProvider>
          <ApplicationShellFrame>
            <ArchivesEditorPage />
          </ApplicationShellFrame>
        </EditorToolsProvider>
      </EditorBusyProvider>,
      { route: "/archives-editor/editor", bindings: [ProjectService, ArchivesService] }
    );

    await userEvent.click(await findByText("texture.dds"));
    await userEvent.click(await findByLabelText("File details"));

    expect(await findByText("Source archive")).toBeInTheDocument();
    expect(await findByText("C:\\game\\database\\configs.db0")).toBeInTheDocument();
    expect(await findByText("0x12345678")).toBeInTheDocument();
    expect(await findByText("Stored")).toBeInTheDocument();
  });

  it("closes successfully before navigating to Archives home", async () => {
    const { findByLabelText, findByText } = renderWithProviders(
      <Routes>
        <Route path={"/archives-editor/editor"} element={<ArchivesEditorPage />} />
        <Route path={"/archives-editor"} element={<div>Archives home</div>} />
      </Routes>,
      { route: "/archives-editor/editor", bindings: [ProjectService, ArchivesService] }
    );

    await userEvent.click(await findByLabelText("Close and go back"));

    expect(await findByText("Archives home")).toBeInTheDocument();
  });

  it("stays open and reports a close failure", async () => {
    setMockInvokeResponses({
      [EArchivesEditorCommand.GET_ARCHIVES_PROJECT]: PROJECT,
      [EArchivesEditorCommand.CLOSE_ARCHIVES_PROJECT]: () => {
        throw new Error("archive is busy");
      },
    });

    const { findByLabelText, findByText, getByText } = renderEditor();

    await userEvent.click(await findByLabelText("Close and go back"));

    expect(await findByText("Could not close archives: archive is busy")).toBeInTheDocument();
    expect(getByText("Archives editor")).toBeInTheDocument();
  });
});
