OUTPUT_DIR="output"
SAVE_DIR="saved"

mapfile -t outputs < <(ls $OUTPUT_DIR | grep '\.ppm$' | sed 's/.ppm//')

echo "Available Scene"
for i in "${!outputs[@]}"; do
  echo "$((i+1))) ${outputs[$i]}"
done

echo "input command
    -o file - [open]
    -s file - [save]
leave blank to exit"

while :; do
    echo "--------------------------"
    echo -n "Enter (mode, index): "
    read cmd choice || break
    [ -z "$cmd" ] || [ -z "$choice" ] && break
    index=$((choice-1))

    if [ $index -lt 0 ] || [ $index -ge ${#outputs[@]} ] ; then
    echo "Invalid: index $choice is out of range"
    fi

    filename="${outputs[$index]}"
    case "$cmd" in 
      "-o")
        echo "opening $filename.ppm"
        xdg-open "$OUTPUT_DIR/$filename.ppm";;
      "-s")
        echo "name to save: ${outputs[$index]}_???.png"
        args=""
        echo -n "input argument: " && read args
        # string is not empty
        [ -n "$args" ] &&  args="_$args"
        echo "saving to $SAVE_DIR/${filename}$args.png"
        # using ImageMagick package
        magick "$OUTPUT_DIR/$filename.ppm" "$SAVE_DIR/${filename}$args.png";;
      *)
        echo "Invalid: command is either -o or -s";;
    esac
    
done

echo "Exit !"
exit 0
