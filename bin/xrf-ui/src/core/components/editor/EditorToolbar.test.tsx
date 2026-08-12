import { describe, expect, it, jest } from "@jest/globals";
import { userEvent } from "@testing-library/user-event";

import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { renderWithProviders } from "@/fixtures/utils/render";

describe("EditorToolbar", () => {
  it("resolves its title from the route rather than a caller supplied string", () => {
    const { getByText } = renderWithProviders(<EditorToolbar />, { route: "/spawn/alife" });

    expect(getByText("Spawn editor")).toBeInTheDocument();
  });

  it("names every application the way the roster does, including nested routes", () => {
    const cases: Array<[string, string]> = [
      ["/archives", "Archives"],
      ["/archives-unpack", "Archives unpack"],
      ["/equipment-icons", "Equipment icons"],
      ["/project-visuals", "Project visuals"],
      ["/translations", "Translations"],
    ];

    for (const [route, expected] of cases) {
      const { getByText, unmount } = renderWithProviders(<EditorToolbar />, { route });

      expect(getByText(expected)).toBeInTheDocument();

      // Renders share `document.body`, so each case is torn down before the next one asserts.
      unmount();
    }
  });

  it("falls back to the application name on a route owned by nothing", () => {
    const { getByText } = renderWithProviders(<EditorToolbar />, { route: "/nonsense" });

    expect(getByText("Tools")).toBeInTheDocument();
  });

  it("shows no leaving control when it can neither navigate nor close", () => {
    const { queryByRole } = renderWithProviders(<EditorToolbar />, { route: "/spawn" });

    expect(queryByRole("button")).not.toBeInTheDocument();
  });

  it("prefers onBack over navigation, so leaving also releases the open file", async () => {
    const onBack = jest.fn();

    const { getByRole } = renderWithProviders(<EditorToolbar backPath={"/"} onBack={onBack} />, {
      route: "/spawn",
    });

    await userEvent.click(getByRole("button"));

    expect(onBack).toHaveBeenCalledTimes(1);
  });

  it("distinguishes a plain back from one that discards state", () => {
    const navigating = renderWithProviders(<EditorToolbar backPath={"/"} />, { route: "/spawn" });

    expect(navigating.getByLabelText("Back")).toBeInTheDocument();

    navigating.unmount();

    const closing = renderWithProviders(<EditorToolbar onBack={() => {}} />, { route: "/spawn" });

    expect(closing.getByLabelText("Close and go back")).toBeInTheDocument();
  });
});
