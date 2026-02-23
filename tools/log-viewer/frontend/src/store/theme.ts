// Theme store — reactive color palette management with localStorage persistence
//
// Organizes all configurable colors into categories:
//   - Backgrounds (primary, secondary, tertiary, hover, active)
//   - Text / Fonts (primary, secondary, muted)
//   - Borders (default, subtle)
//   - Accents (blue, green, orange, purple, yellow)
//   - Log Levels (trace, debug, info, warn, error)
//   - Particle Effects (metal spark, ember, angelic beam, glitter, cinder palette)

import { signal, effect } from '@preact/signals';

// ── Default theme (Dark Souls "Cinder" theme from variables.css) ─────────────

export interface ThemeColors {
  // Backgrounds
  bgPrimary: string;
  bgSecondary: string;
  bgTertiary: string;
  bgHover: string;
  bgActive: string;

  // Text
  textPrimary: string;
  textSecondary: string;
  textMuted: string;

  // Borders
  borderColor: string;
  borderSubtle: string;

  // Accents
  accentBlue: string;
  accentGreen: string;
  accentOrange: string;
  accentPurple: string;
  accentYellow: string;

  // Log levels
  levelTrace: string;
  levelDebug: string;
  levelInfo: string;
  levelWarn: string;
  levelError: string;

  // Particle: Metal Spark
  particleSparkCore: string;
  particleSparkEmber: string;
  particleSparkSteel: string;

  // Particle: Ember / Ash
  particleEmberHot: string;
  particleEmberBase: string;

  // Particle: Angelic Beam
  particleBeamCenter: string;
  particleBeamEdge: string;

  // Particle: Glitter
  particleGlitterWarm: string;
  particleGlitterCool: string;

  // Cinder palette cycle (used in borders/glows)
  cinderEmber: string;
  cinderGold: string;
  cinderAsh: string;
  cinderVine: string;

  // Background smoke tones
  smokeCool: string;
  smokeWarm: string;
  smokeMoss: string;
}

export const DEFAULT_THEME: ThemeColors = {
  // Backgrounds
  bgPrimary: '#0d0c0b',
  bgSecondary: '#141311',
  bgTertiary: '#1a1816',
  bgHover: '#24201c',
  bgActive: '#2a2218',

  // Text
  textPrimary: '#c8c0b4',
  textSecondary: '#8a8478',
  textMuted: '#524e46',

  // Borders
  borderColor: '#2e2a24',
  borderSubtle: '#1e1c18',

  // Accents
  accentBlue: '#3a6a80',
  accentGreen: '#2a5a28',
  accentOrange: '#c85a18',
  accentPurple: '#5a3a6a',
  accentYellow: '#a08018',

  // Log levels
  levelTrace: '#3a3830',
  levelDebug: '#4a5a3a',
  levelInfo: '#3a5a6a',
  levelWarn: '#a07020',
  levelError: '#8a2a18',

  // Particle: Metal Spark
  particleSparkCore: '#ffe699',     // vec3(1.4, 1.1, 0.6) → approx
  particleSparkEmber: '#d94d14',    // cinder_rgb warm
  particleSparkSteel: '#9999b3',    // vec3(0.6, 0.6, 0.7)

  // Particle: Ember / Ash
  particleEmberHot: '#e6b366',      // vec3(1.2, 0.9, 0.4) → clamped
  particleEmberBase: '#d94d14',

  // Particle: Angelic Beam
  particleBeamCenter: '#ffedcc',    // vec3(1.6, 1.5, 1.3) → bright gold-white
  particleBeamEdge: '#cc9933',      // vec3(1.1, 0.85, 0.4) → warm gold

  // Particle: Glitter
  particleGlitterWarm: '#ffdfad',   // vec3(1.3, 1.15, 0.85)
  particleGlitterCool: '#b3bfff',   // vec3(0.85, 0.90, 1.25)

  // Cinder palette
  cinderEmber: '#d94d14',           // vec3(0.85, 0.30, 0.08)
  cinderGold: '#cc8c1f',            // vec3(0.80, 0.55, 0.12)
  cinderAsh: '#595247',             // vec3(0.35, 0.32, 0.28)
  cinderVine: '#2e7326',            // vec3(0.18, 0.45, 0.15)

  // Background smoke
  smokeCool: '#080914',             // vec3(0.03, 0.035, 0.05)
  smokeWarm: '#0e0906',             // vec3(0.055, 0.035, 0.025)
  smokeMoss: '#090a09',             // vec3(0.035, 0.04, 0.035)
};

