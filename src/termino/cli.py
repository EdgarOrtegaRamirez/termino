"""Click-based CLI interface for Termino."""

import sys
import time
import threading
import signal
from typing import Optional

import click

from termino import __version__
from termino.timer import Stopwatch, Countdown, TimerState
from termino.pomodoro import PomodoroTimer, PomodoroPhase
from termino.display import (
    console,
    print_banner,
    print_session_summary,
    print_history,
    print_controls,
    format_duration,
    get_countdown_progress,
)
from termino.notifications import send_notification
from termino.storage import save_session, get_sessions


# Global state for keyboard handling
_current_timer = None
_active = False


def _key_listener(stop_event: threading.Event):
    """Background thread that listens for keyboard input.

    Args:
        stop_event: Event to signal when to stop listening.
    """
    try:
        import termios
        import tty

        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd)
        try:
            tty.setraw(fd)
            while not stop_event.is_set():
                if sys.stdin in select.select([sys.stdin], [], [], 0.1)[0]:
                    ch = sys.stdin.read(1)
                    if ch == "q" or ch == "\x03":  # q or Ctrl+C
                        _handle_key("q")
                    elif ch == " ":
                        _handle_key("space")
                    elif ch == "l":
                        _handle_key("l")
        finally:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
    except (ImportError, AttributeError, OSError):
        # Fallback: just read line by line
        try:
            while not stop_event.is_set():
                ch = sys.stdin.read(1)
                if not ch:
                    break
                if ch == "q" or ch == "\x03":
                    _handle_key("q")
                elif ch == " ":
                    _handle_key("space")
                elif ch == "l":
                    _handle_key("l")
        except (OSError, EOFError):
            pass


def _handle_key(key: str):
    """Handle a key press.

    Args:
        key: The key that was pressed ('q', 'space', 'l').
    """
    global _current_timer, _active
    if not _active or _current_timer is None:
        return

    timer = _current_timer
    if key == "q":
        _active = False
    elif key == "space":
        if isinstance(timer, Stopwatch):
            if timer.state == TimerState.RUNNING:
                timer.pause()
            elif timer.state == TimerState.PAUSED:
                timer.resume()
        elif isinstance(timer, Countdown):
            if timer.state == TimerState.RUNNING:
                timer.pause()
            elif timer.state == TimerState.PAUSED:
                timer.resume()
    elif key == "l":
        if isinstance(timer, Stopwatch):
            try:
                lap = timer.lap()
                console.print(
                    f"  [bold yellow]Lap {lap.lap}:[/bold yellow] "
                    f"[green]{format_duration(lap.split)}[/green] "
                    f"(total: [cyan]{format_duration(lap.total)}[/cyan])"
                )
            except RuntimeError:
                pass


def _run_stopwatch_loop(stopwatch: Stopwatch) -> dict:
    """Run the stopwatch display loop.

    Args:
        stopwatch: The Stopwatch instance.

    Returns:
        Session data dict.
    """
    global _current_timer, _active
    _current_timer = stopwatch
    _active = True

    stop_event = threading.Event()
    key_thread = threading.Thread(target=_key_listener, args=(stop_event,), daemon=True)
    key_thread.start()

    try:
        stopwatch.start()
        console.print("[bold green]▶ Stopwatch started[/bold green]")
        print_controls("stopwatch")

        while _active:
            elapsed = stopwatch.elapsed
            console.print(f"\r  ⏱️  [bold cyan]{format_duration(elapsed)}[/bold cyan]", end="")

            if stopwatch.state == TimerState.STOPPED:
                break

            time.sleep(0.05)

        if stopwatch.state == TimerState.RUNNING or stopwatch.state == TimerState.PAUSED:
            # User pressed q
            final = stopwatch.stop()
            status = "interrupted"
        else:
            final = stopwatch.elapsed
            status = "completed"

        console.print()
        return {
            "type": "stopwatch",
            "duration_seconds": final,
            "status": status,
            "laps": [
                {"lap": l.lap, "split": l.split, "total": l.total}
                for l in stopwatch.laps
            ],
        }
    finally:
        stop_event.set()
        _active = False
        _current_timer = None


