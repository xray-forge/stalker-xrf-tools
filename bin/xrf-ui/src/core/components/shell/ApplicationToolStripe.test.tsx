import { describe, expect, it, jest } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";

import { ApplicationToolStripe } from "@/core/components/shell/ApplicationToolStripe";
import { IEditorTool } from "@/core/components/shell/EditorToolsContext";
import { renderWithProviders } from "@/fixtures/render";

const TOOLS: Array<IEditorTool> = [
  { id: "header", label: "Header", icon: <span>h</span>, render: () => <div>header panel</div> },
  { id: "bones", label: "Bones", icon: <span>b</span>, render: () => <div>bones panel</div> },
];

describe("ApplicationToolStripe", () => {
  it("offers one control per declared tool", () => {
    const { getByLabelText } = renderWithProviders(
      <ApplicationToolStripe tools={TOOLS} activeToolId={null} onToggleTool={jest.fn()} />
    );

    expect(getByLabelText("Header")).toBeInTheDocument();
    expect(getByLabelText("Bones")).toBeInTheDocument();
  });

  it("marks which panel is open", () => {
    const { getByLabelText } = renderWithProviders(
      <ApplicationToolStripe tools={TOOLS} activeToolId={"bones"} onToggleTool={jest.fn()} />
    );

    expect(getByLabelText("Bones")).toHaveAttribute("aria-pressed", "true");
    expect(getByLabelText("Header")).toHaveAttribute("aria-pressed", "false");
  });

  it("reports the tool that was clicked", async () => {
    const onToggleTool = jest.fn();

    const { getByLabelText } = renderWithProviders(
      <ApplicationToolStripe tools={TOOLS} activeToolId={"header"} onToggleTool={onToggleTool} />
    );

    await userEvent.click(getByLabelText("Bones"));

    expect(onToggleTool).toHaveBeenCalledWith("bones");
  });

  it("reports the open tool again when it is clicked, which is how it collapses", async () => {
    const onToggleTool = jest.fn();

    const { getByLabelText } = renderWithProviders(
      <ApplicationToolStripe tools={TOOLS} activeToolId={"header"} onToggleTool={onToggleTool} />
    );

    await userEvent.click(getByLabelText("Header"));

    expect(onToggleTool).toHaveBeenCalledWith("header");
  });

  it("stays present when an editor declares nothing, so the frame does not shift", () => {
    const { container } = renderWithProviders(
      <ApplicationToolStripe tools={[]} activeToolId={null} onToggleTool={jest.fn()} />
    );

    expect(container.firstElementChild).toBeInTheDocument();
    expect(container.querySelectorAll("button")).toHaveLength(0);
  });
});