// ── Preset themes ────────────────────────────────────────────────────────────

export interface ThemePreset {
  name: string;
  description: string;
  colors: ThemeColors;
}

export const THEME_PRESETS: ThemePreset[] = [
  {
    name: 'Cinder (Default)',
    description: 'Dark Souls gothic stone — ember & vine',
    colors: { ...DEFAULT_THEME },
  },
  {
    name: 'Frost',
    description: 'Icy blue — cold steel & aurora',
    colors: {
      ...DEFAULT_THEME,
      bgPrimary: '#080b10',
      bgSecondary: '#0c1018',
      bgTertiary: '#111620',
      bgHover: '#1a2030',
      bgActive: '#1e2838',
      textPrimary: '#b8c8d8',
      textSecondary: '#6878888',
      textMuted: '#384858',
      borderColor: '#1e2838',
      borderSubtle: '#141c28',
      accentBlue: '#4a8aaa',
      accentGreen: '#2a6a58',
      accentOrange: '#aa6030',
      accentPurple: '#5a4a8a',
      accentYellow: '#8a8a40',
      levelTrace: '#283038',
      levelDebug: '#2a4a48',
      levelInfo: '#2a4a6a',
      levelWarn: '#6a5a2a',
      levelError: '#6a2228',
      particleSparkCore: '#aad4ff',
      particleSparkEmber: '#4488cc',
      particleSparkSteel: '#8899bb',
      particleEmberHot: '#88bbff',
      particleEmberBase: '#4488cc',
      particleBeamCenter: '#ddeeff',
      particleBeamEdge: '#6699cc',
      particleGlitterWarm: '#aaccff',
      particleGlitterCool: '#88aadd',
      cinderEmber: '#4488bb',
      cinderGold: '#5588aa',
      cinderAsh: '#445566',
      cinderVine: '#2a6a58',
      smokeCool: '#060810',
      smokeWarm: '#080a12',
      smokeMoss: '#070910',
    },
  },
  {
    name: 'Blood Moon',
    description: 'Crimson darkness — blood & shadow',
    colors: {
      ...DEFAULT_THEME,
      bgPrimary: '#0e0808',
      bgSecondary: '#160c0c',
      bgTertiary: '#1e1212',
      bgHover: '#2a1818',
      bgActive: '#321c1c',
      textPrimary: '#d0b8b0',
      textSecondary: '#8a7068',
      textMuted: '#4e3a34',
      borderColor: '#361e1a',
      borderSubtle: '#241412',
      accentBlue: '#5a4a6a',
      accentGreen: '#3a4a2a',
      accentOrange: '#cc4420',
      accentPurple: '#6a2a4a',
      accentYellow: '#aa6a20',
      levelTrace: '#382828',
      levelDebug: '#3a3828',
      levelInfo: '#3a2a4a',
      levelWarn: '#8a4a18',
      levelError: '#aa2218',
      particleSparkCore: '#ff8866',
      particleSparkEmber: '#cc2210',
      particleSparkSteel: '#8a6666',
      particleEmberHot: '#ff6644',
      particleEmberBase: '#cc2210',
      particleBeamCenter: '#ffccbb',
      particleBeamEdge: '#cc5533',
      particleGlitterWarm: '#ffaa88',
      particleGlitterCool: '#cc88aa',
      cinderEmber: '#cc2210',
      cinderGold: '#aa4422',
      cinderAsh: '#4a3030',
      cinderVine: '#443828',
      smokeCool: '#0a0608',
      smokeWarm: '#100808',
      smokeMoss: '#0a0808',
    },
  },
  {
    name: 'Verdant',
    description: 'Forest depths — moss & ancient growth',
    colors: {
      ...DEFAULT_THEME,
      bgPrimary: '#080c08',
      bgSecondary: '#0c120c',
      bgTertiary: '#121a12',
      bgHover: '#1a261a',
      bgActive: '#1e2e1e',
      textPrimary: '#b4c8b0',
      textSecondary: '#6a8468',
      textMuted: '#3a4e38',
      borderColor: '#1e2e1e',
      borderSubtle: '#141e14',
      accentBlue: '#3a6a5a',
      accentGreen: '#2a6a28',
      accentOrange: '#8a6a28',
      accentPurple: '#4a4a5a',
      accentYellow: '#7a8a28',
      levelTrace: '#283828',
      levelDebug: '#2a5a2a',
      levelInfo: '#2a4a4a',
      levelWarn: '#6a6a20',
      levelError: '#6a3a18',
      particleSparkCore: '#bbff99',
      particleSparkEmber: '#44aa22',
      particleSparkSteel: '#88aa88',
      particleEmberHot: '#99dd66',
      particleEmberBase: '#44aa22',
      particleBeamCenter: '#ddffcc',
      particleBeamEdge: '#66aa44',
      particleGlitterWarm: '#aaffaa',
      particleGlitterCool: '#88ccaa',
      cinderEmber: '#44aa22',
      cinderGold: '#66aa44',
      cinderAsh: '#3a4a38',
      cinderVine: '#228822',
      smokeCool: '#060a06',
      smokeWarm: '#080a06',
      smokeMoss: '#060a06',
    },
  },
];

