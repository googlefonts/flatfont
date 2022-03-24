
from ufoLib2 import Font
from ufoLib2.converters import structure
import flatbuffers
from FlatFont.Ufo import FontInfo, Guideline
from python.FlatFont.Ufo.OpenTypeNameRecord import OpenTypeNameRecord
SCALARS = [int, float, bool] # Other scalars needed # Enum needed
NON_SCALARS = [str, list] # Other none-scalars needed

#fontTools.ufoLib.UFOWriter()




def main():
    with open("ufoff.bin", "rb") as infile:
        buf = infile.read()
    buf = bytearray(buf)
    ufo_info_buffer = FontInfo.FontInfo.GetRootAs(buf, 0)
    attr_dict = {}
    #print(ufo_info_buffer.Ascender.__annotations__)
    
    for i in dir(ufo_info_buffer):

        if getattr(ufo_info_buffer, i):
            print(i, getattr(ufo_info_buffer, i).__annotations__)

    

def test():
    with open("ufoff.bin", "rb") as infile:
        buf = infile.read()
    buf = bytearray(buf)
    ufo_info_buffer = FontInfo.FontInfo.GetRootAs(buf, 0)
    ufo_info_dict = {}    


    ufo_info_dict['familyName'] = ufo_info_buffer.FamilyName() #: string;
    ufo_info_dict['styleName'] = ufo_info_buffer.StyleName() #: string;
    ufo_info_dict['styleMapFamilyName'] = ufo_info_buffer.StyleMapFamilyName() #: string;
    ufo_info_dict['styleMapStyleName'] = ufo_info_buffer.StyleMapStyleName() #: string;
    ufo_info_dict['versionMajor'] = ufo_info_buffer.VersionMajor() #: int;
    ufo_info_dict['versionMinor'] = ufo_info_buffer.VersionMinor() #: uint;
    #? ufo_info_dict['year'] = ufo_info_buffer.Year() #: int (deprecated);

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-legal-information
    ufo_info_dict['copyright'] = ufo_info_buffer.Copyright() #: string;
    ufo_info_dict['trademark'] = ufo_info_buffer.Trademark() #: string;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-dimension-information
    ufo_info_dict['unitsPerEm'] = ufo_info_buffer.UnitsPerEm() #: float;
    ufo_info_dict['descender'] = ufo_info_buffer.Descender() #: float;
    ufo_info_dict['xHeight'] = ufo_info_buffer.XHeight() #: float;
    ufo_info_dict['capHeight'] = ufo_info_buffer.CapHeight() #: float;
    ufo_info_dict['ascender'] = ufo_info_buffer.Ascender() #: float;
    ufo_info_dict['italicAngle'] = ufo_info_buffer.ItalicAngle() #: float;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#generic-miscellaneous-information
    ufo_info_dict['note'] = ufo_info_buffer.Note() #: string;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#gasp-range-record-format
    #? ufo_info_dict['openTypeGaspRangeRecords'] = ufo_info_buffer. #: [OpenTypeGaspRangeRecord];  ###
    if ufo_info_buffer.OpenTypeGaspRangeRecordsIsNone():
        ufo_info_dict['openTypeGaspRangeRecords'] = None
    else:
        ufo_info_dict['openTypeGaspRangeRecords'] = []
        for index in range(ufo_info_buffer.OpenTypeGaspRangeRecordsLength()):
            OpenTypeGaspRangeRecord_dict = {}
            OpenTypeGaspRangeRecord_dict['rangeMaxPPEM'] = ufo_info_buffer.OpenTypeGaspRangeRecords(index).RangeMaxPPEM()
            OpenTypeGaspRangeRecord_dict['rangeGaspBehavior'] = ufo_info_buffer.OpenTypeGaspRangeRecords(index).RangeGaspBehaviorAsNumpy()
            ufo_info_dict['openTypeGaspRangeRecords'].append(OpenTypeGaspRangeRecord_dict)
    

    # ! //// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-head-table-fields
    ufo_info_dict['openTypeHeadCreated'] = ufo_info_buffer.OpenTypeHeadCreated() #: string;
    ufo_info_dict['openTypeHeadLowestRecPPEM'] = ufo_info_buffer.OpenTypeHeadLowestRecPPEM() #: uint;
    ufo_info_dict['openTypeHeadFlags'] = ufo_info_buffer.OpenTypeHeadFlagsAsNumpy() #: [uint];

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-hhea-table-fields
    ufo_info_dict['openTypeHheaAscender'] = ufo_info_buffer.OpenTypeHheaAscender() #: int;
    ufo_info_dict['openTypeHheaDescender'] = ufo_info_buffer.OpenTypeHheaDescender() #: int;
    ufo_info_dict['openTypeHheaLineGap'] = ufo_info_buffer.OpenTypeHheaLineGap() #: int;
    ufo_info_dict['openTypeHheaCaretSlopeRise'] = ufo_info_buffer.OpenTypeHheaCaretSlopeRise() #: int;
    ufo_info_dict['openTypeHheaCaretSlopeRun'] = ufo_info_buffer.OpenTypeHheaCaretSlopeRun() #: int;
    ufo_info_dict['openTypeHheaCaretOffset'] = ufo_info_buffer.OpenTypeHheaCaretOffset() #: int;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-name-table-fields
    ufo_info_dict['openTypeNameDesigner'] = ufo_info_buffer.OpenTypeNameDesigner() #: string;
    ufo_info_dict['openTypeNameDesignerURL'] = ufo_info_buffer.OpenTypeNameDesignerURL() #: string;
    ufo_info_dict['openTypeNameManufacturer'] = ufo_info_buffer.OpenTypeNameManufacturer() #: string;
    ufo_info_dict['openTypeNameManufacturerURL'] = ufo_info_buffer.OpenTypeNameManufacturerURL() #: string;
    ufo_info_dict['openTypeNameLicense'] = ufo_info_buffer.OpenTypeNameLicense() #: string;
    ufo_info_dict['openTypeNameLicenseURL'] = ufo_info_buffer.OpenTypeNameLicenseURL() #: string;
    ufo_info_dict['openTypeNameVersion'] = ufo_info_buffer.OpenTypeNameVersion() #: string;
    ufo_info_dict['openTypeNameUniqueID'] = ufo_info_buffer.OpenTypeNameUniqueID() #: string;
    ufo_info_dict['openTypeNameDescription'] = ufo_info_buffer.OpenTypeNameDescription() #: string;
    ufo_info_dict['openTypeNamePreferredFamilyName'] = ufo_info_buffer.OpenTypeNamePreferredFamilyName() #: string;
    ufo_info_dict['openTypeNamePreferredSubfamilyName'] = ufo_info_buffer.OpenTypeNamePreferredSubfamilyName() #: string;
    ufo_info_dict['openTypeNameCompatibleFullName'] = ufo_info_buffer.OpenTypeNameCompatibleFullName() #: string;
    ufo_info_dict['openTypeNameSampleText'] = ufo_info_buffer.OpenTypeNameSampleText() #: string;
    ufo_info_dict['openTypeNameWWSFamilyName'] = ufo_info_buffer.OpenTypeNameWWSFamilyName() #: string;
    ufo_info_dict['openTypeNameWWSSubfamilyName'] = ufo_info_buffer.OpenTypeNameWWSSubfamilyName() #: string;

    if ufo_info_buffer.OpenTypeNameRecordsIsNone():
        ufo_info_dict['openTypeNameRecords'] = None
    else:
        ufo_info_dict['openTypeNameRecords'] = []
        for index in range(ufo_info_buffer.OpenTypeNameRecordsLength()):
            OpenTypeNameRecord_dict = {}
            OpenTypeNameRecord_dict['nameID'] = ufo_info_buffer.OpenTypeNameRecords(index).NameID()
            OpenTypeNameRecord_dict['platformID'] = ufo_info_buffer.OpenTypeNameRecords(index).PlatformID()
            OpenTypeNameRecord_dict['encodingID'] = ufo_info_buffer.OpenTypeNameRecords(index).EncodingID()
            OpenTypeNameRecord_dict['languageID'] = ufo_info_buffer.OpenTypeNameRecords(index).LanguageID()
            OpenTypeNameRecord_dict['string'] = ufo_info_buffer.OpenTypeNameRecords(index).String()
            ufo_info_dict['openTypeNameRecords'].append(OpenTypeNameRecord_dict)
    
    #? ufo_info_dict['openTypeNameRecords'] = ufo_info_buffer. #: [OpenTypeNameRecord]; ###

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-os2-table-fields
    ufo_info_dict['openTypeOS2WidthClass'] = ufo_info_buffer.OpenTypeOS2WidthClass() #: WidthClass = NORMAL;
    ufo_info_dict['openTypeOS2WeightClass'] = ufo_info_buffer.OpenTypeOS2WeightClass() #: uint;
    ufo_info_dict['openTypeOS2Selection'] = ufo_info_buffer.OpenTypeOS2SelectionAsNumpy() #: [uint];
    ufo_info_dict['openTypeOS2VendorID'] = ufo_info_buffer.OpenTypeOS2VendorID() #: string;
    ufo_info_dict['openTypeOS2Panose'] = ufo_info_buffer.OpenTypeOS2PanoseAsNumpy() #: [uint];
    ufo_info_dict['openTypeOS2FamilyClass'] = ufo_info_buffer.OpenTypeOS2FamilyClassAsNumpy() #: [uint];
    ufo_info_dict['openTypeOS2UnicodeRanges'] = ufo_info_buffer.OpenTypeOS2UnicodeRangesAsNumpy() #: [uint];
    ufo_info_dict['openTypeOS2CodePageRanges'] = ufo_info_buffer.OpenTypeOS2CodePageRangesAsNumpy() #: [uint];
    ufo_info_dict['openTypeOS2TypoAscender'] = ufo_info_buffer.OpenTypeOS2TypoAscender() #: int;
    ufo_info_dict['openTypeOS2TypoDescender'] = ufo_info_buffer.OpenTypeOS2TypoDescender() #: int;
    ufo_info_dict['openTypeOS2TypoLineGap'] = ufo_info_buffer.OpenTypeOS2TypoLineGap() #: int;
    ufo_info_dict['openTypeOS2WinAscent'] = ufo_info_buffer.OpenTypeOS2WinAscent() #: uint;
    ufo_info_dict['openTypeOS2WinDescent'] = ufo_info_buffer.OpenTypeOS2WinDescent() #: uint;
    ufo_info_dict['openTypeOS2SubscriptXSize'] = ufo_info_buffer.OpenTypeOS2SubscriptXSize() #: int;
    ufo_info_dict['openTypeOS2SubscriptYSize'] = ufo_info_buffer.OpenTypeOS2SubscriptYSize() #: int;
    ufo_info_dict['openTypeOS2SubscriptXOffset'] = ufo_info_buffer.OpenTypeOS2SubscriptXOffset() #: int;
    ufo_info_dict['openTypeOS2SubscriptYOffset'] = ufo_info_buffer.OpenTypeOS2SubscriptYOffset() #: int;
    ufo_info_dict['openTypeOS2SuperscriptXSize'] = ufo_info_buffer.OpenTypeOS2SuperscriptXSize() #: int;
    ufo_info_dict['openTypeOS2SuperscriptYSize'] = ufo_info_buffer.OpenTypeOS2SuperscriptYSize() #: int;
    ufo_info_dict['openTypeOS2SuperscriptXOffset'] = ufo_info_buffer.OpenTypeOS2SuperscriptXOffset() #: int;
    ufo_info_dict['openTypeOS2SuperscriptYOffset'] = ufo_info_buffer.OpenTypeOS2SuperscriptYOffset() #: int;
    ufo_info_dict['openTypeOS2StrikeoutSize'] = ufo_info_buffer.OpenTypeOS2StrikeoutSize() #: int;
    ufo_info_dict['openTypeOS2StrikeoutPosition'] = ufo_info_buffer.OpenTypeOS2StrikeoutPosition() #: int;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#opentype-vhea-table-fields
    ufo_info_dict['openTypeVheaVertTypoAscender'] = ufo_info_buffer.OpenTypeVheaVertTypoAscender() #: float;
    ufo_info_dict['openTypeVheaVertTypoDescender'] = ufo_info_buffer.OpenTypeVheaVertTypoDescender() #: float;
    ufo_info_dict['openTypeVheaVertTypoLineGap'] = ufo_info_buffer.OpenTypeVheaVertTypoLineGap() #: float;
    ufo_info_dict['openTypeVheaCaretSlopeRise'] = ufo_info_buffer.OpenTypeVheaCaretSlopeRise() #: int;
    ufo_info_dict['openTypeVheaCaretSlopeRun'] = ufo_info_buffer.OpenTypeVheaCaretSlopeRun() #: int;
    ufo_info_dict['openTypeVheaCaretOffset'] = ufo_info_buffer.OpenTypeVheaCaretOffset() #: int;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#postscript-specific-data
    ufo_info_dict['postscriptFontName'] = ufo_info_buffer.PostscriptFontName() #: string;
    ufo_info_dict['postscriptFullName'] = ufo_info_buffer.PostscriptFullName() #: string;
    ufo_info_dict['postscriptSlantAngle'] = ufo_info_buffer.PostscriptSlantAngle() #: float;
    ufo_info_dict['postscriptUniqueID'] = ufo_info_buffer.PostscriptUniqueID() #: int;
    ufo_info_dict['postscriptUnderlineThickness'] = ufo_info_buffer.PostscriptUnderlineThickness() #: float;
    ufo_info_dict['postscriptUnderlinePosition'] = ufo_info_buffer.PostscriptUnderlinePosition() #: float;
    ufo_info_dict['postscriptIsFixedPitch'] = ufo_info_buffer.PostscriptIsFixedPitch() #: bool;
    ufo_info_dict['postscriptBlueValues'] = ufo_info_buffer.PostscriptBlueValuesAsNumpy() #: [float];
    ufo_info_dict['postscriptOtherBlues'] = ufo_info_buffer.PostscriptOtherBluesAsNumpy() #: [float];
    ufo_info_dict['postscriptFamilyBlues'] = ufo_info_buffer.PostscriptFamilyBluesAsNumpy() #: [float];
    ufo_info_dict['postscriptFamilyOtherBlues'] = ufo_info_buffer.PostscriptOtherBluesAsNumpy() #: [float];
    ufo_info_dict['postscriptStemSnapH'] = ufo_info_buffer.PostscriptStemSnapHAsNumpy() #: [float];
    ufo_info_dict['postscriptStemSnapV'] = ufo_info_buffer.PostscriptStemSnapVAsNumpy() #: [float];
    ufo_info_dict['postscriptBlueFuzz'] = ufo_info_buffer.PostscriptBlueFuzz() #: float;
    ufo_info_dict['postscriptBlueShift'] = ufo_info_buffer.PostscriptBlueShift() #: float;
    ufo_info_dict['postscriptBlueScale'] = ufo_info_buffer.PostscriptBlueScale() #: float;
    ufo_info_dict['postscriptForceBold'] = ufo_info_buffer.PostscriptForceBold() #: bool;
    ufo_info_dict['postscriptDefaultWidthX'] = ufo_info_buffer.PostscriptDefaultWidthX() #: float;
    ufo_info_dict['postscriptNominalWidthX'] = ufo_info_buffer.PostscriptNominalWidthX() #: float;
    ufo_info_dict['postscriptWeightName'] = ufo_info_buffer.PostscriptWeightName() #: string;
    ufo_info_dict['postscriptDefaultCharacter'] = ufo_info_buffer.PostscriptDefaultCharacter() #: string;
    ufo_info_dict['postscriptWindowsCharacterSet'] = ufo_info_buffer.PostscriptWindowsCharacterSet() #: int;

    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#macintosh-fond-resource-data
    ufo_info_dict['macintoshFONDFamilyID'] = ufo_info_buffer.MacintoshFONDFamilyID() #: int;
    ufo_info_dict['macintoshFONDName'] = ufo_info_buffer.MacintoshFONDName() #: string;
    """
    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#woff-data
    ufo_info_dict['woffMajorVersion'] = ufo_info_buffer. #: uint;
    ufo_info_dict['woffMinorVersion'] = ufo_info_buffer. #: uint;
    ufo_info_dict['woffMetadataUniqueID'] = ufo_info_buffer. #: WoffMetadataUniqueID;
    ufo_info_dict['woffMetadataVendor'] = ufo_info_buffer. #: WoffMetadataVendor;
    ufo_info_dict['woffMetadataCredits'] = ufo_info_buffer. #: WoffMetadataCredits;
    ufo_info_dict['woffMetadataDescription'] = ufo_info_buffer. #: WoffMetadataDescription;
    ufo_info_dict['woffMetadataLicense'] = ufo_info_buffer. #: WoffMetadataLicense;
    ufo_info_dict['woffMetadataCopyright'] = ufo_info_buffer. #: WoffMetadataCopyright;
    ufo_info_dict['woffMetadataTrademark'] = ufo_info_buffer. #: WoffMetadataTrademark;
    ufo_info_dict['woffMetadataLicensee'] = ufo_info_buffer. #: WoffMetadataLicensee;
    ufo_info_dict['woffMetadataExtensions'] = ufo_info_buffer. #: [WoffMetadataExtension];
    """
    # ! /// https://unifiedfontobject.org/versions/ufo3/fontinfo.plist/#guidelines
    #? ufo_info_dict['guidelines'] = ufo_info_buffer.  #: [Guideline];
    if ufo_info_buffer.GuidelinesIsNone():
        ufo_info_dict['guidelines'] = None
    else:
        ufo_info_dict['guidelines'] = []
        for index in range(ufo_info_buffer.GuidelinesLength()):
            Guidelines_dict = {}
            Guidelines_dict['x'] = ufo_info_buffer.Guidelines(index).X()
            Guidelines_dict['y'] = ufo_info_buffer.Guidelines(index).Y()
            Guidelines_dict['angle'] = ufo_info_buffer.Guidelines(index).Angle()
            Guidelines_dict['name'] = ufo_info_buffer.Guidelines(index).Name()
            Guidelines_dict['color'] = ufo_info_buffer.Guidelines(index).Color()
            Guidelines_dict['identifier'] = ufo_info_buffer.Guidelines(index).Identifier()
            ufo_info_dict['guidelines'].append(Guidelines_dict)





    ufo_info = structure(ufo_info_dict, Font)
    #for key in ufo_info_dict:
    #    print(key, ufo_info_dict[key])

if __name__ == "__main__":
    #main()
    test()