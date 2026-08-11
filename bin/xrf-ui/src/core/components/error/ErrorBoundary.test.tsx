import { describe, expect, it, jest } from "@jest/globals";
import { RenderResult } from "@testing-library/react";
import { userEvent } from "@testing-library/user-event";
import { ReactElement, useState } from "react";

import { ApplicationCrash } from "@/core/components/error/ApplicationCrash";
import { ErrorBoundary, IErrorBoundaryFallbackProps } from "@/core/components/error/ErrorBoundary";
import { renderWithProviders } from "@/fixtures/render";

function Boom(): ReactElement {
  throw new Error("render exploded");
}

function renderCrash(props: IErrorBoundaryFallbackProps): ReactElement {
  return <ApplicationCrash {...props} />;
}

/**
 * React logs caught render errors to the console itself, on top of anything the boundary reports.
 * Silenced per test so a passing suite stays readable.
 */
function withSilencedConsole(run: () => RenderResult): RenderResult {
  const spy = jest.spyOn(console, "error").mockImplementation(() => {});

  try {
    return run();
  } finally {
    spy.mockRestore();
  }
}

describe("ErrorBoundary", () => {
  it("shows the fallback instead of taking the window down", () => {
    const { getByText } = withSilencedConsole(() =>
      renderWithProviders(
        <ErrorBoundary fallback={renderCrash}>
          <Boom />
        </ErrorBoundary>
      )
    );

    expect(getByText("This tool stopped rendering")).toBeInTheDocument();
  });

  it("renders children untouched when nothing throws", () => {
    const { getByText, queryByText } = renderWithProviders(
      <ErrorBoundary fallback={renderCrash}>
        <div>healthy</div>
      </ErrorBoundary>
    );

    expect(getByText("healthy")).toBeInTheDocument();
    expect(queryByText("This tool stopped rendering")).not.toBeInTheDocument();
  });

  it("re-renders the subtree on retry rather than reloading", async () => {
    function Flaky(): ReactElement {
      if (!(globalThis as { isFixed?: boolean }).isFixed) {
        throw new Error("not yet");
      }

      return <div>recovered</div>;
    }

    const { getByText } = withSilencedConsole(() =>
      renderWithProviders(
        <ErrorBoundary fallback={renderCrash}>
          <Flaky />
        </ErrorBoundary>
      )
    );

    (globalThis as { isFixed?: boolean }).isFixed = true;

    await userEvent.click(getByText("Try again"));

    expect(getByText("recovered")).toBeInTheDocument();

    delete (globalThis as { isFixed?: boolean }).isFixed;
  });

  it("clears the failure when the reset key changes", async () => {
    function Host(): ReactElement {
      const [route, setRoute] = useState<string>("/broken");

      return (
        <>
          <button onClick={() => setRoute("/healthy")}>navigate</button>

          <ErrorBoundary resetKey={route} fallback={renderCrash}>
            {route === "/broken" ? <Boom /> : <div>next tool</div>}
          </ErrorBoundary>
        </>
      );
    }

    const { getByText, queryByText } = withSilencedConsole(() => renderWithProviders(<Host />));

    expect(getByText("This tool stopped rendering")).toBeInTheDocument();

    // Without the reset the fallback outlives the route that caused it, and the next tool never
    // renders - the crash follows the user around the application.
    await userEvent.click(getByText("navigate"));

    expect(getByText("next tool")).toBeInTheDocument();
    expect(queryByText("This tool stopped rendering")).not.toBeInTheDocument();
  });
});