// ── Reactive state ──────────────────────────────────────────────────────────

const STORAGE_KEY = 'log-viewer-theme';
const SETTINGS_KEY = 'log-viewer-effect-settings';

// ── Effect settings (non-color toggles) ──────────────────────────────────────

export interface EffectSettings {
  crtEnabled: boolean;
  /** Horizontal scanlines (+ pixel grid) intensity 0–100. */
  crtScanlinesH: number;
  /** Vertical scanlines (+ pixel grid) intensity 0–100. */
  crtScanlinesV: number;
  /** Edge/border shadow intensity 0–100. */
  crtEdgeShadow: number;
  /** Torch flicker intensity 0–100. */
  crtFlicker: number;
}

export const DEFAULT_EFFECT_SETTINGS: EffectSettings = {
  crtEnabled: true,
  crtScanlinesH: 100,
  crtScanlinesV: 100,
  crtEdgeShadow: 100,
  crtFlicker: 100,
};

function loadEffectSettings(): EffectSettings {
  try {
    const saved = localStorage.getItem(SETTINGS_KEY);
    if (saved) {
      return { ...DEFAULT_EFFECT_SETTINGS, ...JSON.parse(saved) };
    }
  } catch { /* ignore */ }
  return { ...DEFAULT_EFFECT_SETTINGS };
}

export const effectSettings = signal<EffectSettings>(loadEffectSettings());

effect(() => {
  try {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(effectSettings.value));
  } catch { /* storage full */ }
});

export function updateEffectSetting<K extends keyof EffectSettings>(key: K, value: EffectSettings[K]) {
  effectSettings.value = { ...effectSettings.value, [key]: value };
}

function loadSavedTheme(): ThemeColors {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      const parsed = JSON.parse(saved);
      // Merge with defaults to handle new keys added in updates
      return { ...DEFAULT_THEME, ...parsed };
    }
  } catch {
    // ignore corrupt storage
  }
  return { ...DEFAULT_THEME };
}

export const themeColors = signal<ThemeColors>(loadSavedTheme());

// Persist to localStorage on every change
effect(() => {
  const colors = themeColors.value;
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(colors));
  } catch {
    // storage full or unavailable
  }
});

// Apply CSS custom properties on every change via a <style> element
// (Using a <style> tag instead of inline styles so that :root.gpu-active
//  rules in variables.css can still override background/border variables
//  with higher specificity.)
let themeStyleEl: HTMLStyleElement | null = null;

