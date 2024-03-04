import { useCallback, useState } from "react";

export function useTabState<T extends string>(active: T): [T, (active: T) => void, (event: unknown, value: T) => void] {
  const [activeTab, setActiveTab] = useState(active);

  const onActiveTabChange = useCallback((_: unknown, newValue: T) => {
    setActiveTab(newValue);
  }, []);

  return [activeTab, setActiveTab, onActiveTabChange];
}
