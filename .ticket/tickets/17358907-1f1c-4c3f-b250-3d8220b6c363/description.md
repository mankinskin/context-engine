# T9: Feature — Theme Save / Load / Export / Import

## Problem

T3 builds the theme editing UI with 49 color pickers and effect sliders, but custom themes are lost on refresh. Users need persistent storage (save/load), sharing (export/import as JSON files), and management (rename, delete, update). The TS viewer-api-frontend already implements this via localStorage + JSON blob download/upload. This ticket ports that to Leptos and integrates with the ThemeStore factory from viewer-api-leptos (T6).

## Reference: TS Implementation

### Theme Store (viewer-api/frontend/src/store/theme.ts)
- **L512–525**: `SavedTheme` interface: `{ id: string, name: string, colors: ThemeColors, effects: EffectSettings, thumbnail?: string, createdAt: string }`
- **L538–560**: `ThemeSettingsStore` interface: all signals + methods (saveTheme, deleteTheme, applySavedTheme, updateSavedTheme, renameSavedTheme, exportTheme, importTheme)
- **L240–290**: `createThemeStore()` factory — manages localStorage persist, CSS variable injection, global signal syncing

### Ticket Viewer Theme (ticket-viewer/frontend/src/theme.ts)
- **L142–155**: `savedThemes` signal + localStorage persistence (key: `'ticket-viewer-saved-themes'`)
- **L157–180**: `saveTheme(name, thumbnail?)`:
  ```ts
  const theme: SavedTheme = {
    id: crypto.randomUUID(),
    name,
    colors: { ...colors.value },
    effects: { ...effects.value },
    thumbnail,
    createdAt: new Date().toISOString(),
  };
  savedThemes.value = [theme, ...savedThemes.value];
  ```
- **L182–186**: `deleteTheme(id)` — filter + persist
- **L188–191**: `applySavedTheme(t)` — sets colors + effects signals
- **L193–198**: `updateSavedTheme(id, thumbnail?)` — overwrites colors/effects for existing theme
- **L200–205**: `renameSavedTheme(id, newName)` — updates name field
- **L207–215**: `exportTheme(name?)`:
  ```ts
  const data = { version: 1, name, colors: colors.value, effects: effects.value };
  const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  // Trigger download via temporary <a> element
  ```
- **L217–230**: `importTheme(file: File)`:
  ```ts
  const text = await file.text();
  const data = JSON.parse(text);
  colors.value = { ...DEFAULT_COLORS, ...data.colors };
  effects.value = { ...DEFAULT_EFFECTS, ...data.effects };
  ```

### ThemeSettings UI (viewer-api/frontend/src/components/ThemeSettings/ThemeSettings.tsx)
- **L30–70**: `SaveThemeButton()` — inline text input for name, calls `store.saveTheme(name)`
- **L72–135**: `SavedThemeCard()`:
  - Thumbnail or color swatch preview (4-color: bgPrimary, accentBlue, accentGreen, accentRed)
  - Name display (double-click → inline rename input)
  - Date display
  - Apply / Update / Delete action buttons
  - Confirmation dialog for destructive actions (update overwrites, delete removes)
- **L137–160**: `SavedThemesPanel()` — lists all saved themes or empty state message
- **L162–190**: `ImportThemeButton()` — hidden file input, button triggers click, parses .json
- **L195–205**: Action bar: Reset, Randomize, Save, Export, Import buttons

## Design

### Step 1: SavedTheme type

```rust
// In viewer-api-leptos/src/theme/types.rs (shared crate)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedTheme {
    pub id: String,
    pub name: String,
    pub colors: ThemeColors,
    pub effects: EffectSettings,
    pub thumbnail: Option<String>,  // data:image/png;base64,... from canvas capture
    pub created_at: String,         // ISO 8601
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeExport {
    pub version: u32,  // Always 1
    pub name: String,
    pub colors: ThemeColors,
    pub effects: EffectSettings,
}
```

