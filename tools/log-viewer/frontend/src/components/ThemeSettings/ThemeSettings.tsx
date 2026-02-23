/**
 * ThemeSettings — Color theme settings page.
 *
 * Organized into collapsible sections:
 *   - Presets (one-click theme switching)
 *   - Backgrounds
 *   - Text / Fonts
 *   - Borders
 *   - Accents
 *   - Log Levels
 *   - Particle Effects (Metal Sparks, Embers, Angelic Beams, Glitter)
 *   - Cinder Palette
 *   - Background Smoke
 */
import { useState, useRef } from 'preact/hooks';
import {
  themeColors,
  updateThemeColor,
  applyPreset,
  resetTheme,
  randomizeTheme,
  THEME_PRESETS,
  DEFAULT_THEME,
  savedThemes,
  saveCurrentTheme,
  deleteSavedTheme,
  applySavedTheme,
  renameSavedTheme,
  effectSettings,
  updateEffectSetting,
  type ThemeColors,
  type SavedTheme,
} from '../../store/theme';
import './theme-settings.css';

// ── Color picker row ─────────────────────────────────────────────────────────

interface ColorRowProps {
  label: string;
  description?: string;
  colorKey: keyof ThemeColors;
}

function ColorRow({ label, description, colorKey }: ColorRowProps) {
  const value = themeColors.value[colorKey];
  return (
    <div class="theme-color-row">
      <div class="theme-color-info">
        <span class="theme-color-label">{label}</span>
        {description && <span class="theme-color-desc">{description}</span>}
      </div>
      <div class="theme-color-controls">
        <input
          type="color"
          class="theme-color-picker"
          value={value}
          onInput={(e) => updateThemeColor(colorKey, (e.target as HTMLInputElement).value)}
        />
        <input
          type="text"
          class="theme-color-hex"
          value={value}
          maxLength={7}
          onInput={(e) => {
            const v = (e.target as HTMLInputElement).value;
            if (/^#[0-9a-fA-F]{6}$/.test(v)) {
              updateThemeColor(colorKey, v);
            }
          }}
        />
        <button
          class="theme-color-reset"
          title="Reset to default"
          onClick={() => updateThemeColor(colorKey, DEFAULT_THEME[colorKey])}
        >
          ↺
        </button>
      </div>
    </div>
  );
}

// ── Collapsible section ──────────────────────────────────────────────────────

interface SectionProps {
  title: string;
  icon: string;
  children: preact.ComponentChildren;
  defaultOpen?: boolean;
}

function Section({ title, icon, children, defaultOpen = false }: SectionProps) {
  const [open, setOpen] = useState(defaultOpen);
  return (
    <section class={`theme-section ${open ? 'open' : ''}`}>
      <button class="theme-section-header" onClick={() => setOpen(!open)}>
        <span class="theme-section-icon">{icon}</span>
        <span class="theme-section-title">{title}</span>
        <span class="theme-section-chevron">{open ? '▾' : '▸'}</span>
      </button>
      {open && <div class="theme-section-body">{children}</div>}
    </section>
  );
}

// ── Save theme dialog ────────────────────────────────────────────────────────

function SaveThemeButton() {
  const [showInput, setShowInput] = useState(false);
  const [name, setName] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  function handleSave() {
    const trimmed = name.trim();
    if (trimmed) {
      saveCurrentTheme(trimmed);
      setName('');
      setShowInput(false);
    }
  }

  if (!showInput) {
    return (
      <button class="btn btn-primary" onClick={() => { setShowInput(true); setTimeout(() => inputRef.current?.focus(), 0); }}>
        💾 Save Theme
      </button>
    );
  }

  return (
    <div class="save-theme-inline">
      <input
        ref={inputRef}
        type="text"
        class="save-theme-input"
        placeholder="Theme name…"
        value={name}
        maxLength={40}
        onInput={(e) => setName((e.target as HTMLInputElement).value)}
        onKeyDown={(e) => {
          if (e.key === 'Enter') handleSave();
          if (e.key === 'Escape') { setShowInput(false); setName(''); }
        }}
      />
      <button class="btn btn-primary" onClick={handleSave} disabled={!name.trim()}>Save</button>
      <button class="btn btn-secondary" onClick={() => { setShowInput(false); setName(''); }}>✕</button>
    </div>
  );
}

