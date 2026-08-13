import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";

import { Nullable } from "@/lib/types/general";

export interface IAsyncCommandData<T> {
  isLoading: boolean;
  value: Nullable<T>;
  error: Nullable<string>;
  get(args?: InvokeArgs): Promise<Nullable<T>>;
}

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
