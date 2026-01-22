use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map, json};

#[macro_export]
macro_rules! apply_exif_tags {
    ($callback:ident) => {
        $callback!((), [
            (InteropIndex,0x0001,"InteropIndex"), 
            (InteropVersion,0x0002,"InteropVersion"),
            (ImageWidth,0x0100,"ImageWidth"),
            (ImageLength,0x0101,"ImageLength"),
            (BitsPerSample,0x0102,"BitsPerSample"),
            (Compression,0x0103,"Compression"),
            (PhotometricInterpretation,0x0106,"PhotometricInterpretation"),
            (FillOrder,0x010A,"FillOrder"),
            (DocumentName,0x010D,"DocumentName"),
            (ImageDescription,0x010E,"ImageDescription"),
            (MAKE,0x010F,"Make"),
            (MODEL,0x0110,"Model"),
            (StripOffsets,0x0111,"StripOffsets"),
            (ORIENTATION,0x0112,"Orientation"),
            (SamplesPerPixel,0x0115,"SamplesPerPixel"),
            (RowsPerStrip,0x0116,"RowsPerStrip"),
            (StripByteCounts,0x0117,"StripByteCounts"),
            (XResolution,0x011A,"XResolution"),
            (YResolution,0x011B,"YResolution"),
            (PlanarConfiguration,0x011C,"PlanarConfiguration"),
            (ResolutionUnit,0x0128,"ResolutionUnit"),
            (TransferFunction,0x012D,"TransferFunction"),
            (Software,0x0131,"Software"),
            (DATETIME,0x0132,"DateTime"),
            (Artist,0x013B,"Artist"),
            (WhitePoint,0x013E,"WhitePoint"),
            (PrimaryChromaticities,0x013F,"PrimaryChromaticities"),
            (TransferRange,0x0156,"TransferRange"),
            (JPEGProc,0x0200,"JPEGProc"),
            (THUMBNAIL_OFFSET,0x0201,"ThumbnailOffset"),
            (THUMBNAIL_LENGTH,0x0202,"ThumbnailLength"),
            (YCbCrCoefficients,0x0211,"YCbCrCoefficients"),
            (YCbCrSubSampling,0x0212,"YCbCrSubSampling"),
            (YCbCrPositioning,0x0213,"YCbCrPositioning"),
            (ReferenceBlackWhite,0x0214,"ReferenceBlackWhite"),
            (RelatedImageWidth,0x1001,"RelatedImageWidth"),
            (RelatedImageLength,0x1002,"RelatedImageLength"),
            (CFARepeatPatternDim,0x828D,"CFARepeatPatternDim"),
            (CFAPattern,0x828E,"CFAPattern"),
            (BatteryLevel,0x828F,"BatteryLevel"),
            (Copyright,0x8298,"Copyright"),
            (EXPOSURETIME,0x829A,"ExposureTime"),
            (FNUMBER,0x829D,"FNumber"),
            (IPTC_NAA,0x83BB,"IPTC/NAA"),
            (EXIF_OFFSET,0x8769,"ExifOffset"),
            (InterColorProfile,0x8773,"InterColorProfile"),
            (EXPOSURE_PROGRAM,0x8822,"ExposureProgram"),
            (SpectralSensitivity,0x8824,"SpectralSensitivity"),
            (GPSInfo,0x8825,"GPSInfo"),
            (ISO_EQUIVALENT,0x8827,"ISOSpeedRatings"),
            (OECF,0x8828,"OECF"),
            (ExifVersion,0x9000,"ExifVersion"),
            (DATETIME_ORIGINAL,0x9003,"DateTimeOriginal"),
            (DATETIME_DIGITIZED,0x9004,"DateTimeDigitized"),
            (ComponentsConfiguration,0x9101,"ComponentsConfiguration"),
            (CompressedBitsPerPixel,0x9102,"CompressedBitsPerPixel"),
            (SHUTTERSPEED,0x9201,"ShutterSpeedValue"),
            (APERTURE,0x9202,"ApertureValue"),
            (BrightnessValue,0x9203,"BrightnessValue"),
            (EXPOSURE_BIAS,0x9204,"ExposureBiasValue"),
            (MAXAPERTURE,0x9205,"MaxApertureValue"),
            (SUBJECT_DISTANCE,0x9206,"SubjectDistance"),
            (METERING_MODE,0x9207,"MeteringMode"),
            (LIGHT_SOURCE,0x9208,"LightSource"),
            (FLASH,0x9209,"Flash"),
            (FOCALLENGTH,0x920A,"FocalLength"),
            (FlashEnergy_,0x920B,"FlashEnergy"),
            (SpatialFrequencyResponse_,0x920C,"SpatialFrequencyResponse"),
            (FOCALPLANEXRES_,0x920E,"FocalPlaneXResolution"),
            (FocalPlaneYResolution_,0x920F,"FocalPlaneYResolution"),
            (FOCALPLANEUNITS_,0x9210,"FocalPlaneResolutionUnit"),
            (SubjectLocation_,0x9214,"SubjectLocation"),
            (EXPOSURE_INDEX_,0x9215,"ExposureIndex"),
            (SensingMethod_,0x9217,"SensingMethod"),
            (MAKER_NOTE,0x927C,"MakerNote"),
            (USERCOMMENT,0x9286,"UserComment"),
            (SubSecTime,0x9290,"SubSecTime"),
            (SubSecTimeOriginal,0x9291,"SubSecTimeOriginal"),
            (SubSecTimeDigitized,0x9292,"SubSecTimeDigitized"),
            (FlashPixVersion,0xA000,"FlashPixVersion"),
            (ColorSpace,0xA001,"ColorSpace"),
            (EXIF_IMAGEWIDTH,0xa002,"ExifImageWidth"),
            (EXIF_IMAGELENGTH,0xa003,"ExifImageLength"),
            (RelatedAudioFile,0xA004,"RelatedAudioFile"),
            (INTEROP_OFFSET,0xa005,"InteroperabilityOffset"),
            (FlashEnergy,0xA20B,"FlashEnergy"),
            (SpatialFrequencyResponse,0xA20C,"SpatialFrequencyResponse"),
            (FOCALPLANEXRES,0xa20E,"FocalPlaneXResolution"),
            (FocalPlaneYResolution,0xA20F,"FocalPlaneYResolution"),
            (FOCALPLANEUNITS,0xa210,"FocalPlaneResolutionUnit"),
            (SubjectLocation,0xA214,"SubjectLocation"),
            (EXPOSURE_INDEX,0xa215,"ExposureIndex"),
            (SensingMethod,0xA217,"SensingMethod"),
            (FileSource,0xA300,"FileSource"),
            (SceneType,0xA301,"SceneType"),
            (CFA_Pattern,0xA302,"CFA Pattern"),
            (CustomRendered,0xa401,"CustomRendered"),
            (ExposureMode,0xa402,"ExposureMode"),
            (WHITEBALANCE,0xa403,"WhiteBalance"),
            (DigitalZoomRatio,0xa404,"DigitalZoomRatio"),
            (FOCALLENGTH_35MM,0xa405,"FocalLengthIn35mmFilm"),
            (SceneCaptureType,0xa406,"SceneCaptureType"),
            (GainControl,0xa407,"GainControl"),
            (Contrast,0xa408,"Contrast"),
            (Saturation, 0xa409, "Saturation"),
            (Sharpness, 0xa40a, "Sharpness"),
            (SubjectDistanceRange, 0xa40c, "SubjectDistanceRange"),
            (UniqueImageID, 0xa420, "UniqueImageID"),
            (UndefinedTag,0xffff,"UndefinedTag")
        ]);
    };

    ($callback:ident, $extra:expr) => {
        $callback!($extra, [
            (InteropIndex,0x0001,"InteropIndex"), 
            (InteropVersion,0x0002,"InteropVersion"),
            (ImageWidth,0x0100,"ImageWidth"),
            (ImageLength,0x0101,"ImageLength"),
            (BitsPerSample,0x0102,"BitsPerSample"),
            (Compression,0x0103,"Compression"),
            (PhotometricInterpretation,0x0106,"PhotometricInterpretation"),
            (FillOrder,0x010A,"FillOrder"),
            (DocumentName,0x010D,"DocumentName"),
            (ImageDescription,0x010E,"ImageDescription"),
            (MAKE,0x010F,"Make"),
            (MODEL,0x0110,"Model"),
            (StripOffsets,0x0111,"StripOffsets"),
            (ORIENTATION,0x0112,"Orientation"),
            (SamplesPerPixel,0x0115,"SamplesPerPixel"),
            (RowsPerStrip,0x0116,"RowsPerStrip"),
            (StripByteCounts,0x0117,"StripByteCounts"),
            (XResolution,0x011A,"XResolution"),
            (YResolution,0x011B,"YResolution"),
            (PlanarConfiguration,0x011C,"PlanarConfiguration"),
            (ResolutionUnit,0x0128,"ResolutionUnit"),
            (TransferFunction,0x012D,"TransferFunction"),
            (Software,0x0131,"Software"),
            (DATETIME,0x0132,"DateTime"),
            (Artist,0x013B,"Artist"),
            (WhitePoint,0x013E,"WhitePoint"),
            (PrimaryChromaticities,0x013F,"PrimaryChromaticities"),
            (TransferRange,0x0156,"TransferRange"),
            (JPEGProc,0x0200,"JPEGProc"),
            (THUMBNAIL_OFFSET,0x0201,"ThumbnailOffset"),
            (THUMBNAIL_LENGTH,0x0202,"ThumbnailLength"),
            (YCbCrCoefficients,0x0211,"YCbCrCoefficients"),
            (YCbCrSubSampling,0x0212,"YCbCrSubSampling"),
            (YCbCrPositioning,0x0213,"YCbCrPositioning"),
            (ReferenceBlackWhite,0x0214,"ReferenceBlackWhite"),
            (RelatedImageWidth,0x1001,"RelatedImageWidth"),
            (RelatedImageLength,0x1002,"RelatedImageLength"),
            (CFARepeatPatternDim,0x828D,"CFARepeatPatternDim"),
            (CFAPattern,0x828E,"CFAPattern"),
            (BatteryLevel,0x828F,"BatteryLevel"),
            (Copyright,0x8298,"Copyright"),
            (EXPOSURETIME,0x829A,"ExposureTime"),
            (FNUMBER,0x829D,"FNumber"),
            (IPTC_NAA,0x83BB,"IPTC/NAA"),
            (EXIF_OFFSET,0x8769,"ExifOffset"),
            (InterColorProfile,0x8773,"InterColorProfile"),
            (EXPOSURE_PROGRAM,0x8822,"ExposureProgram"),
            (SpectralSensitivity,0x8824,"SpectralSensitivity"),
            (GPSInfo,0x8825,"GPSInfo"),
            (ISO_EQUIVALENT,0x8827,"ISOSpeedRatings"),
            (OECF,0x8828,"OECF"),
            (ExifVersion,0x9000,"ExifVersion"),
            (DATETIME_ORIGINAL,0x9003,"DateTimeOriginal"),
            (DATETIME_DIGITIZED,0x9004,"DateTimeDigitized"),
            (ComponentsConfiguration,0x9101,"ComponentsConfiguration"),
            (CompressedBitsPerPixel,0x9102,"CompressedBitsPerPixel"),
            (SHUTTERSPEED,0x9201,"ShutterSpeedValue"),
            (APERTURE,0x9202,"ApertureValue"),
            (BrightnessValue,0x9203,"BrightnessValue"),
            (EXPOSURE_BIAS,0x9204,"ExposureBiasValue"),
            (MAXAPERTURE,0x9205,"MaxApertureValue"),
            (SUBJECT_DISTANCE,0x9206,"SubjectDistance"),
            (METERING_MODE,0x9207,"MeteringMode"),
            (LIGHT_SOURCE,0x9208,"LightSource"),
            (FLASH,0x9209,"Flash"),
            (FOCALLENGTH,0x920A,"FocalLength"),
            (FlashEnergy_,0x920B,"FlashEnergy"),
            (SpatialFrequencyResponse_,0x920C,"SpatialFrequencyResponse"),
            (FOCALPLANEXRES_,0x920E,"FocalPlaneXResolution"),
            (FocalPlaneYResolution_,0x920F,"FocalPlaneYResolution"),
            (FOCALPLANEUNITS_,0x9210,"FocalPlaneResolutionUnit"),
            (SubjectLocation_,0x9214,"SubjectLocation"),
            (EXPOSURE_INDEX_,0x9215,"ExposureIndex"),
            (SensingMethod_,0x9217,"SensingMethod"),
            (MAKER_NOTE,0x927C,"MakerNote"),
            (USERCOMMENT,0x9286,"UserComment"),
            (SubSecTime,0x9290,"SubSecTime"),
            (SubSecTimeOriginal,0x9291,"SubSecTimeOriginal"),
            (SubSecTimeDigitized,0x9292,"SubSecTimeDigitized"),
            (FlashPixVersion,0xA000,"FlashPixVersion"),
            (ColorSpace,0xA001,"ColorSpace"),
            (EXIF_IMAGEWIDTH,0xa002,"ExifImageWidth"),
            (EXIF_IMAGELENGTH,0xa003,"ExifImageLength"),
            (RelatedAudioFile,0xA004,"RelatedAudioFile"),
            (INTEROP_OFFSET,0xa005,"InteroperabilityOffset"),
            (FlashEnergy,0xA20B,"FlashEnergy"),
            (SpatialFrequencyResponse,0xA20C,"SpatialFrequencyResponse"),
            (FOCALPLANEXRES,0xa20E,"FocalPlaneXResolution"),
            (FocalPlaneYResolution,0xA20F,"FocalPlaneYResolution"),
            (FOCALPLANEUNITS,0xa210,"FocalPlaneResolutionUnit"),
            (SubjectLocation,0xA214,"SubjectLocation"),
            (EXPOSURE_INDEX,0xa215,"ExposureIndex"),
            (SensingMethod,0xA217,"SensingMethod"),
            (FileSource,0xA300,"FileSource"),
            (SceneType,0xA301,"SceneType"),
            (CFA_Pattern,0xA302,"CFA Pattern"),
            (CustomRendered,0xa401,"CustomRendered"),
            (ExposureMode,0xa402,"ExposureMode"),
            (WHITEBALANCE,0xa403,"WhiteBalance"),
            (DigitalZoomRatio,0xa404,"DigitalZoomRatio"),
            (FOCALLENGTH_35MM,0xa405,"FocalLengthIn35mmFilm"),
            (SceneCaptureType,0xa406,"SceneCaptureType"),
            (GainControl,0xa407,"GainControl"),
            (Contrast,0xa408,"Contrast"),
            (Saturation, 0xa409, "Saturation"),
            (Sharpness, 0xa40a, "Sharpness"),
            (SubjectDistanceRange, 0xa40c, "SubjectDistanceRange"),
            (UniqueImageID, 0xa420, "UniqueImageID"),
            (UndefinedTag,0xffff,"UndefinedTag")
        ])
    };
}

