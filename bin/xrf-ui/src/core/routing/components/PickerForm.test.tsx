import { describe, expect, it } from "@jest/globals";

import { PickerForm } from "@/core/routing/components/PickerForm";
import { renderWithProviders } from "@/fixtures/utils/render";

describe("PickerForm", () => {
  it("carries the standard toolbar so the frame does not change between a form and a workspace", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Provide spawn file to open"} />, {
      route: "/spawn",
    });

    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(getByText("Provide spawn file to open")).toBeInTheDocument();
  });

  it("leaves through the breadcrumb root, rather than carrying a back button of its own", () => {
    const { getByText, queryByLabelText } = renderWithProviders(<PickerForm title={"Open"} />, {
      route: "/spawn",
    });

    expect(getByText("XRF")).toBeInTheDocument();
    expect(queryByLabelText("Close document")).not.toBeInTheDocument();
  });

  it("stops the way out while an operation is in flight", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} isLoading />, { route: "/spawn" });

    // Disabled rather than removed: a control that vanishes mid-operation is harder to trust.
    expect(getByText("XRF")).toBeDisabled();
  });

  it("shows progress only while an operation is in flight", () => {
    const idle = renderWithProviders(<PickerForm title={"Open"} />, { route: "/spawn" });

    expect(idle.queryByRole("progressbar")).not.toBeInTheDocument();

    idle.unmount();

    const busy = renderWithProviders(<PickerForm title={"Open"} isLoading />, { route: "/spawn" });

    expect(busy.getByRole("progressbar")).toBeInTheDocument();
  });

  it("surfaces an error without hiding the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} error={"Failed to read file"} />, {
      route: "/spawn",
    });

    expect(getByText("Failed to read file")).toBeInTheDocument();
    expect(getByText("Open")).toBeInTheDocument();
  });

  it("renders a result alongside the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Unpack"} result={<div>unpacked 512 files</div>} />, {
      route: "/archives-unpack",
    });

    expect(getByText("unpacked 512 files")).toBeInTheDocument();
  });
});
