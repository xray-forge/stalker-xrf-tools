import { Nullable } from "@/lib/types/general";

/**
 * Immutable value, loading, and failure state for an asynchronous resource.
 *
 * `value` may remain available while a refresh is loading or has failed, so callers can keep showing the last
 * successful result. The transition helpers return a new instance and clear the state that no longer applies.
 */
export class Loadable<T, E = Error> {
  /**
   * Creates a resource state.
   *
   * @param value - The latest value, including a stale value during loading or after failure.
   * @param isLoading - Whether the resource is currently being fetched or produced.
   * @param error - The latest failure, or `null` when the state is not failed.
   */
  public constructor(
    public readonly value: Nullable<T> = null,
    public readonly isLoading: boolean = false,
    public readonly error: Nullable<E> = null
  ) {}

  /**
   * Marks the resource ready and clears any loading or failure state.
   *
   * @param value - Ready value, defaulting to the current value.
   * @returns A new ready state.
   */
  public asReady(value: Nullable<T> = this.value): Loadable<T, E> {
    return createLoadable<T, E>(value, false, null);
  }

  /**
   * Replaces the value while preserving loading and error state by default.
   *
   * @param value - Replacement value.
   * @param isLoading - Loading state to retain or replace.
   * @param error - Failure to retain or replace.
   * @returns A new state with the supplied fields.
   */
  public asUpdated(value: T, isLoading: boolean = this.isLoading, error: Nullable<E> = this.error): Loadable<T, E> {
    return createLoadable(value, isLoading, error);
  }

  /**
   * Marks the resource loading and clears any previous failure.
   *
   * @param value - Value to retain while loading.
   * @returns A new loading state.
   */
  public asLoading(value: Nullable<T> = this.value): Loadable<T, E> {
    return createLoadable<T, E>(value, true, null);
  }

  /**
   * Marks the resource failed while optionally retaining the previous value.
   *
   * @param error - Failure to expose.
   * @param value - Value to retain after failure.
   * @returns A new failed state.
   */
  public asFailed(error: E, value: Nullable<T> = this.value): Loadable<T, E> {
    return createLoadable(value, false, error);
  }
}

/**
 * Creates an immutable asynchronous-resource state.
 *
 * @param value - Current value, or `null` when no value is available.
 * @param isLoading - Whether production of the value is still in progress.
 * @param error - Failure to expose, or `null` when the state is not failed.
 * @returns A new loadable state containing the supplied fields.
 */
export function createLoadable<T, E = Error>(
  value: Nullable<T> = null,
  isLoading: boolean = false,
  error: Nullable<E> = null
): Loadable<T, E> {
  return new Loadable(value, isLoading, error);
}