macro_rules! make_exif_enum {
    ($unused:tt, [ $( ($name:ident, $id:expr, $str:expr) ),* ]) => {
        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
        #[repr(u16)]
        #[allow(dead_code)]
        #[allow(non_camel_case_types)]
        pub enum ExifTagId {
            $( $name = $id ),*
        }
    };
}
apply_exif_tags!{ make_exif_enum }

macro_rules! make_exif_struct {
    ($unused:tt, [ $( ($name:ident, $id:expr, $str:expr) ),* ]) => {

        impl ExifBlock {
            fn init_exif_tags(&mut self) {
                $( self.exif_tags.push( ExifTag{ id: $id, enu: ExifTagId:: $name , name: $str.to_string() }); )*
            }
        }
    };
}
apply_exif_tags!{ make_exif_struct }


/*macro_rules! make_exif_tag {
    ($val:expr, [ $( ($name:ident, $id:expr, $str:expr) ),* ]) => {
        match $val {
            $( $name => Some( ExifTag{ $val, $id, $str} ) )*
            _ => None
        }
    };
}

pub fn get_exif_tag(tag_id: u16) -> Option<ExifTag> {
    apply_exif_tags! { make_exif_tag, tag_id }
}*/


#[macro_export]
macro_rules! apply_gps_tags {
    ($callback:ident) => {
        $callback!((), [
            (0x00u16,VersionID,        "VersionID",        BTyp, 4),
            (0x01u16,LatitudeRef,      "LatitudeRef",      ATyp, 2), // 'N' = North | 'S' = South
            (0x02u16,Latitude,         "Latitude",         RTyp, 3),
            (0x03u16,LongitudeRef,     "LongitudeRef",     ATyp, 2), // 'E' = East | 'W' = West
            (0x04u16,Longitude,        "Longitude",        RTyp, 3),
            (0x05u16,AltitudeRef,      "AltitudeRef",      ATyp, 1), // 0 = Above Sea Level | 1 = Below Sea Level
            (0x06u16,Altitude,         "Altitude",         RTyp, 1),
            (0x07u16,TimeStamp,        "TimeStamp",        RTyp, 3),
            (0x08u16,Satelites,        "Satelites",        ATyp,-1),
            (0x09u16,Status,           "Status",           ATyp, 2), //'A' = Measurement Active | 'V' = Measurement Void
            (0x0au16,MeasureMode,      "MeasureMode",      ATyp, 2), // 2 = 2-Dimensional Measurement | 3 = 3-Dimensional Measurement
            (0x0bu16,DOP,              "DOP",              RTyp, 1),
            (0x0cu16,SpeedRef,         "SpeedRef",         ATyp, 2), // 'K' = km/h | 'M' = mph | 'N' = knots
            (0x0du16,Speed,            "Speed",            RTyp, 1),
            (0x0eu16,TrackRef,         "TrackRef",         ATyp, 2), // 'M' = Magnetic North | 'T' = True North
            (0x0fu16,Track,            "Track",            RTyp, 1),
            (0x10u16,ImgDirectionRef,  "ImgDirectionRef",  ATyp, 2), // 'M' = Magnetic North | 'T' = True North
            (0x11u16,ImgDirection,     "ImgDirection",     RTyp, 1),
            (0x12u16,MapDatum,         "MapDatum",         ATyp,-1),
            (0x13u16,DestLatitudeRef,  "DestLatitudeRef",  ATyp, 2), // 'N' = North | 'S' = South
            (0x14u16,DestLatitude,     "DestLatitude",     RTyp, 3),
            (0x15u16,DestLongitudeRef, "DestLongitudeRef", ATyp, 2), // 'E' = East | 'W' = West
            (0x16u16,DestLongitude,    "DestLongitude",    RTyp, 3),
            (0x17u16,DestBearingRef,   "DestBearingRef",   ATyp, 2), // 'M' = Magnetic North | 'T' = True North
            (0x18u16,DestBearing,      "DestBearing",      RTyp, 1),
            (0x19u16,DestDistanceRef,  "DestDistanceRef",  ATyp, 2), // 'K' = Kilometers | 'M' = Miles | 'N' = Nautical Miles
            (0x1au16,DestDistance,     "DestDistance",     RTyp, 1),
            (0x1bu16,ProcessingMethod, "ProcessingMethod", UTyp,-1), // "GPS", "CELLID", "WLAN" or "MANUAL"
            (0x1cu16,AreaInformation,  "AreaInformation",  UTyp,-1),
            (0x1du16,DateStamp,        "DateStamp",        ATyp,11), // Format is YYYY:mm:dd
            (0x1eu16,Differential,     "Differential",     STyp, 2), // 0 = No Correction | 1 = Differential Corrected
            (0x1fu16,HPositioningError,"HPositioningError",RTyp, 1)
        ]);
    };

    ($callback:ident, $extra:expr) => {
        $callback!($extra, [
            (0x00u16,VersionID,        "VersionID",        BTyp, 4),
            (0x01u16,LatitudeRef,      "LatitudeRef",      ATyp, 2), // 'N' = North | 'S' = South
            (0x02u16,Latitude,         "Latitude",         RTyp, 3),
            (0x03u16,LongitudeRef,     "LongitudeRef",     ATyp, 2), // 'E' = East | 'W' = West
            (0x04u16,Longitude,        "Longitude",        RTyp, 3),
            (0x05u16,AltitudeRef,      "AltitudeRef",      ATyp, 1), // 0 = Above Sea Level | 1 = Below Sea Level
            (0x06u16,Altitude,         "Altitude",         RTyp, 1),
            (0x07u16,TimeStamp,        "TimeStamp",        RTyp, 3),
            (0x08u16,Satelites,        "Satelites",        ATyp,-1),
            (0x09u16,Status,           "Status",           ATyp, 2), //'A' = Measurement Active | 'V' = Measurement Void
            (0x0au16,MeasureMode,      "MeasureMode",      ATyp, 2), // 2 = 2-Dimensional Measurement | 3 = 3-Dimensional Measurement
            (0x0bu16,DOP,              "DOP",              RTyp, 1),
            (0x0cu16,SpeedRef,         "SpeedRef",         ATyp, 2), // 'K' = km/h | 'M' = mph | 'N' = knots
            (0x0du16,Speed,            "Speed",            RTyp, 1),
            (0x0eu16,TrackRef,         "TrackRef",         ATyp, 2), // 'M' = Magnetic North | 'T' = True North
            (0x0fu16,Track,            "Track",            RTyp, 1),
            (0x10u16,ImgDirectionRef,  "ImgDirectionRef",  ATyp, 2), // 'M' = Magnetic North | 'T' = True North
            (0x11u16,ImgDirection,     "ImgDirection",     RTyp, 1),
            (0x12u16,MapDatum,         "MapDatum",         ATyp,-1),
            (0x13u16,DestLatitudeRef,  "DestLatitudeRef",  ATyp, 2), // 'N' = North | 'S' = South
            (0x14u16,DestLatitude,     "DestLatitude",     RTyp, 3),
            (0x15u16,DestLongitudeRef, "DestLongitudeRef", ATyp, 2), // 'E' = East | 'W' = West
            (0x16u16,DestLongitude,    "DestLongitude",    RTyp, 3),
            (0x17u16,DestBearingRef,   "DestBearingRef",   ATyp, 2), // 'M' = Magnetic North | 'T' = True North
            (0x18u16,DestBearing,      "DestBearing",      RTyp, 1),
            (0x19u16,DestDistanceRef,  "DestDistanceRef",  ATyp, 2), // 'K' = Kilometers | 'M' = Miles | 'N' = Nautical Miles
            (0x1au16,DestDistance,     "DestDistance",     RTyp, 1),
            (0x1bu16,ProcessingMethod, "ProcessingMethod", UTyp,-1), // "GPS", "CELLID", "WLAN" or "MANUAL"
            (0x1cu16,AreaInformation,  "AreaInformation",  UTyp,-1),
            (0x1du16,DateStamp,        "DateStamp",        ATyp,11), // Format is YYYY:mm:dd
            (0x1eu16,Differential,     "Differential",     STyp, 2), // 0 = No Correction | 1 = Differential Corrected
            (0x1fu16,HPositioningError,"HPositioningError",RTyp, 1)
        ])
    };
}

