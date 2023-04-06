use image::{self, DynamicImage, ImageError};

pub fn read_metadata(file_p: &String) -> Result<DynamicImage, ImageError> {
    log::info!("Reading file: {}", file_p);

    let img = image::io::Reader::open(std::path::Path::new(&file_p))?.decode();
    match &img {
        Err(err) => {
            log::warn!("Can't properly read the image: {:?}", err);
        },
        Ok(_) => ()
    }
    return img;
}

pub fn landscape(coef: f32, img_d: &DynamicImage) -> bool {
    let height = img_d.height() as f32;
    let width = img_d.width() as f32;
    if !(height*coef > width) {
        log::info!("Image has landscape orientation");
        return true;
    } else {
        log::info!("Image has portrait orientation");
        return false;
    }
}

#[allow(non_snake_case)]
pub fn qual_control(mut minMps: f32, mut maxMps: f32, img_d: &DynamicImage) -> bool {
    if minMps > maxMps {
        let tmp1 = minMps;
        let tmp2 = maxMps;
        maxMps = tmp1;
        minMps = tmp2;
        drop(tmp1);
        drop(tmp2);
    }
    let mps: f32 = img_d.height() as f32 * img_d.width() as f32;

    log::info!("Megapixels: {}", mps/1_000_000.0);

    if mps > minMps * 1_000_000.0 && mps < maxMps * 1_000_000.0 {
        log::info!("Image has good quality");
        return true;
    } else {
        log::info!("Image has bad quality");
        return false;
    }
}