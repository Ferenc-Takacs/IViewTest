#ifndef _EXIF_H_
#define _EXIF_H_

#include "uFile.h"
//--------------------------------------------------------------------------
// This structure stores Exif header image elements in a simple manner
// Used to store camera data as extracted from the various ways that it can be
// stored in an exif header
#define MAX_DATE_COPIES 10
#define MAX_COMMENT 200
enum ROTATION {
   ROTATION_UNDEFINED,
   ROTATION_NORMAL,
   ROTATION_FLIP_HORIZONTAL,
   ROTATION_180,
   ROTATION_FLIP_VERTICAL,
   ROTATION_TRANSPOSE, // Flipped about top-left <--> bottom-right axis.
   ROTATION_90,
   ROTATION_TRANSVERSE,// Flipped about top-right <--> bottom-left axis
   ROTATION_270
};

struct cExifInfo {
   char  CameraMake [32];
   char  CameraModel [40];
   char  DateTime [20];
   char  DateTimeOriginal [20];
   char  DateTimeDigitized [20];
   int   Height, Width;
   ROTATION Orientation;
   int   IsColor;
   int   Process;
   int   FlashUsed;
   int   ResolutionUnit;
   double XResolution;
   double YResolution;
   float FocalLength;
   float ExposureTime;
   float ApertureFNumber;
   float Distance;
   float CCDWidth;
   float ExposureBias;
   int   FocalLength35mmEquiv; // Exif 2.2 tag - usually not present.
   int   Whitebalance;
   int   MeteringMode;
   int   ExposureProgram;
   int   ISOequivalent;
   int   LightSource;
   char  Comments [MAX_COMMENT];
   unsigned char * ThumbnailPointer;  // Pointer at the thumbnail
   unsigned ThumbnailSize;     // Size of thumbnail.

   cExifInfo(){clear();};
   void clear(void){memset(this,0,sizeof(cExifInfo));}
};

//#pragma pack(push,1)
struct jFileInfo {
   jFileInfo():file_size(0){}
   cExifInfo ExifInfo;
   string_ ExifText;
   string_ ExifData;
   string_ Comment;
   size_t file_size;
   void clear(void){
      ExifInfo.clear();
      ExifData();
      ExifText();
      Comment();
	  file_size=0;
   }
   ~jFileInfo(){clear();}
   jFileInfo& operator=(const jFileInfo &c){
      if(this!=&c){
         ExifInfo=c.ExifInfo;
         ExifText=c.ExifText;
         ExifData=c.ExifData;
         Comment=c.Comment;
         file_size=c.file_size;
      }
      return *this;
   }
};
//#pragma pack(pop)

#endif // _EXIF_H_
