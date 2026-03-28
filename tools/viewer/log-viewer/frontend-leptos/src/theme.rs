/// Theme and effect settings — port of the TS `EffectSettings` system.
///
/// Provides GPU overlay effect parameters (smoke, particles, CRT, glass) and
/// a set of built-in presets matching the TypeScript frontend's themes.

use std::cell::RefCell;

// ── Effect Settings ───────────────────────────────────────────────────────────

/// All numeric fields are **0–100 UI percentages** — they are divided by 100
/// when packed into the GPU uniform buffer (matching the TS convention).
#[derive(Debug, Clone)]
pub struct EffectSettings {
    // Glass panel
    pub glass_opacity: f32,
    pub glass_blur: f32,

    // CRT
    pub crt_enabled: bool,
    pub crt_scanlines_h: f32,
    pub crt_scanlines_v: f32,
    pub crt_edge_shadow: f32,
    pub crt_flicker: f32,
    pub crt_line_width: f32,
    pub crt_color: [f32; 3], // 0–255 per channel

    // Background smoke
    pub smoke_enabled: bool,
    pub smoke_intensity: f32,
    pub smoke_speed: f32,
    pub smoke_warm_scale: f32,
    pub smoke_cool_scale: f32,
    pub smoke_moss_scale: f32,

    // Film grain + vignette
    pub grain_intensity: f32,
    pub grain_coarseness: f32,
    pub grain_size: f32,
    pub vignette_strength: f32,
    pub underglow_strength: f32,

    // Particle: sparks (metal grinding)
    pub sparks_enabled: bool,
    pub spark_speed: f32,
    pub spark_count: f32,
    pub spark_size: f32,

    // Particle: embers (rising ash)
    pub embers_enabled: bool,
    pub ember_speed: f32,
    pub ember_count: f32,
    pub ember_size: f32,

    // Particle: beams (angelic rays)
    pub beams_enabled: bool,
    pub beam_speed: f32,
    pub beam_height: f32,
    pub beam_drift: f32,
    pub beam_count: f32,

    // Particle: glitter (twinkling sparkles)
    pub glitter_enabled: bool,
    pub glitter_speed: f32,
    pub glitter_count: f32,
    pub glitter_size: f32,

    // Cinder
    pub cinder_enabled: bool,
    pub cinder_size: f32,
}

impl Default for EffectSettings {
    fn default() -> Self {
        Self {
            glass_opacity: 35.0,
            glass_blur: 25.0,

            crt_enabled: true,
            crt_scanlines_h: 20.0,
            crt_scanlines_v: 12.0,
            crt_edge_shadow: 35.0,
            crt_flicker: 12.0,
            crt_line_width: 50.0,
            crt_color: [100.0, 80.0, 60.0],

            smoke_enabled: true,
            smoke_intensity: 40.0,
            smoke_speed: 50.0,
            smoke_warm_scale: 100.0,
            smoke_cool_scale: 100.0,
            smoke_moss_scale: 100.0,

            grain_intensity: 20.0,
            grain_coarseness: 40.0,
            grain_size: 35.0,
            vignette_strength: 40.0,
            underglow_strength: 25.0,

            sparks_enabled: true,
            spark_speed: 70.0,
            spark_count: 40.0,
            spark_size: 70.0,

            embers_enabled: true,
            ember_speed: 70.0,
            ember_count: 40.0,
            ember_size: 70.0,

            beams_enabled: true,
            beam_speed: 50.0,
            beam_height: 35.0,
            beam_drift: 80.0,
            beam_count: 48.0,

            glitter_enabled: true,
            glitter_speed: 60.0,
            glitter_count: 40.0,
            glitter_size: 60.0,

            cinder_enabled: true,
            cinder_size: 70.0,
        }
    }
}

// ── Palette colors ────────────────────────────────────────────────────────────

/// 24 × vec4f = 96 floats = 384 bytes (matches `ThemePalette` in palette.wgsl).
pub type PaletteData = [f32; 96];

/// Default palette colors (Cinder theme) — matches the TS `CINDER` preset.
pub fn default_palette() -> PaletteData {
    let mut p = [0.0f32; 96];
    let slots: &[[f32; 4]] = &[
        [1.0, 0.95, 0.70, 1.0], // [0]  spark_core
        [1.0, 0.50, 0.10, 0.8], // [1]  spark_ember
        [0.8, 0.85, 0.90, 0.6], // [2]  spark_steel
        [1.0, 0.60, 0.10, 0.9], // [3]  ember_hot
        [0.9, 0.80, 0.40, 0.9], // [4]  beam_center
        [0.7, 0.40, 0.10, 0.4], // [5]  beam_edge
        [1.0, 0.90, 0.50, 0.8], // [6]  glitter_warm
        [0.5, 0.80, 1.00, 0.8], // [7]  glitter_cool
        [1.0, 0.50, 0.10, 0.9], // [8]  cinder_ember
        [1.0, 0.80, 0.20, 0.9], // [9]  cinder_gold
        [0.7, 0.65, 0.60, 0.7], // [10] cinder_ash
        [0.3, 0.50, 0.20, 0.7], // [11] cinder_vine
        [0.15, 0.18, 0.25, 1.0], // [12] smoke_cool
        [0.25, 0.20, 0.15, 1.0], // [13] smoke_warm
        [0.12, 0.18, 0.12, 1.0], // [14] smoke_moss
        [0.3, 0.40, 0.55, 1.0], // [15] kind_structural
        [0.9, 0.20, 0.20, 1.0], // [16] kind_error
        [0.9, 0.70, 0.10, 1.0], // [17] kind_warn
        [0.2, 0.70, 0.90, 1.0], // [18] kind_info
        [0.4, 0.70, 0.40, 1.0], // [19] kind_debug
        [0.2, 0.80, 0.50, 1.0], // [20] kind_span
        [0.95, 0.55, 0.15, 1.0], // [21] kind_selected
        [0.95, 0.15, 0.15, 1.0], // [22] kind_panic
        [0.0, 0.0, 0.0, 0.0],  // [23] _pad
    ];
    for (i, s) in slots.iter().enumerate() {
        p[i * 4..i * 4 + 4].copy_from_slice(s);
    }
    p
}

