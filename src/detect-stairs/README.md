This is an experimental, incomplete attempt in using opencv to automate the detection of stair locations

# Preparation

0. Install [tesseract](https://github.com/tesseract-ocr/tesseract/) for OCR
1. Download the Japanese OCR training data from https://github.com/tesseract-ocr/tessdata/blob/main/jpn.traineddata
2. Copy it to a dir named tessdata
3. `echo 'tessedit_create_tsv 1' > tessdata/configs/tsv`

4. Install dependencies
- opencv-python==4.7.0.72
- numpy==1.24.1
- pandas==1.4.3

# Usage

From repo root, run: 
- `mkdir -p stairs`
- `python src/detect-stairs/main.py <absolute_path_to_tesseract>`

The script scans pngs in the map dir and output images with the stairs it found to the stairs dir.

The absolute path to tesseract is needed because for some reason, opencv won't work if it is somewhere in the PATH, or a relative path is used:

```
QObject::moveToThread: Current thread (0x55e6782dec40) is not the object's thread (0x55e678574650).
Cannot move to target thread (0x55e6782dec40)

qt.qpa.plugin: Could not load the Qt platform plugin "xcb" in "/home/me/.local/lib/python3.10/site-packages/cv2/qt/plugins" even though it was found.
This application failed to start because no Qt platform plugin could be initialized. Reinstalling the application may fix this problem.

Available platform plugins are: xcb, eglfs, linuxfb, minimal, minimalegl, offscreen, vnc, wayland-egl, wayland, wayland-xcomposite-egl, wayland-xcomposite-glx.
```

The minimal reproduction is 3 lines of python:

```
import cv2
import subprocess
subprocess.run(['tesseract'])
```

It is absolutely hilarious that the most popular computer vision library in python is extremely fragile and is trivial to break completely. But it's no longer funny when I'm just trying to be productive. At some point in time, we have taken a wrong turn. Instead of statically linked binaries, everything is dynamic and nothing matters until it is ran. Instead of self contained, deterministic, and one-command package management, we build absurd abstractions over virtual machines over Docker containers.

