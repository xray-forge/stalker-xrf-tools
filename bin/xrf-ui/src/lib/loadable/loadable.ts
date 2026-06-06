import { Optional } from "@/core/types/general";

export class Loadable<T, E = Error> {
  public constructor(
    public readonly value: Optional<T> = null,
    public readonly isLoading: boolean = false,
    public readonly error: Optional<E> = null
  ) {}

  public asReady(value: Optional<T> = this.value): Loadable<T, E> {
    return createLoadable<T, E>(value, false, null);
  }

  public asUpdated(value: T, isLoading: boolean = this.isLoading, error: Optional<E> = this.error): Loadable<T, E> {
    return createLoadable(value, isLoading, error);
  }

  public asLoading(value: Optional<T> = this.value): Loadable<T, E> {
    return createLoadable<T, E>(value, true, null);
  }

  public asFailed(error: E, value: Optional<T> = this.value): Loadable<T, E> {
    return createLoadable(value, false, error);
  }
}

export function createLoadable<T, E = Error>(
  value: Optional<T> = null,
  isLoading: boolean = false,
  error: Optional<E> = null
): Loadable<T, E> {
  return new Loadable(value, isLoading, error);
}