macro_rules! make_gps_enum {
    ($unused:tt, [ $( ($id:expr ,$name:ident, $str:expr, $rtyp:ident, $len:expr) ),* ]) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        #[repr(u16)]
        #[allow(dead_code)]
        #[allow(non_camel_case_types)]
        pub enum GpsTagId {
            $( $name = $id ),*
        }
    };
}
apply_gps_tags!{ make_gps_enum }

macro_rules! make_gps_struct {
    ($unused:tt, [ $( ($id:expr ,$name:ident, $str:expr, $rtyp:ident, $len:expr) ),* ]) => {

        impl GpsBlock {
            fn init_gps_tags(&mut self) {
                $( self.gps_tags.push( GpsTag{ id: $id, enu: GpsTagId:: $name, name: $str.to_string(), rtyp: Rtype:: $rtyp, len: $len }); )*
            }
        }
    };
}
apply_gps_tags!{ make_gps_struct }



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Rtype {
    RTyp,
    ATyp,
    BTyp,
    STyp,
    UTyp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExifTag {
    pub id: u16,
    pub enu: ExifTagId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialOrd, PartialEq)]
#[allow(non_camel_case_types)]
pub enum FMT {
   NONE,
   BYTE,
   STRING,
   USHORT,
   ULONG,
   URATIONAL,
   SBYTE,
   UNDEFINED,
   SSHORT,
   SLONG,
   SRATIONAL,
   SINGLE,
   DOUBLE,
   NUM_FORMATS,
}
impl FMT {
    fn from(v: u16) -> Self {
        match v {
            0 => FMT::NONE,
            1 => FMT::BYTE,
            2 => FMT::STRING,
            3 => FMT::USHORT,
            4 => FMT::ULONG,
            5 => FMT::URATIONAL,
            6 => FMT::SBYTE,
            7 => FMT::UNDEFINED,
            8 => FMT::SSHORT,
            9 => FMT::SLONG,
           10 => FMT::SRATIONAL,
           11 => FMT::SINGLE,
           12 => FMT::DOUBLE,
            _ => FMT::NUM_FORMATS,
        }
    }
}

