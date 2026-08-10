import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";

import { Optional } from "@/core/types/general";

export interface IAsyncCommandData<T> {
  isLoading: boolean;
  value: Optional<T>;
  error: Optional<string>;
  get(args?: InvokeArgs): Promise<Optional<T>>;
}

export function useInvokeCommand<T>(command: string): IAsyncCommandData<T> {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [value, setValue] = useState<Optional<T>>(null);
  const [error, setError] = useState<Optional<string>>(null);

  const get = useCallback(async (args?: InvokeArgs) => {
    setIsLoading(true);
    setError(null);

    try {
      const value: Optional<T> = await invoke<T>(command, args);

      setValue(value);
      setIsLoading(false);

      return value;
    } catch (error) {
      setError(error as string);
      setValue(null);
      setIsLoading(false);

      return null;
    }
  }, []);

  return { value, isLoading, error, get };
}
