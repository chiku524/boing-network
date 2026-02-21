# Website scripts

## Extracting hero layers from `boing_robot_hero.png`

Two approaches:

### 1. Simple (recommended): one layer per color segment

```bash
pip install -r requirements.txt
python extract_hero_layers_simple.py
```

- Uses k-means in LAB space at reduced size for speed.
- Writes `public/hero_layer_0.png` … `public/hero_layer_5.png` (6 layers).
- The homepage hero stacks these and gives each layer its own 3D motion.

### 2. Full: one layer per connected component

```bash
python extract_hero_layers.py
```

- Segments by k-means then splits each color region into connected components.
- Writes many `public/hero_element_*.png` and `public/hero_elements_manifest.json`.
- Slower on large images; useful if you want every small object (e.g. each jellyfish) as its own file.

Requirements: `opencv-python-headless`, `numpy` (see `requirements.txt`).
