import { appWindow, windowCommand } from '#lib/window.js';

export type ThemePref = 'system' | 'light' | 'dark';

const THEME_KEY = 'kakuro.theme';
/** Boolean preferences: localStorage key and value until the user changes it. */
const FLAGS = {
  /** Solving aids are opt-in. */
  showCombinations: { key: 'kakuro.aid.combinations', initial: false },
  dimDigits: { key: 'kakuro.aid.dimDigits', initial: false },
  pauseWhenMinimized: { key: 'kakuro.pauseWhenMinimized', initial: true },
} as const;
export type Flag = keyof typeof FLAGS;

function readFlag(flag: Flag): boolean {
  const stored = localStorage.getItem(FLAGS[flag].key);
  return stored === null ? FLAGS[flag].initial : stored === 'true';
}
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
  // Native title bar (where there is one); cosmetic, so a failure only warns.
  windowCommand('setTheme', () => appWindow.setTheme(pref === 'system' ? null : pref));
}

/** UI preferences kept in localStorage (game progress lives in Rust). */
class Settings {
  theme = $state<ThemePref>(readTheme());
  /** Combinations panel for the selected cell's runs. */
  showCombinations = $state(readFlag('showCombinations'));
  /** Fade pad digits that cannot go in the selected cell. */
  dimDigits = $state(readFlag('dimDigits'));
  /** Stop the play clock while the window is minimized or hidden. */
  pauseWhenMinimized = $state(readFlag('pauseWhenMinimized'));

  cycleTheme = () => {
    this.theme = NEXT[this.theme];
    localStorage.setItem(THEME_KEY, this.theme);
    applyTheme(this.theme);
  };

  setFlag = (flag: Flag, on: boolean) => {
    this[flag] = on;
    localStorage.setItem(FLAGS[flag].key, String(on));
  };
}

export const settings = new Settings();
applyTheme(settings.theme);
