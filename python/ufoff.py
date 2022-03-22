
from ufoLib2 import Font
from ufoLib2.converters import unstructure
import flatbuffers
from FlatFont.Ufo import FontInfo
from FlatFont.Ufo import OpenTypeNameRecord
#from FlatFont.Ufo import 
from enum import IntEnum

SCALARS = [int, float, bool] # Other scalars needed # Enum needed
NON_SCALARS = [str, list] # Other none-scalars needed

def Capital(string: str): # seek equivalent buit-in function
    '''Capitalizes the string without affecting other characters'''
    return string[0].upper() + string[1:]

def StartAttrVector(attr: str):
    '''
    Simplifies calling buffer functions
    Returns a string representing the buffer function 
    Suitable for the "Getattr" function input
    '''
    return "Start" + Capital(attr) + "Vector"

def AddAttr(attr: str): # how about make it a subclass to str, add a method with or without @property: attr.AddAttr or attr.StartAttrVector
    '''
    Simplifies calling buffer functions
    Returns a string representing the buffer function 
    Suitable for the "Getattr" function input
    '''
    return "Add" + Capital(attr)

def BuildAttrString(builder: flatbuffers.Builder, string: str):
    '''Creates a buffer for strings'''
    return builder.CreateString(string)

def BuildAttrVector(builder: flatbuffers.Builder, attr: str ,vector: list, Value_Type: type = float):   
    '''Creates a buffer for Vectors by passing the name of the attribute as a string'''
    getattr(FontInfo, StartAttrVector(attr))(builder, len(vector)) # Fontinfo.StartAttrVector()
    for index in reversed(range(0, len(vector))):
        if Value_Type == float:
            builder.PrependFloat32(vector[index])
        if Value_Type == int:
            builder.PrependInt32(vector[index])
        if Value_Type == bool:
            builder.PrependBool(vector[index])
    return builder.EndVector()

def BuildUfoBuffer(builder: flatbuffers.Builder, ufo: Font):
    '''
    Creates a dict that contains references for each data type that needs the buffer
    Includes a for-loop to generate a reference for the buffer-needed fields
    To avoid NestedError
    '''
    attr_dict = dict()
    # A for-loop to create a reference to all fields which needs the buffer to be created for avoiding NestedError
    info_dict = unstructure(ufo.info)
    for attr in info_dict:
        if type(info_dict[attr]) in SCALARS:
            attr_dict[attr] = info_dict[attr]

        elif type(info_dict[attr]) == str: # How about writing a BuildAttr Func
            attr_dict[attr] = BuildAttrString(builder, info_dict[attr])

        elif type(info_dict[attr]) == list: 
            if not info_dict[attr]:
                continue

            if type(info_dict[attr][0]) in [int, float]: # have to discuss type conversions
                attr_dict[attr] = BuildAttrVector(builder, attr, info_dict[attr])
                
            else:
                if attr == "OpenTypeNameRecords":
                    OpenTypeNameRecord_refs = []
                    for i in range(len(ufo.info.openTypeNameRecords)):
                        OpenTypeNameRecord_string = builder.CreateString(ufo.info.openTypeNameRecords[i].string)
                        OpenTypeNameRecord.Start(builder)
                        OpenTypeNameRecord.AddEncodingID(ufo.info.openTypeNameRecords[i].encodingID)
                        OpenTypeNameRecord.AddLanguageID(ufo.info.openTypeNameRecords[i].languageID)
                        OpenTypeNameRecord.AddPlatformID(ufo.info.openTypeNameRecords[i].platformID)
                        OpenTypeNameRecord.AddNameID(ufo.info._openTypeNameRecords[i].nameID)
                        OpenTypeNameRecord.AddString(OpenTypeNameRecord_string)
                        OpenTypeNameRecord_refs[i] = OpenTypeNameRecord.End(builder)

                    FontInfo.StartOpenTypeNameRecordsVector(builder, len(ufo.info.openTypeNameRecords))
                    for i in reversed(range(len(ufo.info.openTypeNameRecords))):
                        builder.PrependUOffsetTRelative(OpenTypeNameRecord_refs[i])
                    attr_dict[attr] = builder.EndVector()

            # TODO other nested references
    
    '''Building the buffer with all scalar types and a reference for non-scalar types'''
    FontInfo.Start(builder)    
    for attr in info_dict:
        if not info_dict[attr]: continue
        # Writing scalar fields: (Other option: just use np.isscalar()!)
        # Seeking an alternative to inclusde IntEnum in SCALARS without using "issubclass"
        if type(info_dict[attr]) in SCALARS or issubclass(type(info_dict[attr]), IntEnum): 
            getattr(FontInfo, AddAttr(attr))(builder, info_dict[attr])

        # writing non-scalar fields:
        if type(info_dict[attr]) in NON_SCALARS: # can be emplemented just with an "else"!
            getattr(FontInfo, AddAttr(attr))(builder, attr_dict[attr])

            # TODO other types
    
    flat_ufo_info = FontInfo.End(builder)
    builder.Finish(flat_ufo_info)
    return builder.Output()   

def main():
    '''
    Converts UFO (From UfoLib2.Font) to flatbuffer binary
    for JSON conversion, execute:
    flatc --json --raw-binary -o json ../schemas/ufo/fontinfo.fbs -- ufoff.bin    
    '''
    ufo = Font.open("../../OswaldFont/legacy/3.0/Roman/400/src/Oswald--400.ufo")
    builder = flatbuffers.Builder(0)

    # Building the buffer by passing the references
    buf = BuildUfoBuffer(builder, ufo)

    with open("ufoff.bin", "wb") as     outfile:
        outfile.write(buf)

if __name__ == "__main__":
    main()