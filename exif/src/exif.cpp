//--------------------------------------------------------------------------
// Program to pull the information out of various types of EXIF digital
// camera files and show it in a reasonably consistent way
//
// This module parses the very complicated exif structures.
//
// Matthias Wandel,  Dec 1999 - Dec 2002
//--------------------------------------------------------------------------
#define JPEG_INTERNALS
#define _CRT_SECURE_NO_WARNINGS
#include "jinclude.h"
#include "jpeglib.h"
#include <math.h>

typedef unsigned char byte;
typedef unsigned short int word;
typedef unsigned int dword;

#ifndef TRUE
   #define TRUE 1
   #define FALSE 0
#endif

#include "..//jpt//exif.h"

namespace glob {
class cGlobalLevel {
private:
   int nest;
public:
   void operator =(int a) {  nest=a; }
   int operator++() {  nest++; return nest<=20;}
   void operator--() {  nest--; }
   operator int() {return nest<0?0:(nest>20?20:nest);}
   cGlobalLevel(){nest=0;}
};
};
static glob::cGlobalLevel level;

class Level {
public:
   inline Level(){ ++level; }
   inline ~Level(){ --level; }
};


enum FMT {
   FMT_NONE,
   FMT_BYTE,
   FMT_STRING,
   FMT_USHORT,
   FMT_ULONG,
   FMT_URATIONAL,
   FMT_SBYTE,
   FMT_UNDEFINED,
   FMT_SSHORT,
   FMT_SLONG,
   FMT_SRATIONAL,
   FMT_SINGLE,
   FMT_DOUBLE,
   NUM_FORMATS
};
int BytesPerFormat[NUM_FORMATS] = {0,1,1,2,4,8,1,1,2,4,8,4,8};
int    AsciiFormat[NUM_FORMATS] = {0,0,1,0,0,0,0,1,0,0,0,0,0};

struct exif {
   bool open(byte *ExifSection,unsigned int length,j_decompress_ptr cinfo);
   void PrintFormatNumber(void * ValuePtr, FMT Format, int ByteCount);
   double ConvertAnyFormat(void * ValuePtr, FMT Format);
   void ProcessExifDir(byte * DirStart, byte * OffsetBase, unsigned ExifLength);
   void ProcessGPSInfo(byte * DirStart, byte * OffsetBase, unsigned ExifLength);
   void ProcessMakerNote(byte * ValuePtr, int ByteCount, byte * OffsetBase, unsigned ExifLength);
   void ProcessCannonMakerNoteDir(byte * DirStart, byte * OffsetBase, unsigned ExifLength);
   void ShowMakerNoteGeneric(byte * ValuePtr, int ByteCount);

   inline word Get16u(word*i) { if( !MotorolaOrder ) return *i;
      union wb { byte b[2]; word w; };
      wb &w=*(wb*)i; register wb o = {{w.b[1],w.b[0]}}; return o.w;
   }
   inline int Get32s(int*i) { if( !MotorolaOrder ) return *i;
      union db { byte b[4]; int d; };
      db &d=*(db*)i; register db o = {{d.b[3],d.b[2],d.b[1],d.b[0]}}; return o.d;
   }
   inline dword Get32u(dword*i) { if( !MotorolaOrder ) return *i;
      union db { byte b[4]; dword d; };
      db &d=*(db*)i; register db o = {{d.b[3],d.b[2],d.b[1],d.b[0]}}; return o.d;
   }
   inline byte* DIR_ENTRY_ADDR(byte* Start, int Entry) {
      return Start+2+12*Entry;
   }
   jFileInfo *inf;
   j_decompress_ptr cinfo;
   byte * LastExifRefd;
   byte * DirWithThumbnailPtrs;
   double FocalplaneXRes;
   double FocalplaneUnits;
   int ExifImageWidth;
   int MotorolaOrder;

};
//--------------------------------------------------------------------------
// Describes tag values

