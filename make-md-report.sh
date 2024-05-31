#!/bin/bash

src_path="./target/criterion/"
# Check if a path was provided as an argument
if [[ -n "$1" ]]; then
  src_path=$1
fi

dst_path="./criterion"

old_ext=".html"
new_ext=".md"


rsync -av --delete "$src_path" "$dst_path"

# Find all HTML files starting from the specified path and execute the command on each file
find "$dst_path" -type f -name "*.html" -print0 | while IFS= read -r -d '' file; do
  echo "Processing: $file"
  base_name="${file%$old_ext}"
  new_file="${base_name}${new_ext}"
  html2md -i "$file" > "$new_file"
  sed -i 's/html/md/g' "$new_file"
  rm "$file"
done

echo "Processing complete."

