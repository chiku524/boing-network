#!/usr/bin/env python3
"""
Extract distinct elements from boing_robot_hero.png as separate PNG layers
using color segmentation (k-means) + connected components. Each layer gets
its own file with transparency for use in the hero with independent 3D motion.
"""
from pathlib import Path
import json
import sys

import cv2
import numpy as np


def load_image(path: Path):
    img = cv2.imread(str(path))
    if img is None:
        raise SystemExit(f"Could not read image: {path}")
    img = cv2.cvtColor(img, cv2.COLOR_BGR2BGRA)
    return img


def segment_by_kmeans(img_bgra: np.ndarray, K: int = 7, max_dim: int = 600) -> np.ndarray:
    """Segment by color in LAB space; return label map (H, W). Uses smaller size for speed."""
    h, w = img_bgra.shape[:2]
    if max(h, w) > max_dim:
        scale = max_dim / max(h, w)
        small = cv2.resize(img_bgra, (int(w * scale), int(h * scale)), interpolation=cv2.INTER_AREA)
    else:
        small = img_bgra
    img_rgb = cv2.cvtColor(small[:, :, :3], cv2.COLOR_BGRA2BGR)
    lab = cv2.cvtColor(img_rgb, cv2.COLOR_BGR2LAB)
    sh, sw = lab.shape[:2]
    pixels = np.float32(lab.reshape(-1, 3))
    criteria = (cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 15, 1.0)
    _, labels, _ = cv2.kmeans(pixels, K, None, criteria, 5, cv2.KMEANS_PP_CENTERS)
    labels = labels.reshape(sh, sw).astype(np.uint8)
    if small.shape[0] != h or small.shape[1] != w:
        labels = cv2.resize(labels, (w, h), interpolation=cv2.INTER_NEAREST)
    return labels


def extract_components_from_labels(
    img_bgra: np.ndarray, labels: np.ndarray, min_area_ratio: float = 0.0012
) -> list[tuple[np.ndarray, int, int]]:
    """
    For each unique label, find connected components; for each component above
    min_area_ratio of image, yield (rgba_layer, label_id, component_id).
    """
    h, w = img_bgra.shape[:2]
    total = h * w
    min_area = int(total * min_area_ratio)
    out = []

    for label_val in np.unique(labels):
        mask = (labels == label_val).astype(np.uint8)
        num_cc, cc_labels, stats, _ = cv2.connectedComponentsWithStats(mask, connectivity=8)
        for i in range(1, num_cc):
            area = stats[i, cv2.CC_STAT_AREA]
            if area < min_area:
                continue
            comp_mask = (cc_labels == i).astype(np.uint8)
            kernel = np.ones((2, 2), np.uint8)
            comp_mask = cv2.dilate(cv2.erode(comp_mask, kernel), kernel)
            rgba = img_bgra.copy()
            rgba[:, :, 3] = (rgba[:, :, 3].astype(np.float32) * comp_mask).astype(np.uint8)
            out.append((rgba, int(label_val), i))
    return out


def save_layer(rgba: np.ndarray, out_path: Path):
    """Save RGBA image as PNG (OpenCV uses BGRA)."""
    cv2.imwrite(str(out_path), rgba, [cv2.IMWRITE_PNG_COMPRESSION, 6])


def main():
    repo_root = Path(__file__).resolve().parent.parent
    src = repo_root / "public" / "boing_robot_hero.png"
    out_dir = repo_root / "public"
    out_dir.mkdir(parents=True, exist_ok=True)

    if not src.exists():
        print(f"Source image not found: {src}", file=sys.stderr)
        sys.exit(1)

    img = load_image(src)
    K = 7
    labels = segment_by_kmeans(img, K=K)
    layers = extract_components_from_labels(img, labels, min_area_ratio=0.0015)

    # Sort by approximate vertical center (top-first for layering) then by area (larger first per row)
    def sort_key(item):
        rgba, _, _ = item
        alpha = rgba[:, :, 3]
        ys, xs = np.where(alpha > 128)
        if len(ys) == 0:
            return (0, 0)
        cy = np.mean(ys)
        area = np.sum(alpha > 128)
        return (cy, -area)

    layers.sort(key=sort_key)

    manifest = []
    for idx, (rgba, label_id, comp_id) in enumerate(layers):
        name = f"hero_element_{idx}"
        out_path = out_dir / f"{name}.png"
        save_layer(rgba, out_path)
        manifest.append({"file": f"{name}.png", "label": label_id, "component": comp_id})
        print(f"  {out_path.name}")

    manifest_path = out_dir / "hero_elements_manifest.json"
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)
    print(f"Wrote {len(manifest)} layers and {manifest_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