### Step 2: ThemeStore save/load methods

Add to the ThemeStore from T6:

```rust
// In viewer-api-leptos/src/theme/store.rs

impl ThemeStore {
    // --- Persistence ---
    
    pub fn load_saved_themes(&self) {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        let key = format!("{}-saved-themes", self.config.storage_key);
        if let Ok(Some(json)) = storage.get_item(&key) {
            if let Ok(themes) = serde_json::from_str::<Vec<SavedTheme>>(&json) {
                self.saved_themes.set(themes);
            }
        }
    }
    
    fn persist_saved_themes(&self) {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        let key = format!("{}-saved-themes", self.config.storage_key);
        let json = serde_json::to_string(&self.saved_themes.get_untracked()).unwrap();
        let _ = storage.set_item(&key, &json);
    }
    
    // --- Save ---
    
    pub fn save_theme(&self, name: &str, thumbnail: Option<String>) {
        let theme = SavedTheme {
            id: uuid_v4(),  // web_sys crypto.randomUUID()
            name: name.to_string(),
            colors: self.colors.get_untracked(),
            effects: self.effects.get_untracked(),
            thumbnail,
            created_at: js_sys::Date::new_0().to_iso_string().into(),
        };
        self.saved_themes.update(|themes| themes.insert(0, theme));
        self.persist_saved_themes();
    }
    
    // --- Delete ---
    
    pub fn delete_theme(&self, id: &str) {
        self.saved_themes.update(|themes| themes.retain(|t| t.id != id));
        self.persist_saved_themes();
    }
    
    // --- Apply ---
    
    pub fn apply_saved_theme(&self, theme: &SavedTheme) {
        self.colors.set(theme.colors.clone());
        self.effects.set(theme.effects.clone());
    }
    
    // --- Update (overwrite saved with current) ---
    
    pub fn update_saved_theme(&self, id: &str, thumbnail: Option<String>) {
        self.saved_themes.update(|themes| {
            if let Some(t) = themes.iter_mut().find(|t| t.id == id) {
                t.colors = self.colors.get_untracked();
                t.effects = self.effects.get_untracked();
                if let Some(thumb) = thumbnail {
                    t.thumbnail = Some(thumb);
                }
            }
        });
        self.persist_saved_themes();
    }
    
    // --- Rename ---
    
    pub fn rename_saved_theme(&self, id: &str, new_name: &str) {
        self.saved_themes.update(|themes| {
            if let Some(t) = themes.iter_mut().find(|t| t.id == id) {
                t.name = new_name.to_string();
            }
        });
        self.persist_saved_themes();
    }
    
    // --- Export ---
    
    pub fn export_theme(&self, name: Option<&str>) {
        let export = ThemeExport {
            version: 1,
            name: name.unwrap_or("Custom Theme").to_string(),
            colors: self.colors.get_untracked(),
            effects: self.effects.get_untracked(),
        };
        let json = serde_json::to_string_pretty(&export).unwrap();
        
        // Create blob + trigger download
        let array = js_sys::Array::new();
        array.push(&wasm_bindgen::JsValue::from_str(&json));
        let blob = web_sys::Blob::new_with_str_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new().type_("application/json"),
        ).unwrap();
        
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let a = document.create_element("a").unwrap()
            .dyn_into::<web_sys::HtmlElement>().unwrap();
        a.set_attribute("href", &url).unwrap();
        let filename = format!("{}.json", name.unwrap_or("theme"));
        a.set_attribute("download", &filename).unwrap();
        a.click();
        web_sys::Url::revoke_object_url(&url).unwrap();
    }
    
    // --- Import ---
    
    pub fn import_theme(&self, json_text: &str) -> Result<(), String> {
        let export: ThemeExport = serde_json::from_str(json_text)
            .map_err(|e| format!("Invalid theme file: {}", e))?;
        
        if export.version != 1 {
            return Err(format!("Unsupported version: {}", export.version));
        }
        
        // Merge with defaults to handle missing fields from older exports
        let mut colors = ThemeColors::default();
        // ... merge export.colors fields
        colors = export.colors;  // Direct assignment if struct is fully populated
        
        self.colors.set(colors);
        self.effects.set(export.effects);
        Ok(())
    }
}

// Helper: generate UUID v4 via Web Crypto API
fn uuid_v4() -> String {
    let crypto = web_sys::window().unwrap().crypto().unwrap();
    js_sys::Reflect::get(&crypto, &"randomUUID".into())
        .ok()
        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
        .map(|f| f.call0(&crypto).unwrap().as_string().unwrap())
        .unwrap_or_else(|| {
            // Fallback: generate from getRandomValues
            let arr = js_sys::Uint8Array::new_with_length(16);
            crypto.get_random_values_with_u8_array(&arr).unwrap();
            format!("{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
                /* ... format bytes as UUID v4 ... */
                0, 0, 0, 0, 0) // simplified
        })
}
```