const bytesperformat: [usize; 13] = [0,1,1,2,4,8,1,1,2,4,8,4,8];
const           asciiformat: [i32; 13] = [0,0,1,0,0,0,0,1,0,0,0,0,0];

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct ExifBlock {
    exif_tags: Vec<ExifTag>,
    raw_exif: Vec<u8>,
    json_data: Option<Map<String, Value>>,
    lastexifrefd: usize,
    dirwiththumbnailptrs: usize,
    focalplanexres :f64,
    focalplaneunits: f64,
    exifimagewidth: usize,
    motorola_order: bool,
    nesting_level: i32,
}


impl Default for ExifBlock {
    fn default() -> Self {
        let mut tmp = Self {
            exif_tags: Vec::new(),
            raw_exif: Vec::new(),
            json_data: None,
            lastexifrefd: 0,
            dirwiththumbnailptrs: 0,
            focalplanexres :0.0,
            focalplaneunits: 0.0,
            exifimagewidth: 0,
            motorola_order: false, //true: MM Big-endian, false: II Little-endian
            nesting_level: 0,
        };
        tmp.init_exif_tags();
        tmp
    }
}


impl ExifBlock {
    
    pub fn get_name(&self, id : u16) -> Option<&String> {
        for tag in &self.exif_tags {
            if tag.id == id { return Some(&tag.name); }
        }
        None
    }
    
