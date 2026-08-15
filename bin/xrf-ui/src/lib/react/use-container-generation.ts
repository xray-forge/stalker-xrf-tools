import { Binding, ContainerConfig, ServiceToken } from "@wirestate/core";
import { useState } from "react";

import { isDevelopmentBuild } from "@/lib/env";
import { Nullable } from "@/lib/types/general";

// todo: Confirm whether worth wirestate integration.

interface IContainerGeneration {
  config: Nullable<ContainerConfig>;
  generation: number;
}

/**
 * Counts the times a container config stopped describing the container that was built from it.
 *
 * @param config - Config handed to the provider, or `null` when no container is created.
 * @returns Generation to fold into the provider key.
 */
export function useContainerGeneration(config: Nullable<ContainerConfig>): number {
  const [state, setState] = useState<IContainerGeneration>({ config, generation: 0 });

  // State rather than a ref, and set during render rather than in an effect. A ref would survive a
  // render React discards, counting a rebuild that never happened; an effect would count it a commit
  // late, after the injections it exists to keep answerable have already run against the old
  // container. Bounded: the new state satisfies the check, so the extra render pass is the last one.
  if (isDevelopmentBuild() && !hasSameTokens(state.config, config)) {
    setState({ config, generation: state.generation + 1 });
  }

  return state.generation;
}

/**
 * Checks whether two configs still resolve through the same tokens.
 *
 * Compares tokens rather than whole bindings so that a descriptor rebuilt around an unchanged token
 * does not read as a change, and ignores `plugins` because those are commonly constructed inline: a
 * fresh instance every render would rebuild the container on every edit rather than on the ones that
 * moved a token.
 *
 * @param previous - Config the live container was built from.
 * @param next - Config the provider was handed this render.
 * @returns Whether the live container still answers to the tokens the config asks for.
 */
function hasSameTokens(previous: Nullable<ContainerConfig>, next: Nullable<ContainerConfig>): boolean {
  if (previous === next) {
    return true;
  }

  if (!previous || !next || previous.parent !== next.parent) {
    return false;
  }

  const previousBindings: ReadonlyArray<Binding> = previous.bindings ?? [];
  const nextBindings: ReadonlyArray<Binding> = next.bindings ?? [];

  return (
    previousBindings.length === nextBindings.length &&
    previousBindings.every(
      (binding: Binding, index: number) => getBindingToken(binding) === getBindingToken(nextBindings[index])
    )
  );
}

/**
 * Reads the token a binding registers under.
 *
 * Wirestate keeps its own `getBindingToken` internal, and a bare class binds under itself.
 *
 * @param binding - Bare service class or binding descriptor.
 * @returns Token the binding is keyed by.
 */
function getBindingToken(binding: Binding): ServiceToken {
  return typeof binding === "function" ? binding : binding.token;
}