#define EXIFTAGS \
   EXIF(InteropIndex,0x0001,"InteropIndex")  \
   EXIF(InteropVersion,0x0002,"InteropVersion") \
   EXIF(ImageWidth,0x0100,"ImageWidth") \
   EXIF(ImageLength,0x0101,"ImageLength") \
   EXIF(BitsPerSample,0x0102,"BitsPerSample") \
   EXIF(Compression,0x0103,"Compression") \
   EXIF(PhotometricInterpretation,0x0106,"PhotometricInterpretation") \
   EXIF(FillOrder,0x010A,"FillOrder") \
   EXIF(DocumentName,0x010D,"DocumentName") \
   EXIF(ImageDescription,0x010E,"ImageDescription") \
   EXIF(MAKE,0x010F,"Make") \
   EXIF(MODEL,0x0110,"Model") \
   EXIF(StripOffsets,0x0111,"StripOffsets") \
   EXIF(ORIENTATION,0x0112,"Orientation") \
   EXIF(SamplesPerPixel,0x0115,"SamplesPerPixel") \
   EXIF(RowsPerStrip,0x0116,"RowsPerStrip") \
   EXIF(StripByteCounts,0x0117,"StripByteCounts") \
   EXIF(XResolution,0x011A,"XResolution") \
   EXIF(YResolution,0x011B,"YResolution") \
   EXIF(PlanarConfiguration,0x011C,"PlanarConfiguration") \
   EXIF(ResolutionUnit,0x0128,"ResolutionUnit") \
   EXIF(TransferFunction,0x012D,"TransferFunction") \
   EXIF(Software,0x0131,"Software") \
   EXIF(DATETIME,0x0132,"DateTime") \
   EXIF(Artist,0x013B,"Artist") \
   EXIF(WhitePoint,0x013E,"WhitePoint") \
   EXIF(PrimaryChromaticities,0x013F,"PrimaryChromaticities") \
   EXIF(TransferRange,0x0156,"TransferRange") \
   EXIF(JPEGProc,0x0200,"JPEGProc") \
   EXIF(THUMBNAIL_OFFSET,0x0201,"ThumbnailOffset") \
   EXIF(THUMBNAIL_LENGTH,0x0202,"ThumbnailLength") \
   EXIF(YCbCrCoefficients,0x0211,"YCbCrCoefficients") \
   EXIF(YCbCrSubSampling,0x0212,"YCbCrSubSampling") \
   EXIF(YCbCrPositioning,0x0213,"YCbCrPositioning") \
   EXIF(ReferenceBlackWhite,0x0214,"ReferenceBlackWhite") \
   EXIF(RelatedImageWidth,0x1001,"RelatedImageWidth") \
   EXIF(RelatedImageLength,0x1002,"RelatedImageLength") \
   EXIF(CFARepeatPatternDim,0x828D,"CFARepeatPatternDim") \
   EXIF(CFAPattern,0x828E,"CFAPattern") \
   EXIF(BatteryLevel,0x828F,"BatteryLevel") \
   EXIF(Copyright,0x8298,"Copyright") \
   EXIF(EXPOSURETIME,0x829A,"ExposureTime") \
   EXIF(FNUMBER,0x829D,"FNumber") \
   EXIF(IPTC_NAA,0x83BB,"IPTC/NAA") \
   EXIF(EXIF_OFFSET,0x8769,"ExifOffset") \
   EXIF(InterColorProfile,0x8773,"InterColorProfile") \
   EXIF(EXPOSURE_PROGRAM,0x8822,"ExposureProgram") \
   EXIF(SpectralSensitivity,0x8824,"SpectralSensitivity") \
   EXIF(GPSInfo,0x8825,"GPSInfo") \
   EXIF(ISO_EQUIVALENT,0x8827,"ISOSpeedRatings") \
   EXIF(OECF,0x8828,"OECF") \
   EXIF(ExifVersion,0x9000,"ExifVersion") \
   EXIF(DATETIME_ORIGINAL,0x9003,"DateTimeOriginal") \
   EXIF(DATETIME_DIGITIZED,0x9004,"DateTimeDigitized") \
   EXIF(ComponentsConfiguration,0x9101,"ComponentsConfiguration") \
   EXIF(CompressedBitsPerPixel,0x9102,"CompressedBitsPerPixel") \
   EXIF(SHUTTERSPEED,0x9201,"ShutterSpeedValue") \
   EXIF(APERTURE,0x9202,"ApertureValue") \
   EXIF(BrightnessValue,0x9203,"BrightnessValue") \
   EXIF(EXPOSURE_BIAS,0x9204,"ExposureBiasValue") \
   EXIF(MAXAPERTURE,0x9205,"MaxApertureValue") \
   EXIF(SUBJECT_DISTANCE,0x9206,"SubjectDistance") \
   EXIF(METERING_MODE,0x9207,"MeteringMode") \
   EXIF(LIGHT_SOURCE,0x9208,"LightSource") \
   EXIF(FLASH,0x9209,"Flash") \
   EXIF(FOCALLENGTH,0x920A,"FocalLength") \
   EXIF(FlashEnergy_,0x920B,"FlashEnergy") \
   EXIF(SpatialFrequencyResponse_,0x920C,"SpatialFrequencyResponse") \
   EXIF(FOCALPLANEXRES_,0x920E,"FocalPlaneXResolution") \
   EXIF(FocalPlaneYResolution_,0x920F,"FocalPlaneYResolution") \
   EXIF(FOCALPLANEUNITS_,0x9210,"FocalPlaneResolutionUnit") \
   EXIF(SubjectLocation_,0x9214,"SubjectLocation") \
   EXIF(EXPOSURE_INDEX_,0x9215,"ExposureIndex") \
   EXIF(SensingMethod_,0x9217,"SensingMethod") \
   EXIF(MAKER_NOTE,0x927C,"MakerNote") \
   EXIF(USERCOMMENT,0x9286,"UserComment") \
   EXIF(SubSecTime,0x9290,"SubSecTime") \
   EXIF(SubSecTimeOriginal,0x9291,"SubSecTimeOriginal") \
   EXIF(SubSecTimeDigitized,0x9292,"SubSecTimeDigitized") \
   EXIF(FlashPixVersion,0xA000,"FlashPixVersion") \
   EXIF(ColorSpace,0xA001,"ColorSpace") \
   EXIF(EXIF_IMAGEWIDTH,0xa002,"ExifImageWidth") \
   EXIF(EXIF_IMAGELENGTH,0xa003,"ExifImageLength") \
   EXIF(RelatedAudioFile,0xA004,"RelatedAudioFile") \
   EXIF(INTEROP_OFFSET,0xa005,"InteroperabilityOffset") \
   EXIF(FlashEnergy,0xA20B,"FlashEnergy") \
   EXIF(SpatialFrequencyResponse,0xA20C,"SpatialFrequencyResponse") \
   EXIF(FOCALPLANEXRES,0xa20E,"FocalPlaneXResolution") \
   EXIF(FocalPlaneYResolution,0xA20F,"FocalPlaneYResolution") \
   EXIF(FOCALPLANEUNITS,0xa210,"FocalPlaneResolutionUnit") \
   EXIF(SubjectLocation,0xA214,"SubjectLocation") \
   EXIF(EXPOSURE_INDEX,0xa215,"ExposureIndex") \
   EXIF(SensingMethod,0xA217,"SensingMethod") \
   EXIF(FileSource,0xA300,"FileSource") \
   EXIF(SceneType,0xA301,"SceneType") \
   EXIF(CFA_Pattern,0xA302,"CFA Pattern") \
   EXIF(CustomRendered,0xa401,"CustomRendered") \
   EXIF(ExposureMode,0xa402,"ExposureMode") \
   EXIF(WHITEBALANCE,0xa403,"WhiteBalance") \
   EXIF(DigitalZoomRatio,0xa404,"DigitalZoomRatio") \
   EXIF(FOCALLENGTH_35MM,0xa405,"FocalLengthIn35mmFilm") \
   EXIF(SceneCaptureType,0xa406,"SceneCaptureType") \
   EXIF(GainControl,0xa407,"GainControl") \
   EXIF(Contrast,0xa408,"Contrast") \
   EXIF(Saturation,0xa409,"Saturation") \
   EXIF(Sharpness,0xa40a,"Sharpness") \
   EXIF(SubjectDistanceRange,0xa40c,"SubjectDistanceRange") \
   EXIF(UniqueImageID,0xa420,"UniqueImageID")