// ── Theme presets ─────────────────────────────────────────────────────────────

pub struct ThemePreset {
    pub name: &'static str,
    pub effects: EffectSettings,
    pub palette: PaletteData,
}

/// Built-in "Cinder" preset — Dark Souls aesthetic, heavy embers/sparks/smoke.
pub fn preset_cinder() -> ThemePreset {
    let mut e = EffectSettings::default();
    e.sparks_enabled = true;  e.spark_count = 50.0; e.spark_size = 80.0; e.spark_speed = 80.0;
    e.embers_enabled = true;  e.ember_count = 50.0; e.ember_size = 80.0; e.ember_speed = 65.0;
    e.beams_enabled = false;
    e.glitter_enabled = false;
    e.cinder_enabled = true; e.cinder_size = 80.0;
    e.smoke_enabled = true; e.smoke_intensity = 45.0; e.smoke_speed = 45.0;
    e.crt_enabled = true; e.crt_scanlines_h = 20.0; e.crt_scanlines_v = 10.0; e.crt_edge_shadow = 35.0;
    e.grain_intensity = 25.0; e.vignette_strength = 50.0; e.underglow_strength = 35.0;
    ThemePreset { name: "Cinder", effects: e, palette: default_palette() }
}

/// "Frost" — icy cool tones, aurora beams, snowfall glitter.
pub fn preset_frost() -> ThemePreset {
    let mut e = EffectSettings::default();
    e.sparks_enabled = false;
    e.embers_enabled = false;
    e.beams_enabled = true; e.beam_count = 24.0; e.beam_height = 40.0; e.beam_speed = 30.0; e.beam_drift = 60.0;
    e.glitter_enabled = true; e.glitter_count = 55.0; e.glitter_size = 55.0; e.glitter_speed = 35.0;
    e.cinder_enabled = true; e.cinder_size = 60.0;
    e.smoke_enabled = true; e.smoke_intensity = 30.0; e.smoke_speed = 35.0;
    e.crt_enabled = false;
    e.grain_intensity = 15.0; e.vignette_strength = 30.0; e.underglow_strength = 20.0;
    let mut pal = default_palette();
    // Shift smoke tones cooler
    pal[12 * 4..12 * 4 + 4].copy_from_slice(&[0.15, 0.22, 0.35, 1.0]); // smoke_cool → icy blue
    pal[13 * 4..13 * 4 + 4].copy_from_slice(&[0.18, 0.20, 0.28, 1.0]); // smoke_warm → steel
    pal[14 * 4..14 * 4 + 4].copy_from_slice(&[0.12, 0.18, 0.22, 1.0]); // smoke_moss → dark teal
    // Glitter: cool tones
    pal[6 * 4..6 * 4 + 4].copy_from_slice(&[0.7, 0.85, 1.0, 0.9]); // glitter_warm → ice white
    pal[7 * 4..7 * 4 + 4].copy_from_slice(&[0.4, 0.65, 1.0, 0.8]); // glitter_cool → sky blue
    ThemePreset { name: "Frost", effects: e, palette: pal }
}

/// "Elysium" — dominant angelic beams, gentle backdrop.
pub fn preset_elysium() -> ThemePreset {
    let mut e = EffectSettings::default();
    e.sparks_enabled = false;
    e.embers_enabled = false;
    e.beams_enabled = true; e.beam_count = 48.0; e.beam_height = 50.0; e.beam_speed = 35.0; e.beam_drift = 120.0;
    e.glitter_enabled = true; e.glitter_count = 30.0; e.glitter_size = 45.0; e.glitter_speed = 25.0;
    e.cinder_enabled = false;
    e.smoke_enabled = true; e.smoke_intensity = 25.0; e.smoke_speed = 20.0;
    e.crt_enabled = false;
    e.grain_intensity = 10.0; e.vignette_strength = 20.0; e.underglow_strength = 15.0;
    // Shift palette: golden beams
    let mut pal = default_palette();
    pal[4 * 4..4 * 4 + 4].copy_from_slice(&[1.0, 0.92, 0.65, 1.0]); // beam_center → bright gold
    pal[5 * 4..5 * 4 + 4].copy_from_slice(&[0.85, 0.65, 0.25, 0.5]); // beam_edge → warm amber
    ThemePreset { name: "Elysium", effects: e, palette: pal }
}

