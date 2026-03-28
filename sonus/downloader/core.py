import yt_dlp
import time
from .logger import log_message

def download_audio(url, options):
    log_message(f"Start Download: {url}")
    start_time = time.time()
    try:
        with yt_dlp.YoutubeDL(options) as ydl:
            ydl.download([url])
        elapsed = time.time() - start_time
        log_message(f"Download Complete in {elapsed:.2f}s")
        return elapsed
    except Exception as e:
        log_message(f"Error: {e}")
        return 0

        
