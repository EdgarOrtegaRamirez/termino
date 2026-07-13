"""Desktop notification support for Termino."""

import subprocess
import sys
import shutil


def _notify_send(title: str, message: str) -> bool:
    """Send notification via notify-send (Linux).

    Args:
        title: Notification title.
        message: Notification body.

    Returns:
        True if notification was sent successfully.
    """
    try:
        subprocess.run(
            ["notify-send", title, message],
            capture_output=True,
            timeout=5,
        )
        return True
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return False


def _terminal_bell(title: str, message: str) -> bool:
    """Fallback notification: print to terminal with bell character.

    Args:
        title: Notification title.
        message: Notification body.

    Returns:
        True (always succeeds).
    """
    print(f"\a[{title}] {message}", file=sys.stderr, flush=True)
    return True


def send_notification(title: str, message: str, force_bell: bool = False) -> bool:
    """Send a desktop notification.

    Uses notify-send on Linux if available, falls back to terminal bell.

    Args:
        title: Notification title.
        message: Notification body.
        force_bell: If True, skip notify-send and use terminal bell.

    Returns:
        True if notification was sent.
    """
    if force_bell or not shutil.which("notify-send"):
        return _terminal_bell(title, message)
    if not _notify_send(title, message):
        return _terminal_bell(title, message)
    return True