def _run_countdown_loop(countdown: Countdown) -> dict:
    """Run the countdown display loop.

    Args:
        countdown: The Countdown instance.

    Returns:
        Session data dict.
    """
    global _current_timer, _active
    _current_timer = countdown
    _active = True

    stop_event = threading.Event()
    key_thread = threading.Thread(target=_key_listener, args=(stop_event,), daemon=True)
    key_thread.start()

    try:
        countdown.start()
        console.print(f"[bold green]▶ Countdown started: {format_duration(countdown.duration)}[/bold green]")
        print_controls("countdown")

        while _active:
            remaining = countdown.remaining
            elapsed = countdown.elapsed
            progress = get_countdown_progress(countdown.duration, elapsed)

            bar_len = 30
            filled = int(bar_len * progress)
            bar = "█" * filled + "░" * (bar_len - filled)
            console.print(
                f"\r  ⏳ [bold yellow]{format_duration(remaining)}[/bold yellow] "
                f"[cyan]{bar}[/cyan] {int(progress * 100)}%",
                end="",
            )

            if countdown.is_finished:
                console.print()
                console.print("[bold green]⏰ Time's up![/bold green]")
                send_notification("Termino", "Countdown finished!")
                status = "completed"
                final = countdown.duration
                break

            if countdown.state == TimerState.STOPPED:
                status = "interrupted"
                final = countdown.elapsed
                break

            time.sleep(0.1)
        else:
            # While loop exited via _active = False
            final = countdown.stop()
            status = "interrupted"

        console.print()
        return {
            "type": "countdown",
            "duration_seconds": final,
            "status": status,
            "laps": [],
        }
    finally:
        stop_event.set()
        _active = False
        _current_timer = None


def _run_pomodoro_loop(pomodoro: PomodoroTimer) -> dict:
    """Run the pomodoro display loop.

    Args:
        pomodoro: The PomodoroTimer instance.

    Returns:
        Session data dict.
    """
    global _current_timer, _active

    session_data = {"type": "pomodoro", "status": "completed", "cycles": []}
    total_duration = 0.0

    try:
        while not pomodoro.is_complete:
            phase_name = pomodoro.phase.value.title()
            dur = pomodoro.get_current_duration()
            countdown = Countdown(duration=dur)
            _current_timer = countdown
            _active = True

            stop_event = threading.Event()
            key_thread = threading.Thread(target=_key_listener, args=(stop_event,), daemon=True)
            key_thread.start()

            try:
                countdown.start()
                cycle_label = f"Cycle {pomodoro.completed_cycles + 1}" if pomodoro.phase == PomodoroPhase.WORK else "Break"
                console.print(f"\n[bold]{cycle_label} — {phase_name} ({format_duration(dur)})[/bold]")
                print_controls("pomodoro")

                while _active:
                    remaining = countdown.remaining
                    elapsed = countdown.elapsed
                    progress = get_countdown_progress(dur, elapsed)

                    bar_len = 30
                    filled = int(bar_len * progress)
                    bar = "█" * filled + "░" * (bar_len - filled)
                    phase_color = "green" if pomodoro.phase == PomodoroPhase.WORK else "yellow"
                    label = "🍅" if pomodoro.phase == PomodoroPhase.WORK else "☕"
                    console.print(
                        f"\r  {label} [bold {phase_color}]{format_duration(remaining)}[/bold {phase_color}] "
                        f"[cyan]{bar}[/cyan] {int(progress * 100)}%",
                        end="",
                    )

                    if countdown.is_finished:
                        console.print()
                        if pomodoro.phase == PomodoroPhase.WORK:
                            send_notification("Termino", f"Work cycle {pomodoro.completed_cycles + 1} complete! Take a break.")
                        else:
                            send_notification("Termino", "Break over! Time to focus.")
                        pomodoro.record_cycle(dur)
                        total_duration += dur
                        pomodoro.advance_phase()
                        break

                    if countdown.state == TimerState.STOPPED:
                        # User interrupted
                        pomodoro.record_cycle(countdown.elapsed)
                        total_duration += countdown.elapsed
                        session_data["status"] = "interrupted"
                        console.print()
                        console.print("[yellow]Pomodoro interrupted.[/yellow]")
                        return session_data

                    time.sleep(0.1)
                else:
                    # _active = False from user pressing q
                    countdown.stop()
                    pomodoro.record_cycle(countdown.elapsed)
                    total_duration += countdown.elapsed
                    session_data["status"] = "interrupted"
                    console.print()
                    console.print("[yellow]Pomodoro interrupted.[/yellow]")
                    return session_data

            finally:
                stop_event.set()
                _active = False
                _current_timer = None

        console.print()
        console.print("[bold green]🎉 Pomodoro session complete![/bold green]")
        send_notification("Termino", "Pomodoro session complete! Great work!")
        session_data["cycles"] = [
            {"cycle": c.cycle, "type": c.type, "duration": c.duration}
            for c in pomodoro.cycles
        ]
        session_data["duration_seconds"] = total_duration
        return session_data

    except KeyboardInterrupt:
        session_data["status"] = "interrupted"
        session_data["duration_seconds"] = total_duration
        return session_data


