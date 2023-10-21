use std::io::Cursor;

use crate::model::CardData;
use image::{
    imageops::{overlay, vertical_gradient, FilterType},
    DynamicImage, Pixel, Rgba,
};
use imageproc::drawing::{draw_text_mut, text_size};
use kmeans_colors::{CentroidData, Sort};
use palette::{rgb::Rgb, IntoColor, Lab, Srgb};
use rusttype::{point, Font, Scale};

const JACKET_OFFSET: u32 = 30;
const TEXT_OFFSET_Y: u32 = 30;
const TEXT_SPACING: u32 = 60;
const WHITE: Rgba<u8> = Rgba([255; 4]);
const BLACK: Rgba<u8> = Rgba([0, 0, 0, 191]);
const TRANSPARENT: Rgba<u8> = Rgba([0; 4]);
const TEXT_SCALE: f32 = 60.0;
const GENRES_SCALE: f32 = 10.0;
const DEFAULT_FONT: &[u8] = include_bytes!("../../common/assets/Montserrat-Bold.ttf");
const JAPANESE_FONT: &[u8] = include_bytes!("../../common/assets/MPLUS2-Bold.ttf");
const REGEX_JA: &str =
    r"[\u3040-\u30ff\u3400-\u4dbf\u4e00-\u9fff\uf900-\ufaff\uff66-\uff9f\u3131-\uD79D]";

#[derive(Debug, Clone, PartialEq)]
pub struct GradientColors {
    pub plain: Rgba<u8>,
    pub gradient: Option<(Rgba<u8>, Rgba<u8>)>,
}

impl GradientColors {
    pub fn gradient_blend(&self) -> Rgba<u8> {
        let (start, end) = self.gradient.unwrap();
        let mut blend = start.clone();
        blend.blend(&Rgba([end[0], end[1], end[2], 127]));
        blend
    }

    pub fn gradient_custom_blend(&self, blend_factor: u8) -> Rgba<u8> {
        let (start, end) = self.gradient.unwrap();
        let mut blend = start.clone();
        blend.blend(&Rgba([end[0], end[1], end[2], blend_factor]));
        blend
    }

    pub fn gradient_start(&self) -> Rgba<u8> {
        self.gradient.unwrap().0
    }