### Step 3: SaveThemeButton component

```rust
// In viewer-api-leptos/src/theme/settings.rs (add to ThemeSettings)

#[component]
fn SaveThemeButton(store: ThemeStore) -> impl IntoView {
    let is_saving = create_rw_signal(false);
    let name_input = create_rw_signal(String::new());
    
    let on_save = move |_| {
        let name = name_input.get();
        if name.trim().is_empty() { return; }
        // Optional: capture thumbnail from GPU canvas
        store.save_theme(name.trim(), None);
        name_input.set(String::new());
        is_saving.set(false);
    };
    
    view! {
        <Show when=move || is_saving.get()
              fallback=move || view! {
                  <button class="action-btn save-btn"
                          on:click=move |_| is_saving.set(true)>
                      "Save Theme"
                  </button>
              }>
            <div class="save-input-row">
                <input type="text"
                       placeholder="Theme name..."
                       prop:value=name_input
                       on:input=move |e| name_input.set(event_target_value(&e))
                       on:keydown=move |e| {
                           if e.key() == "Enter" { on_save(e); }
                           if e.key() == "Escape" { is_saving.set(false); }
                       }
                />
                <button class="confirm-btn" on:click=on_save>"Save"</button>
                <button class="cancel-btn" on:click=move |_| is_saving.set(false)>"×"</button>
            </div>
        </Show>
    }
}
```

### Step 4: SavedThemeCard component

