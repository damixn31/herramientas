import sys
from downloader.core import download_audio
from downloader.config import get_ydl_options
from ui.spinner import Spinner

def main():
    if len(sys.argv) < 2:
        print("Use: python main.py <URL>")
        sys.exit(1)

    url = sys.argv[1]

    spinner = Spinner(f"downloading {url[:25]}...")
    spinner.start()

    elapsed = download_audio(url, get_ydl_options())

    spinner.stop()

    if elapsed > 0:
        print(f"[+] Downloaded in {elapsed:.2f}s")
    else:
        print("Error Downloading")

if __name__ == "__main__":
    main()
