use gpui_kit::*;

const CJK_FONT_FAMILY: &str = "Panda Reader CJK";

pub(crate) fn sidebar_font() -> Font {
    let mut face = font("Inter");
    face.fallbacks = Some(chinese_sans_fallbacks());
    face
}

pub(in crate::ui) fn app_ui_font() -> Font {
    let mut face = font("Source Sans 3");
    face.fallbacks = Some(chinese_sans_fallbacks());
    face
}

pub(in crate::ui) fn article_font_family(serif: bool) -> Font {
    let mut face = font(if serif {
        "Source Serif 4"
    } else {
        "Source Sans 3"
    });
    face.fallbacks = Some(if serif {
        FontFallbacks::from_fonts(vec![
            "Noto Serif SC".into(),
            "Songti SC".into(),
            "SimSun".into(),
            CJK_FONT_FAMILY.into(),
        ])
    } else {
        chinese_sans_fallbacks()
    });
    face
}

fn chinese_sans_fallbacks() -> FontFallbacks {
    FontFallbacks::from_fonts(vec![
        // GPUI's Linux fallback chain takes the first face in a family. The
        // original variable font defaulted to Thin (100), even for regular UI
        // text. This bundled family has one static Regular (400) face, retains
        // the full CJK character map, and cannot collide with a system family.
        CJK_FONT_FAMILY.into(),
        "Microsoft YaHei UI".into(),
        "PingFang SC".into(),
        "Segoe UI".into(),
    ])
}