```rust
#[component]
fn SavedThemeCard(
    theme: SavedTheme,
    store: ThemeStore,
) -> impl IntoView {
    let is_renaming = create_rw_signal(false);
    let rename_input = create_rw_signal(theme.name.clone());
    let confirm_delete = create_rw_signal(false);
    let confirm_update = create_rw_signal(false);
    let id = theme.id.clone();
    
    // Color swatch preview (4 colors)
    let swatches = [
        &theme.colors.bg_primary,
        &theme.colors.accent_blue,
        &theme.colors.accent_green,
        &theme.colors.accent_red,
    ];
    
    view! {
        <div class="saved-theme-card">
            // Thumbnail or swatches
            <div class="card-preview">
                {theme.thumbnail.as_ref().map(|t| {
                    view! { <img src=t alt="Theme preview" /> }
                }).unwrap_or_else(|| {
                    view! {
                        <div class="color-swatches">
                            {swatches.iter().map(|c| {
                                view! { <div class="swatch" style=format!("background:{}", c) /> }
                            }).collect_view()}
                        </div>
                    }
                })}
            </div>
            
            // Name (double-click to rename)
            <Show when=move || is_renaming.get()
                  fallback=move || view! {
                      <span class="card-name" on:dblclick=move |_| is_renaming.set(true)>
                          {&theme.name}
                      </span>
                  }>
                <input type="text"
                       class="rename-input"
                       prop:value=rename_input
                       on:input=move |e| rename_input.set(event_target_value(&e))
                       on:keydown=move |e| {
                           if e.key() == "Enter" {
                               store.rename_saved_theme(&id, &rename_input.get());
                               is_renaming.set(false);
                           }
                           if e.key() == "Escape" { is_renaming.set(false); }
                       }
                       on:blur=move |_| is_renaming.set(false)
                />
            </Show>
            
            // Date
            <span class="card-date">{format_date(&theme.created_at)}</span>
            
            // Actions
            <div class="card-actions">
                <button class="apply-btn" on:click=move |_| store.apply_saved_theme(&theme)>
                    "Apply"
                </button>
                
                // Update with confirmation
                <Show when=move || confirm_update.get()
                      fallback=move || view! {
                          <button class="update-btn" on:click=move |_| confirm_update.set(true)>
                              "Update"
                          </button>
                      }>
                    <span class="confirm-msg">"Overwrite?"</span>
                    <button on:click=move |_| {
                        store.update_saved_theme(&id, None);
                        confirm_update.set(false);
                    }>"Yes"</button>
                    <button on:click=move |_| confirm_update.set(false)>"No"</button>
                </Show>
                
                // Delete with confirmation
                <Show when=move || confirm_delete.get()
                      fallback=move || view! {
                          <button class="delete-btn" on:click=move |_| confirm_delete.set(true)>
                              "×"
                          </button>
                      }>
                    <span class="confirm-msg">"Delete?"</span>
                    <button on:click=move |_| {
                        store.delete_theme(&id);
                        confirm_delete.set(false);
                    }>"Yes"</button>
                    <button on:click=move |_| confirm_delete.set(false)>"No"</button>
                </Show>
            </div>
        </div>
    }
}
```

### Step 5: ImportThemeButton

```rust
#[component]
fn ImportThemeButton(store: ThemeStore) -> impl IntoView {
    let file_input_ref = create_node_ref::<html::Input>();
    let import_error = create_rw_signal::<Option<String>>(None);
    
    let on_file_selected = move |_| {
        let input = file_input_ref.get().unwrap();
        let files = input.files().unwrap();
        if let Some(file) = files.get(0) {
            let store = store.clone();
            let error_sig = import_error;
            spawn_local(async move {
                let text = wasm_bindgen_futures::JsFuture::from(file.text())
                    .await
                    .map(|v| v.as_string().unwrap_or_default())
                    .unwrap_or_default();
                
                match store.import_theme(&text) {
                    Ok(()) => error_sig.set(None),
                    Err(e) => error_sig.set(Some(e)),
                }
            });
        }
        // Reset input so same file can be re-imported
        input.set_value("");
    };
    
    view! {
        <div class="import-btn-wrapper">
            <input type="file"
                   accept=".json"
                   style="display:none"
                   node_ref=file_input_ref
                   on:change=on_file_selected
            />
            <button class="action-btn import-btn"
                    on:click=move |_| file_input_ref.get().unwrap().click()>
                "Import"
            </button>
            {move || import_error.get().map(|e| view! {
                <span class="import-error">{e}</span>
            })}
        </div>
    }
}
```

### Step 6: SavedThemesPanel + action bar integration

```rust
#[component]
fn SavedThemesPanel(store: ThemeStore) -> impl IntoView {
    view! {
        <div class="saved-themes-panel">
            <Show when=move || !store.saved_themes.get().is_empty()
                  fallback=|| view! {
                      <p class="empty-state">"No saved themes yet. Create one above!"</p>
                  }>
                <div class="saved-themes-grid">
                    <For each=move || store.saved_themes.get()
                         key=|t| t.id.clone()
                         let:theme>
                        <SavedThemeCard theme=theme store=store.clone() />
                    </For>
                </div>
            </Show>
        </div>
    }
}
```