/// "Void" — starlight beams, deep cosmic tones.
pub fn preset_void() -> ThemePreset {
    let mut e = EffectSettings::default();
    e.sparks_enabled = false;
    e.embers_enabled = false;
    e.beams_enabled = true; e.beam_count = 20.0; e.beam_height = 50.0; e.beam_speed = 22.0; e.beam_drift = 120.0;
    e.glitter_enabled = true; e.glitter_count = 45.0; e.glitter_size = 45.0; e.glitter_speed = 25.0;
    e.cinder_enabled = true; e.cinder_size = 55.0;
    e.smoke_enabled = true; e.smoke_intensity = 35.0; e.smoke_speed = 25.0;
    e.crt_enabled = false;
    e.grain_intensity = 15.0; e.vignette_strength = 45.0; e.underglow_strength = 10.0;
    let mut pal = default_palette();
    // Shift smoke dark/deep
    pal[12 * 4..12 * 4 + 4].copy_from_slice(&[0.08, 0.06, 0.18, 1.0]); // smoke_cool → deep indigo
    pal[13 * 4..13 * 4 + 4].copy_from_slice(&[0.10, 0.04, 0.14, 1.0]); // smoke_warm → purple-black
    pal[14 * 4..14 * 4 + 4].copy_from_slice(&[0.05, 0.08, 0.12, 1.0]); // smoke_moss → dark navy
    // Glitter: starlight
    pal[6 * 4..6 * 4 + 4].copy_from_slice(&[0.9, 0.92, 1.0, 0.9]); // glitter_warm → white
    pal[7 * 4..7 * 4 + 4].copy_from_slice(&[0.5, 0.55, 1.0, 0.8]); // glitter_cool → pale blue
    ThemePreset { name: "Void", effects: e, palette: pal }
}

/// "Blood Moon" — crimson embers, deep red vignette.
pub fn preset_blood_moon() -> ThemePreset {
    let mut e = EffectSettings::default();
    e.sparks_enabled = true;  e.spark_count = 30.0; e.spark_size = 85.0; e.spark_speed = 55.0;
    e.embers_enabled = true;  e.ember_count = 55.0; e.ember_size = 85.0; e.ember_speed = 50.0;
    e.beams_enabled = false;
    e.glitter_enabled = false;
    e.cinder_enabled = true; e.cinder_size = 85.0;
    e.smoke_enabled = true; e.smoke_intensity = 50.0; e.smoke_speed = 35.0;
    e.crt_enabled = true; e.crt_scanlines_h = 15.0; e.crt_scanlines_v = 8.0; e.crt_edge_shadow = 50.0;
    e.crt_color = [140.0, 30.0, 20.0];
    e.grain_intensity = 30.0; e.vignette_strength = 60.0; e.underglow_strength = 20.0;
    let mut pal = default_palette();
    // Smoke red-tinted
    pal[12 * 4..12 * 4 + 4].copy_from_slice(&[0.20, 0.08, 0.10, 1.0]); // smoke_cool → dark crimson
    pal[13 * 4..13 * 4 + 4].copy_from_slice(&[0.25, 0.10, 0.08, 1.0]); // smoke_warm → blood red
    pal[14 * 4..14 * 4 + 4].copy_from_slice(&[0.15, 0.06, 0.06, 1.0]); // smoke_moss → burgundy
    ThemePreset { name: "Blood Moon", effects: e, palette: pal }
}

/// Returns all built-in presets.
pub fn all_presets() -> Vec<ThemePreset> {
    vec![
        preset_cinder(),
        preset_frost(),
        preset_elysium(),
        preset_void(),
        preset_blood_moon(),
    ]
}

// ── Thread-local effect settings for GPU readback ─────────────────────────────

thread_local! {
    static EFFECT_SETTINGS: RefCell<EffectSettings> = RefCell::new(EffectSettings::default());
    static PALETTE_DATA: RefCell<PaletteData> = RefCell::new(default_palette());
}

/// Set the active effect settings (called whenever the theme changes).
pub fn set_effect_settings(s: &EffectSettings) {
    EFFECT_SETTINGS.with(|cell| *cell.borrow_mut() = s.clone());
}

/// Set the active palette data (called whenever the theme changes).
pub fn set_palette_data(p: &PaletteData) {
    PALETTE_DATA.with(|cell| *cell.borrow_mut() = *p);
}

/// Read effect settings for uniform packing (called every frame by overlay).
pub fn with_effect_settings<R>(f: impl FnOnce(&EffectSettings) -> R) -> R {
    EFFECT_SETTINGS.with(|cell| f(&cell.borrow()))
}

/// Read palette data for GPU upload.
pub fn with_palette_data<R>(f: impl FnOnce(&PaletteData) -> R) -> R {
    PALETTE_DATA.with(|cell| f(&cell.borrow()))
}