def _parse_duration(value: str) -> float:
    """Parse a duration string like 5m, 1h30m, 90s, 2h.

    Args:
        value: Duration string.

    Returns:
        Duration in seconds.

    Raises:
        click.BadParameter: If the format is invalid.
    """
    if not value:
        raise click.BadParameter("Duration cannot be empty")

    total = 0.0
    i = 0
    n = len(value)
    while i < n:
        start = i
        while i < n and value[i].isdigit():
            i += 1
        if start == i:
            raise click.BadParameter(f"Expected number at position {i} in '{value}'")
        num = int(value[start:i])
        if i >= n:
            total += num  # Treat as seconds
            break
        unit = value[i]
        i += 1
        if unit == "h":
            total += num * 3600
        elif unit == "m":
            total += num * 60
        elif unit == "s":
            total += num
        else:
            raise click.BadParameter(f"Unknown unit '{unit}' in '{value}' (use h, m, s)")

    if total <= 0:
        raise click.BadParameter("Duration must be positive")
    return total


@click.group(invoke_without_command=False)
@click.version_option(version=__version__, prog_name="termino")
def cli():
    """Termino ⏱️ — A sleek terminal-based timer and stopwatch CLI tool."""
    pass


@cli.command()
@click.option("--duration", "-d", default="25m", help="Countdown duration (e.g., 5m, 1h30m, 90s)")
def countdown(duration: str):
    """Start a countdown timer.

    DURATION format examples: 5m, 1h30m, 90s, 2h
    """
    try:
        seconds = _parse_duration(duration)
    except click.BadParameter as e:
        console.print(f"[red]Error: {e.format_message()}[/red]")
        sys.exit(1)

    print_banner()
    console.print(f"[bold]Countdown:[/bold] {duration} ({format_duration(seconds)})")

    cdt = Countdown(duration=seconds)
    data = _run_countdown_loop(cdt)
    save_session(**data)
    print_session_summary(data)


@cli.command()
def stopwatch():
    """Start a stopwatch with lap timing."""
    print_banner()
    console.print("[bold]Stopwatch[/bold]")

    sw = Stopwatch()
    data = _run_stopwatch_loop(sw)
    save_session(**data)
    print_session_summary(data)


@cli.command()
@click.option("--work", "-w", default=25, type=int, help="Work duration in minutes")
@click.option("--break-duration", "-b", default=5, type=int, help="Break duration in minutes")
@click.option("--cycles", "-c", default=4, type=int, help="Number of work cycles before long break")
@click.option("--long-break", "-l", default=15, type=int, help="Long break duration in minutes")
def pomodoro(work: int, break_duration: int, cycles: int, long_break: int):
    """Start a pomodoro timer with work/break cycling."""
    if work <= 0:
        console.print("[red]Error: Work duration must be positive[/red]")
        sys.exit(1)
    if break_duration <= 0:
        console.print("[red]Error: Break duration must be positive[/red]")
        sys.exit(1)
    if cycles <= 0:
        console.print("[red]Error: Cycles must be positive[/red]")
        sys.exit(1)
    if long_break <= 0:
        console.print("[red]Error: Long break duration must be positive[/red]")
        sys.exit(1)

    print_banner()
    console.print(
        f"[bold]Pomodoro:[/bold] {work}min work, {break_duration}min break, "
        f"{cycles} cycles, {long_break}min long break"
    )

    pt = PomodoroTimer(
        work_duration=work * 60,
        break_duration=break_duration * 60,
        long_break_duration=long_break * 60,
        cycles_target=cycles,
    )
    data = _run_pomodoro_loop(pt)
    save_session(**data)
    print_session_summary(data)


@cli.command()
@click.option("--limit", "-n", default=10, type=int, help="Number of sessions to show")
@click.option("--type", "-t", "session_type", help="Filter by session type (stopwatch, countdown, pomodoro)")
def history(limit: int, session_type: Optional[str]):
    """Show session history."""
    if session_type and session_type not in ("stopwatch", "countdown", "pomodoro"):
        console.print(f"[red]Error: Invalid session type '{session_type}'. Use: stopwatch, countdown, or pomodoro[/red]")
        sys.exit(1)

    sessions = get_sessions(session_type=session_type, limit=limit)
    print_banner()
    print_history(sessions, limit=limit)


# Import select at module level for use in key_listener
import select

if __name__ == "__main__":
    cli()