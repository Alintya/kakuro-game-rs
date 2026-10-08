import { getCurrentWindow } from '@tauri-apps/api/window';

export type ThemePref = 'system' | 'light' | 'dark';

const THEME_KEY = 'kakuro.theme';
const HELPER_KEY = 'kakuro.showHelper';
const NEXT: Record<ThemePref, ThemePref> = { system: 'light', light: 'dark', dark: 'system' };

export const THEME_LABEL: Record<ThemePref, string> = {
  system: 'System',
  light: 'Light',
  dark: 'Dark',
};

function readTheme(): ThemePref {
  const stored = localStorage.getItem(THEME_KEY);
  return stored === 'light' || stored === 'dark' ? stored : 'system';
}

function applyTheme(pref: ThemePref) {
  if (pref === 'system') delete document.documentElement.dataset.theme;
  else document.documentElement.dataset.theme = pref;
  // Native title bar; cosmetic, so a failure only warns.
  getCurrentWindow()
    .setTheme(pref === 'system' ? null : pref)
    .catch((e) => console.warn('setTheme failed', e));
}

/** UI preferences kept in localStorage (game progress lives in Rust). */
class Settings {
  theme = $state<ThemePref>(readTheme());
  showHelper = $state(localStorage.getItem(HELPER_KEY) !== 'false');

  cycleTheme = () => {
    this.theme = NEXT[this.theme];
    localStorage.setItem(THEME_KEY, this.theme);
    applyTheme(this.theme);
  };

  setShowHelper = (on: boolean) => {
    this.showHelper = on;
    localStorage.setItem(HELPER_KEY, String(on));
  };
}

export const settings = new Settings();
applyTheme(settings.theme);