#define EXIF(id,code,name) TAG_##id = code ,
enum TAG {
   EXIFTAGS
};
#undef EXIF

typedef struct {
   const unsigned short Tag;
   const char * Desc;
}TagTable_t;

#define EXIF(id,code,name) { TAG_##id, name },
static TagTable_t TagTable[] = {
   EXIFTAGS
} ;
#undef EXIF
const int SizeTagTable = sizeof( TagTable ) / sizeof( TagTable_t );

#define GPSTAGS \
   GPS(0,VersionID,       b,4) \
   GPS(1,LatitudeRef,     a,2) /* 'N' = North | 'S' = South */ \
   GPS(2,Latitude,        r,3) \
   GPS(3,LongitudeRef,    a,2) /* 'E' = East | 'W' = West */ \
   GPS(4,Longitude,       r,3) \
   GPS(5,AltitudeRef,     a,1) /* 0 = Above Sea Level | 1 = Below Sea Level */ \
   GPS(6,Altitude,        r,1) \
   GPS(7,TimeStamp,       r,3) \
   GPS(8,Satelites,       a,-1) \
   GPS(9,Status,          a,2) /* 'A' = Measurement Active | 'V' = Measurement Void */ \
   GPS(0xa,MeasureMode,     a,2) /* 2 = 2-Dimensional Measurement | 3 = 3-Dimensional Measurement */ \
   GPS(0xb,DOP,             r,1) \
   GPS(0xc,SpeedRef,        a,2) /* 'K' = km/h | 'M' = mph | 'N' = knots */ \
   GPS(0xd,Speed,           r,1) \
   GPS(0xe,TrackRef,        a,2) /* 'M' = Magnetic North | 'T' = True North */ \
   GPS(0xf,Track,           r,1) \
   GPS(0x10,ImgDirectionRef, a,2) /* 'M' = Magnetic North | 'T' = True North */ \
   GPS(0x11,ImgDirection,    r,1) \
   GPS(0x12,MapDatum,        a,-1) \
   GPS(0x13,DestLatitudeRef, a,2) /* 'N' = North | 'S' = South */ \
   GPS(0x14,DestLatitude,    r,3) \
   GPS(0x15,DestLongitudeRef,a,2) /* 'E' = East | 'W' = West */ \
   GPS(0x16,DestLongitude,   r,3) \
   GPS(0x17,DestBearingRef,  a,2) /* 	'M' = Magnetic North | 'T' = True North */ \
   GPS(0x18,DestBearing,     r,1) \
   GPS(0x19,DestDistanceRef, a,2) /* 'K' = Kilometers | 'M' = Miles | 'N' = Nautical Miles */ \
   GPS(0x1a,DestDistance,    r,1) \
   GPS(0x1b,ProcessingMethod,u,-1) /* "GPS", "CELLID", "WLAN" or "MANUAL" */ \
   GPS(0x1c,AreaInformation, u,-1) \
   GPS(0x1d,DateStamp,       a,11) /* Format is YYYY:mm:dd */ \
   GPS(0x1e,Differential,    s,2) /* 0 = No Correction | 1 = Differential Corrected */ \
   GPS(0x1f,HPositioningError,    r,1)


#define GPS(name,type,len,d)
   GPSTAGS
#undef GPS


//--------------------------------------------------------------------------
// Convert a 16 bit unsigned value from file's native byte order
//--------------------------------------------------------------------------

//--------------------------------------------------------------------------
// Display a number as one of its many formats
//--------------------------------------------------------------------------
void exif::PrintFormatNumber(void * ValuePtr, FMT Format, int ByteCount)
{
   int i = 0;
   do {
      if( i ) inf->ExifText(",");
      switch(Format){
         case FMT_SBYTE:
         case FMT_BYTE:      inf->ExifText("%02x",*(byte*)ValuePtr); break;
         case FMT_USHORT:    inf->ExifText("%d",(dword)Get16u((word*)ValuePtr)); break;
         case FMT_ULONG:
         case FMT_SLONG:     inf->ExifText("%d",Get32s((int*)ValuePtr)); break;
         case FMT_SSHORT:    inf->ExifText("%hd",(int)(signed short)Get16u((word*)ValuePtr)); break;
         case FMT_URATIONAL:
         case FMT_SRATIONAL: inf->ExifText("%d/%d",Get32s((int*)ValuePtr), Get32s((int*)ValuePtr+1)); break;
         case FMT_SINGLE:    inf->ExifText("%f",(double)*(float*)ValuePtr); break;
         case FMT_DOUBLE:    inf->ExifText("%f",*(double*)ValuePtr); break;
         default:            inf->ExifText("Unknown format %d", Format); return;
      }
      ByteCount -= BytesPerFormat[Format];
      ValuePtr = ((byte*)ValuePtr + BytesPerFormat[Format]);
      i++;
      if( i>=10 ) {
         inf->ExifText("...");
         break;
      }
   } while( ByteCount );
}

