import { describe, expect, it } from "@jest/globals";

import { ApplicationLoader } from "@/core/shell/loading/ApplicationLoader";
import { renderWithProviders } from "@/fixtures/utils/render";

describe("ApplicationLoader", () => {
  it("delays displaying the progress indicator for 500 milliseconds", () => {
    const { getByRole, getByTestId, queryByRole } = renderWithProviders(<ApplicationLoader data-testid={"loader"} />);
    const loader: HTMLElement = getByTestId("loader");

    expect(loader).not.toBeVisible();
    expect(loader).toHaveStyle({ animationDelay: "500ms", animationDuration: "0s", visibility: "hidden" });
    expect(queryByRole("progressbar")).not.toBeInTheDocument();
    expect(getByRole("progressbar", { hidden: true })).toBeInTheDocument();
  });
});
