import { describe, expect, it } from "@jest/globals";

import { PickerForm } from "@/core/shell/editor/PickerForm";
import { renderWithProviders } from "@/fixtures/utils/render";

describe("PickerForm", () => {
  it("carries the standard toolbar so the frame does not change between a form and a workspace", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Provide spawn file to open"} />, {
      route: "/spawn-editor",
    });

    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(getByText("Provide spawn file to open")).toBeInTheDocument();
  });

  it("leaves through the breadcrumb root, rather than carrying a back button of its own", () => {
    const { getByText, queryByLabelText } = renderWithProviders(<PickerForm title={"Open"} />, {
      route: "/spawn-editor",
    });

    expect(getByText("XRF")).toBeInTheDocument();
    expect(queryByLabelText("Close document")).not.toBeInTheDocument();
  });

  it("stops the way out while an operation is in flight", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} isLoading />, { route: "/spawn-editor" });

    // Disabled rather than removed: a control that vanishes mid-operation is harder to trust.
    expect(getByText("XRF")).toBeDisabled();
  });

  it("shows progress only while an operation is in flight", () => {
    const idle = renderWithProviders(<PickerForm title={"Open"} />, { route: "/spawn-editor" });

    expect(idle.queryByRole("progressbar")).not.toBeInTheDocument();

    idle.unmount();

    const busy = renderWithProviders(<PickerForm title={"Open"} isLoading />, { route: "/spawn-editor" });

    expect(busy.getByRole("progressbar")).toBeInTheDocument();
  });

  it("surfaces an error without hiding the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} error={"Failed to read file"} />, {
      route: "/spawn-editor",
    });

    expect(getByText("Failed to read file")).toBeInTheDocument();
    expect(getByText("Open")).toBeInTheDocument();
  });

  it("says what the command touches before it is run", () => {
    const { getByText } = renderWithProviders(
      <PickerForm title={"Unpack"} description={"Writes the chunks into the destination directory."} />,
      { route: "/spawn-unpacker" }
    );

    expect(getByText("Writes the chunks into the destination directory.")).toBeInTheDocument();
  });

  it("keeps the actions with the parameters they act on", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} submitLabel={"Open file"} />, {
      route: "/spawn-editor",
    });

    // Both buttons belong to the one panel, rather than the form floating at the top of a window whose
    // bottom edge holds the buttons.
    const panel: HTMLElement | null = getByText("Open").closest(".MuiPaper-root");

    expect(panel).toContainElement(getByText("Open file"));
    expect(panel).toContainElement(getByText("Back"));
  });

  it("renders a result alongside the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Unpack"} result={<div>unpacked 512 files</div>} />, {
      route: "/archives-unpacker",
    });

    expect(getByText("unpacked 512 files")).toBeInTheDocument();
  });
});