//--------------------------------------------------------------------------
// Evaluate number, be it int, rational, or float from directory.
//--------------------------------------------------------------------------
double exif::ConvertAnyFormat(void * ValuePtr, FMT Format)
{
   double Value = 0;
   int Num,Den;
   switch(Format){
      case FMT_SBYTE:     Value = *(signed char *)ValuePtr;  break;
      case FMT_BYTE:      Value = *(byte *)ValuePtr;        break;
      case FMT_USHORT:    Value = Get16u((word*)ValuePtr);          break;
      case FMT_ULONG:     Value = Get32u((dword*)ValuePtr);          break;
      case FMT_URATIONAL:
      case FMT_SRATIONAL:
            Num = Get32s((int*)ValuePtr);
            Den = Get32s((int*)ValuePtr+1);
            Value = Den ? (double)Num/Den : 0;
            break;
      case FMT_SSHORT:    Value = (signed short)Get16u((word*)ValuePtr);  break;
      case FMT_SLONG:     Value = Get32s((int*)ValuePtr);                break;
         // Not sure if this is correct (never seen float used in Exif format)
      case FMT_SINGLE:    Value = (double)*(float *)ValuePtr;      break;
      case FMT_DOUBLE:    Value = *(double *)ValuePtr;             break;
   }
   return Value;
}