effect(() => {
  const c = themeColors.value;
  if (!themeStyleEl) {
    themeStyleEl = document.createElement('style');
    themeStyleEl.id = 'theme-overrides';
    document.head.appendChild(themeStyleEl);
  }
  themeStyleEl.textContent = `:root {
  --bg-primary: ${c.bgPrimary};
  --bg-secondary: ${c.bgSecondary};
  --bg-tertiary: ${c.bgTertiary};
  --bg-hover: ${c.bgHover};
  --bg-active: ${c.bgActive};
  --text-primary: ${c.textPrimary};
  --text-secondary: ${c.textSecondary};
  --text-muted: ${c.textMuted};
  --border-color: ${c.borderColor};
  --border-subtle: ${c.borderSubtle};
  --accent-blue: ${c.accentBlue};
  --accent-green: ${c.accentGreen};
  --accent-orange: ${c.accentOrange};
  --accent-purple: ${c.accentPurple};
  --accent-yellow: ${c.accentYellow};
  --level-trace: ${c.levelTrace};
  --level-debug: ${c.levelDebug};
  --level-info: ${c.levelInfo};
  --level-warn: ${c.levelWarn};
  --level-error: ${c.levelError};
}`;
});

// ── Actions ─────────────────────────────────────────────────────────────────

export function updateThemeColor<K extends keyof ThemeColors>(key: K, value: string) {
  themeColors.value = { ...themeColors.value, [key]: value };
}

export function applyPreset(preset: ThemeColors) {
  themeColors.value = { ...preset };
}

export function resetTheme() {
  themeColors.value = { ...DEFAULT_THEME };
}

/** Generate a random hex colour, optionally clamping lightness to a range. */
function randHex(minL = 0, maxL = 1): string {
  // Generate in HSL then convert to hex for better perceptual distribution
  const h = Math.random() * 360;
  const s = 0.4 + Math.random() * 0.5;           // 40-90 % saturation
  const l = minL + Math.random() * (maxL - minL); // lightness in range

  // HSL → RGB (standard conversion)
  const c = (1 - Math.abs(2 * l - 1)) * s;
  const x = c * (1 - Math.abs(((h / 60) % 2) - 1));
  const m = l - c / 2;
  let r1: number, g1: number, b1: number;
  if (h < 60)       { r1 = c; g1 = x; b1 = 0; }
  else if (h < 120) { r1 = x; g1 = c; b1 = 0; }
  else if (h < 180) { r1 = 0; g1 = c; b1 = x; }
  else if (h < 240) { r1 = 0; g1 = x; b1 = c; }
  else if (h < 300) { r1 = x; g1 = 0; b1 = c; }
  else              { r1 = c; g1 = 0; b1 = x; }
  return vec3ToHex(r1 + m, g1 + m, b1 + m);
}

