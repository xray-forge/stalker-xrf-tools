import { XRayError } from "@/lib/xrf/bindings/xrf-error";

/**
 * One scheme problem, unwrapped from the error variant that carries it.
 *
 * The backend reports failures as its own error enum, which serde tags externally: a scheme problem
 * arrives as `{ LtxScheme: { ... } }` rather than as its fields. Verification only ever produces that
 * variant, so this narrows to it and hands back the payload the table actually renders.
 */
export type TLtxSchemeError = Extract<XRayError, { LtxScheme: unknown }>["LtxScheme"];

/**
 * Take the scheme problems out of a verification result, discarding any other error variant.
 */
export function toLtxSchemeErrors(errors: Array<XRayError>): Array<TLtxSchemeError> {
  return errors
    .filter((it: XRayError): it is Extract<XRayError, { LtxScheme: unknown }> => "LtxScheme" in it)
    .map((it) => it.LtxScheme);
}