//--------------------------------------------------------------------------
// Process one of the nested EXIF directories.
//--------------------------------------------------------------------------
void exif::ProcessExifDir(byte * DirStart, byte * OffsetBase,
                          unsigned ExifLength)
{
   int de;
   int a;
   int NumDirEntries;
   unsigned ThumbnailOffset = 0;
   unsigned ThumbnailSize = 0;

   if (level > 4){
      WARNMS(cinfo,JWRN_EXIF_1);
      return;
   }

   NumDirEntries = Get16u((word*)DirStart);

   {
      byte * DirEnd = DIR_ENTRY_ADDR(DirStart, NumDirEntries);
      if (DirEnd+4 > (OffsetBase+ExifLength)){
         if (DirEnd+2 == OffsetBase+ExifLength || DirEnd == OffsetBase+ExifLength){
            // Version 1.3 of jhead would truncate a bit too much.
            // This also caught later on as well.
         }else{
            // Note: Files that had thumbnails trimmed with jhead 1.3 or earlier
            // might trigger this.
            WARNMS(cinfo,JWRN_EXIF_2);
            return;
         }
      }
      if (DirEnd > LastExifRefd) LastExifRefd = DirEnd;
   }

   inf->ExifText("Entries [ %d ] = {\n",NumDirEntries);

   {
	   Level l;
	   for (de=0;de<NumDirEntries;de++){
		  byte * DirEntry = DIR_ENTRY_ADDR(DirStart, de);
		  TAG Tag = (TAG)Get16u((word*)DirEntry);
		  FMT Format = (FMT)Get16u((word*)DirEntry+1);
		  unsigned int Components = Get32u((dword*)DirEntry+1);
		  if (Format >= NUM_FORMATS) {
			 // (-1) catches illegal zero case as unsigned underflows to positive large.
			 WARNMS2(cinfo,JWRN_EXIF_3, Format, Tag);
			 continue;
		  }
		  int ByteCount = Components * BytesPerFormat[Format];
		  byte * ValuePtr;
		  if (ByteCount > 4){
			 unsigned OffsetVal;
			 OffsetVal = Get32u((dword*)DirEntry+2);
			 // If its bigger than 4 bytes, the dir entry contains an offset.
			 if (OffsetVal+ByteCount > ExifLength){
				// Bogus pointer offset and / or bytecount value
				WARNMS1(cinfo,JWRN_EXIF_4,Tag);
				continue;
			 }
			 ValuePtr = OffsetBase+OffsetVal;
		  }else{
			 // 4 bytes or less and value is in the dir entry itself
			 ValuePtr = DirEntry+8;
		  }

		  if (LastExifRefd < ValuePtr+ByteCount){
			 // Keep track of last byte in the exif header that was actually referenced.
			 // That way, we know where the discardable thumbnail data begins.
			 LastExifRefd = ValuePtr+ByteCount;

		  }

		  if (Tag == TAG_GPSInfo){
			byte * SubdirStart;
			SubdirStart = OffsetBase + Get32u((dword*)ValuePtr);
			if (SubdirStart < OffsetBase || SubdirStart > OffsetBase+ExifLength){
				WARNMS(cinfo,JWRN_EXIF_7);
			}else{
				ProcessGPSInfo(SubdirStart, OffsetBase, ExifLength);
			}
			continue;
		  }

		  if (Tag == TAG_MAKER_NOTE){
			 ProcessMakerNote(ValuePtr, ByteCount, OffsetBase, ExifLength);
			 continue;
		  }

		  if (Tag != TAG_INTEROP_OFFSET && Tag != TAG_EXIF_OFFSET ){
			 // Show tag name
			 for (a=0;;a++){
				if (a >= SizeTagTable ){
				   inf->ExifText("%*cUnknown Tag %04x Value = ", level*4,' ', Tag);
				   break;
				}
				if (TagTable[a].Tag == Tag){
				   if( (Components > 1 && !AsciiFormat[Format]) ||  Components > 125 )
					  inf->ExifText("%*c%s [ %d ] = ", level*4,' ', TagTable[a].Desc, Components);
              else
					  inf->ExifText("%*c%s = ", level*4,' ', TagTable[a].Desc);
				   break;
				}
			 }

			 // Show tag value.
			 switch(Format){
				case FMT_UNDEFINED:
				   // Undefined is typically an ascii string.
				case FMT_STRING:
				   // String arrays printed without function call (different from int arrays)
				   {
					  inf->ExifText("'");
					  for (a=0;a<ByteCount;a++){
                    if( ByteCount>125 && a > 59 && a < ByteCount-56 ) {
                       if( a == 60 ) inf->ExifText+=' ';
                       if( a == 61 ) inf->ExifText+=' ';
                       if( a == 62 ) inf->ExifText+='.';
                       if( a == 63 ) inf->ExifText+='.';
                       if( a == 64 ) inf->ExifText+='.';
                       if( a == 65 ) inf->ExifText+=' ';
                       if( a == 66 ) inf->ExifText+=' ';
                    }
                    else {
						 if (ValuePtr[a] >= 32 && ValuePtr[a] < 127 ) {
							inf->ExifText+=(char)ValuePtr[a];
						 }else{
							   inf->ExifText+='.';
						 }
                    }
					  }
					  inf->ExifText("'\n");
				   }
				   break;
				default:
				   // Handle arrays of numbers later (will there ever be?)
				   PrintFormatNumber(ValuePtr, Format, ByteCount);
				   inf->ExifText("\n");
			 }
		  }

		  // Extract useful components of tag
		  switch(Tag){
			 case TAG_MAKE:
				strncpy(inf->ExifInfo.CameraMake, (const char*)ValuePtr, 31);
				break;
			 case TAG_MODEL:
				strncpy(inf->ExifInfo.CameraModel, (char*)ValuePtr, 39);
				break;
			 case TAG_DATETIME_ORIGINAL:
				strncpy(inf->ExifInfo.DateTimeOriginal, (char*)ValuePtr, 19);
				break;
			 case TAG_DATETIME_DIGITIZED:
				strncpy(inf->ExifInfo.DateTimeDigitized, (char*)ValuePtr, 19);
				break;
			 case TAG_DATETIME:
				strncpy(inf->ExifInfo.DateTime, (char*)ValuePtr, 19);
				break;
			 case TAG_USERCOMMENT:
				// Olympus has this padded with trailing spaces.  Remove these first.
				for (a=ByteCount;;){
				   a--;
				   if ((ValuePtr)[a] != ' ') break;
				   (ValuePtr)[a] = '\0';
				   if (a == 0) break;
				}
				// Copy the comment
				if (memcmp(ValuePtr, "ASCII",5) == 0){
				   for (a=5;a<10;a++){
					  int c;
					  c = (ValuePtr)[a];
					  if (c != '\0' && c != ' '){
						 strncpy(inf->ExifInfo.Comments, (char*)ValuePtr+a, MAX_COMMENT-1);
						 break;
					  }
				   }
				}else{
				   strncpy(inf->ExifInfo.Comments, (char*)ValuePtr, MAX_COMMENT-1);
				}
				break;
			 case TAG_FNUMBER:
				// Simplest way of expressing aperture, so I trust it the most.
				// (overwrite previously computd value if there is one)
				inf->ExifInfo.ApertureFNumber = (float)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_APERTURE:
			 case TAG_MAXAPERTURE:
				// More relevant info always comes earlier, so only use this field if we don't
				// have appropriate aperture information yet.
				if (inf->ExifInfo.ApertureFNumber == 0){
				   inf->ExifInfo.ApertureFNumber
					  = (float)exp(ConvertAnyFormat(ValuePtr, Format)*log(2.0)*0.5);
				}
				break;
			 case TAG_FOCALLENGTH:
				// Nice digital cameras actually save the focal length as a function
				// of how farthey are zoomed in.
				inf->ExifInfo.FocalLength = (float)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_SUBJECT_DISTANCE:
				// Inidcates the distacne the autofocus camera is focused to.
				// Tends to be less accurate as distance increases.
				inf->ExifInfo.Distance = (float)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_EXPOSURETIME:
				// Simplest way of expressing exposure time, so I trust it most.
				// (overwrite previously computd value if there is one)
				inf->ExifInfo.ExposureTime = (float)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_SHUTTERSPEED:
				// More complicated way of expressing exposure time, so only use
				// this value if we don't already have it from somewhere else.
				if (inf->ExifInfo.ExposureTime == 0){
				   inf->ExifInfo.ExposureTime
					  = (float)(1/exp(ConvertAnyFormat(ValuePtr, Format)*log(2.0)));
				}
				break;
			 case TAG_FLASH:
				if ((int)ConvertAnyFormat(ValuePtr, Format) & 1){
				   inf->ExifInfo.FlashUsed = TRUE;
				}else{
				   inf->ExifInfo.FlashUsed = FALSE;
				}
				break;
          case TAG_ResolutionUnit:
            if( inf->ExifInfo.ResolutionUnit == 0 )
               inf->ExifInfo.ResolutionUnit = (int)ConvertAnyFormat(ValuePtr, Format);
            break;
			 case TAG_XResolution:
            if( inf->ExifInfo.XResolution == 0 )
               inf->ExifInfo.XResolution = (double)ConvertAnyFormat(ValuePtr, Format);
            break;
			 case TAG_YResolution:
            if( inf->ExifInfo.YResolution == 0 )
				   inf->ExifInfo.YResolution = (double)ConvertAnyFormat(ValuePtr, Format);
            break;
			 case TAG_ORIENTATION:
				inf->ExifInfo.Orientation = (ROTATION)(int)ConvertAnyFormat(ValuePtr, Format);
				if (inf->ExifInfo.Orientation < ROTATION_NORMAL || inf->ExifInfo.Orientation > ROTATION_270){
				   WARNMS1(cinfo,JWRN_EXIF_6,inf->ExifInfo.Orientation);
				   inf->ExifInfo.Orientation = ROTATION_UNDEFINED;
				}
				break;
			 case TAG_EXIF_IMAGELENGTH:
			 case TAG_EXIF_IMAGEWIDTH:
				// Use largest of height and width to deal with images that have been
				// rotated to portrait format.
				a = (int)ConvertAnyFormat(ValuePtr, Format);
				if (ExifImageWidth < a) ExifImageWidth = a;
				break;
			 case TAG_FOCALPLANEXRES:
				FocalplaneXRes = ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_FOCALPLANEUNITS:
				switch((int)ConvertAnyFormat(ValuePtr, Format)){
				   case 1: FocalplaneUnits = 25.4; break; // inch
				   case 2:
					  // According to the information I was using, 2 means meters.
					  // But looking at the Cannon powershot's files, inches is the only
					  // sensible value.
					  FocalplaneUnits = 25.4;
					  break;
				   case 3: FocalplaneUnits = 10;   break;  // centimeter
				   case 4: FocalplaneUnits = 1;    break;  // milimeter
				   case 5: FocalplaneUnits = .001; break;  // micrometer
				}
				break;
				// Remaining cases contributed by: Volker C. Schoech (schoech@gmx.de)
			 case TAG_EXPOSURE_BIAS:
				inf->ExifInfo.ExposureBias = (float)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_WHITEBALANCE:
				inf->ExifInfo.Whitebalance = (int)ConvertAnyFormat(ValuePtr, Format);
				break;
				//Quercus: 17-1-2004 Lightsource
			 case TAG_LIGHT_SOURCE:
				inf->ExifInfo.LightSource = (int)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_METERING_MODE:
				inf->ExifInfo.MeteringMode = (int)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_EXPOSURE_PROGRAM:
				inf->ExifInfo.ExposureProgram = (int)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_EXPOSURE_INDEX:
				if (inf->ExifInfo.ISOequivalent == 0){
				   // Exposure index and ISO equivalent are often used interchangeably,
				   // so we will do the same in jhead.
				   // http://photography.about.com/library/glossary/bldef_ei.htm
				   inf->ExifInfo.ISOequivalent = (int)ConvertAnyFormat(ValuePtr, Format);
				}
				break;
			 case TAG_ISO_EQUIVALENT:
				inf->ExifInfo.ISOequivalent = (int)ConvertAnyFormat(ValuePtr, Format);
				if ( inf->ExifInfo.ISOequivalent < 50 ){
				   // Fixes strange encoding on some older digicams.
				   inf->ExifInfo.ISOequivalent *= 200;
				}
				break;
			 case TAG_THUMBNAIL_OFFSET:
				ThumbnailOffset = (unsigned)ConvertAnyFormat(ValuePtr, Format);
				DirWithThumbnailPtrs = DirStart;
				break;
			 case TAG_THUMBNAIL_LENGTH:
				ThumbnailSize = (unsigned)ConvertAnyFormat(ValuePtr, Format);
				break;
			 case TAG_EXIF_OFFSET:
				inf->ExifText("%*cExif ",level*4,' ');
				goto offset;
			 case TAG_INTEROP_OFFSET:
				inf->ExifText("%*cInterop ",level*4,' ');
				;offset:;
				{
				   byte * SubdirStart;
				   SubdirStart = OffsetBase + Get32u((dword*)ValuePtr);
				   if (SubdirStart < OffsetBase || SubdirStart > OffsetBase+ExifLength){
					  WARNMS(cinfo,JWRN_EXIF_7);
				   }else{
					  ProcessExifDir(SubdirStart, OffsetBase, ExifLength);
				   }
				   continue;
				}
			 case TAG_FOCALLENGTH_35MM:
				// The focal length equivalent 35 mm is a 2.2 tag (defined as of April 2002)
				// if its present, use it to compute equivalent focal length instead of
				// computing it from sensor geometry and actual focal length.
				inf->ExifInfo.FocalLength35mmEquiv = (unsigned)ConvertAnyFormat(ValuePtr, Format);
				break;
		  }
	   }
   }
   inf->ExifText("%*c};\n",level*4,' ');

   {
      // In addition to linking to subdirectories via exif tags,
      // there's also a potential link to another directory at the end of each
      // directory.  this has got to be the result of a comitee!
      byte * SubdirStart;
      unsigned Offset;

      if (DIR_ENTRY_ADDR(DirStart, NumDirEntries) + 4 <= OffsetBase+ExifLength){
         Offset = Get32u((dword*)(DirStart+2+12*NumDirEntries));
         if (Offset){
            SubdirStart = OffsetBase + Offset;
            if (SubdirStart > OffsetBase+ExifLength){
               if (SubdirStart < OffsetBase+ExifLength+20){
                  // Jhead 1.3 or earlier would crop the whole directory!
                  // As Jhead produces this form of format incorrectness,
                  // I'll just let it pass silently
                  inf->ExifText("Thumbnail removed with Jhead 1.3 or earlier\n");
               }else{
                  WARNMS(cinfo,JWRN_EXIF_8);
               }
            }else{
               if (SubdirStart <= OffsetBase+ExifLength){
                  inf->ExifText("%*cContinued ",level*4,' ');
                  ProcessExifDir(SubdirStart, OffsetBase, ExifLength);
               }
            }
         }
      }else{
         // The exif header ends before the last next directory pointer.
      }
   }
   if (ThumbnailSize && ThumbnailOffset){
      if (ThumbnailSize + ThumbnailOffset <= ExifLength){
         // The thumbnail pointer appears to be valid.  Store it.
         inf->ExifInfo.ThumbnailPointer = OffsetBase + ThumbnailOffset;
         inf->ExifInfo.ThumbnailSize = ThumbnailSize;

         //inf->ExifText("%*cThumbnail size [ %d bytes ]\n",level*4,' ',ThumbnailSize);
      }
   }
}


