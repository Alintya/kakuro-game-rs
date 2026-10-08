import { getCurrentWindow } from '@tauri-apps/api/window';

export type ThemePref = 'system' | 'light' | 'dark';

const THEME_KEY = 'kakuro.theme';
/** Solving aids are opt-in; stored as 'true' once enabled in Settings. */
const AID_KEYS = {
  showCombinations: 'kakuro.aid.combinations',
  dimDigits: 'kakuro.aid.dimDigits',
} as const;
export type Aid = keyof typeof AID_KEYS;
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
  /** Combinations panel for the selected cell's runs. */
  showCombinations = $state(localStorage.getItem(AID_KEYS.showCombinations) === 'true');
  /** Fade pad digits that cannot go in the selected cell. */
  dimDigits = $state(localStorage.getItem(AID_KEYS.dimDigits) === 'true');

  cycleTheme = () => {
    this.theme = NEXT[this.theme];
    localStorage.setItem(THEME_KEY, this.theme);
    applyTheme(this.theme);
  };

  setAid = (aid: Aid, on: boolean) => {
    this[aid] = on;
    localStorage.setItem(AID_KEYS[aid], String(on));
  };
}

export const settings = new Settings();
applyTheme(settings.theme);
