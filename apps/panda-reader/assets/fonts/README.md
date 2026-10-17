# Bundled CJK fallback

`PandaReaderCJK-Regular.ttf` is a static Regular (400) instance of Noto Sans SC,
renamed to the private family `Panda Reader CJK`. It remains under the SIL Open
Font License in `NOTO_SANS_SC_OFL.txt`.

The original variable font defaults to Thin (100). GPUI 0.3.9's Linux fallback
chain selects the first face in a fallback family, without matching the parent
font's requested weight. A single static face prevents Chinese text from
unexpectedly rendering with hairline strokes. The private family name also
prevents installed system fonts from changing which fallback face is selected.

This is not a character subset: all 30,890 Unicode mappings from the original
font are preserved. Existing Latin UI fonts remain Inter and Source Sans 3.
`NotoSansSC-VF.ttf` remains bundled for quote posters, whose SVG renderer selects
the requested variable weight correctly, including the heavy quote text.

To reproduce with fontTools 4.38.0:

```sh
python .github/scripts/prepare-cjk-font.py NotoSansSC-VF.ttf \
  apps/panda-reader/assets/fonts/PandaReaderCJK-Regular.ttf
```

Source variable TTF SHA-256:
`a3041811a78c361b1de50f953c805e0244951c21c5bd412f7232ef0d899af0da`

Generated static TTF SHA-256:
`5645facda8df085e53fdaeae24da13bcf713280d2f910f0678d999f517379c05`
