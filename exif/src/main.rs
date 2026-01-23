mod exif_my;
use exif_my::*;
use std::io::Read;

fn main() {
    let mut exifblock = ExifBlock::default();
    let mut dialog = rfd::FileDialog::new()
        .add_filter(
            "Images",
            &["bmp", "jpg", "jpeg", "png", "tif", "tiff", "gif", "webp"],
        )
        .add_filter("Png", &["png"])
        .add_filter("Jpeg kép", &["jpg", "jpeg"])
        .add_filter("Webp", &["webp"])
        .add_filter("Tiff", &["tif", "tiff"])
        .add_filter("Gif", &["gif"])
        .add_filter("Windows bitmap", &["bmp"]);
    if let Some(path) = dialog.pick_file() {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();
        //if let Ok(mut img) = image::open(&path) {
        //}
        if let Ok(mut f) = std::fs::File::open(&path) {
            let mut buffer = Vec::new();
            if f.read_to_end(&mut buffer).is_ok() {
                if let Ok(jpeg) = img_parts::jpeg::Jpeg::from_bytes(buffer.into()) {
                    let raw_exif = jpeg.segments().iter()
                        .find(|s: &&img_parts::jpeg::JpegSegment| s.marker() == 0xE1)
                        .map(|s: &img_parts::jpeg::JpegSegment| s.contents().to_vec());
                    if let Some(data) = raw_exif {
                        let len = data.len();
                        if let Ok(result) = exifblock.open( &data, len) {
                            let szep_json = serde_json::to_string_pretty(&result.json_data.unwrap()).expect("Hiba a JSON formázásakor");
                            println!("{}", szep_json);
                        }
                    }
                }
            }
        } else {
            println!("Hello, world! No Exif");
        }
    }
        
    println!("Hello, world!");
//    println!("{:?}",exifblock);
    println!("Bye!");
}
