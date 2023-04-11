use std::io::Cursor;

use crate::model::CardData;
use image::{
    imageops::{overlay, vertical_gradient, FilterType},
    DynamicImage, Pixel, Rgba,
};
use imageproc::drawing::{draw_text_mut, text_size};
use kmeans_colors::{CentroidData, Sort};
use palette::{rgb::Rgb, IntoColor, Lab, Pixel as PalettePixel, Srgb};
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

struct GradientColors {
    plain: Rgba<u8>,
    gradient: Option<(Rgba<u8>, Rgba<u8>)>,
}

fn luminance(color: Rgba<u8>) -> f32 {
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
        Srgb::from_raw_slice(data).iter().map(|x| x.into_format().into_color()).collect();

    let mut res = kmeans_colors::Kmeans::new();
    for i in 0..2 {
        let run_res = kmeans_colors::get_kmeans_hamerly(8, 20, 5.0, false, &lab, 0 + i as u64);
        (run_res.score < res.score).then(|| res = run_res);
    }

    Lab::sort_indexed_colors(&res.centroids, &res.indices)
}

fn find_best_colors(image: image::DynamicImage) -> GradientColors {
    let color_filter = |x: &CentroidData<Lab>, bright_limit, dark_limit| {
        let c: Srgb = x.centroid.into_color();
        let color = c.into_components();
        let filter = ((color.0 + color.1 + color.2) < bright_limit)
            && ((color.0 + color.1 + color.2) > dark_limit);
        filter.then_some(c).or(None)
    };

    let palette_to_rgb_pixel = |rgb: &Rgb| {
        Rgba([
            (rgb.red * 255.0) as u8,
            (rgb.green * 255.0) as u8,
            (rgb.blue * 255.0) as u8,
            255,
        ])
    };

    let mut res = get_kmeans_colors(image.as_bytes());

    let mut dominant_colors =
        res.iter().filter_map(|x| color_filter(x, 2.3, 0.8)).collect::<Vec<Rgb>>();
    dominant_colors.dedup();

    res.sort_unstable_by(|a, b| (b.percentage).partial_cmp(&a.percentage).unwrap());
    let mut res = res.iter().filter_map(|x| color_filter(x, 2.1, 0.8)).collect::<Vec<Rgb>>();
    res.dedup();

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

pub fn generate_card(card_data: CardData, image: &[u8]) -> Vec<u8> {
    let jacket = image::load_from_memory(image).unwrap();
    let jacket_size = (card_data.jacket_size as f32 * 0.75) as u32;
    let resized_jacket = jacket.resize(jacket_size, jacket_size, FilterType::Triangle);

    let text_offset_x = jacket_size + JACKET_OFFSET * 2 - 5;
    let text_area_width = (jacket_size as f32 * 0.84).round() as u32;

    let canvas_width = jacket_size + text_area_width + JACKET_OFFSET * 3;
    let canvas_height = jacket_size + JACKET_OFFSET * 2;
    let mut canvas = DynamicImage::new_rgba8(canvas_width, canvas_height);
	
    let GradientColors { plain, gradient } = find_best_colors(jacket);
    let (start, end) = gradient.unwrap();
	let mut blend = start.clone();
	blend.blend(&Rgba([end[0], end[1], end[2], 127]));
	
    let font = Font::try_from_bytes(DEFAULT_FONT).expect("Font not found");
    let ja_font = Font::try_from_bytes(JAPANESE_FONT).expect("Font not found");
    let regex = regex::Regex::new(REGEX_JA).unwrap();

    let select_font = |s: &str| regex.is_match(s).then_some(&ja_font).unwrap_or(&font);

    let adjust_text = |s: &str, scale: Scale| {
        let (text_width, ..) = text_size(scale, select_font(s), s);
        (text_width > text_area_width as i32)
            .then_some(Scale::uniform(scale.x * (text_area_width as f32 / text_width as f32)))
            .unwrap_or(scale)
    };

    let text_color = |s: &str, is_genres: bool| {
        let avg_luminance = is_genres.then_some(luminance(end)).unwrap_or(luminance(blend));
        (avg_luminance > 0.179)
            .then_some((s.len() == 1).then_some(BLACK).unwrap_or(TRANSPARENT))
            .unwrap_or(WHITE)
    };

    let mut scales: Vec<Scale> = Vec::new();
    let texts = vec![&card_data.name, &card_data.album, &card_data.artists, &card_data.genres];

    for i in 0..4 {
        scales.push(adjust_text(
            texts[i],
            (i == 3).then_some(Scale::uniform(GENRES_SCALE)).unwrap_or(Scale::uniform(TEXT_SCALE)),
        ));
    }

    let name_y_pos = TEXT_OFFSET_Y as i32;
    let album_y_pos = (TEXT_OFFSET_Y * 2 + TEXT_SPACING) as i32;
    let artists_y_pos = (TEXT_OFFSET_Y * 3 + TEXT_SPACING * 2) as i32;
    let genres_y_pos = (canvas_height - JACKET_OFFSET - 15) as i32;
    let y_pos = vec![name_y_pos, album_y_pos, artists_y_pos, genres_y_pos];

    vertical_gradient(&mut canvas, &start, &end);
    overlay(&mut canvas, &resized_jacket, JACKET_OFFSET as i64, JACKET_OFFSET as i64);

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
            let text = transparent_text_box(texts[i], select_font(texts[i]), scales[i]);
            overlay(&mut canvas, &text, text_offset_x as i64 - 2, y_offset_by_idx(i, &text));
            continue;
        }

        draw_text_mut(
            &mut canvas,
            color_by_idx(i),
            text_offset_x as i32,
            y_pos[i],
            scales[i],
            select_font(texts[i]),
            texts[i],
        );
    }

    //canvas.save("gradient.png").expect("saving fucked up");
    let mut buffer: Vec<u8> = vec![];
    canvas.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png).unwrap();
    buffer
    /*
	log::info!("\nmaking plain version\n");
	let text_color_plain = |s: &str| {
        let avg_luminance = luminance(plain);
        (avg_luminance > 0.179)
            .then_some((s.len() == 1).then_some(BLACK).unwrap_or(TRANSPARENT))
            .unwrap_or(WHITE)
    };

    vertical_gradient(&mut canvas, &plain, &plain);
    overlay(&mut canvas, &resized_jacket, JACKET_OFFSET as i64, JACKET_OFFSET as i64);

    for i in 0..4 {
        if i == 1 && &card_data.album_type == "single" {
            continue;
        }

        if text_color_plain(texts[i]) == TRANSPARENT {
            let text = transparent_text_box(texts[i], select_font(texts[i]), scales[i]);
            overlay(&mut canvas, &text, text_offset_x as i64 - 2, y_offset_by_idx(i, &text));
            continue;
        }

        draw_text_mut(
            &mut canvas,
            text_color_plain(texts[i]),
            text_offset_x as i32,
            y_pos[i],
            scales[i],
            select_font(texts[i]),
            texts[i],
        );
    }

    canvas.save("plain.png").unwrap(); */
}
