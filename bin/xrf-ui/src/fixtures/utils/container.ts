import { Binding, Container, EventsPlugin, ServiceToken } from "@wirestate/core";

import { AssetService } from "@/core/assets/services";

export interface IInjectedServiceMockDescriptor<T> {
  service: T;
  container: Container;
}

/**
 * Build a service the way the application does, through a container.
 *
 * Services that resolve dependencies with `inject()` cannot be constructed with `new`: there is no
 * injection context, and the call throws. Resolving without provisioning on purpose, so `@OnProvision`
 * does not fire and a test still sees a service that has asked the backend for nothing.
 */
export function mockInjectedService<T>(
  token: ServiceToken<T>,
  bindings: Array<Binding> = []
): IInjectedServiceMockDescriptor<T> {
  const container: Container = new Container({
    bindings: [AssetService, ...bindings, token as Binding],
    plugins: [new EventsPlugin()],
  });

  return {
    container,
    service: container.get(token),
  };
}
