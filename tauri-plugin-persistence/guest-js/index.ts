import { invoke } from '@tauri-apps/api/core'

import * as SDK from "./commands";
import { commands } from "./commands";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:persistence|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}
