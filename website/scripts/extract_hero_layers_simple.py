#!/usr/bin/env python3
"""
Extract k-means color layers from boing_robot_hero.png (one PNG per segment).
Fast: segments at reduced size then applies masks at full res. No connected-components.
Output: hero_layer_0.png .. hero_layer_K-1.png for use with independent 3D motion.
"""
from pathlib import Path
import sys

import cv2
import numpy as np

K = 6
MAX_DIM = 400


def main():
    repo_root = Path(__file__).resolve().parent.parent
    src = repo_root / "public" / "boing_robot_hero.png"
    out_dir = repo_root / "public"
    out_dir.mkdir(parents=True, exist_ok=True)

    if not src.exists():
        print(f"Source not found: {src}", file=sys.stderr)
        sys.exit(1)

    img = cv2.imread(str(src))
    if img is None:
        sys.exit(1)
    img = cv2.cvtColor(img, cv2.COLOR_BGR2BGRA)
    h, w = img.shape[:2]

    # Segment at small size
    scale = min(1.0, MAX_DIM / max(h, w))
    sw, sh = int(w * scale), int(h * scale)
    small = cv2.resize(img, (sw, sh), interpolation=cv2.INTER_AREA)
    lab = cv2.cvtColor(small[:, :, :3], cv2.COLOR_BGRA2BGR)
    lab = cv2.cvtColor(lab, cv2.COLOR_BGR2LAB)
    pixels = np.float32(lab.reshape(-1, 3))
    criteria = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 10, 1.0)
    _, labels, _ = cv2.kmeans(pixels, K, None, criteria, 3, cv2.KMEANS_PP_CENTERS)
    labels_small = labels.reshape(sh, sw).astype(np.uint8)

    # Upscale labels to full size
    labels_full = cv2.resize(labels_small, (w, h), interpolation=cv2.INTER_NEAREST)

    # Export one RGBA per label (original colors, alpha = mask)
    for i in range(K):
        mask = (labels_full == i).astype(np.uint8)
        # Soften edges
        mask = cv2.GaussianBlur(mask, (3, 3), 0.5)
        mask = (mask * 255).astype(np.uint8)
        rgba = img.copy()
        rgba[:, :, 3] = cv2.min(rgba[:, :, 3], mask)
        out_path = out_dir / f"hero_layer_{i}.png"
        cv2.imwrite(str(out_path), rgba, [cv2.IMWRITE_PNG_COMPRESSION, 6])
        print(out_path.name)
    print(f"Wrote {K} layers to {out_dir}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