// ── Saved theme card ─────────────────────────────────────────────────────────

function SavedThemeCard({ theme }: { theme: SavedTheme }) {
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [editing, setEditing] = useState(false);
  const [editName, setEditName] = useState(theme.name);

  function handleRename() {
    const trimmed = editName.trim();
    if (trimmed && trimmed !== theme.name) {
      renameSavedTheme(theme.id, trimmed);
    }
    setEditing(false);
  }

  const date = new Date(theme.createdAt);
  const dateStr = `${date.toLocaleDateString()} ${date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;

  return (
    <div class="saved-theme-card">
      <div class="saved-theme-swatches">
        <span class="theme-preset-swatch" style={{ background: theme.colors.bgPrimary }} />
        <span class="theme-preset-swatch" style={{ background: theme.colors.accentOrange }} />
        <span class="theme-preset-swatch" style={{ background: theme.colors.accentBlue }} />
        <span class="theme-preset-swatch" style={{ background: theme.colors.levelError }} />
        <span class="theme-preset-swatch" style={{ background: theme.colors.cinderEmber }} />
        <span class="theme-preset-swatch" style={{ background: theme.colors.textPrimary }} />
      </div>
      <div class="saved-theme-info">
        {editing ? (
          <input
            type="text"
            class="save-theme-input saved-theme-rename"
            value={editName}
            maxLength={40}
            onInput={(e) => setEditName((e.target as HTMLInputElement).value)}
            onKeyDown={(e) => {
              if (e.key === 'Enter') handleRename();
              if (e.key === 'Escape') { setEditing(false); setEditName(theme.name); }
            }}
            onBlur={handleRename}
            autoFocus
          />
        ) : (
          <strong class="saved-theme-name" onDblClick={() => setEditing(true)} title="Double-click to rename">
            {theme.name}
          </strong>
        )}
        <span class="saved-theme-date">{dateStr}</span>
      </div>
      <div class="saved-theme-actions">
        <button class="btn btn-primary btn-sm" onClick={() => applySavedTheme(theme)} title="Apply this theme">
          Apply
        </button>
        {confirmDelete ? (
          <button class="btn btn-danger btn-sm" onClick={() => { deleteSavedTheme(theme.id); setConfirmDelete(false); }}>
            Confirm
          </button>
        ) : (
          <button class="btn btn-secondary btn-sm" onClick={() => setConfirmDelete(true)} title="Delete this theme">
            🗑
          </button>
        )}
      </div>
    </div>
  );
}

// ── Saved themes panel ───────────────────────────────────────────────────────

function SavedThemesPanel() {
  const themes = savedThemes.value;

  return (
    <div class="saved-themes-panel">
      <h3 class="saved-themes-title">Saved Themes</h3>
      <p class="saved-themes-subtitle">
        Your custom themes, stored in the browser.
      </p>
      {themes.length === 0 ? (
        <div class="saved-themes-empty">
          <span class="saved-themes-empty-icon">◇</span>
          <p>No saved themes yet.</p>
          <p class="saved-themes-empty-hint">Use the "💾 Save Theme" button to save your current color configuration.</p>
        </div>
      ) : (
        <div class="saved-themes-list">
          {themes.map(t => <SavedThemeCard key={t.id} theme={t} />)}
        </div>
      )}
    </div>
  );
}

// ── Main component ──────────────────────────────────────────────────────────

export function ThemeSettings() {
  return (
    <div class="theme-settings-layout">
    <div class="theme-settings">
      <div class="theme-settings-header">
        <h2 class="theme-settings-title">Color Theme Settings</h2>
        <p class="theme-settings-subtitle">
          Customize every color in the palette. Changes are applied instantly and saved to your browser.
        </p>
        <div class="theme-settings-actions">
          <button class="btn btn-primary" onClick={resetTheme}>
            Reset to Default
          </button>
          <button class="btn btn-primary" onClick={randomizeTheme}>
            🎲 Randomize
          </button>
          <SaveThemeButton />
        </div>
      </div>

      {/* ── Presets ── */}
      <Section title="Theme Presets" icon="◆" defaultOpen={true}>
        <div class="theme-presets-grid">
          {THEME_PRESETS.map((preset) => (
            <button
              key={preset.name}
              class="theme-preset-card"
              onClick={() => applyPreset(preset.colors)}
            >
              <div class="theme-preset-swatches">
                <span class="theme-preset-swatch" style={{ background: preset.colors.bgPrimary }} />
                <span class="theme-preset-swatch" style={{ background: preset.colors.accentOrange }} />
                <span class="theme-preset-swatch" style={{ background: preset.colors.accentBlue }} />
                <span class="theme-preset-swatch" style={{ background: preset.colors.levelError }} />
                <span class="theme-preset-swatch" style={{ background: preset.colors.cinderEmber }} />
              </div>
              <div class="theme-preset-info">
                <strong>{preset.name}</strong>
                <span>{preset.description}</span>
              </div>
            </button>
          ))}
        </div>
      </Section>

      {/* ── Backgrounds ── */}
      <Section title="Backgrounds" icon="▧">
        <ColorRow label="Primary" description="Main app background" colorKey="bgPrimary" />
        <ColorRow label="Secondary" description="Header, panels" colorKey="bgSecondary" />
        <ColorRow label="Tertiary" description="Inputs, nested areas" colorKey="bgTertiary" />
        <ColorRow label="Hover" description="Hovered elements" colorKey="bgHover" />
        <ColorRow label="Active" description="Active/pressed state" colorKey="bgActive" />
      </Section>

      {/* ── Text / Fonts ── */}
      <Section title="Text & Fonts" icon="A">
        <ColorRow label="Primary Text" description="Main content text" colorKey="textPrimary" />
        <ColorRow label="Secondary Text" description="Labels, metadata" colorKey="textSecondary" />
        <ColorRow label="Muted Text" description="Disabled, hints" colorKey="textMuted" />
      </Section>

      {/* ── Borders ── */}
      <Section title="Borders" icon="□">
        <ColorRow label="Border" description="Panel and input borders" colorKey="borderColor" />
        <ColorRow label="Subtle Border" description="Very faint separators" colorKey="borderSubtle" />
      </Section>

      {/* ── Accents ── */}
      <Section title="Accent Colors" icon="◈">
        <ColorRow label="Blue" description="Links, focus rings" colorKey="accentBlue" />
        <ColorRow label="Green" description="Success, vine" colorKey="accentGreen" />
        <ColorRow label="Orange" description="Primary accent, bonfire" colorKey="accentOrange" />
        <ColorRow label="Purple" description="Special highlights" colorKey="accentPurple" />
        <ColorRow label="Yellow" description="Tarnished gold" colorKey="accentYellow" />
      </Section>

      {/* ── Log Levels ── */}
      <Section title="Log Level Colors" icon="▤">
        <ColorRow label="TRACE" description="Faintest level" colorKey="levelTrace" />
        <ColorRow label="DEBUG" description="Debug output" colorKey="levelDebug" />
        <ColorRow label="INFO" description="Informational" colorKey="levelInfo" />
        <ColorRow label="WARN" description="Warnings" colorKey="levelWarn" />
        <ColorRow label="ERROR" description="Errors" colorKey="levelError" />
      </Section>

      {/* ── Particle: Metal Sparks ── */}
      <Section title="Particles: Metal Sparks" icon="✦">
        <p class="theme-section-hint">
          Sparks spawn at the mouse cursor when hovering over elements.
        </p>
        <ColorRow label="Hot Core" description="White-yellow center" colorKey="particleSparkCore" />
        <ColorRow label="Ember" description="Outer ember glow" colorKey="particleSparkEmber" />
        <ColorRow label="Steel" description="Metallic highlight" colorKey="particleSparkSteel" />
      </Section>

      {/* ── Particle: Embers ── */}
      <Section title="Particles: Embers / Ash" icon="🔥">
        <p class="theme-section-hint">
          Rising embers/ash from hovered element borders.
        </p>
        <ColorRow label="Hot" description="Bright center glow" colorKey="particleEmberHot" />
        <ColorRow label="Base" description="Outer ember color" colorKey="particleEmberBase" />
      </Section>

      {/* ── Particle: Angelic Beams ── */}
      <Section title="Particles: Angelic Beams" icon="✧">
        <p class="theme-section-hint">
          Pixel-thin vertical rays rising from the selected element.
        </p>
        <ColorRow label="Center" description="Bright core color" colorKey="particleBeamCenter" />
        <ColorRow label="Edge" description="Warm outer glow" colorKey="particleBeamEdge" />
      </Section>

      {/* ── Particle: Glitter ── */}
      <Section title="Particles: Glitter" icon="✨">
        <p class="theme-section-hint">
          Twinkling sparkles drifting along hovered element borders.
        </p>
        <ColorRow label="Warm" description="Golden-white base" colorKey="particleGlitterWarm" />
        <ColorRow label="Cool" description="Blue-white variation" colorKey="particleGlitterCool" />
      </Section>

      {/* ── Cinder Palette ── */}
      <Section title="Cinder Palette" icon="◎">
        <p class="theme-section-hint">
          The four-color cycle used for border glows and hover effects.
        </p>
        <ColorRow label="Ember" description="Deep orange-red" colorKey="cinderEmber" />
        <ColorRow label="Gold" description="Tarnished gold" colorKey="cinderGold" />
        <ColorRow label="Ash" description="Cool grey" colorKey="cinderAsh" />
        <ColorRow label="Vine" description="Deep green" colorKey="cinderVine" />
      </Section>

      {/* ── Background Smoke ── */}
      <Section title="Background Smoke" icon="☁">
        <p class="theme-section-hint">
          Base tones and noise parameters for the animated smoky background layers.
        </p>
        <ColorRow label="Cool Tone" description="Blue-grey base" colorKey="smokeCool" />
        <ColorRow label="Warm Tone" description="Brown-amber base" colorKey="smokeWarm" />
        <ColorRow label="Moss Tone" description="Mossy mid-tone" colorKey="smokeMoss" />
        {[
          { key: 'smokeIntensity' as const, label: 'Intensity', desc: 'Overall smoke brightness/amount', max: 100 },
          { key: 'smokeSpeed' as const, label: 'Speed', desc: 'Smoke drift and animation speed (up to 5×)', max: 500 },
          { key: 'smokeWarmScale' as const, label: 'Warm Scale', desc: 'UV scale for warm base smoke layers', max: 200 },
          { key: 'smokeCoolScale' as const, label: 'Cool Scale', desc: 'UV scale for cool blue-tinted wisps', max: 200 },
          { key: 'smokeFineScale' as const, label: 'Fine Scale', desc: 'UV scale for fine fast wisps', max: 200 },
          { key: 'grainIntensity' as const, label: 'Grain Intensity', desc: 'Grain brightness / amplitude', max: 100 },
          { key: 'grainCoarseness' as const, label: 'Grain Coarseness', desc: 'Lower = finer detail, higher = chunkier', max: 100 },
          { key: 'grainSize' as const, label: 'Grain Size', desc: 'Pixel block size for grain pattern', max: 100 },
          { key: 'vignetteStrength' as const, label: 'Vignette', desc: 'Edge darkening intensity', max: 100 },
          { key: 'underglowStrength' as const, label: 'Underglow', desc: 'Warm glow from bottom edge', max: 100 },
        ].map(({ key, label, desc, max }) => (
          <div class="theme-slider-row" key={key}>
            <div class="theme-color-info">
              <span class="theme-color-label">{label}</span>
              <span class="theme-color-desc">{desc}</span>
            </div>
            <div class="theme-slider-controls">
              <input
                type="range"
                min="0"
                max={String(max)}
                step="1"
                value={effectSettings.value[key]}
                onInput={(e) => updateEffectSetting(key, parseInt((e.target as HTMLInputElement).value, 10))}
                class="theme-range-slider"
              />
              <span class="theme-slider-value">{effectSettings.value[key]}{max === 100 ? '%' : ''}</span>
            </div>
          </div>
        ))}
      </Section>

      {/* ── Cursor Style ── */}
      <Section title="Cursor" icon="⇱" defaultOpen={true}>
        <p class="theme-section-hint">
          Custom GPU-rendered cursor with shading and lighting effects.
        </p>
        <div class="theme-toggle-row">
          <div class="theme-color-info">
            <span class="theme-color-label">Cursor Style</span>
            <span class="theme-color-desc">Choose a GPU-rendered cursor or use the default</span>
          </div>
          <select
            class="theme-cursor-select"
            value={effectSettings.value.cursorStyle}
            onChange={(e) => updateEffectSetting('cursorStyle', (e.target as HTMLSelectElement).value as any)}
          >
            <option value="default">Default</option>
            <option value="metal">Metal</option>
            <option value="glass">Glass</option>
          </select>
        </div>
      </Section>

      {/* ── CRT Effect ── */}
      <Section title="CRT Effect" icon="▤" defaultOpen={true}>
        <p class="theme-section-hint">
          Retro CRT post-processing — scanlines, pixel grid, edge shadow, torch flicker.
        </p>
        <div class="theme-toggle-row">
          <div class="theme-color-info">
            <span class="theme-color-label">Enable CRT</span>
            <span class="theme-color-desc">Toggle the CRT overlay effect</span>
          </div>
          <label class="toggle-switch">
            <input
              type="checkbox"
              checked={effectSettings.value.crtEnabled}
              onChange={(e) => updateEffectSetting('crtEnabled', (e.target as HTMLInputElement).checked)}
            />
            <span class="toggle-slider"></span>
          </label>
        </div>
        {effectSettings.value.crtEnabled && [
          { key: 'crtScanlinesH' as const, label: 'H Scanlines', desc: 'Horizontal lines and grid rows' },
          { key: 'crtScanlinesV' as const, label: 'V Scanlines', desc: 'Vertical lines and grid columns' },
          { key: 'crtEdgeShadow' as const, label: 'Edge Shadow', desc: 'Border/vignette darkening' },
          { key: 'crtFlicker' as const, label: 'Flicker', desc: 'Torch-like brightness variation' },
        ].map(({ key, label, desc }) => (
          <div class="theme-slider-row" key={key}>
            <div class="theme-color-info">
              <span class="theme-color-label">{label}</span>
              <span class="theme-color-desc">{desc}</span>
            </div>
            <div class="theme-slider-controls">
              <input
                type="range"
                min="0"
                max="100"
                step="1"
                value={effectSettings.value[key]}
                onInput={(e) => updateEffectSetting(key, parseInt((e.target as HTMLInputElement).value, 10))}
                class="theme-range-slider"
              />
              <span class="theme-slider-value">{effectSettings.value[key]}%</span>
            </div>
          </div>
        ))}
      </Section>
    </div>
    <SavedThemesPanel />
    </div>
  );
}
