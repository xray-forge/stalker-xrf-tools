import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";

import { Nullable } from "@/lib/types/general";

export interface IAsyncCommandData<T> {
  /** Whether the most recent invocation is still pending. */
  isLoading: boolean;
  /** Last successful response, or `null` before success and after a failed invocation. */
  value: Nullable<T>;
  /** Most recent command failure, or `null` when no failure is recorded. */
  error: Nullable<string>;
  /**
   * Invokes the command and updates `isLoading`, `value`, and `error`.
   *
   * @param args - Arguments forwarded to Tauri.
   * @returns The response, or `null` after a failed invocation.
   */
  get(args?: InvokeArgs): Promise<Nullable<T>>;
}

/**
 * Keeps the result of a Tauri command in React state.
 *
 * Each invocation clears the previous error; a failure clears the previous value and resolves to `null` instead of
 * propagating, so callers can render the returned state without a second error boundary.
 *
 * @param command - Tauri command name passed to `invoke`.
 * @returns Command state and an invocation callback.
 */
export function useInvokeCommand<T>(command: string): IAsyncCommandData<T> {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [value, setValue] = useState<Nullable<T>>(null);
  const [error, setError] = useState<Nullable<string>>(null);

  const get = useCallback(
    async (args?: InvokeArgs) => {
      setIsLoading(true);
      setError(null);

      try {
        const value: Nullable<T> = await invoke<T>(command, args);

        setValue(value);
        setIsLoading(false);

        return value;
      } catch (error) {
        setError(error as string);
        setValue(null);
        setIsLoading(false);

        return null;
      }
    },
    [command]
  );

  return { value, isLoading, error, get };
}
