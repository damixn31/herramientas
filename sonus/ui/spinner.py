import threading
import itertools
import sys
import time
import shutil

COLORS = [
    "\033[91m",  # Rojo
    "\033[92m",  # Verde
    "\033[93m",  # Amarillo
    "\033[94m",  # Azul
    "\033[95m",  # Magenta
    "\033[96m",  # Cyan
]
RESET = "\033[0m"

class Spinner:
    def __init__(self, message="Download "):
        self.message = message
        self.running = False
        self.thread = None

    def start(self):
        self.running = True
        self.thread = threading.Thread(target=self.__spin, daemon=True)
        self.thread.start()

    def stop(self):
        self.running = False
        self.thread.join()

    def __spin(self):
        chars_points = itertools.cycle(['_', '•', '°', '·'])
        chars = itertools.cycle(['|', '/', '-', '\\'])
        color_cycle = itertools.cycle(COLORS)
        letter_cycle =itertools.cycle([str.upper, str.lower])
        indices = itertools.cycle(range(len(self.message)))

        print("\n")

        message = self.message

        while self.running:
            color = next(color_cycle)
            char_point = next(chars_points)
            char = next(chars)

            message_list = list(message)

            for _ in range(2):
                index = next(indices)
                message_list[index] = next(letter_cycle) (message_list[index])
            modified_message = ''.join(message_list)

            sys.stdout.write(f"\r{color}{char_point}{RESET} {color}{char}{RESET} {modified_message}")
            sys.stdout.flush()
            time.sleep(0.1)
            message = modified_message

        sys.stdout.write("\r" + " " * shutil.get_terminal_size().columns + "\r")
        sys.stdout.flush()
