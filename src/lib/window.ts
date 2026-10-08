import { getCurrentWindow } from '@tauri-apps/api/window';

/** The app's only window. */
export const appWindow = getCurrentWindow();

/** Runs a window command from a UI event; a failure (e.g. a missing permission) only warns. */
export function windowCommand(name: string, call: () => Promise<unknown>): void {
  call().catch((e) => console.warn(`window ${name} failed`, e));
}