    pub fn get_tag(&self, id : u16) -> Option<&ExifTag> {
        self.exif_tags.iter().find(|t| t.id == id)
    }
    
    fn read_buff_u16(&self, buff :&[u8], pos: usize) -> u16 {
        let bytes = buff[pos..pos + 2].try_into().unwrap();
        if self.motorola_order { u16::from_be_bytes(bytes) }
        else { u16::from_le_bytes(bytes) }
    }
    
    fn read_u16(&self, pos: usize) -> u16 {
        let bytes = self.raw_exif[pos..pos + 2].try_into().unwrap();
        if self.motorola_order { u16::from_be_bytes(bytes) }
        else { u16::from_le_bytes(bytes) }
    }
    
    fn read_u32(&self, pos: usize) -> u32 {
        let bytes = self.raw_exif[pos..pos + 4].try_into().unwrap();
        if self.motorola_order { u32::from_be_bytes(bytes) }
        else { u32::from_le_bytes(bytes) }
    }
    
    fn read_i32(&self, pos: usize) -> i32 {
        let bytes = self.raw_exif[pos..pos + 4].try_into().unwrap();
        if self.motorola_order { i32::from_be_bytes(bytes) }
        else { i32::from_le_bytes(bytes) }
    }
    
    fn read_f32(&self, pos: usize) -> f32 {
        let bytes = self.raw_exif[pos..pos + 4].try_into().unwrap();
        if self.motorola_order { f32::from_be_bytes(bytes) }
        else { f32::from_le_bytes(bytes) }
    }
    
    fn read_f64(&self, pos: usize) -> f64 {
        let bytes = self.raw_exif[pos..pos + 4].try_into().unwrap();
        if self.motorola_order { f64::from_be_bytes(bytes) }
        else { f64::from_le_bytes(bytes) }
    }

    fn convert_format_usize(&self, valueptr: usize, format:& FMT) -> usize {
       match format {
            FMT::BYTE   => (self.raw_exif[valueptr] as u8) as usize,
            FMT::SBYTE  => (self.raw_exif[valueptr] as i8) as usize,
            FMT::USHORT => (self.read_u16(valueptr)) as usize,
            FMT::SSHORT => (self.read_u16(valueptr) as i16) as usize,
            FMT::ULONG  => (self.read_u32(valueptr)) as usize,
            FMT::SLONG  => (self.read_i32(valueptr)) as usize,
            _ => (0) as usize, 
        }
    }


