import { describe, expect, it, jest } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";

import { ApplicationPanelStripe } from "@/core/components/shell/ApplicationPanelStripe";
import { IEditorPanel } from "@/core/components/shell/EditorPanelsContext";
import { renderWithProviders } from "@/fixtures/utils/render";
import { LAYOUT } from "@/lib/theme/tokens";

const PANELS: Array<IEditorPanel> = [
  { id: "header", label: "Header", icon: <span>h</span>, render: () => <div>header panel</div> },
  { id: "bones", label: "Bones", icon: <span>b</span>, render: () => <div>bones panel</div> },
];

describe("ApplicationPanelStripe", () => {
  it("offers one control per declared panel", () => {
    const { getByLabelText } = renderWithProviders(
      <ApplicationPanelStripe
        side={"right"}
        panels={PANELS}

        activePanelId={null}
        onTogglePanel={jest.fn()}
      />
    );

    expect(getByLabelText("Header")).toBeInTheDocument();
    expect(getByLabelText("Bones")).toBeInTheDocument();
  });

  it("marks which panel is open", () => {
    const { getByLabelText } = renderWithProviders(
      <ApplicationPanelStripe
        side={"right"}
        panels={PANELS}

        activePanelId={"bones"}
        onTogglePanel={jest.fn()}
      />
    );

    expect(getByLabelText("Bones")).toHaveAttribute("aria-pressed", "true");
    expect(getByLabelText("Header")).toHaveAttribute("aria-pressed", "false");
  });

  it("reports the panel that was clicked", async () => {
    const onTogglePanel = jest.fn();

    const { getByLabelText } = renderWithProviders(
      <ApplicationPanelStripe
        side={"right"}
        panels={PANELS}

        activePanelId={"header"}
        onTogglePanel={onTogglePanel}
      />
    );

    await userEvent.click(getByLabelText("Bones"));

    expect(onTogglePanel).toHaveBeenCalledWith("bones");
  });

  it("reports the open panel again when it is clicked, which is how it collapses", async () => {
    const onTogglePanel = jest.fn();

    const { getByLabelText } = renderWithProviders(
      <ApplicationPanelStripe
        side={"right"}
        panels={PANELS}

        activePanelId={"header"}
        onTogglePanel={onTogglePanel}
      />
    );

    await userEvent.click(getByLabelText("Header"));

    expect(onTogglePanel).toHaveBeenCalledWith("header");
  });

  it("gives the header the toolbar's own height, so it lines up with the title beside it", () => {
    const { getByText } = renderWithProviders(
      <ApplicationPanelStripe
        side={"left"}
        panels={PANELS}
        activePanelId={null}
        header={<span>home</span>}
        footer={<span>settings</span>}
        onTogglePanel={jest.fn()}
      />
    );

    const band: HTMLElement = getByText("home").parentElement!;

    expect(getComputedStyle(band).height).toBe(`${LAYOUT.toolbarHeight}px`);
  });

  it("puts the header first and the footer last, whatever the application declared between them", () => {
    const { container } = renderWithProviders(
      <ApplicationPanelStripe
        side={"left"}
        panels={PANELS}
        activePanelId={null}
        header={<span>home</span>}
        footer={<span>settings</span>}
        onTogglePanel={jest.fn()}
      />
    );

    const stripe: Element = container.firstElementChild!;
    const order: Array<string> = Array.from(stripe.querySelectorAll("span, button"))
      .map((it: Element) => it.textContent ?? "")
      .filter((it: string) => it.length > 0);

    expect(order[0]).toBe("home");
    expect(order.at(-1)).toBe("settings");
  });

  it("stays present when an application declares nothing, so the frame does not shift", () => {
    const { container } = renderWithProviders(
      <ApplicationPanelStripe
        side={"right"}
        panels={[]}

        activePanelId={null}
        onTogglePanel={jest.fn()}
      />
    );

    expect(container.firstElementChild).toBeInTheDocument();
    expect(container.querySelectorAll("button")).toHaveLength(0);
  });
});