/** Randomize every color in the theme. */
export function randomizeTheme() {
  themeColors.value = {
    // Backgrounds — keep dark (lightness 0.02–0.15)
    bgPrimary:   randHex(0.02, 0.08),
    bgSecondary: randHex(0.05, 0.12),
    bgTertiary:  randHex(0.07, 0.14),
    bgHover:     randHex(0.08, 0.16),
    bgActive:    randHex(0.10, 0.18),

    // Text — keep light (0.55–0.95)
    textPrimary:   randHex(0.80, 0.95),
    textSecondary: randHex(0.60, 0.78),
    textMuted:     randHex(0.40, 0.55),

    // Borders (0.15–0.30)
    borderColor:  randHex(0.15, 0.28),
    borderSubtle: randHex(0.10, 0.20),

    // Accents (0.35–0.65)
    accentBlue:   randHex(0.35, 0.60),
    accentGreen:  randHex(0.35, 0.60),
    accentOrange: randHex(0.40, 0.65),
    accentPurple: randHex(0.35, 0.55),
    accentYellow: randHex(0.45, 0.65),

    // Log levels (0.35–0.60)
    levelTrace: randHex(0.30, 0.50),
    levelDebug: randHex(0.30, 0.50),
    levelInfo:  randHex(0.40, 0.60),
    levelWarn:  randHex(0.45, 0.65),
    levelError: randHex(0.40, 0.55),

    // Particles — vivid (0.45–0.85)
    particleSparkCore:  randHex(0.70, 0.90),
    particleSparkEmber: randHex(0.35, 0.55),
    particleSparkSteel: randHex(0.40, 0.60),
    particleEmberHot:   randHex(0.55, 0.75),
    particleEmberBase:  randHex(0.30, 0.50),
    particleBeamCenter: randHex(0.70, 0.90),
    particleBeamEdge:   randHex(0.40, 0.60),
    particleGlitterWarm: randHex(0.60, 0.80),
    particleGlitterCool: randHex(0.50, 0.70),

    // Cinder palette (0.25–0.55)
    cinderEmber: randHex(0.30, 0.50),
    cinderGold:  randHex(0.35, 0.55),
    cinderAsh:   randHex(0.20, 0.35),
    cinderVine:  randHex(0.20, 0.40),

    // Smoke tones — very dark (0.02–0.08)
    smokeCool: randHex(0.02, 0.06),
    smokeWarm: randHex(0.02, 0.06),
    smokeMoss: randHex(0.02, 0.06),
  };
}

// ── Saved themes (user-created, persisted in localStorage) ──────────────────

export interface SavedTheme {
  id: string;
  name: string;
  colors: ThemeColors;
  createdAt: number;
}

const SAVED_THEMES_KEY = 'log-viewer-saved-themes';

function loadSavedThemes(): SavedTheme[] {
  try {
    const raw = localStorage.getItem(SAVED_THEMES_KEY);
    if (raw) return JSON.parse(raw) as SavedTheme[];
  } catch { /* ignore */ }
  return [];
}

function persistSavedThemes(themes: SavedTheme[]) {
  try {
    localStorage.setItem(SAVED_THEMES_KEY, JSON.stringify(themes));
  } catch { /* storage full */ }
}

export const savedThemes = signal<SavedTheme[]>(loadSavedThemes());

/** Save the current theme under the given name. */
export function saveCurrentTheme(name: string) {
  const theme: SavedTheme = {
    id: Date.now().toString(36) + Math.random().toString(36).slice(2, 6),
    name,
    colors: { ...themeColors.value },
    createdAt: Date.now(),
  };
  const updated = [...savedThemes.value, theme];
  savedThemes.value = updated;
  persistSavedThemes(updated);
}

/** Delete a saved theme by id. */
export function deleteSavedTheme(id: string) {
  const updated = savedThemes.value.filter(t => t.id !== id);
  savedThemes.value = updated;
  persistSavedThemes(updated);
}

/** Apply a saved theme. */
export function applySavedTheme(theme: SavedTheme) {
  themeColors.value = { ...theme.colors };
}

/** Rename a saved theme. */
export function renameSavedTheme(id: string, newName: string) {
  const updated = savedThemes.value.map(t =>
    t.id === id ? { ...t, name: newName } : t
  );
  savedThemes.value = updated;
  persistSavedThemes(updated);
}

// ── Helpers for converting hex to shader-compatible vec3 ────────────────────

/** Convert "#rrggbb" to [r, g, b] in 0..1 range */
export function hexToVec3(hex: string): [number, number, number] {
  const h = hex.replace('#', '');
  const r = parseInt(h.slice(0, 2), 16) / 255;
  const g = parseInt(h.slice(2, 4), 16) / 255;
  const b = parseInt(h.slice(4, 6), 16) / 255;
  return [r, g, b];
}

/** Convert [r, g, b] (0..1) to "#rrggbb" */
export function vec3ToHex(r: number, g: number, b: number): string {
  const clamp = (v: number) => Math.max(0, Math.min(255, Math.round(v * 255)));
  return '#' + [r, g, b].map(v => clamp(v).toString(16).padStart(2, '0')).join('');
}
