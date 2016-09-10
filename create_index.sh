#!/bin/bash

out="$(pwd)/$2"
tempfile=$(mktemp -t "not-stakkr-man-create_index.XXXXXXXXXX" --suffix=".html")

(
    cat index_head.htm
    for f in $(find "$1" -name "*.md" | awk '{ print length, $0 }' | sort -n -s | cut -d" " -f2-); do
        f=$(basename "$f" .md)
        echo '<li><a href="man/'"$f"'.1.html"><strong>'"$f"'(1)</strong></a></li>'
    done
    cat index_tail.htm
) > "$tempfile"

pushd "$(dirname "$tempfile")" > /dev/null
curl -X POST -s --data-urlencode "input@$(basename "$tempfile")" http://html-minifier.com/raw -o "$out"
popd > /dev/null

sed -i -e 's/> />/g' -e 's/ </</g' "$out"
echo "Built $out"