    pub fn gradient_end(&self) -> Rgba<u8> {
        self.gradient.unwrap().1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasAssets {
    pub jacket_size: u32,
    pub jacket: DynamicImage,
    pub colors: GradientColors,
}

impl CanvasAssets {
    pub fn set_jacket_size(&mut self, jacket_size: u32) {
        self.jacket_size = jacket_size;
    }

    pub fn text_offset_x(&self) -> u32 {
        self.jacket_size + JACKET_OFFSET * 2 - 5
    }

    pub fn text_area_width(&self) -> u32 {
        (self.jacket_size as f32 * 0.84).round() as u32
    }

    pub fn canvas_height(&self) -> u32 {
        self.jacket_size + JACKET_OFFSET * 2
    }

    pub fn canvas_width(&self) -> u32 {
        self.jacket_size + self.text_area_width() + JACKET_OFFSET * 3
    }
}

#[derive(Debug, Clone)]
pub struct TextAssets {
    pub name: String,
    pub name_transparent: DynamicImage,
    pub album: String,
    pub album_transparent: DynamicImage,
    pub artists: String,
    pub artists_transparent: DynamicImage,
    pub genres: String,
    pub genres_transparent: DynamicImage,
    pub scales: Vec<Scale>,
    pub font: Font<'static>,
    pub jp_font: Font<'static>,
    pub regex: regex::Regex,
}

fn luminance(color: &Rgba<u8>) -> f32 {
    let srgb_to_rgb = |val| {
        let val = val as f32 / 255.0;
        (val <= 0.04045).then_some(val / 12.92).unwrap_or(((val + 0.055) / 1.055).powf(2.4))
    };

    0.2126 * srgb_to_rgb(color[0]) + 0.7152 * srgb_to_rgb(color[1]) + 0.0722 * srgb_to_rgb(color[2])
}

fn transparent_text_box(text: &str, font: &Font<'static>, scale: Scale) -> DynamicImage {
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(text, scale, point(20.0, 20.0 + v_metrics.ascent)).collect();
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();
    let glyphs_width = {
        let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
        let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
        (max_x - min_x) as u32
    };
    let height = if glyphs_height > 12.0 { (glyphs_height * 0.8).round() } else { glyphs_height };
    let y_pos = if height < 11.0 {
        0
    } else if height < 48.0 {
        -(height * 0.1667).round() as i32
    } else {
        -8
    };
    let mut text_box = DynamicImage::new_rgba8(glyphs_width + 6, height as u32);
    vertical_gradient(&mut text_box, &BLACK, &BLACK);
    draw_text_mut(&mut text_box, TRANSPARENT, 2, y_pos, scale, font, text);
    text_box
}

fn get_kmeans_colors(data: &[u8]) -> Vec<CentroidData<Lab>> {
    let lab: Vec<Lab> =
        palette::cast::from_component_slice::<Srgb<u8>>(data).iter().map(|x| x.into_format().into_color()).collect();

    let mut res = kmeans_colors::Kmeans::new();
    for i in 0..5 {
        let run_res = kmeans_colors::get_kmeans_hamerly(8, 20, 5.0, false, &lab, 0 + i as u64);
        (run_res.score < res.score).then(|| res = run_res);
    }

    Lab::sort_indexed_colors(&res.centroids, &res.indices)
}

fn find_best_colors(image: &[u8]) -> GradientColors {
    let color_filter = |x: &CentroidData<Lab>, bright_limit, dark_limit| {
        let c: Srgb = x.centroid.into_color();
        let color = c.into_components();
        let filter = ((color.0 + color.1 + color.2) < bright_limit)
            && ((color.0 + color.1 + color.2) > dark_limit);
        filter.then_some(c).or(None)
    };

    let palette_to_rgb_pixel = |rgb: &Rgb| {
        Rgba([(rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8, 255])
    };

    let mut res = get_kmeans_colors(image);

    let mut dominant_colors =
        res.iter().filter_map(|x| color_filter(x, 2.3, 0.8)).collect::<Vec<Rgb>>();
    dominant_colors.dedup();

    res.sort_unstable_by(|a, b| (b.percentage).partial_cmp(&a.percentage).unwrap());
    let mut res = res.iter().filter_map(|x| color_filter(x, 2.1, 0.8)).collect::<Vec<Rgb>>();
    res.dedup();
	log::info!("{:?}",res);
    GradientColors {
        plain: palette_to_rgb_pixel(res.first().unwrap()),
        gradient: (!dominant_colors.is_empty() || dominant_colors.len() > 1)
            .then(|| {
                let brightest = palette_to_rgb_pixel(dominant_colors.last().unwrap());
                let darkest = palette_to_rgb_pixel(dominant_colors.first().unwrap());

                (brightest, darkest)
            })
            .or(None),
    }
}

pub fn generate_text_assets(card_data: CardData, canvas_assets: CanvasAssets) -> TextAssets {
    let font = Font::try_from_bytes(DEFAULT_FONT).expect("Font not found");
    let jp_font = Font::try_from_bytes(JAPANESE_FONT).expect("Font not found");
    let regex = regex::Regex::new(REGEX_JA).unwrap();

    let select_font = |s: &str| regex.is_match(s).then_some(&jp_font).unwrap_or(&font);

    let adjust_text = |s: &str, scale: Scale| {
        let (text_width, ..) = text_size(scale, select_font(s), s);
        (text_width > canvas_assets.text_area_width() as i32)
            .then_some(Scale::uniform(
                scale.x * (canvas_assets.text_area_width() as f32 / text_width as f32),
            ))
            .unwrap_or(scale)
    };

    let texts = vec![&card_data.name, &card_data.album, &card_data.artists, &card_data.genres];

    let mut scales: Vec<Scale> = Vec::new();

    for i in 0..4 {
        scales.push(adjust_text(
            texts[i],
            (i == 3).then_some(Scale::uniform(GENRES_SCALE)).unwrap_or(Scale::uniform(TEXT_SCALE)),
        ));
    }

    TextAssets {
        name: card_data.name.to_string(),
        name_transparent: transparent_text_box(texts[0], select_font(texts[0]), scales[0]),
        album: card_data.album.to_string(),
        album_transparent: transparent_text_box(texts[1], select_font(texts[1]), scales[1]),
        artists: card_data.artists.to_string(),
        artists_transparent: transparent_text_box(texts[2], select_font(texts[2]), scales[2]),
        genres: card_data.genres.to_string(),
        genres_transparent: transparent_text_box(texts[3], select_font(texts[3]), scales[3]),
        scales,
        font,
        jp_font,
        regex,
    }
}

pub fn generate_canvas_assets(card_data: CardData) -> CanvasAssets {
    let jacket = image::load_from_memory(&card_data.jacket_bytes).unwrap();
    let jacket_size = (card_data.jacket_size as f32 * 0.75) as u32;
    let resized_jacket = jacket.resize(jacket_size, jacket_size, FilterType::Triangle);

    let colors = find_best_colors(jacket.as_bytes());

    CanvasAssets { jacket_size, jacket: resized_jacket, colors }
}

pub fn generate_card(
    card_data: CardData, canvas_assets: CanvasAssets, text_assets: TextAssets, bg_type: String,
) -> Vec<u8> {
    let mut canvas =
        DynamicImage::new_rgba8(canvas_assets.canvas_width(), canvas_assets.canvas_height());

    match bg_type.as_str() {
        "plain" => {
            vertical_gradient(&mut canvas, &canvas_assets.colors.plain, &canvas_assets.colors.plain)
        }
        "gradient" => vertical_gradient(
            &mut canvas,
            &canvas_assets.colors.gradient_start(),
            &canvas_assets.colors.gradient_end(),
        ),
        "inverted" => vertical_gradient(
            &mut canvas,
            &canvas_assets.colors.gradient_end(),
            &canvas_assets.colors.gradient_start(),
        ),
        _ => vertical_gradient(
            &mut canvas,
            &canvas_assets.colors.gradient_start(),
            &canvas_assets.colors.gradient_end(),
        ),
    };

    overlay(&mut canvas, &canvas_assets.jacket, JACKET_OFFSET as i64, JACKET_OFFSET as i64);

    let name_y_pos = TEXT_OFFSET_Y as i32;
    let album_y_pos = (TEXT_OFFSET_Y * 2 + TEXT_SPACING) as i32;
    let artists_y_pos = (TEXT_OFFSET_Y * 3 + TEXT_SPACING * 2) as i32;
    let genres_y_pos = (canvas_assets.canvas_height() - JACKET_OFFSET - 15) as i32;
    let y_pos = vec![name_y_pos, album_y_pos, artists_y_pos, genres_y_pos];

    let text_color = |s: &str, is_genres: bool| {
        let avg_luminance = is_genres
            .then_some(match bg_type.as_str() {
                "plain" => luminance(&canvas_assets.colors.plain),
				"inverted" => luminance(&canvas_assets.colors.gradient_start()),
                _ => luminance(&canvas_assets.colors.gradient_end()),
            })
            .unwrap_or(match bg_type.as_str() {
                "plain" => luminance(&canvas_assets.colors.plain),
                _ => luminance(&canvas_assets.colors.gradient_blend()),
            });
        (avg_luminance > 0.179)
            .then_some((s.len() == 1).then_some(BLACK).unwrap_or(TRANSPARENT))
            .unwrap_or(WHITE)
    };

    let texts = vec![&card_data.name, &card_data.album, &card_data.artists, &card_data.genres];
    let transparent_texts = vec![
        &text_assets.name_transparent,
        &text_assets.album_transparent,
        &text_assets.artists_transparent,
        &text_assets.genres_transparent,
    ];

    let select_font = |s: &str| {
        text_assets.regex.is_match(s).then_some(&text_assets.jp_font).unwrap_or(&text_assets.font)
    };

    let color_by_idx = |i: usize| text_color(texts[i], if i == 3 { true } else { false });
    let text_box_offset = |i: usize, text: &DynamicImage| {
        if text.height() < 11 {
            y_pos[i] as i64
        } else if text.height() < 48 {
            y_pos[i] as i64 + (text.height() as f32 * 0.1667).round() as i64
        } else {
            y_pos[i] as i64 + 8
        }
    };
    let y_offset_by_idx = |i: usize, text: &DynamicImage| {
        (i == 3).then_some(y_pos[i] as i64).unwrap_or(text_box_offset(i, text))
    };

    for i in 0..4 {
        if i == 1 && &card_data.album_type == "single" {
            continue;
        }

        if color_by_idx(i) == TRANSPARENT {
            overlay(
                &mut canvas,
                transparent_texts[i],
                canvas_assets.text_offset_x() as i64 - 2,
                y_offset_by_idx(i, transparent_texts[i]),
            );
            continue;
        }

        draw_text_mut(
            &mut canvas,
            color_by_idx(i),
            canvas_assets.text_offset_x() as i32,
            y_pos[i],
            text_assets.scales[i],
            select_font(texts[i]),
            texts[i],
        );
    }

    let mut buffer: Vec<u8> = vec![];
    canvas.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png).unwrap();
    buffer
}
