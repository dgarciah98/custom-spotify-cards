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
pub struct ColorSelectorEmit {
    pub new_color: Rgba<u8>,
    pub row: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradientColors {
    pub plain: Rgba<u8>,
    pub gradient: Option<(Rgba<u8>, Rgba<u8>)>,
    pub custom_gradient: Option<(Rgba<u8>, Rgba<u8>)>,
    pub all_colors: Vec<Rgba<u8>>,
}

impl GradientColors {
    pub fn gradient_blend(&self, is_custom_gradient: bool) -> Rgba<u8> {
        let (start, end) = match is_custom_gradient {
            false => self.gradient.unwrap(),
            _ => self.custom_gradient.unwrap(),
        };

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

    pub fn gradient_start(&self, is_custom_gradient: bool) -> Rgba<u8> {
        match is_custom_gradient {
            false => self.gradient.unwrap().0,
            _ => self.custom_gradient.unwrap().0,
        }
    }

    pub fn gradient_end(&self, is_custom_gradient: bool) -> Rgba<u8> {
        match is_custom_gradient {
            false => self.gradient.unwrap().1,
            _ => self.custom_gradient.unwrap().1,
        }
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

fn generate_text_box(text: &str, font: &Font<'static>, scale: Scale, text_color: Rgba<u8>, is_genres: bool) -> DynamicImage {
	let v_metrics = font.v_metrics(scale);
	let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil();
	let height = if glyphs_height > 12.0 { (glyphs_height * 0.8).round() } else { glyphs_height };
	let y_pos = if height < 11.0 {
		0
	} else if height < 48.0 {
		-(height * 0.1667).round() as i32
	} else {
		-8
	};
	let lines = textwrap::wrap(text, if is_genres { 10000 } else { 36 });
	let line_values = lines.iter().map(|line| {
		let glyphs: Vec<_> = font.layout(&line, scale, point(20.0, 20.0 + v_metrics.ascent)).collect();
		
		let glyphs_width = {
			let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
			let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
			(max_x - min_x) as u32
		};
		(glyphs_width, line)
	}).collect::<Vec<_>>();

	let total_height = line_values.len() as f32 * (height.ceil() + 4.0);
	let glyphs_width = line_values.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
	let mut final_text_box = DynamicImage::new_rgba8(glyphs_width + 6, total_height.ceil() as u32);
	let mut final_text_box_cursor = 0;
	line_values.iter().for_each(|val| {
		let mut text_box = DynamicImage::new_rgba8(val.0 + 6, height.ceil() as u32 + 4);
		if text_color == TRANSPARENT { vertical_gradient(&mut text_box, &BLACK, &BLACK); }
		draw_text_mut(&mut text_box, text_color, 2, y_pos, scale, font, &val.1);
		overlay(&mut final_text_box, &text_box, 0, final_text_box_cursor as i64);
		final_text_box_cursor += text_box.height() as i32;
	});

    final_text_box
}

fn get_kmeans_colors(data: &[u8]) -> Vec<CentroidData<Lab>> {
    let lab: Vec<Lab> = palette::cast::from_component_slice::<Srgb<u8>>(data)
        .iter()
        .map(|x| x.into_format().into_color())
        .collect();

    let mut res = kmeans_colors::Kmeans::new();
    for i in 0..8 {
        let run_res = kmeans_colors::get_kmeans_hamerly(9, 20, 5.0, false, &lab, 0 + i as u64);
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

    let mut res: Vec<CentroidData<Lab>> = get_kmeans_colors(image);

    let gradient = {
        let mut dominant_colors =
            res.iter().filter_map(|x| color_filter(x, 2.9, 0.7)).collect::<Vec<Rgb>>();
        dominant_colors.dedup();
        (!dominant_colors.is_empty() || dominant_colors.len() > 1)
            .then(|| {
                let brightest =
                    palette_to_rgb_pixel(dominant_colors.get(dominant_colors.len() - 2).unwrap());
                let darkest = palette_to_rgb_pixel(dominant_colors.first().unwrap());

                (brightest, darkest)
            })
            .or(None)
    };

    res.sort_unstable_by(|a, b| (b.percentage).partial_cmp(&a.percentage).unwrap());
    let plain = res.iter().filter_map(|x| color_filter(x, 2.1, 0.8)).collect::<Vec<Rgb>>();

    let all_colors = {
        let mut color_list =
            res.iter().filter_map(|x| color_filter(x, 2.9, 0.7)).collect::<Vec<Rgb>>();
        color_list.dedup();
        color_list.iter().map(|x| palette_to_rgb_pixel(x)).collect()
    };

    GradientColors {
        plain: palette_to_rgb_pixel(plain.first().unwrap()),
        gradient,
        custom_gradient: gradient,
        all_colors,
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
		let lines_scales = textwrap::wrap(texts[i], 36).iter().map(|line| {
			adjust_text(
				line,
				(i == 3).then_some(Scale::uniform(GENRES_SCALE)).unwrap_or(Scale::uniform(TEXT_SCALE))
			)
		}).collect::<Vec<_>>();
		scales.push(*lines_scales.iter().min_by(|a, b| (a.x).partial_cmp(&b.x).unwrap()).unwrap());
    }

    TextAssets {
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
    let is_custom = bg_type == "custom";

    match bg_type.as_str() {
        "plain" => {
            vertical_gradient(&mut canvas, &canvas_assets.colors.plain, &canvas_assets.colors.plain)
        }
        "inverted" => vertical_gradient(
            &mut canvas,
            &canvas_assets.colors.gradient_end(false),
            &canvas_assets.colors.gradient_start(false),
        ),
        _ => vertical_gradient(
            &mut canvas,
            &canvas_assets.colors.gradient_start(is_custom),
            &canvas_assets.colors.gradient_end(is_custom),
        ),
    };

    overlay(&mut canvas, &canvas_assets.jacket, JACKET_OFFSET as i64, JACKET_OFFSET as i64);

    let genres_y_pos = (canvas_assets.canvas_height() - JACKET_OFFSET - 15) as i32;

    let text_color = |s: &str, is_genres: bool| {
        let avg_luminance = is_genres
            .then_some(match bg_type.as_str() {
                "plain" => luminance(&canvas_assets.colors.plain),
                "inverted" => luminance(&canvas_assets.colors.gradient_start(false)),
                _ => luminance(&canvas_assets.colors.gradient_end(is_custom)),
            })
            .unwrap_or(match bg_type.as_str() {
                "plain" => luminance(&canvas_assets.colors.plain),
                _ => luminance(&canvas_assets.colors.gradient_blend(is_custom)),
            });
        (avg_luminance > 0.179)
            .then_some((s.len() == 1).then_some(BLACK).unwrap_or(TRANSPARENT))
            .unwrap_or(WHITE)
    };

    let texts = vec![&card_data.name, &card_data.album, &card_data.artists, &card_data.genres];

    let select_font = |s: &str| {
        text_assets.regex.is_match(s).then_some(&text_assets.jp_font).unwrap_or(&text_assets.font)
    };

    let color_by_idx = |i: usize| text_color(texts[i], i == 3);
    let text_box_offset = |text: &DynamicImage, cursor: i64| {
        if text.height() < 11 {
            cursor
        } else if text.height() < 48 {
            cursor + (text.height() as f32 * 0.1667).round() as i64
        } else {
            cursor + 8
        }
    };
    let y_offset_by_idx = |i: usize, text: &DynamicImage, cursor: i64| {
        (i == 3).then_some(genres_y_pos as i64).unwrap_or(text_box_offset(text, cursor))
    };

	let mut y_pos_cursor = TEXT_OFFSET_Y as i64;
    for i in 0..4 {
		if texts[i] != "" {
			if i == 3 && color_by_idx(i) != TRANSPARENT {
				draw_text_mut(
					&mut canvas,
					color_by_idx(i),
					canvas_assets.text_offset_x() as i32,
					genres_y_pos,
					text_assets.scales[i],
					select_font(texts[i]),
					texts[i],
				);		
				continue;
			}
			
			let text_box = generate_text_box(texts[i], select_font(texts[i]), text_assets.scales[i], color_by_idx(i), i == 3);

			if i == 1 && &card_data.album_type == "single" {
				y_pos_cursor += (text_box.height() + TEXT_SPACING) as i64;
				continue;
			}
			
			overlay(
				&mut canvas,
				&text_box,
				canvas_assets.text_offset_x() as i64 - 2,
				y_offset_by_idx(i, &text_box, y_pos_cursor),
			);
			y_pos_cursor += (text_box.height() + TEXT_SPACING) as i64;
		}
	}

    let mut buffer: Vec<u8> = vec![];
    canvas.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png).unwrap();
    buffer
}
