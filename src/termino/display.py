"""Rich-based terminal display formatting for Termino."""

from datetime import timedelta
from rich.table import Table
from rich.console import Console
from rich.progress import Progress, BarColumn, TextColumn, TimeElapsedColumn, TimeRemainingColumn
from rich.text import Text
from rich import box

console = Console()


def format_duration(seconds: float) -> str:
    """Format a duration in seconds to HH:MM:SS.mmm."""
    td = timedelta(seconds=seconds)
    total_seconds = int(td.total_seconds())
    hours, remainder = divmod(total_seconds, 3600)
    minutes, secs = divmod(remainder, 60)
    millis = int((seconds - int(seconds)) * 1000)
    if hours > 0:
        return f"{hours:02d}:{minutes:02d}:{secs:02d}.{millis:03d}"
    return f"{minutes:02d}:{secs:02d}.{millis:03d}"


def print_banner():
    """Print the termino banner."""
    banner = Text()
    banner.append("╔══════════════════════════════╗\n", style="blue")
    banner.append("║    ", style="blue")
    banner.append("Termino ⏱️", style="bold cyan")
    banner.append("     ║\n", style="blue")
    banner.append("╚══════════════════════════════╝\n", style="blue")
    console.print(banner)


def print_session_summary(data: dict):
    """Print a session summary after completion.

    Args:
        data: Session data dict with type, duration_seconds, laps, etc.
    """
    console.print()
    console.print("[bold green]✓ Session Complete[/bold green]")
    console.print(f"  Type:     {data.get('type', 'unknown')}")
    console.print(f"  Duration: {format_duration(data.get('duration_seconds', 0))}")
    console.print(f"  Status:   {data.get('status', 'unknown')}")

    laps = data.get("laps", [])
    if laps:
        lap_table = Table(title="Laps", box=box.SIMPLE)
        lap_table.add_column("Lap #", style="cyan")
        lap_table.add_column("Split", style="yellow")
        lap_table.add_column("Total", style="green")
        for lap in laps:
            lap_table.add_row(
                str(lap["lap"]),
                format_duration(lap["split"]),
                format_duration(lap["total"]),
            )
        console.print(lap_table)

    pomo_cycles = data.get("cycles", [])
    if pomo_cycles:
        cycle_table = Table(title="Pomodoro Cycles", box=box.SIMPLE)
        cycle_table.add_column("Cycle", style="cyan")
        cycle_table.add_column("Type", style="yellow")
        cycle_table.add_column("Duration", style="green")
        for cycle in pomo_cycles:
            cycle_table.add_row(
                str(cycle["cycle"]),
                cycle["type"],
                format_duration(cycle["duration"]),
            )
        console.print(cycle_table)

    console.print()


def print_history(sessions: list, limit: int = 10):
    """Print session history as a table.

    Args:
        sessions: List of session dicts.
        limit: Maximum number of sessions to show.
    """
    if not sessions:
        console.print("[yellow]No sessions found.[/yellow]")
        return

    table = Table(title=f"Session History (last {min(limit, len(sessions))})", box=box.SIMPLE)
    table.add_column("#", style="dim")
    table.add_column("Type", style="cyan")
    table.add_column("Duration", style="green")
    table.add_column("Date", style="yellow")
    table.add_column("Status", style="magenta")

    for i, session in enumerate(sessions[:limit], 1):
        started = session.get("started", "")[:19]  # Trim to YYYY-MM-DDTHH:MM:SS
        dur = format_duration(session.get("duration_seconds", 0))
        status = session.get("status", "unknown")
        status_style = "green" if status == "completed" else "red" if status == "interrupted" else "yellow"
        table.add_row(
            str(i),
            session.get("type", "unknown"),
            dur,
            started,
            Text(status, style=status_style),
        )

    console.print(table)
    console.print(f"\nTotal sessions: [bold]{len(sessions)}[/bold]")


def print_controls(mode: str = "stopwatch"):
    """Print keyboard controls for the current mode.

    Args:
        mode: One of 'stopwatch', 'countdown', 'pomodoro'.
    """
    console.print()
    if mode == "stopwatch":
        console.print("[dim]Controls: [l] Lap  [Space] Pause/Resume  [q/Ctrl+C] Stop[/dim]")
    elif mode == "countdown":
        console.print("[dim]Controls: [Space] Pause/Resume  [q/Ctrl+C] Stop[/dim]")
    elif mode == "pomodoro":
        console.print("[dim]Controls: [Space] Pause/Resume  [q/Ctrl+C] Stop[/dim]")


def get_countdown_progress(duration: float, elapsed: float) -> float:
    """Calculate countdown progress as a 0-1 float.

    Args:
        duration: Total countdown duration in seconds.
        elapsed: Time elapsed in seconds.

    Returns:
        Progress ratio (0.0 to 1.0).
    """
    if duration <= 0:
        return 0.0
    return min(max(elapsed / duration, 0.0), 1.0)