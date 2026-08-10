export type AnyObject = Record<string, any>;

export type Nullable<T> = T | null;

export type Maybe<T> = T | null | undefined;

export type AnyCallable<T = void> = (...args: Array<unknown>) => T;
