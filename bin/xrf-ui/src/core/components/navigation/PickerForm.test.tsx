import { describe, expect, it } from "@jest/globals";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { renderWithProviders } from "@/fixtures/render";

describe("PickerForm", () => {
  it("carries the standard toolbar so the frame does not change between a form and a workspace", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Provide spawn file to open"} />, {
      route: "/spawn_editor/open",
    });

    expect(getByText("Spawn editor")).toBeInTheDocument();
    expect(getByText("Provide spawn file to open")).toBeInTheDocument();
  });

  it("leaves via the toolbar only, rather than carrying a back button of its own", () => {
    const { getAllByRole, getByLabelText, queryByRole } = renderWithProviders(
      <PickerForm title={"Open"} backPath={"/spawn_editor"} />,
      { route: "/spawn_editor/open" }
    );

    expect(getByLabelText("Back")).toBeInTheDocument();
    expect(queryByRole("button", { name: "Back" })).toBe(getByLabelText("Back"));
    expect(getAllByRole("button")).toHaveLength(1);
  });

  it("suppresses leaving while an operation is in flight", () => {
    const { queryByLabelText } = renderWithProviders(
      <PickerForm title={"Open"} backPath={"/spawn_editor"} backDisabled />,
      { route: "/spawn_editor/open" }
    );

    expect(queryByLabelText("Back")).not.toBeInTheDocument();
  });

  it("surfaces an error without hiding the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Open"} error={"Failed to read file"} />, {
      route: "/spawn_editor/open",
    });

    expect(getByText("Failed to read file")).toBeInTheDocument();
    expect(getByText("Open")).toBeInTheDocument();
  });

  it("renders a result alongside the form", () => {
    const { getByText } = renderWithProviders(<PickerForm title={"Unpack"} result={<div>unpacked 512 files</div>} />, {
      route: "/archives_editor/unpacker",
    });

    expect(getByText("unpacked 512 files")).toBeInTheDocument();
  });
});
