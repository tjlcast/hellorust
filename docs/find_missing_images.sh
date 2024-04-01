#!/bin/bash

# 获取images目录下所有图片文件的名称
images_dir="images"
images=$(find "$images_dir" -maxdepth 1 -type f \( -name "*.jpg" -o -name "*.png" -o -name "*.gif" \))

# 获取当前目录下所有非images目录的文件路径
other_files=()
while IFS= read -r other_file; do
    other_files+=("$other_file")
done < <(find . -type f ! -path "*.images/*" ! -path "." -print)

# 用于存储不在其他文件中出现的图片文件名
missing_images=()

# 遍历images文件，检查是否在其他文件中出现
for image in $images; do
    image_name=$(basename "$image")

    found=0
    for other_file in "${other_files[@]}"; do
        if grep -q "$image_name" "$other_file"; then
            found=1
            break
        fi
    done
    if [ $found -eq 0 ]; then
        missing_images+=("$image_name")
    fi
done

echo "Missing images: "$missing_images
for missing_image in "${missing_images[@]}"; do
    echo "$missing_image"
done
