/**
 * Push-to-Talk / Push-to-Mute state management.
 *
 * `resolveState` is a pure function that derives the mute flag from config
 * and current key states.  `PttController` wraps it with key tracking,
 * release-delay timers, and a callback into the audio subsystem.
 */

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** Minimal config slice needed by resolveState. */
export interface PttConfig {
  pttEnabled: boolean;
  ptmEnabled: boolean;
}

/** Full runtime config used by PttController. */
export interface PttFullConfig {
  pttEnabled: boolean;
  pttKey: string | null;
  pttReleaseDelay: number; // ms, default 200
  ptmEnabled: boolean;
  ptmKey: string | null;
  ptmReleaseDelay: number; // ms, default 200
}

// ---------------------------------------------------------------------------
// Pure state resolution
// ---------------------------------------------------------------------------

/**
 * Determine whether the mic should be muted given the current config and
 * key states.
 *
 * Truth table (muted = true):
 *  - PTT only:  rest=muted,   PTT held=unmuted
 *  - PTM only:  rest=unmuted, PTM held=muted
 *  - Both:      rest=muted (PTT defines rest), PTT held=unmuted, PTM held=muted
 *  - Both held: muted (mute wins — safety first)
 *  - Neither:   unmuted (VAD mode)
 */
export function resolveState(
  config: PttConfig,
  pttHeld: boolean,
  ptmHeld: boolean,
): boolean {
  // PTM held always wins — muting takes priority for safety.
  if (config.ptmEnabled && ptmHeld) return true;

  // PTT mode: muted when key is NOT held.
  if (config.pttEnabled) return !pttHeld;

  // Neither PTT nor PTM enabled → unmuted (VAD / open-mic mode).
  return false;
}

// ---------------------------------------------------------------------------
// Controller
// ---------------------------------------------------------------------------

export class PttController {
  private readonly setMute: (muted: boolean) => void | Promise<void>;

  private config: PttFullConfig | null = null;
  private pttHeld = false;
  private ptmHeld = false;
  private releaseTimer: ReturnType<typeof setTimeout> | null = null;

  constructor(setMuteFn: (muted: boolean) => void | Promise<void>) {
    this.setMute = setMuteFn;
  }

  // -----------------------------------------------------------------------
  // Lifecycle
  // -----------------------------------------------------------------------

  /** Register config and apply the initial mute state. */
  activate(config: PttFullConfig): void {
    this.config = config;
    this.pttHeld = false;
    this.ptmHeld = false;
    this.clearTimer();
    this.applyState();
  }

  /** Clear all state and cancel pending timers. */
  deactivate(): void {
    this.config = null;
    this.pttHeld = false;
    this.ptmHeld = false;
    this.clearTimer();
  }

  // -----------------------------------------------------------------------
  // Key events
  // -----------------------------------------------------------------------

  /** Track a key press. Ignores repeats and unrelated keys. */
  handleKeyDown(code: string): void {
    if (!this.config) return;

    const role = this.roleForKey(code);
    if (role === null) return;

    if (role === "ptt") {
      if (this.pttHeld) return; // ignore repeat
      this.pttHeld = true;
    } else {
      if (this.ptmHeld) return; // ignore repeat
      this.ptmHeld = true;
    }

    // Key press cancels any pending release-delay timer and applies
    // the new state immediately.
    this.clearTimer();
    this.applyState();
  }

  /** Start a release-delay timer for the given key. */
  handleKeyUp(code: string): void {
    if (!this.config) return;

    const role = this.roleForKey(code);
    if (role === null) return;

    const delay =
      role === "ptt"
        ? this.config.pttReleaseDelay
        : this.config.ptmReleaseDelay;

    // Mark the key as released, but defer the state change by the
    // configured release delay so the user's last syllable isn't clipped.
    if (role === "ptt") {
      this.pttHeld = false;
    } else {
      this.ptmHeld = false;
    }

    this.clearTimer();
    this.releaseTimer = setTimeout(() => {
      this.releaseTimer = null;
      this.applyState();
    }, delay);
  }

  /** Immediately release all held keys (e.g. on window blur). */
  releaseAll(): void {
    if (!this.config) return;
    this.pttHeld = false;
    this.ptmHeld = false;
    this.clearTimer();
    this.applyState();
  }

  // -----------------------------------------------------------------------
  // Queries
  // -----------------------------------------------------------------------

  /** Whether the controller has an active config. */
  isActive(): boolean {
    return this.config !== null;
  }

  /** Whether PTT or PTM is enabled in the current config. */
  isPttOrPtmEnabled(): boolean {
    if (!this.config) return false;
    return this.config.pttEnabled || this.config.ptmEnabled;
  }

  // -----------------------------------------------------------------------
  // Internal helpers
  // -----------------------------------------------------------------------

  private roleForKey(code: string): "ptt" | "ptm" | null {
    if (!this.config) return null;
    if (this.config.pttEnabled && this.config.pttKey === code) return "ptt";
    if (this.config.ptmEnabled && this.config.ptmKey === code) return "ptm";
    return null;
  }

  private applyState(): void {
    if (!this.config) return;
    const muted = resolveState(this.config, this.pttHeld, this.ptmHeld);
    this.setMute(muted);
  }

  private clearTimer(): void {
    if (this.releaseTimer !== null) {
      clearTimeout(this.releaseTimer);
      this.releaseTimer = null;
    }
  }
}
