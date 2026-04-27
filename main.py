import signal

from PyObjCTools import AppHelper
from dotenv import load_dotenv
from pathlib import Path

from src.listener import MusicListener
from src.scrobbler import ScrobbleManager


def main():
    # allow ctrl+c
    signal.signal(signal.SIGINT, signal.SIG_DFL)

    manager = ScrobbleManager()

    listener = MusicListener.alloc().initWithManager_(manager)
    listener.startListening()

    try:
        # run apple event listener
        AppHelper.runEventLoop()
    except KeyboardInterrupt:
        print("\nExiting Scrobbler...")


if __name__ == "__main__":
    config_path = Path.home() / ".config" / "my_scrobbler" / "config.env"
    load_dotenv(dotenv_path=config_path)
    main()
