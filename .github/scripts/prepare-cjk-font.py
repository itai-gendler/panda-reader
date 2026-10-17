"""Build the bundled single-face CJK fallback from Noto Sans SC's variable TTF.

Requires fontTools. This preserves the complete character map; it is not a subset.
"""

import argparse
import hashlib
from pathlib import Path

from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    font = TTFont(args.source, recalcTimestamp=False)
    original_characters = set(font.getBestCmap())
    axis = next(axis for axis in font["fvar"].axes if axis.axisTag == "wght")
    print(f"Source wght axis: {axis.minValue}/{axis.defaultValue}/{axis.maxValue}")

    # GPUI's Linux fallback chain chooses the first face, rather than matching
    # the requested weight. The variable font's default face is Thin (100), so
    # ship one static Regular face to avoid unexpectedly hairline CJK glyphs.
    font = instantiateVariableFont(font, {"wght": 400}, inplace=True)
    names = {
        1: "Panda Reader CJK",
        2: "Regular",
        3: "PandaReaderCJK-Regular-" + font["name"].getDebugName(5),
        4: "Panda Reader CJK Regular",
        6: "PandaReaderCJK-Regular",
        16: "Panda Reader CJK",
        17: "Regular",
    }
    for record in list(font["name"].names):
        if record.nameID in names:
            font["name"].setName(
                names[record.nameID], record.nameID,
                record.platformID, record.platEncID, record.langID,
            )
    for name_id, value in names.items():
        font["name"].setName(value, name_id, 3, 1, 0x409)
    font["OS/2"].usWeightClass = 400
    font["OS/2"].fsSelection = (font["OS/2"].fsSelection & ~0x21) | 0x40
    font["head"].macStyle &= ~0x3
    args.output.parent.mkdir(parents=True, exist_ok=True)
    font.save(args.output)

    result = TTFont(args.output)
    assert "fvar" not in result, "Output must contain one fixed-weight face"
    assert result["OS/2"].usWeightClass == 400
    assert set(result.getBestCmap()) == original_characters, "Character coverage changed"
    print(f"Output: {len(original_characters)} characters, {args.output.stat().st_size} bytes")
    print(f"SHA-256: {hashlib.sha256(args.output.read_bytes()).hexdigest()}")


if __name__ == "__main__":
    main()