Add to ThemeSettings action bar (from T3):
```rust
// In the ThemeSettings overlay header area:
<div class="theme-actions">
    <button class="action-btn" on:click=move |_| store.apply_preset("default")>"Reset"</button>
    <SaveThemeButton store=store.clone() />
    <button class="action-btn" on:click=move |_| store.export_theme(None)>"Export"</button>
    <ImportThemeButton store=store.clone() />
</div>

// Below presets grid:
<SavedThemesPanel store=store.clone() />
```

### Step 7: CSS

```css
/* Save input */
.save-input-row { display: flex; gap: 4px; align-items: center; }
.save-input-row input { flex: 1; padding: 4px 8px; font-size: 13px; background: var(--bg-tertiary); border: 1px solid var(--border-color); border-radius: 4px; color: var(--text-primary); }

/* Saved themes grid */
.saved-themes-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 8px; padding: 8px 0; }

/* Theme card */
.saved-theme-card { display: flex; flex-direction: column; gap: 4px; padding: 8px; border-radius: 6px; background: var(--bg-secondary); border: 1px solid var(--border-color); }
.saved-theme-card:hover { border-color: var(--accent-blue); }

/* Preview */
.card-preview { height: 48px; border-radius: 4px; overflow: hidden; }
.card-preview img { width: 100%; height: 100%; object-fit: cover; }
.color-swatches { display: grid; grid-template-columns: repeat(4, 1fr); height: 100%; }
.swatch { width: 100%; height: 100%; }

/* Name + date */
.card-name { font-size: 13px; font-weight: 500; cursor: default; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.card-date { font-size: 11px; color: var(--text-muted); }
.rename-input { width: 100%; font-size: 13px; padding: 2px 4px; }

/* Actions */
.card-actions { display: flex; gap: 4px; margin-top: auto; }
.card-actions button { font-size: 11px; padding: 2px 8px; border-radius: 3px; cursor: pointer; }
.apply-btn { background: var(--accent-blue); color: white; }
.update-btn { background: var(--bg-tertiary); color: var(--text-primary); }
.delete-btn { background: none; color: var(--text-muted); }
.confirm-msg { font-size: 11px; color: var(--text-warning, #f0a500); }

/* Import */
.import-error { font-size: 11px; color: var(--text-error, #e06c75); margin-left: 4px; }
.empty-state { color: var(--text-muted); font-style: italic; padding: 16px; text-align: center; }
```

## Files to Create

| File | Purpose |
|------|---------|
| (none — all code lives in viewer-api-leptos shared crate modules) | |

## Files to Modify

| File | Change |
|------|--------|
| `viewer-api-leptos/src/theme/types.rs` | Add SavedTheme, ThemeExport structs |
| `viewer-api-leptos/src/theme/store.rs` | Add save/delete/apply/update/rename/export/import methods + localStorage persistence |
| `viewer-api-leptos/src/theme/settings.rs` | Add SaveThemeButton, SavedThemeCard, SavedThemesPanel, ImportThemeButton components |
| `viewer-api-leptos/src/theme/settings.css` (or style.css) | Add saved theme card/grid/action CSS |

## Acceptance Criteria

1. Save: name input → creates SavedTheme in localStorage (prepended to list)
2. Load: saved themes persist across page refresh via localStorage
3. Apply: click "Apply" on card → restores colors + effects
4. Update: click "Update" on card → overwrites saved colors/effects with current (with confirmation)
5. Delete: click delete on card → removes (with confirmation)
6. Rename: double-click name → inline input → Enter commits, Escape cancels
7. Export: generates JSON file download (`theme.json`) with version, name, colors, effects
8. Import: file picker accepts `.json`, parses ThemeExport, applies colors + effects
9. Import validation: rejects malformed JSON with user-visible error message
10. Version field in export — currently 1, checked on import
11. Color swatch preview (4 colors) when no thumbnail available
12. Works across all viewers (log-viewer, doc-viewer, ticket-viewer) via shared ThemeStore
13. Storage key is per-viewer (e.g. `log-viewer-saved-themes`, `ticket-viewer-saved-themes`)
