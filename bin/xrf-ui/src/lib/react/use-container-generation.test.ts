import { describe, expect, it } from "@jest/globals";
import { renderHook } from "@testing-library/react";
import { Container, EventsPlugin, InjectionToken } from "@wirestate/core";
import { StrictMode } from "react";

import { useContainerGeneration } from "@/lib/react/use-container-generation";

class FirstService {}

class SecondService {}

const VALUE_TOKEN: InjectionToken<string> = new InjectionToken<string>("VALUE");

describe("useContainerGeneration", () => {
  it("holds still while a rebuilt config keeps asking for the same tokens", () => {
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [FirstService] },
    });

    rerender({ bindings: [FirstService] });
    rerender({ bindings: [FirstService] });

    expect(result.current).toBe(0);
  });

  it("moves when a service class is replaced, the way a hot update replaces it", () => {
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [FirstService] },
    });

    rerender({ bindings: [SecondService] });

    expect(result.current).toBe(1);
  });

  it("counts one generation per change however many times a render is replayed", () => {
    // The root renders under StrictMode, which invokes the component twice. Counting the replay would
    // rebuild a container that is already the right one and throw away its services to do it.
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [FirstService] },
      wrapper: StrictMode,
    });

    rerender({ bindings: [SecondService] });

    expect(result.current).toBe(1);
  });

  it("moves when the parent container is replaced, since a child inherits through it", () => {
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [FirstService], parent: new Container() },
    });

    rerender({ bindings: [FirstService], parent: new Container() });

    expect(result.current).toBe(1);
  });

  it("moves when a binding is added or dropped", () => {
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [FirstService] },
    });

    rerender({ bindings: [FirstService, SecondService] });

    expect(result.current).toBe(1);
  });

  it("reads a descriptor by its token, so rebuilding one around the same token is not a change", () => {
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [{ token: VALUE_TOKEN, value: "first" }] },
    });

    rerender({ bindings: [{ token: VALUE_TOKEN, value: "second" }] });

    expect(result.current).toBe(0);
  });

  it("ignores plugins, which are constructed inline and would report a change every render", () => {
    const { result, rerender } = renderHook(useContainerGeneration, {
      initialProps: { bindings: [FirstService], plugins: [new EventsPlugin()] },
    });

    rerender({ bindings: [FirstService], plugins: [new EventsPlugin()] });

    expect(result.current).toBe(0);
  });

  it("stays at its first generation when no container is created", () => {
    const { result, rerender } = renderHook(useContainerGeneration, { initialProps: null });

    rerender(null);

    expect(result.current).toBe(0);
  });
});