    pub fn open(&mut self, exifsection: &[u8],  length: usize) -> Result<ExifBlock, String> {
        let exifheader: [u8; 6] = [b'E',b'x',b'i',b'f',0,0];
        if exifsection[0..6] != exifheader {
            return Err("No exif header".into());
        }
        let motorola: [u8; 2] = [b'M',b'M'];
        let intel: [u8; 2] = [b'I',b'I'];
        if exifsection[6..8] == motorola {
            self.motorola_order = true;
        } else if exifsection[6..8] == intel {
            self.motorola_order = false;
        }else{
            return Err("Corrupt exif header: Invalid Exif alignment marker".into());
        }

        if self.read_buff_u16(&exifsection,8) != 0x2a {
            return Err("Corrupt exif header: Invalid Exif start (1)".into())
        }

        let firstoffset = self.read_buff_u16(&exifsection,10) as usize;
        if firstoffset < 8 || firstoffset > 32000 {
            return Err("Corrupt exif header: Suspicious offset of first IFD value".into());
        }

        self.raw_exif = exifsection.to_vec();
        self.lastexifrefd = 0;
        self.dirwiththumbnailptrs = 0;
        // First directory starts 16 bytes in.  All offset are relative to 8 bytes in.
        self.nesting_level+=1;
        let json = self.process_exif_dir(firstoffset+6, 6, length-6)?;
        self.nesting_level-=1;
        let mut parent = Map::new();
        parent.insert("ExifHeader".to_string(), json!(json));
        self.json_data = Some(parent);
        Ok(self.clone())
    }


    fn dir_entry_addr(start: &usize, entry: &usize) -> usize {
        start + 2 + 12 * entry
    }


