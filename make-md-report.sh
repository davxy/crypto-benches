#!/bin/bash

# Check if a path was provided as an argument
if [ -z "$1" ]; then
  echo "Usage: $0 <path>"
  exit 1
fi

# Get the specified path from the first argument
search_path=$1

# Define the command to run on each HTML file
command="your_command_here"

old_ext=".html"
new_ext=".md"

rm -rf "./report"
cp -r "$search_path" "./report"

# Find all HTML files starting from the specified path and execute the command on each file
find "./report" -type f -name "*.html" -print0 | while IFS= read -r -d '' file; do
  echo "Processing: $file"
  base_name="${file%$old_ext}"
  new_file="${base_name}${new_ext}"
  html2md -i "$file" > "$new_file"
  sed -i 's/html/md/g' "$new_file"
  rm "$file"
done

echo "Processing complete."
