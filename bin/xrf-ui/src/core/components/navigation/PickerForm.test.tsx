import { describe, expect, it } from "@jest/globals";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { renderWithProviders } from "@/fixtures/render";

describe("PickerForm", () => {
  it("carries the standard toolbar so the frame does not change between a form and a workspace", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Provide spawn file to open"} />, {
      route: "/spawn-editor/open",
    });

    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(getByText("Provide spawn file to open")).toBeInTheDocument();
  });

  it("leaves via the toolbar only, rather than carrying a back button of its own", () => {
    const { getAllByRole, getByLabelText, queryByRole } = renderWithProviders(
      <PickerForm title={"Open"} backPath={"/spawn-editor"} />,
      { route: "/spawn-editor/open" }
    );

    expect(getByLabelText("Back")).toBeInTheDocument();
    expect(queryByRole("button", { name: "Back" })).toBe(getByLabelText("Back"));
    expect(getAllByRole("button")).toHaveLength(1);
  });

  it("keeps leaving visible but inert while an operation is in flight", () => {
    const { getByLabelText } = renderWithProviders(
      <PickerForm title={"Open"} backPath={"/spawn-editor"} backDisabled />,
      { route: "/spawn-editor/open" }
    );

    // Disabled rather than removed: a control that vanishes mid-operation is harder to trust.
    expect(getByLabelText("Back")).toBeDisabled();
  });

  it("shows progress only while an operation is in flight", () => {
    const idle = renderWithProviders(<PickerForm title={"Open"} />, { route: "/spawn-editor/open" });

    expect(idle.queryByRole("progressbar")).not.toBeInTheDocument();

    idle.unmount();

    const busy = renderWithProviders(<PickerForm title={"Open"} isLoading />, { route: "/spawn-editor/open" });

    expect(busy.getByRole("progressbar")).toBeInTheDocument();
  });

  it("surfaces an error without hiding the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} error={"Failed to read file"} />, {
      route: "/spawn-editor/open",
    });

    expect(getByText("Failed to read file")).toBeInTheDocument();
    expect(getByText("Open")).toBeInTheDocument();
  });

  it("renders a result alongside the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Unpack"} result={<div>unpacked 512 files</div>} />, {
      route: "/archives-editor/unpacker",
    });

    expect(getByText("unpacked 512 files")).toBeInTheDocument();
  });
});