GLOBAL(bool)
examine_app1 (j_decompress_ptr cinfo, JOCTET FAR * data,
	       unsigned int datalen)
{
   exif e;
   return e.open(data,datalen,cinfo);
}

//--------------------------------------------------------------------------
// Process a EXIF marker
// Describes all the drivel that most digital cameras include...
//--------------------------------------------------------------------------
bool exif::open(byte * ExifSection,unsigned int length,j_decompress_ptr cinfo_)
{
   MotorolaOrder=0;
   FocalplaneXRes=0;
   FocalplaneUnits=0;
   ExifImageWidth=0;
   cinfo=cinfo_;
   inf = cinfo->fileInfo;
   level=0;
   {   // Check the EXIF header component
      static byte ExifHeader[] = "Exif\0\0";
      if (memcmp(ExifSection, ExifHeader,6)){
         //static byte HttpHeader[] = "http";
         //if (memcmp(ExifSection, HttpHeader,4)){
         //   WARNMS(cinfo,JWRN_EXIF_9);
         //}
         return false;
      }
   }

   if (memcmp(ExifSection+6,"II",2) == 0){
      MotorolaOrder = 0;
   }else{
      if (memcmp(ExifSection+6,"MM",2) == 0){
         MotorolaOrder = 1;
      }else{
         WARNMS(cinfo,JWRN_EXIF_10);
         return false;
      }
   }

   inf->ExifText("%s Exif [ %d bytes] = {\n",MotorolaOrder?"Motorola":"Intel",length);

   // Check the next value for correctness.
   if (Get16u((word*)(ExifSection+8)) != 0x2a){
      WARNMS(cinfo,JWRN_EXIF_11);
      return false;
   }
   int FirstOffset = Get32u((dword*)(ExifSection+10));
   if (FirstOffset < 8 || FirstOffset > 32000){
      // I used to ensure this was set to 8 (website I used indicated its 8)
      // but PENTAX Optio 230 has it set differently, and uses it as offset. (Sept 11 2002)
      WARNMS(cinfo,JWRN_EXIF_12);
   }
   ++level;
   inf->ExifText("%*c",level*4,' ');
   LastExifRefd = ExifSection;
   DirWithThumbnailPtrs = NULL;
   // First directory starts 16 bytes in.  All offset are relative to 8 bytes in.
   ProcessExifDir(ExifSection+6+FirstOffset, ExifSection+6, length-6);
   // Compute the CCD width, in milimeters.
   if (FocalplaneXRes != 0){
      // Note: With some cameras, its not possible to compute this correctly because
      // they don't adjust the indicated focal plane resolution units when using less
      // than maximum resolution, so the CCDWidth value comes out too small.  Nothing
      // that Jhad can do about it - its a camera problem.
      inf->ExifInfo.CCDWidth = (float)(ExifImageWidth * FocalplaneUnits / FocalplaneXRes);
      if (inf->ExifInfo.FocalLength && inf->ExifInfo.FocalLength35mmEquiv == 0){
         // Compute 35 mm equivalent focal length based on sensor geometry if we haven't
         // already got it explicitly from a tag.
         inf->ExifInfo.FocalLength35mmEquiv = (int)(inf->ExifInfo.FocalLength/inf->ExifInfo.CCDWidth*36 + 0.5);
      }
   }
   //inf->ExifText("%*cNon settings part [ %d bytes ]\n",level*4,' ',ExifSection+length-LastExifRefd);
   --level;
   inf->ExifText("};");
   return true;
}


