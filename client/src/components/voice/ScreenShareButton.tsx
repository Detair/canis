import { Component, createSignal, createMemo } from "solid-js";
import { MonitorUp, MonitorOff } from "lucide-solid";
import { voiceState, stopScreenShare } from "@/stores/voice";
import { currentUser } from "@/stores/auth";
import { showToast } from "@/components/ui/Toast";

/** Maximum concurrent screen shares per user. */
const MAX_SCREEN_SHARES = 3;

interface ScreenShareButtonProps {
  /** Show source picker first (native capture), then quality picker. */
  onShowSourcePicker?: () => void;
  onShowQualityPicker?: () => void;
}

/**
 * Screen share toggle button.
 *
 * In multi-stream mode the button always opens the source/quality picker to
 * start a new stream (up to `MAX_SCREEN_SHARES`). Stopping individual
 * streams is handled via the viewer UI.
 */
const ScreenShareButton: Component<ScreenShareButtonProps> = (props) => {
  const [loading, setLoading] = createSignal(false);

  /** Number of screen shares the local user currently owns. */
  const ownShareCount = createMemo(() => {
    const userId = currentUser()?.id;
    if (!userId) return 0;
    return voiceState.screenShares.filter((s) => s.user_id === userId).length;
  });

  const atLimit = createMemo(() => ownShareCount() >= MAX_SCREEN_SHARES);

  const handleClick = async () => {
    if (atLimit()) {
      // At the stream limit — stop all shares
      setLoading(true);
      try {
        await stopScreenShare();
      } catch (err) {
        console.error("Failed to stop screen share:", err);
        showToast({
          type: "error",
          title: "Could not stop screen share.",
          duration: 8000,
        });
      } finally {
        setLoading(false);
      }
    } else {
      // Open picker to start a new share
      if (props.onShowSourcePicker) {
        props.onShowSourcePicker();
      } else {
        props.onShowQualityPicker?.();
      }
    }
  };

  const buttonTitle = () => {
    if (atLimit()) return "Stop All Shares";
    if (voiceState.screenSharing) return `Share Another Screen (${ownShareCount()}/${MAX_SCREEN_SHARES})`;
    return "Share Screen";
  };

  return (
    <button
      onClick={handleClick}
      disabled={voiceState.state !== "connected" || loading()}
      class={`p-2 rounded-full transition-colors ${
        voiceState.screenSharing
          ? atLimit()
            ? "bg-warning/20 text-warning hover:bg-danger/20 hover:text-danger"
            : "bg-success/20 text-success hover:bg-success/30 hover:text-success"
          : "bg-background-secondary text-text-secondary hover:bg-background-primary hover:text-text-primary"
      } ${loading() ? "opacity-50 cursor-wait" : ""}`}
      title={buttonTitle()}
    >
      {atLimit() ? (
        <MonitorOff class="w-5 h-5" />
      ) : (
        <MonitorUp class="w-5 h-5" />
      )}
    </button>
  );
};

export default ScreenShareButton;
