import { Nullable } from "@/core/types/general";

export class Loadable<T, E = Error> {
  public constructor(
    public readonly value: Nullable<T> = null,
    public readonly isLoading: boolean = false,
    public readonly error: Nullable<E> = null
  ) {}

  public asReady(value: Nullable<T> = this.value): Loadable<T, E> {
    return createLoadable<T, E>(value, false, null);
  }

  public asUpdated(value: T, isLoading: boolean = this.isLoading, error: Nullable<E> = this.error): Loadable<T, E> {
    return createLoadable(value, isLoading, error);
  }

  public asLoading(value: Nullable<T> = this.value): Loadable<T, E> {
    return createLoadable<T, E>(value, true, null);
  }

  public asFailed(error: E, value: Nullable<T> = this.value): Loadable<T, E> {
    return createLoadable(value, false, error);
  }
}

export function createLoadable<T, E = Error>(
  value: Nullable<T> = null,
  isLoading: boolean = false,
  error: Nullable<E> = null
): Loadable<T, E> {
  return new Loadable(value, isLoading, error);
}