void exif::ProcessCannonMakerNoteDir(byte * DirStart, byte * OffsetBase,
                                     unsigned ExifLength)
{
   int de;
   int a;
   int NumDirEntries;

   NumDirEntries = Get16u((word*)DirStart);

   {
      byte * DirEnd = DIR_ENTRY_ADDR(DirStart, NumDirEntries);
      if (DirEnd > (OffsetBase+ExifLength)){
         // Note: Files that had thumbnails trimmed with jhead 1.3 or earlier
         // might trigger this.
         WARNMS(cinfo,JWRN_EXIF_2);
         return;
      }
   }

   inf->ExifText(" Entries [ %d ] = {\n",NumDirEntries);
   {
   Level l;

   for (de=0;de<NumDirEntries;de++){
      byte * ValuePtr;
      int ByteCount;
      byte * DirEntry = DIR_ENTRY_ADDR(DirStart, de);

      TAG Tag = (TAG)Get16u((word*)DirEntry);
      FMT Format = (FMT)Get16u((word*)DirEntry+1);
      int Components = Get32u((dword*)DirEntry+1);

      if ((Format-1) >= NUM_FORMATS) {
         // (-1) catches illegal zero case as unsigned underflows to positive large.
         WARNMS2(cinfo,JWRN_EXIF_3, Format, Tag);
         continue;
      }

      ByteCount = Components * BytesPerFormat[Format];

      if (ByteCount > 4){
         unsigned OffsetVal;
         OffsetVal = Get32u((dword*)DirEntry+2);
         // If its bigger than 4 bytes, the dir entry contains an offset.
         if (OffsetVal+ByteCount > ExifLength){
            // Bogus pointer offset and / or bytecount value
            WARNMS1(cinfo,JWRN_EXIF_4, Tag);
            continue;
         }
         ValuePtr = OffsetBase+OffsetVal;
      }else{
         // 4 bytes or less and value is in the dir entry itself
         ValuePtr = DirEntry+8;
      }

      // Show tag name
	  inf->ExifText("%*cCanon maker tag ",level*4,' ');
	  char *s = 0;
#define CANON(tag,name) case tag : s = name; break;
	  switch(Tag){
#include "canontag.h"
		default: inf->ExifText("%04x ",Tag);
	  }
      if(s ) inf->ExifText(0,"%s ",s);
      if( Components > 1 && !AsciiFormat[Format] )
         inf->ExifText("[ %d ] = ",Components);
      else
         inf->ExifText("= ");

      // Show tag value.
      switch(Format){
         case FMT_UNDEFINED:
            // Undefined is typically an ascii string.
         case FMT_STRING:
            // String arrays printed without function call (different from int arrays)
            inf->ExifText("'");
            for (a=0;a<ByteCount;a++){
               int ZeroSkipped = 0;
               if (ValuePtr[a] >= 32){
                  if (ZeroSkipped){
                     inf->ExifText("?");
                     ZeroSkipped = 0;
                  }
                  inf->ExifText+=(char)ValuePtr[a];
               }
               else {
                  if (ValuePtr[a] == 0) ZeroSkipped = 1;
               }
            }
            inf->ExifText("'\n");
            break;

         default:
            // Handle arrays of numbers later (will there ever be?)
            PrintFormatNumber(ValuePtr, Format, ByteCount);
            inf->ExifText("\n");
      }
   }
   }
   inf->ExifText("%*c};\n",level*4,' ');
}

