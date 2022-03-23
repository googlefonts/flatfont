from ufoLib2 import Font
from ufoLib2.converters import unstructure
import flatbuffers
from FlatFont.Ufo import (
    FontInfo,
    OpenTypeNameRecord,
    OpenTypeGaspRangeRecord,
    Guideline,
    Glyph,
)

SCALARS = [int, float, bool]  # Other scalars needed


def capital(string: str):  # seek equivalent buit-in function
    """Capitalizes the string without affecting other characters"""
    return string[0].upper() + string[1:]


def build_attr_vector(
    builder: flatbuffers.Builder, attr: str, vector: list, Value_Type: type = float
):
    """Creates a buffer for Vectors by passing the name of the attribute as a string"""
    getattr(FontInfo, "Start" + capital(attr) + "Vector")(builder, len(vector))
    for index in reversed(range(0, len(vector))):
        if Value_Type == float:
            builder.PrependFloat32(vector[index])
        if Value_Type == int:
            builder.PrependInt32(vector[index])
        if Value_Type == bool:
            builder.PrependBool(vector[index])
    return builder.EndVector()


def build_guideline_vector(builder: flatbuffers.Builder, guideline_vector: list):
    """Builds a buffer for vector if Guidelines"""
    guideline_refs = []
    for index in range(len(guideline_vector)):
        guideline_name = builder.CreateString(guideline_vector[index].name)
        guideline_color = builder.CreateString(guideline_vector[index].color)
        guideline_identifier = builder.CreateString(guideline_vector[index].identifier)
        Guideline.Start(builder)
        Guideline.AddX(builder, guideline_vector[index].x)
        Guideline.AddY(builder, guideline_vector[index].y)
        Guideline.AddAngle(builder, guideline_vector[index].angle)
        Guideline.AddName(builder, guideline_name)
        Guideline.AddColor(builder, guideline_color)
        Guideline.AddIdentifier(builder, guideline_identifier)
        guideline_refs.append(Guideline.End(builder))
    return guideline_refs


def build_ufo_info_buffer(builder: flatbuffers.Builder, ufo: Font):
    """
    Creates a dict that contains references for each data type that needs the buffer
    Includes a for-loop to generate a reference for the buffer-needed fields
    To avoid NestedError
    """
    attr_dict = dict()
    # A for-loop to create a reference to all fields which needs the buffer to be created for avoiding NestedError
    info_dict = unstructure(ufo.info)
    for attr in info_dict:

        if type(info_dict[attr]) in SCALARS:
            attr_dict[attr] = info_dict[attr]
            continue

        if type(info_dict[attr]) == str:
            attr_dict[attr] = builder.CreateString(info_dict[attr])
            continue

        if type(info_dict[attr]) == list:  # should I create reference for empty list?
            if not info_dict[attr]:
                continue

            if type(info_dict[attr][0]) in [
                int,
                float,
            ]:  # have to discuss type conversions
                attr_dict[attr] = build_attr_vector(builder, attr, info_dict[attr])
                continue

        if attr == "OpenTypeNameRecords":
            OpenTypeNameRecord_refs = []
            for index in range(len(info_dict[attr])):
                OpenTypeNameRecord_string = builder.CreateString(
                    info_dict[attr][index].string
                )
                OpenTypeNameRecord.Start(builder)
                OpenTypeNameRecord.AddNameID(builder, info_dict[attr][index].nameID)
                OpenTypeNameRecord.AddPlatformID(
                    builder, info_dict[attr][index].platformID
                )
                OpenTypeNameRecord.AddEncodingID(
                    builder, info_dict[attr][index].encodingID
                )
                OpenTypeNameRecord.AddLanguageID(
                    builder, info_dict[attr][index].languageID
                )
                OpenTypeNameRecord.AddString(builder, OpenTypeNameRecord_string)
                OpenTypeNameRecord_refs.append(OpenTypeNameRecord.End(builder))

            FontInfo.StartOpenTypeNameRecordsVector(builder, len(info_dict[attr]))
            for index in reversed(range(len(info_dict[attr]))):
                builder.PrependUOffsetTRelative(OpenTypeNameRecord_refs[index])
            attr_dict[attr] = builder.EndVector()
            continue

        if attr == "openTypeGaspRangeRecords":
            openTypeGaspRangeRecord_refs = []
            for index in range(len(info_dict[attr])):
                OpenTypeGaspRangeRecord.StartRangeGaspBehaviorVector(
                    builder, len(info_dict[attr][index].rangeGaspBehavior)
                )
                for j in reversed(range(info_dict[attr][index].rangeGaspBehavior)):
                    builder.PrependUint32(info_dict[attr][index].rangeGaspBehavior[j])
                rangeGaspBehaviorVector = builder.EndVector()
                OpenTypeGaspRangeRecord.start(builder)
                OpenTypeGaspRangeRecord.AddRangeMaxPPEM(
                    builder, info_dict[attr][index].rangeMaxPPEM
                )
                OpenTypeGaspRangeRecord.AddRangeGaspBehavior(
                    builder, rangeGaspBehaviorVector
                )
                openTypeGaspRangeRecord_refs.append(
                    OpenTypeGaspRangeRecord.End(builder)
                )

            FontInfo.StartOpenTypeGaspRangeRecordsVector(builder, len(info_dict[attr]))
            for index in reversed(range(len(info_dict[attr]))):
                builder.PrependUOffsetTRelative(openTypeGaspRangeRecord_refs[index])
            attr_dict[attr] = builder.EndVector()
            continue

        if attr == "guidelines":
            guideline_refs = build_guideline_vector(builder, info_dict[attr])
            FontInfo.StartGuidelinesVector(builder, len(info_dict[attr]))
            for index in reversed(range(len(info_dict[attr]))):
                builder.PrependUOffsetTRelative(guideline_refs[index])
            attr_dict[attr] = builder.EndVector()
            continue

    """Building the buffer with all scalar types and a reference for non-scalar types"""
    FontInfo.Start(builder)
    for attr in info_dict:
        if not info_dict[attr]:
            continue
        getattr(FontInfo, "Add" + capital(attr))(builder, attr_dict[attr])
    flat_ufo_info = FontInfo.End(builder)
    builder.Finish(flat_ufo_info)
    return builder.Output()


def main():
    """
    Converts UFO (From UfoLib2.Font) to flatbuffer binary
    for JSON conversion, execute:
    flatc --json --raw-binary ../schemas/ufo/fontinfo.fbs -- ufoff.bin
    """
    ufo = Font.open("../../OswaldFont/legacy/3.0/Roman/400/src/Oswald--400.ufo")
    builder = flatbuffers.Builder(0)

    # Building the buffer by passing the references
    buf_ufo_info = build_ufo_info_buffer(builder, ufo)
    with open("ufoff.bin", "wb") as outfile:
        outfile.write(buf_ufo_info)


if __name__ == "__main__":
    main()
