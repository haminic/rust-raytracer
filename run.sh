#!/bin/bash

EXAMPLES_DIR="examples"
OUTPUT_DIR="output"

# Collect example <scnene>.rs
mapfile -t examples < <(ls $EXAMPLES_DIR | grep '\.rs$' | sed 's/.rs//')

if [ ${#examples[@]} -eq 0 ]; then
  echo "No examples found in $EXAMPLES_DIR"
  exit 1
fi

echo "Available scenes:"
for i in "${!examples[@]}"; do
  echo "$((i+1))) ${examples[$i]}"
done

read -p "Choose a scene: " choice

INDEX=$((choice-1))

if [ $INDEX -lt 0 ] || [ $INDEX -ge ${#examples[@]} ]; then
  echo "Invalid choice"
  exit 1
fi

SCENE="${examples[$INDEX]}"

echo "Running example: $SCENE"
cargo run --release --example "$SCENE"

# Open the <output>.ppm
echo "Searching for rendered images..."

if [ "$SCENE" = "bouncing_balls" ]; then
  FILES=(
    "$OUTPUT_DIR/bouncing_balls_mt_bvh.ppm"
    "$OUTPUT_DIR/bouncing_balls_mt.ppm"
    "$OUTPUT_DIR/bouncing_balls_st.ppm"
  )
else
  FILES=($(ls "$OUTPUT_DIR/$SCENE"*.ppm 2>/dev/null))
fi

if [ ${#FILES[@]} -eq 0 ]; then
  echo "No rendered files found."
  exit 0
fi

echo "Opening rendered images..."

for f in "${FILES[@]}"; do
  if [ -f "$f" ]; then
    xdg-open "$f" >/dev/null 2>&1 &
  fi
done
