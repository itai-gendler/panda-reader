#[path = "../src/ui/quote_poster.rs"]
mod quote_poster;

#[test]
fn bundled_cjk_fallback_is_one_regular_face() {
    // Linux's GPUI fallback chain takes the first face in a family. A variable
    // Noto Sans SC asset exposes Thin (100) first and silently makes CJK text
    // much lighter than the Latin UI, regardless of the requested UI weight.
    let mut fonts = resvg::usvg::fontdb::Database::new();
    fonts.load_font_data(include_bytes!("../assets/fonts/PandaReaderCJK-Regular.ttf").to_vec());
    let faces: Vec<_> = fonts.faces().collect();
    assert_eq!(faces.len(), 1);
    assert_eq!(faces[0].weight.0, 400);
    assert!(
        faces[0]
            .families
            .iter()
            .any(|(family, _)| family == "Panda Reader CJK")
    );
}