//--------------------------------------------------------------------------
// Show generic maker note - just hex bytes.
//--------------------------------------------------------------------------
void exif::ShowMakerNoteGeneric(byte * ValuePtr, int ByteCount)
{
   inf->ExifText(" bytes [ %d ] = {", ByteCount);
   for(int a=0;a<ByteCount;a++){
      if (a > 10){
         inf->ExifText("...");
         break;
      }
      if( a ) inf->ExifText(",");
      inf->ExifText(" %02x",ValuePtr[a]);
   }
   inf->ExifText(" };\n");

}

//--------------------------------------------------------------------------
// Process maker note - to the limited extent that its supported.
//--------------------------------------------------------------------------
void exif::ProcessMakerNote(byte * ValuePtr, int ByteCount,
                            byte * OffsetBase, unsigned ExifLength)
{
   inf->ExifText("%*cMaker note",level*4,' ');
   if (strstr(inf->ExifInfo.CameraMake, "Canon"))
      ProcessCannonMakerNoteDir(ValuePtr, OffsetBase, ExifLength);
   else
      ShowMakerNoteGeneric(ValuePtr, ByteCount);
}

//--------------------------------------------------------------------------
// Process GPS Info.
//--------------------------------------------------------------------------
void exif::ProcessGPSInfo(byte * DirStart, byte * OffsetBase, unsigned ExifLength)
{
   int de;
   int a;
   int NumDirEntries;

   NumDirEntries = Get16u((word*)DirStart);

   {
      byte * DirEnd;
      DirEnd = DIR_ENTRY_ADDR(DirStart, NumDirEntries);
      if (DirEnd > (OffsetBase+ExifLength)){
         // Note: Files that had thumbnails trimmed with jhead 1.3 or earlier
         // might trigger this.
         WARNMS(cinfo,JWRN_EXIF_2);
         return;
      }
   }

   inf->ExifText("%*cGPS Info Entries [ %d ] = {\n",level*4,' ',NumDirEntries);
   {
   Level l;

   for (de=0;de<NumDirEntries;de++){
      byte * ValuePtr;
      int ByteCount;
      byte * DirEntry = DIR_ENTRY_ADDR(DirStart, de);

      TAG Tag = (TAG)Get16u((word*)DirEntry);
      FMT Format = (FMT)Get16u((word*)DirEntry+1);
      int Components = Get32u((dword*)DirEntry+1);

      if ((Format-1) >= NUM_FORMATS) {
         // (-1) catches illegal zero case as unsigned underflows to positive large.
         WARNMS2(cinfo,JWRN_EXIF_3, Format, Tag);
         continue;
      }

      ByteCount = Components * BytesPerFormat[Format];

      if (ByteCount > 4){
         unsigned OffsetVal;
         OffsetVal = Get32u((dword*)DirEntry+2);
         // If its bigger than 4 bytes, the dir entry contains an offset.
         if (OffsetVal+ByteCount > ExifLength){
            // Bogus pointer offset and / or bytecount value
            WARNMS1(cinfo,JWRN_EXIF_4, Tag);
            continue;
         }
         ValuePtr = OffsetBase+OffsetVal;
      }else{
         // 4 bytes or less and value is in the dir entry itself
         ValuePtr = DirEntry+8;
      }

      // Show tag name
	  inf->ExifText("%*cGPS ",level*4,' ');
	  char *s = 0;
#define GPS(tag,name,type,len) case tag : s = #name; break;
	  switch(Tag){
         GPSTAGS
         default: inf->ExifText("%04x ",Tag);
	  }
#undef GPS
      if(s ) inf->ExifText("%s ",s);
      if( Components > 1 && !AsciiFormat[Format] )
         inf->ExifText("[ %d ] = ",Components);
      else
         inf->ExifText("= ");

      // Show tag value.
      switch(Format){
         case FMT_UNDEFINED:
            // Undefined is typically an ascii string.
         case FMT_STRING:
            // String arrays printed without function call (different from int arrays)
            inf->ExifText("'");
            for (a=0;a<ByteCount;a++){
               int ZeroSkipped = 0;
               if (ValuePtr[a] >= 32){
                  if (ZeroSkipped){
                     inf->ExifText("?");
                     ZeroSkipped = 0;
                  }
                  inf->ExifText+=(char)ValuePtr[a];
               }
               else {
                  if (ValuePtr[a] == 0) ZeroSkipped = 1;
               }
            }
            inf->ExifText("'\n");
            break;

         default:
            // Handle arrays of numbers later (will there ever be?)
            PrintFormatNumber(ValuePtr, Format, ByteCount);
            inf->ExifText("\n");
      }
   }
   }
   inf->ExifText("%*c};\n",level*4,' ');
}
