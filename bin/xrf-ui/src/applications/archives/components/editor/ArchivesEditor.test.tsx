import { beforeEach, describe, expect, it, jest } from "@jest/globals";
import * as dialog from "@tauri-apps/plugin-dialog";
import { fireEvent, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { ArchivesApplication } from "@/applications/archives/ArchivesApplication";
import { ArchivesService } from "@/applications/archives/store/archives";
import { ApplicationShellFrame } from "@/core/components/shell/ApplicationShellFrame";
import { ApplicationStatusBar } from "@/core/components/shell/ApplicationStatusBar";
import { EditorBusyProvider } from "@/core/components/shell/EditorBusyContext";
import { EditorPanelsProvider } from "@/core/components/shell/EditorPanelsContext";
import { ProjectService } from "@/core/store/project";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { renderWithProviders } from "@/fixtures/utils/render";
import { IArchivesProject } from "@/lib/archive";
import { EArchivesEditorCommand } from "@/lib/ipc";

const TEXT_FILE = mockArchiveFileDescriptor({ name: "readme.ltx", sizeReal: 1024, sizeCompressed: 1024 });

const BINARY_FILE = mockArchiveFileDescriptor({
  extension: "dds",
  name: "texture.dds",
  sizeReal: 2048,
  sizeCompressed: 2048,
});

const MESH_FILE = mockArchiveFileDescriptor({
  extension: "ogf",
  name: "actor.ogf",
  sizeReal: 4096,
  sizeCompressed: 4096,
});

const PROJECT: IArchivesProject = mockArchivesProject([TEXT_FILE, BINARY_FILE, MESH_FILE]);

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
      [EArchivesEditorCommand.READ_ARCHIVE_IMAGE]: {
        name: BINARY_FILE.name,
        width: 64,
        height: 64,
        base64: "iVBORw0KGgo=",
      },
    });
  });

  function renderEditor() {
    return renderWithProviders(
      <>
        <ArchivesApplication />
        <ApplicationStatusBar />
      </>,
      { route: "/archives", bindings: [ProjectService, ArchivesService] }
    );
  }

  it("presents archive context, aggregate status, and a guided empty state", async () => {
    const { findByText, getByText } = renderEditor();

    expect(await findByText("Select a file to preview")).toBeInTheDocument();
    expect(getByText("Archives")).toBeInTheDocument();
    expect(getByText("C:\\game\\database")).toBeInTheDocument();
    expect(getByText("1 archives")).toBeInTheDocument();
    expect(getByText("3 files")).toBeInTheDocument();
    expect(getByText("7 KB")).toBeInTheDocument();
  });

  it("selects and renders readable files as code with line numbers", async () => {
    const { findByLabelText, findByText } = renderEditor();

    await userEvent.click(await findByText("readme.ltx"));

    const viewer: HTMLElement = await findByLabelText("Contents of readme.ltx");
    const [lineNumbers, contents] = Array.from(viewer.querySelectorAll("pre"));

    expect(lineNumbers).toHaveTextContent("1 2");
    expect(contents).toHaveTextContent("line one line two");
  });

  it("decodes a texture into a picture rather than refusing it", async () => {
    const { findByAltText, findByText } = renderEditor();

    await userEvent.click(await findByText("texture.dds"));

    // Compressed and not a readable extension, so the text path would have refused it outright.
    expect(await findByAltText(BINARY_FILE.name)).toHaveAttribute("src", "data:image/png;base64,iVBORw0KGgo=");
    expect(await findByText("64 x 64")).toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, {
      path: BINARY_FILE.name,
    });
  });

  it("selects genuinely unsupported files without asking the backend to read them", async () => {
    const { findByText, getByText } = renderEditor();

    await userEvent.click(await findByText("actor.ogf"));

    expect(getByText("Preview unavailable")).toBeInTheDocument();
    expect(getByText(/this file type does not have a text preview/)).toBeInTheDocument();
    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_FILE, {
      path: MESH_FILE.name,
    });
  });

  it("filters files and clears the filter", async () => {
    const { findByRole, findByText, getByLabelText, getByText, queryByText } = renderEditor();
    const search: HTMLElement = await findByRole("textbox", { name: "Filter archive files" });

    fireEvent.change(search, { target: { value: "readme" } });

    expect(search).toHaveValue("readme");

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
        <EditorPanelsProvider>
          <ApplicationShellFrame>
            <ArchivesApplication />
          </ApplicationShellFrame>
        </EditorPanelsProvider>
      </EditorBusyProvider>,
      { route: "/archives", bindings: [ProjectService, ArchivesService] }
    );

    const detailsButton: HTMLElement = await findByLabelText("File details");

    expect(queryByText("Select a file to inspect its archive metadata.")).not.toBeInTheDocument();

    await userEvent.click(detailsButton);

    expect(await findByText("Select a file to inspect its archive metadata.")).toBeInTheDocument();
  });

  it("renders the selected file metadata in Details", async () => {
    const { findByLabelText, findByText } = renderWithProviders(
      <EditorBusyProvider>
        <EditorPanelsProvider>
          <ApplicationShellFrame>
            <ArchivesApplication />
          </ApplicationShellFrame>
        </EditorPanelsProvider>
      </EditorBusyProvider>,
      { route: "/archives", bindings: [ProjectService, ArchivesService] }
    );

    await userEvent.click(await findByText("texture.dds"));
    await userEvent.click(await findByLabelText("File details"));

    expect(await findByText("Source archive")).toBeInTheDocument();
    expect(await findByText("C:\\game\\database\\configs.db0")).toBeInTheDocument();
    expect(await findByText("0x12345678")).toBeInTheDocument();
    expect(await findByText("Stored")).toBeInTheDocument();
  });

  it("closes into its own picker rather than navigating away", async () => {
    // Closing used to leave for the archives landing pane. There is no pane above an application any
    // more, and the application already draws its picker whenever nothing is open.
    const { findByLabelText, findByText, queryByText } = renderWithProviders(<ArchivesApplication />, {
      route: "/archives",
      bindings: [ProjectService, ArchivesService],
    });

    await userEvent.click(await findByLabelText("Close and go back"));

    expect(await findByText("Open game archives")).toBeInTheDocument();
    expect(queryByText("C:\\game\\database")).not.toBeInTheDocument();
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
    expect(getByText("Archives")).toBeInTheDocument();
  });

  it("locks navigation while a file is being written to disk", async () => {
    // Extraction writes outside the archive; leaving mid-write leaves it running against a screen
    // nobody can see. The rail is the thing that has to stop, not just the button that started it.
    const save = jest.spyOn(dialog, "save").mockResolvedValue("C:\\out\\readme.ltx");

    setMockInvokeResponses({
      [EArchivesEditorCommand.GET_ARCHIVES_PROJECT]: PROJECT,
      [EArchivesEditorCommand.READ_ARCHIVE_FILE]: { name: TEXT_FILE.name, content: "line", size: 4 },
      // Never settles, so the editor stays mid-extraction for the length of the assertion.
      [EArchivesEditorCommand.EXTRACT_ARCHIVE_FILE]: () => new Promise(() => {}),
    });

    const { findByLabelText, findByText, getByLabelText } = renderWithProviders(
      <EditorBusyProvider>
        <EditorPanelsProvider>
          <ApplicationShellFrame>
            <ArchivesApplication />
          </ApplicationShellFrame>
        </EditorPanelsProvider>
      </EditorBusyProvider>,
      { route: "/archives", bindings: [ProjectService, ArchivesService] }
    );

    await userEvent.click(await findByText("readme.ltx"));
    await userEvent.click(await findByLabelText("Extract file"));

    // Home is the only navigating control on the rail now, so it is the one that has to stop.
    await waitFor(() => expect(getByLabelText("Home")).toBeDisabled());
    expect(getByLabelText("Close and go back")).toBeDisabled();

    save.mockRestore();
  });

  it("refuses a second selection while a read is still in flight", async () => {
    setMockInvokeResponses({
      [EArchivesEditorCommand.GET_ARCHIVES_PROJECT]: PROJECT,
      // Never settles, so the first selection stays in flight for the length of the assertion.
      [EArchivesEditorCommand.READ_ARCHIVE_FILE]: () => new Promise(() => {}),
    });

    const { findByText, getByText } = renderEditor();

    await userEvent.click(await findByText("readme.ltx"));

    // A texture, so it would reach the backend on its own decode command rather than being ignored
    // as an unreadable type. Letting it through would leave the tree pointing at one file while the
    // content area still belongs to another.
    await userEvent.click(getByText("texture.dds"));

    expect(mockInvoke).not.toHaveBeenCalledWith(EArchivesEditorCommand.READ_ARCHIVE_IMAGE, expect.anything());
  });
});
