#!/usr/bin/env python3
"""
Extract the Boing robot (foreground) and the oceanic environment (background)
from boing_robot_hero.png. Uses rembg for foreground segmentation and OpenCV
inpainting to produce an environment-only image. Outputs:
  - boing_robot_only.png   (robot on transparent, for mascot & 3D use)
  - boing_environment.png  (coral, jellyfish, ocean — no robot, for site background)
Run from website/scripts/; requires: pip install rembg[gpu] or rembg pillow opencv-python-headless
"""
from pathlib import Path
import sys

import cv2
import numpy as np

# Optional: use rembg for best quality; fallback to k-means if not installed
try:
    from rembg import remove as rembg_remove
    from PIL import Image as PILImage
    HAS_REMBG = True
except ImportError:
    HAS_REMBG = False


def extract_with_rembg(src_path: Path, out_robot: Path, out_env: Path) -> bool:
    """Extract robot (transparent BG) and environment (inpainted) using rembg."""
    pil_img = PILImage.open(src_path).convert("RGBA")
    # rembg: returns RGBA with background removed (foreground = robot)
    no_bg = rembg_remove(pil_img)
    no_bg_arr = np.array(no_bg)
    # Robot-only: save as PNG (already has transparency)
    no_bg.save(str(out_robot))
    # Mask: alpha > 128 = foreground (robot). For inpainting we need mask = 255 where to fill.
    alpha = no_bg_arr[:, :, 3]
    robot_mask = (alpha > 128).astype(np.uint8) * 255
    # Environment = inpaint the original where the robot was
    orig_bgr = cv2.imread(str(src_path))
    if orig_bgr is None:
        return False
    env_bgr = cv2.inpaint(orig_bgr, robot_mask, 7, cv2.INPAINT_TELEA)
    cv2.imwrite(str(out_env), env_bgr)
    return True


def extract_with_kmeans(src_path: Path, out_robot: Path, out_env: Path) -> bool:
    """Fallback: use k-means to get main subject (largest central blob) as robot."""
    img = cv2.imread(str(src_path))
    if img is None:
        return False
    img = cv2.cvtColor(img, cv2.COLOR_BGR2BGRA)
    h, w = img.shape[:2]
    # Downscale for speed
    small = cv2.resize(img, (max(1, w // 3), max(1, h // 3)), interpolation=cv2.INTER_AREA)
    lab = cv2.cvtColor(small[:, :, :3], cv2.COLOR_BGR2LAB)
    pixels = np.float32(lab.reshape(-1, 3))
    K = 5
    criteria = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 10, 1.0)
    _, labels, centers = cv2.kmeans(pixels, K, None, criteria, 3, cv2.KMEANS_PP_CENTERS)
    labels = labels.reshape(small.shape[0], small.shape[1])
    # Central region is likely the robot; find the label that dominates the center
    cy, cx = small.shape[0] // 2, small.shape[1] // 2
    r = min(small.shape[0], small.shape[1]) // 3
    center_labels = labels[max(0, cy - r):cy + r, max(0, cx - r):cx + r]
    from collections import Counter
    main_label = Counter(center_labels.flatten()).most_common(1)[0][0]
    robot_mask_small = (labels == main_label).astype(np.uint8) * 255
    robot_mask = cv2.resize(robot_mask_small, (w, h), interpolation=cv2.INTER_NEAREST)
    # Smooth and dilate slightly so we don't miss edges
    robot_mask = cv2.dilate(cv2.erode(robot_mask, np.ones((3, 3), np.uint8)), np.ones((5, 5), np.uint8))
    # Robot only: original with alpha = mask
    robot_rgba = img.copy()
    robot_rgba[:, :, 3] = robot_mask
    cv2.imwrite(str(out_robot), robot_rgba)
    # Environment: inpaint
    env_bgr = cv2.inpaint(img[:, :, :3], robot_mask, 11, cv2.INPAINT_TELEA)
    cv2.imwrite(str(out_env), env_bgr)
    return True


def main():
    repo_root = Path(__file__).resolve().parent.parent
    src = repo_root / "public" / "boing_robot_hero.png"
    out_dir = repo_root / "public"
    out_dir.mkdir(parents=True, exist_ok=True)
    out_robot = out_dir / "boing_robot_only.png"
    out_env = out_dir / "boing_environment.png"

    if not src.exists():
        print(f"Source not found: {src}", file=sys.stderr)
        return 1

    if HAS_REMBG:
        print("Using rembg for extraction...")
        ok = extract_with_rembg(src, out_robot, out_env)
    else:
        print("rembg not installed; using k-means fallback (install rembg for best quality).")
        ok = extract_with_kmeans(src, out_robot, out_env)

    if not ok:
        print("Extraction failed.", file=sys.stderr)
        return 1
    print(f"  {out_robot.name}")
    print(f"  {out_env.name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
