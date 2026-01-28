/**
 * Appearance Settings
 *
 * Theme selector with visual radio cards.
 */

import { Component, For } from "solid-js";
import { Check } from "lucide-solid";
import { themeState, setTheme, type ThemeDefinition } from "@/stores/theme";

const AppearanceSettings: Component = () => {
  return (
    <div>
      <h3 class="text-lg font-semibold mb-4 text-text-primary">Theme</h3>
      <p class="text-sm text-text-secondary mb-6">
        Choose your preferred color scheme
      </p>

      <div class="space-y-3">
        <For each={themeState.availableThemes}>
          {(theme) => (
            <button
              onClick={() => setTheme(theme.id)}
              class="w-full text-left p-4 rounded-xl border-2 transition-all duration-200"
              classList={{
                "border-accent-primary bg-accent-primary/10":
                  themeState.currentTheme === theme.id,
                "border-white/10 hover:border-accent-primary/50 hover:bg-white/5":
                  themeState.currentTheme !== theme.id,
              }}
            >
              <div class="flex items-start gap-3">
                {/* Radio indicator */}
                <div
                  class="w-5 h-5 rounded-full border-2 flex items-center justify-center flex-shrink-0 mt-0.5 transition-colors"
                  classList={{
                    "border-accent-primary bg-accent-primary":
                      themeState.currentTheme === theme.id,
                    "border-white/30": themeState.currentTheme !== theme.id,
                  }}
                >
                  {themeState.currentTheme === theme.id && (
                    <Check class="w-3 h-3 text-surface-base" />
                  )}
                </div>

                {/* Theme info */}
                <div class="flex-1">
                  <div class="flex items-center gap-2">
                    <span class="font-semibold text-text-primary">
                      {theme.name}
                    </span>
                    <span
                      class="text-xs px-1.5 py-0.5 rounded"
                      classList={{
                        "bg-surface-highlight text-text-secondary": theme.isDark,
                        "bg-amber-100 text-amber-800": !theme.isDark,
                      }}
                    >
                      {theme.isDark ? "Dark" : "Light"}
                    </span>
                  </div>
                  <div class="text-sm text-text-secondary mt-0.5">
                    {theme.description}
                  </div>
                </div>

                {/* Color preview dots */}
                <div class="flex gap-1">
                  <PreviewDot theme={theme} type="surface" />
                  <PreviewDot theme={theme} type="accent" />
                  <PreviewDot theme={theme} type="text" />
                </div>
              </div>
            </button>
          )}
        </For>
      </div>
    </div>
  );
};

// Color preview dot component
const PreviewDot: Component<{
  theme: ThemeDefinition;
  type: "surface" | "accent" | "text";
}> = (props) => {
  return (
    <div
      class="w-4 h-4 rounded-full border border-white/20"
      style={{ "background-color": props.theme.preview[props.type] }}
    />
  );
};

export default AppearanceSettings;
