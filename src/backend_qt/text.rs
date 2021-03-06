// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use qt;
use usvg;
use usvg::prelude::*;

// self
use super::prelude::*;
use backend_utils::text::{
    self,
    FontMetrics,
};
use super::{
    fill,
    stroke,
};

pub use backend_utils::text::draw_blocks;


pub struct QtFontMetrics<'a> {
    p: &'a qt::Painter,
}

impl<'a> QtFontMetrics<'a> {
    pub fn new(p: &'a qt::Painter) -> Self {
        QtFontMetrics { p }
    }
}

impl<'a> FontMetrics<qt::Font> for QtFontMetrics<'a> {
    fn set_font(&mut self, font: &usvg::Font) {
        let font = init_font(font);
        self.p.set_font(&font);
    }

    fn font(&self) -> qt::Font {
        self.p.font()
    }

    fn width(&self, text: &str) -> f64 {
        self.p.font_metrics().width(text)
    }

    fn ascent(&self) -> f64 {
        self.p.font_metrics().ascent()
    }

    fn height(&self) -> f64 {
        self.p.font_metrics().height()
    }
}

pub fn draw(
    tree: &usvg::Tree,
    text_node: &usvg::Text,
    opt: &Options,
    p: &qt::Painter,
) -> Rect {
    let mut fm = QtFontMetrics::new(p);
    draw_blocks(text_node, &mut fm, |block| draw_block(tree, block, opt, p))
}

fn draw_block(
    tree: &usvg::Tree,
    block: &text::TextBlock<qt::Font>,
    opt: &Options,
    p: &qt::Painter,
) {
    p.set_font(&block.font);
    let font_metrics = p.font_metrics();

    let bbox = block.bbox;

    let old_ts = p.get_transform();

    if !block.rotate.is_fuzzy_zero() {
        let mut ts = usvg::Transform::default();
        ts.rotate_at(block.rotate, bbox.x, bbox.y + font_metrics.ascent());
        p.apply_transform(&ts.to_native());
    }

    let mut line_rect = Rect::new(bbox.x, 0.0, bbox.width, font_metrics.line_width());

    // Draw underline.
    //
    // Should be drawn before/under text.
    //
    // a-text-decoration-001.svg
    // a-text-decoration-009.svg
    if let Some(ref style) = block.decoration.underline {
        line_rect.y = bbox.y + font_metrics.height() - font_metrics.underline_pos();
        draw_line(tree, line_rect, &style.fill, &style.stroke, opt, p);
    }

    // Draw overline.
    //
    // Should be drawn before/under text.
    //
    // a-text-decoration-002.svg
    if let Some(ref style) = block.decoration.overline {
        line_rect.y = bbox.y + font_metrics.height() - font_metrics.overline_pos();
        draw_line(tree, line_rect, &style.fill, &style.stroke, opt, p);
    }

    // Draw text.
    fill::apply(tree, &block.fill, opt, bbox, p);
    stroke::apply(tree, &block.stroke, opt, bbox, p);

    p.draw_text(bbox.x, bbox.y, &block.text);

    // Draw line-through.
    //
    // Should be drawn after/over text.
    //
    // a-text-decoration-003.svg
    if let Some(ref style) = block.decoration.line_through {
        line_rect.y = bbox.y + font_metrics.ascent() - font_metrics.strikeout_pos();
        draw_line(tree, line_rect, &style.fill, &style.stroke, opt, p);
    }

    p.set_transform(&old_ts);
}

fn init_font(dom_font: &usvg::Font) -> qt::Font {
    let mut font = qt::Font::new();

    // a-font-family-001.svg
    // a-font-family-002.svg
    // a-font-family-003.svg
    // a-font-family-004.svg
    // a-font-family-005.svg
    // a-font-family-006.svg
    // a-font-family-007.svg
    // a-font-family-008.svg
    // a-font-family-009.svg
    // a-font-family-010.svg
    font.set_family(&dom_font.family);

    // a-font-style-001.svg
    // a-font-style-002.svg
    let font_style = match dom_font.style {
        usvg::FontStyle::Normal => qt::FontStyle::StyleNormal,
        usvg::FontStyle::Italic => qt::FontStyle::StyleItalic,
        usvg::FontStyle::Oblique => qt::FontStyle::StyleOblique,
    };
    font.set_style(font_style);

    // a-font-variant-001.svg
    if dom_font.variant == usvg::FontVariant::SmallCaps {
        font.set_small_caps(true);
    }

    // a-font-weight-009.svg
    let font_weight = match dom_font.weight {
        usvg::FontWeight::W100       => qt::FontWeight::Thin,
        usvg::FontWeight::W200       => qt::FontWeight::ExtraLight,
        usvg::FontWeight::W300       => qt::FontWeight::Light,
        usvg::FontWeight::W400       => qt::FontWeight::Normal,
        usvg::FontWeight::W500       => qt::FontWeight::Medium,
        usvg::FontWeight::W600       => qt::FontWeight::DemiBold,
        usvg::FontWeight::W700       => qt::FontWeight::Bold,
        usvg::FontWeight::W800       => qt::FontWeight::ExtraBold,
        usvg::FontWeight::W900       => qt::FontWeight::Black,
    };
    font.set_weight(font_weight);

    // a-font-stretch-001.svg
    let font_stretch = match dom_font.stretch {
        usvg::FontStretch::Normal         => qt::FontStretch::Unstretched,
        usvg::FontStretch::Narrower |
        usvg::FontStretch::Condensed      => qt::FontStretch::Condensed,
        usvg::FontStretch::UltraCondensed => qt::FontStretch::UltraCondensed,
        usvg::FontStretch::ExtraCondensed => qt::FontStretch::ExtraCondensed,
        usvg::FontStretch::SemiCondensed  => qt::FontStretch::SemiCondensed,
        usvg::FontStretch::SemiExpanded   => qt::FontStretch::SemiExpanded,
        usvg::FontStretch::Wider |
        usvg::FontStretch::Expanded       => qt::FontStretch::Expanded,
        usvg::FontStretch::ExtraExpanded  => qt::FontStretch::ExtraExpanded,
        usvg::FontStretch::UltraExpanded  => qt::FontStretch::UltraExpanded,
    };
    font.set_stretch(font_stretch);

    // a-font-size-001.svg
    font.set_size(dom_font.size);

    font
}

fn draw_line(
    tree: &usvg::Tree,
    r: Rect,
    fill: &Option<usvg::Fill>,
    stroke: &Option<usvg::Stroke>,
    opt: &Options,
    p: &qt::Painter,
) {
    fill::apply(tree, fill, opt, r, p);
    stroke::apply(tree, stroke, opt, r, p);
    p.draw_rect(r.x, r.y, r.width + 1.0, r.height);
}
