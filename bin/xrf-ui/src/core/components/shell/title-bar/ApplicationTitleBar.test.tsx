import { describe, expect, it } from "@jest/globals";
import { act, waitFor } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";

import { ApplicationTitleBar } from "@/core/components/shell/title-bar/ApplicationTitleBar";
import { mockAppWindow, setMockWindowMaximized } from "@/fixtures/mocks/tauri.mocks";
import { renderWithProviders } from "@/fixtures/utils/render";

describe("ApplicationTitleBar", () => {
  it("drives the host window from its controls", async () => {
    const { getByLabelText } = renderWithProviders(<ApplicationTitleBar />);

    await userEvent.click(getByLabelText("Minimize"));
    expect(mockAppWindow.minimize).toHaveBeenCalledTimes(1);

    await userEvent.click(getByLabelText("Close"));
    expect(mockAppWindow.close).toHaveBeenCalledTimes(1);
  });

  it("follows the window when it is maximized by something other than its own button", async () => {
    const { getByLabelText, queryByLabelText } = renderWithProviders(<ApplicationTitleBar />);

    await waitFor(() => expect(getByLabelText("Maximize")).toBeInTheDocument());

    // A snap gesture or a double click on the drag region reaches the bar only as a resize.
    act(() => setMockWindowMaximized(true));

    await waitFor(() => expect(getByLabelText("Restore down")).toBeInTheDocument());
    expect(queryByLabelText("Maximize")).not.toBeInTheDocument();
  });

  it("keeps the caption draggable outside the controls", () => {
    const { container, getByLabelText, getByAltText } = renderWithProviders(<ApplicationTitleBar />);

    // Without a drag region the window has no way to be moved at all, the system frame being gone.
    // It has to be `deep` and not `true`, or only the bar's own padding would drag.
    expect(container.querySelector("#application-title-bar")).toHaveAttribute("data-tauri-drag-region", "deep");

    // Tauri stops walking at a clickable element that declares no region of its own, so the controls
    // are excluded by carrying no attribute. Adding one here would make the buttons drag the window.
    expect(getByLabelText("Close")).not.toHaveAttribute("data-tauri-drag-region");

    // An image is not clickable to tauri, but it is draggable to the browser, which would win.
    expect(getByAltText("XRF tools")).toHaveAttribute("draggable", "false");
  });

  it("identifies the window by icon rather than by repeating the name below it", () => {
    const { getByAltText, queryByText } = renderWithProviders(<ApplicationTitleBar />);

    expect(getByAltText("XRF tools")).toBeInTheDocument();
    expect(queryByText("XRF tools")).not.toBeInTheDocument();
  });

  it("reserves the space between the icon and the controls", () => {
    const { getByAltText, getByLabelText } = renderWithProviders(<ApplicationTitleBar />);

    // The gap is a real element rather than a margin, so a menu bar can land in it without the
    // controls or the icon moving.
    const reserved: Element = getByAltText("XRF tools").nextElementSibling as Element;

    expect(reserved).toBeInTheDocument();
    expect(reserved.contains(getByLabelText("Minimize"))).toBe(false);
  });
});
