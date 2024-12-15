import type { State, StoreBackendRawOptions } from './store';

/** @internal */
export interface ConfigChangePayload {
  config: StoreBackendRawOptions;
  id: string;
}

/** @internal */
export interface StateChangePayload {
  id: string;
  state: State;
}