    fn process_exif_dir(&mut self, dirstart: usize, offsetbase: usize, exiflength: usize) ->  Result<Map<String, Value>, String> {
        let mut thumbnailoffset : usize = 0;
        let mut thumbnailsize : usize = 0;
        let mut make : String;

        let numdirentries = self.read_u16(dirstart) as usize;
        if self.nesting_level > 4 {
            return Err("Corrupt exif header: Maximum directory nesting exceeded".into());
        }

        let dirend = Self::dir_entry_addr(&dirstart, &numdirentries);
        if dirend+4 > offsetbase+exiflength {
            if dirend+2 == offsetbase+exiflength || dirend == offsetbase+exiflength {
                // version 1.3 of jhead would truncate a bit too much.
                // this also caught later on as well.
            }else{
                // note: files that had thumbnails trimmed with jhead 1.3 or earlier
                // might trigger this.
                return Err("Corrupt exif header: Illegally sized directory".into());
            }
        }
        if dirend > self.lastexifrefd { self.lastexifrefd = dirend; }

        let mut result = Map::new();

        for de_idx in 0..numdirentries {
            let idx = de_idx as usize;
            let direntry = Self::dir_entry_addr(&dirstart, &idx);
            
            let tag = self.get_tag(self.read_u16(direntry))
                .unwrap_or_else(|| self.get_tag(0xffff).expect("Undefined tag missing"));
            let format = FMT::from(self.read_u16(direntry+2));
            if format >= FMT::NUM_FORMATS {
                return Err(format!("Corrupt exif header: Illegal number format {:?} for tag {:?}", format, tag.name));
            }
            let components = self.read_u32(direntry+4) as usize;
            let bytecount = components * bytesperformat[format.clone() as usize];
            
            let mut json_tag: Map<String, Value> = Map::new();
            json_tag.insert("type".to_string(),json!(format));
            json_tag.insert("count".to_string(),json!(components));
            
            let valueptr = if bytecount > 4 {
                // if its bigger than 4 bytes, the dir entry contains an offset.
                let offsetval = self.read_u32(direntry+8) as usize;
                if offsetval+bytecount > exiflength {
                    return Err(format!("Corrupt exif header: Illegal value pointer for tag {:?}",tag.name));
                }
                offsetbase+offsetval
            }else{
                // 4 bytes or less and value is in the dir entry itself
                direntry+8
            };

            if self.lastexifrefd < valueptr+bytecount {
                // keep track of last byte in the exif header that was actually referenced.
                // that way, we know where the discardable thumbnail data begins.
                self.lastexifrefd = valueptr+bytecount;
            }

            match tag.enu.clone() {
                ExifTagId::GPSInfo => {
                        let subdirstart = offsetbase + self.read_u32(valueptr) as usize;
                        if subdirstart < offsetbase || subdirstart > offsetbase+exiflength {
                            return Err("Corrupt exif header: Illegal exif or interop ofset directory link".into());
                        }else{
                            self.nesting_level+=1;
                            let json = self.process_gps_info(subdirstart, offsetbase, exiflength)?;
                            self.nesting_level-=1;
                            result.insert("GPSInfo".to_string(), json!(json));
                        }
                        continue;
                    },
                ExifTagId::THUMBNAIL_OFFSET => {
                        thumbnailoffset = self.convert_format_usize(valueptr, &format);
                        self.dirwiththumbnailptrs = dirstart;
                    },
                ExifTagId::THUMBNAIL_LENGTH => {
                        thumbnailsize = self.convert_format_usize(valueptr, &format);
                    },
                ExifTagId::EXIF_OFFSET | ExifTagId::INTEROP_OFFSET => {
                        let subdirstart = offsetbase + self.read_u32(valueptr) as usize;
                        if subdirstart < offsetbase || subdirstart > offsetbase+exiflength {
                            return Err("Corrupt exif header: Illegal exif or interop offset directory link".into());
                        }else{
                            self.nesting_level+=1;
                            self.process_exif_dir(subdirstart, offsetbase, exiflength);
                            self.nesting_level-=1;
                        }
                        continue;
                    },
                 _ => {},
                }

            match format {
                FMT::UNDEFINED | FMT::STRING =>{
                    let raw_bytes = &self.raw_exif[valueptr..valueptr + bytecount];
                    let clean_bytes = raw_bytes.split(|&b| b == 0).next().unwrap_or(&[]);
                    let text = String::from_utf8_lossy(clean_bytes).to_string();
                    json_tag.insert("val".to_string(), json!(text));
                    result.insert(tag.name.clone(), json!(json_tag));
                    if tag.enu.clone() == ExifTagId::MAKE {
                        make = text;
                    }
                    continue;
                },
                FMT::BYTE   => {
                    //if tag.enu == ExifTagId::MAKER_NOTE && maker == "Canon" {
                    //    let json = process_maker_note(valueptr, bytecount, offsetbase, exiflength)?;
                    //    result.insert("MAKER_NOTE".to_string(), json!(json));
                    //    continue;
                    //}
                    let raw_bytes = &self.raw_exif[valueptr..valueptr + bytecount];
                    let value = if bytecount<=120 { json!(raw_bytes) }
                    else {
                        json!(general_purpose::STANDARD.encode(raw_bytes))
                    };
                    json_tag.insert("val".to_string(), value);
                    result.insert(tag.name.clone(), json!(json_tag));
                    continue;
                },
                FMT::SBYTE  => {
                    let value = if bytecount<=120 {
                        let signed_bytes: Vec<i8> = self.raw_exif[valueptr..valueptr + bytecount]
                            .iter().map(|&b| b as i8).collect();
                        json!(signed_bytes)
                    }
                    else {
                        let raw_bytes = &self.raw_exif[valueptr..valueptr + bytecount];
                        json!(general_purpose::STANDARD.encode(raw_bytes))
                    };
                    json_tag.insert("val".to_string(), value);
                    result.insert(tag.name.clone(), json!(json_tag));
                    continue;
                    }
                FMT::USHORT => {
                    json_tag.insert("val".to_string(), json!(self.read_u16(valueptr)));
                    result.insert(tag.name.clone(),  json!(json_tag));
                    continue;
                    }
                FMT::SSHORT => {
                    json_tag.insert("val".to_string(), json!(self.read_u16(valueptr) as i16));
                    result.insert(tag.name.clone(),  json!(json_tag));
                    continue;
                    }
                FMT::ULONG  => {
                    json_tag.insert("val".to_string(), json!(self.read_u32(valueptr)));
                    result.insert(tag.name.clone(),  json!(json_tag));
                    continue;
                    }
                FMT::SLONG  => {
                    json_tag.insert("val".to_string(), json!(self.read_i32(valueptr)));
                    result.insert(tag.name.clone(),  json!(json_tag));
                    continue;
                    }
                FMT::URATIONAL | FMT::SRATIONAL => {
                    let num = self.read_u32(valueptr);
                    let den = self.read_u32(valueptr + 4);
                    json_tag.insert("val".to_string(), json!([num, den]));
                    result.insert(tag.name.clone(), json!(json_tag));
                    continue;
                },
                FMT::SINGLE => {
                    json_tag.insert("val".to_string(), json!(self.read_f32(valueptr)));
                    result.insert(tag.name.clone(), json!(json_tag));
                    continue;
                    }
                FMT::DOUBLE => {
                    json_tag.insert("val".to_string(), json!(self.read_f64(valueptr)));
                    result.insert(tag.name.clone(), json!(json_tag));
                    continue;
                    }
                _ => json!(null), // Ismeretlen formátum esetén
            };

        }

        // In addition to linking to subdirectories via exif tags,
        // there's also a potential link to another directory at the end of each
        // directory.  this has got to be the result of a comitee!
        if Self::dir_entry_addr(&dirstart, &numdirentries) + 4 <= offsetbase+exiflength {
             let offset = self.read_u32(dirstart+2+12*numdirentries) as usize;
             if offset != 0 {
                let subdirstart = offsetbase + offset;
                if subdirstart > offsetbase+exiflength {
                   /*if (subdirstart < offsetbase+exiflength+20){
                      // jhead 1.3 or earlier would crop the whole directory!
                      // as jhead produces this form of format incorrectness,
                      // i'll just let it pass silently
                      inf->exiftext("thumbnail removed with jhead 1.3 or earlier\n");
                   }else{
                      warnms(cinfo,jwrn_exif_8);
                   }*/
                } else {
                   if subdirstart <= offsetbase+exiflength {
                      //inf->exiftext("%*ccontinued ",level*4,' ');
                      self.nesting_level+=1;
                      self.process_exif_dir(subdirstart, offsetbase, exiflength);
                      self.nesting_level-=1;
                   }
                }
             }
        } else {
             // The exif header ends before the last next directory pointer.
        }
        
        if thumbnailsize != 0 && thumbnailoffset != 0 {
            if thumbnailsize + thumbnailoffset <= exiflength {
                // the thumbnail pointer appears to be valid.  store it.
                //inf->exifinfo.thumbnailpointer = offsetbase + thumbnailoffset;
                //inf->exifinfo.thumbnailsize = thumbnailsize;
                //inf->ExifText("%*cThumbnail size [ %d bytes ]\n",level*4,' ',ThumbnailSize);
            }
        }
        Ok(result)
    }
    
