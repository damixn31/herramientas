# 🎧 SONUS

> CLI tool para descargar audio en alta calidad desde URLs (YouTube, etc.) usando `yt-dlp`.

--- 

## ⚡ Features

- 🎵 Descarga audio en formato MP3 (320kbps)
- 🎨 Spinner animado con colores en terminal
- 🧵 Descarga con feedback en tiempo real
- 📝 Logging automático (`download.log`)
- ⚙️ Basado en `yt-dlp` + `ffmpeg`

---

## 📦 Requisitos

- Python 3.10+
- ffmpeg instalado en el sistema

### Instalar ffmpeg (Arch Linux)

```bash
sudo pacman -S ffmpeg
```

### Instalar ffmpeg (Debian)

```bash
sudo apt install ffmpeg
```

```bash
git clone https://github.com/Damixn31/sonus.git
cd sonus
```

```bash
python -m venv .venv
source .venv/bin/activate
```

```bash
pip install -r requirements.txt
```

### USO

```bash
python main.py <URL>
```
