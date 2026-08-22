import { beforeEach, describe, expect, it, jest } from "@jest/globals";
import { open } from "@tauri-apps/plugin-dialog";
import { act, renderHook } from "@testing-library/react";
import { StrictMode } from "react";

import { EApplicationId } from "@/core/routing/application";
import { IPathField, usePathField } from "@/core/ui/form/use-path-field";
import { Nullable } from "@/lib/types/general";

describe("usePathField", () => {
  const STORAGE_KEY: string = "xrf.form.archives-packer.source";

  function renderField(seed?: () => Promise<Nullable<string>>) {
    return renderHook(() => usePathField({ application: EApplicationId.ARCHIVES_PACKER, id: "source", seed }), {
      wrapper: StrictMode,
    });
  }

  beforeEach(() => {
    window.localStorage.clear();
    jest.mocked(open).mockResolvedValue(null);
  });

  it("restores the remembered path on the first render", () => {
    window.localStorage.setItem(STORAGE_KEY, "C:\\projects\\stored");

    const { result } = renderField();

    expect(result.current.value).toBe("C:\\projects\\stored");
  });

  it("writes nothing while only mounting and remounting", () => {
    window.localStorage.setItem(STORAGE_KEY, "C:\\projects\\stored");

    const setItem = jest.spyOn(Storage.prototype, "setItem");
    const removeItem = jest.spyOn(Storage.prototype, "removeItem");

    try {
      renderField().unmount();
      renderField().unmount();

      expect(setItem).not.toHaveBeenCalled();
      expect(removeItem).not.toHaveBeenCalled();
      expect(window.localStorage.getItem(STORAGE_KEY)).toBe("C:\\projects\\stored");
    } finally {
      setItem.mockRestore();
      removeItem.mockRestore();
    }
  });

  it("remembers what the dialog returned", async () => {
    jest.mocked(open).mockResolvedValue("C:\\projects\\picked");

    const { result } = renderField();

    await act(() => result.current.select());

    expect(result.current.value).toBe("C:\\projects\\picked");
    expect(window.localStorage.getItem(STORAGE_KEY)).toBe("C:\\projects\\picked");
  });

  it("leaves the remembered path alone when the dialog is cancelled", async () => {
    window.localStorage.setItem(STORAGE_KEY, "C:\\projects\\stored");

    const { result } = renderField();

    await act(() => result.current.select());

    expect(result.current.value).toBe("C:\\projects\\stored");
    expect(window.localStorage.getItem(STORAGE_KEY)).toBe("C:\\projects\\stored");
  });

  it("remembers a path a caller sets and forgets a cleared one", () => {
    const { result }: { result: { current: IPathField } } = renderField();

    act(() => result.current.setValue("C:\\projects\\typed"));

    expect(window.localStorage.getItem(STORAGE_KEY)).toBe("C:\\projects\\typed");

    act(() => result.current.clear());

    expect(result.current.value).toBeNull();
    expect(window.localStorage.getItem(STORAGE_KEY)).toBeNull();
  });

  it("fills from the seed without remembering it, and only when nothing was remembered", async () => {
    const seed = jest.fn<() => Promise<Nullable<string>>>(async () => "C:\\projects\\seeded");

    const { result } = renderField(seed);

    await act(async () => undefined);

    expect(seed).toHaveBeenCalledTimes(1);
    expect(result.current.value).toBe("C:\\projects\\seeded");
    expect(window.localStorage.getItem(STORAGE_KEY)).toBeNull();

    window.localStorage.setItem(STORAGE_KEY, "C:\\projects\\stored");
    seed.mockClear();

    const stored = renderField(seed);

    await act(async () => undefined);

    expect(seed).not.toHaveBeenCalled();
    expect(stored.result.current.value).toBe("C:\\projects\\stored");
  });
});