    //fn PrintFormatNumber(&mut self,valueptr: usize, format: FMT, bytecount: i32) {}
    fn process_gps_info(&mut self, dirstart: usize, offsetbase: usize, exiflength: usize) ->  Result<Map<String, Value>, String>  {
        let mut result = Map::new();
        Ok(result)
    }

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GpsTag {
    pub id: u16,
    pub enu: GpsTagId,
    pub name: String,
    pub rtyp: Rtype,
    pub len: i8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GpsBlock {
    pub gps_tags: Vec<GpsTag>,
}

impl Default for GpsBlock {
    fn default() -> Self {
        let mut tmp = Self {
            gps_tags: Vec::new(),
        };
        tmp.init_gps_tags();
        tmp
    }
}

impl GpsBlock {
    pub fn GetName(&self, id : u16) -> Option<&String> {
        for tag in &self.gps_tags {
            if tag.id == id { return Some(&tag.name); }
        }
        None
    }
}

/*
JMESSAGE(JWRN_EXIF_1, "Corrupt exif header: Maximum directory nesting exceeded)")
JMESSAGE(JWRN_EXIF_2, "Corrupt exif header: Illegally sized directory")
JMESSAGE(JWRN_EXIF_3, "Corrupt exif header: Illegal number format %d for tag %04x")
JMESSAGE(JWRN_EXIF_4, "Corrupt exif header: Illegal value pointer for tag %04x")
JMESSAGE(JWRN_EXIF_5, "Corrupt exif header: More than %d date fields!  This is nuts")
JMESSAGE(JWRN_EXIF_6, "Corrupt exif header: Undefined rotation value %d")
JMESSAGE(JWRN_EXIF_7, "Corrupt exif header: Illegal exif or interop ofset directory link")
JMESSAGE(JWRN_EXIF_8, "Corrupt exif header: Illegal subdirectory link")
JMESSAGE(JWRN_EXIF_9, "Incorrect exif header")
JMESSAGE(JWRN_EXIF_10, "Corrupt exif header: Invalid Exif alignment marker")
JMESSAGE(JWRN_EXIF_11, "Corrupt exif header: Invalid Exif start (1)")
JMESSAGE(JWRN_EXIF_12, "Corrupt exif header: Suspicious offset of first IFD value")


fn main() {
    // 1. Létrehozunk egy üres JSON objektumot (Map)
    let mut exif_data = Map::new();

    // 2. Értékek hozzáadása (a bejárás során)
    // A kulcs String, az érték serde_json::Value
    exif_data.insert("Make".to_string(), json!("Canon"));
    exif_data.insert("ISO".to_string(), json!(100));
    
    // RATIONAL típus kezelése tömbként
    exif_data.insert("ExposureTime".to_string(), json!([1, 250]));

    // 3. Ha van egy GPS blokkod, azt is beágyazhatod
    let mut gps_map = Map::new();
    gps_map.insert("Latitude".to_string(), json!(47.4979));
    exif_data.insert("GPS".to_string(), Value::Object(gps_map));

    // 4. Végső Value létrehozása a Map-ből
    let root = Value::Object(exif_data);

    // 5. Kiíratás String-be (szépen formázva)
    let json_string = serde_json::to_string_pretty(&root).unwrap();
    println!("{}", json_string);
}


let values = vec![8, 8, 8];
exif_data.insert("BitsPerSample".to_string(), json!(values));


if let Some(tag_name) = get_tag_name(current_id) {
    let value_to_insert = match current_type {
        2 => json!(read_string(p)), // ASCII
        3 => json!(read_u16(p)),    // SHORT
        5 => json!([read_u32(p), read_u32(p+4)]), // RATIONAL
        _ => json!(null),
    };
    exif_map.insert(tag_name, value_to_insert);
}


pub struct ExifParser {
    // Ez tárolja a JSON objektum elemeit
    pub data: Map<String, Value>,
}

impl ExifParser {
    pub fn new() -> Self {
        Self {
            data: Map::new(),
        }
    }

    // Példa egy mező hozzáadására a bejárás közben
    pub fn add_entry(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }

    // A végén így kapod meg a kész JSON-t
    pub fn build_json(self) -> Value {
        Value::Object(self.data)
    }
}


let mut gps_subtree = Map::new();
gps_subtree.insert("Latitude".to_string(), serde_json::json!(47.123));
gps_subtree.insert("Longitude".to_string(), serde_json::json!(19.456));

// A fő struktúrába való beillesztés:
parser.data.insert("GPS".to_string(), Value::Object(gps_subtree));



let mut values = Vec::new();
for i in 0..count {
    values.push(read_value(i));
}
exif_map.insert(tag_name, json!(values)); // Ez automatikusan Array lesz a JSON-ban




// Feltételezve, hogy a 'tag' tartalmazza: id, type_code, count, valueptr
let tag_name = get_tag_name(tag.id).unwrap_or_else(|| format!("0x{:04X}", tag.id));

// Az érték feldolgozása (a korábbi logikád alapján)
let processed_value = match tag.type_code {
    2 => { // ASCII
        let bytes = &self.raw_exif[tag.valueptr .. tag.valueptr + tag.count as usize];
        json!(String::from_utf8_lossy(bytes.split(|&b| b == 0).next().unwrap_or(&[])))
    },
    5 | 10 => { // RATIONAL / SRATIONAL
        let num = self.read_u32(tag.valueptr);
        let den = self.read_u32(tag.valueptr + 4);
        json!([num, den])
    },
    1 | 6 => { // BYTE / SBYTE
        let raw_bytes = &self.raw_exif[tag.valueptr .. tag.valueptr + tag.count as usize];
        if tag.count > 120 {
            json!(general_purpose::STANDARD.encode(raw_bytes))
        } else {
            json!(raw_bytes.iter().map(|&b| b as i8).collect::<Vec<_>>())
        }
    },
    // ... többi típus (SHORT, LONG stb.)
    _ => json!(null),
};

// A struktúra felépítése, ami tartalmazza a típust is
let entry = json!({
    "type": tag.type_code,
    "count": tag.count,
    "val": processed_value
});

// Beszúrás a fő Map-be
result.insert(tag_name, entry);


*/