/**
 * Context Menu System
 *
 * Provides a global right-click context menu that can be triggered
 * from anywhere in the app via showContextMenu().
 * Follows the same global signal + Portal pattern as Toast.tsx.
 */

import { Component, For, Show, createSignal, onMount, onCleanup } from "solid-js";
import { Dynamic, Portal } from "solid-js/web";

// --- Types ---

export interface ContextMenuItem {
  label: string;
  icon?: Component<{ class?: string }>;
  action: () => void;
  danger?: boolean;
  disabled?: boolean;
}

export interface ContextMenuSeparator {
  separator: true;
}

export type ContextMenuEntry = ContextMenuItem | ContextMenuSeparator;

interface ContextMenuState {
  visible: boolean;
  x: number;
  y: number;
  items: ContextMenuEntry[];
}

// --- Type Guard ---

export function isSeparator(entry: ContextMenuEntry): entry is ContextMenuSeparator {
  return "separator" in entry && entry.separator === true;
}

// --- Global State ---

const [menuState, setMenuState] = createSignal<ContextMenuState>({
  visible: false,
  x: 0,
  y: 0,
  items: [],
});

// --- Exported Functions ---

/**
 * Show a context menu at the mouse position.
 * Automatically flips position if near the viewport edge.
 */
export function showContextMenu(event: MouseEvent, items: ContextMenuEntry[]): void {
  event.preventDefault();
  event.stopPropagation();

  // Estimate menu dimensions for viewport edge flipping
  const menuWidth = 200;
  const menuHeight = items.length * 36;

  const viewportW = window.innerWidth;
  const viewportH = window.innerHeight;

  let x = event.clientX;
  let y = event.clientY;

  if (x + menuWidth > viewportW) {
    x = viewportW - menuWidth - 8;
  }
  if (y + menuHeight > viewportH) {
    y = viewportH - menuHeight - 8;
  }

  // Ensure we don't go negative
  x = Math.max(4, x);
  y = Math.max(4, y);

  setMenuState({ visible: true, x, y, items });
}

/**
 * Hide the context menu.
 */
export function hideContextMenu(): void {
  setMenuState({ visible: false, x: 0, y: 0, items: [] });
}

// --- Internal Helpers ---

/**
 * Remove leading, trailing, and consecutive separators from entries.
 */
function cleanSeparators(entries: ContextMenuEntry[]): ContextMenuEntry[] {
  const result: ContextMenuEntry[] = [];
  for (const entry of entries) {
    if (isSeparator(entry)) {
      // Skip if first item or previous item is also a separator
      if (result.length === 0 || isSeparator(result[result.length - 1])) {
        continue;
      }
    }
    result.push(entry);
  }
  // Remove trailing separator
  if (result.length > 0 && isSeparator(result[result.length - 1])) {
    result.pop();
  }
  return result;
}

// --- Components ---

const ContextMenuItemButton: Component<{ item: ContextMenuItem }> = (props) => {
  const handleClick = () => {
    if (props.item.disabled) return;
    hideContextMenu();
    props.item.action();
  };

  return (
    <button
      type="button"
      class={`
        w-full flex items-center gap-2.5 px-3 py-1.5 text-sm text-left rounded
        transition-colors cursor-default
        ${props.item.disabled
          ? "opacity-40 cursor-not-allowed"
          : props.item.danger
            ? "text-accent-error hover:bg-accent-error/10"
            : "text-text-primary hover:bg-white/5"
        }
      `}
      disabled={props.item.disabled}
      onClick={handleClick}
    >
      <Show when={props.item.icon}>
        {(Icon) => <Dynamic component={Icon()} class="w-4 h-4 flex-shrink-0" />}
      </Show>
      <span>{props.item.label}</span>
    </button>
  );
};

/**
 * Context menu container component.
 * Renders the context menu as a Portal overlay.
 * Mount this once in your app layout.
 */
export const ContextMenuContainer: Component = () => {
  let menuRef: HTMLDivElement | undefined;

  const handleClickOutside = (e: MouseEvent) => {
    if (menuState().visible && menuRef && !menuRef.contains(e.target as Node)) {
      hideContextMenu();
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (menuState().visible && e.key === "Escape") {
      hideContextMenu();
    }
  };

  const handleScroll = () => {
    if (menuState().visible) {
      hideContextMenu();
    }
  };

  onMount(() => {
    document.addEventListener("click", handleClickOutside, true);
    document.addEventListener("keydown", handleKeyDown);
    window.addEventListener("scroll", handleScroll, true);
  });

  onCleanup(() => {
    document.removeEventListener("click", handleClickOutside, true);
    document.removeEventListener("keydown", handleKeyDown);
    window.removeEventListener("scroll", handleScroll, true);
  });

  return (
    <Portal>
      <Show when={menuState().visible}>
        <div
          ref={menuRef}
          class="fixed z-[9999] min-w-48 bg-surface-base border border-white/10 rounded-lg shadow-xl py-1"
          style={{
            left: `${menuState().x}px`,
            top: `${menuState().y}px`,
          }}
          role="menu"
        >
          <For each={cleanSeparators(menuState().items)}>
            {(entry) => (
              <Show
                when={!isSeparator(entry)}
                fallback={
                  <div class="my-1 border-t border-white/10" role="separator" />
                }
              >
                <ContextMenuItemButton item={entry as ContextMenuItem} />
              </Show>
            )}
          </For>
        </div>
      </Show>
    </Portal>
  );
};

export default ContextMenuContainer;
