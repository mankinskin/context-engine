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
import { useState } from 'preact/hooks';
import {
  themeColors,
  updateThemeColor,
  applyPreset,
  resetTheme,
  THEME_PRESETS,
  DEFAULT_THEME,
  type ThemeColors,
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

// ── Main component ──────────────────────────────────────────────────────────

export function ThemeSettings() {
  return (
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
          Base tones for the animated smoky background layers.
        </p>
        <ColorRow label="Cool Tone" description="Blue-grey base" colorKey="smokeCool" />
        <ColorRow label="Warm Tone" description="Brown-amber base" colorKey="smokeWarm" />
        <ColorRow label="Moss Tone" description="Mossy mid-tone" colorKey="smokeMoss" />
      </Section>
    </div>
  );
}
