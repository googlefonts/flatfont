
from ufoLib2 import Font
import flatbuffers
from FlatFont.Ufo import FontInfo
from enum import IntEnum

SCALARS = [int, float, bool] # Other scalars needed # Enum needed
NON_SCALARS = [str, list] # Other none-scalars needed

def Capital(string: str): # seek equivalent buit-in function
    '''Capitalizes the string without affecting other characters'''
    return string[0].upper() + string[1:]

def StartAttrVector(attr: str):
    '''Simplifies calling buffer functions by returning a string representing the buffer function suitable for the "Getattr" function input'''
    return "Start" + Capital(attr) + "Vector"

def AddAttr(attr: str): # how about make it a subclass to str, add a method with or without @property: attr.AddAttr or attr.StartAttrVector
    '''Simplifies calling buffer functions by returning a string representing the buffer function suitable for the "Getattr" function input'''
    return "Add" + Capital(attr)

def BuildAttrString(builder: flatbuffers.Builder, string: str):
    '''Creates a buffer for strings'''
    return builder.CreateString(string)

def BuildAttrVector(builder: flatbuffers.Builder, attr: str ,vector: list, Value_Type: type = float):   
    '''Creates a buffer for Vectors by passing the name of the attribute as a string'''
    getattr(FontInfo, StartAttrVector(attr))(builder, len(vector))
    for index in reversed(range(0, len(vector))):
        if Value_Type == float:
            builder.PrependFloat32(vector[index])
        if Value_Type == int:
            builder.PrependInt32(vector[index])
        if Value_Type == bool:
            builder.PrependBool(vector[index])
    return builder.EndVector()




def GetAttrReference(builder: flatbuffers.Builder, ufo: Font) -> dict: 
    '''
    Creates a dictionary  that contains references for each data type that needs the buffer
    Includes a for-loop to generate a reference for the buffer-needed fields
    To avoid NestedError
    '''
    attr_dict = dict()
    # A for-loop to create a reference to all fields which needs the buffer to be created for avoiding NestedError
    for attr in dir(ufo.info):
        attribute_value = getattr(ufo.info, attr) 

        if attr[0] != "_" and attribute_value: # put "or True" the "attribute_value" for adding empty list or None_type
            if type(attribute_value) == str: # How about writing a BuildAttr Func
                attr_dict[attr] = BuildAttrString(builder, attribute_value)

            if type(attribute_value) == list:
                if type(attribute_value[0]) in [int, float]: # have to discuss type conversions
                    attr_dict[attr] = BuildAttrVector(builder, attr, attribute_value)
            
            if type(attribute_value) == object:
                pass

            # TODO other nested references
    
    return attr_dict

def BuildUfoBuffer(builder: flatbuffers.Builder, ufo: Font, attr_dict: dict):
    '''Building the buffer with all scalar types and a reference for non-scalar types'''
    FontInfo.Start(builder)    
    for attr in dir(ufo.info):
        attribute_value = getattr(ufo.info, attr)

        if attr[0] != "_" and attribute_value: # Avoiding private fields and "__" functions (Double-check!)
            if attribute_value: # Check if the field exists

                # Writing scalar fields: (Other option: just use np.isscalar()!)
                # Seeking an alternative to inclusde IntEnum in SCALARS without using "issubclass"
                if type(attribute_value) in SCALARS or issubclass(type(attribute_value), IntEnum): 
                    getattr(FontInfo, AddAttr(attr))(builder, attribute_value)

                # writing non-scalar fields:
                if type(attribute_value) in NON_SCALARS: # can be emplemented just with an "else"!
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

    ufo = Font.open("../OswaldFont/legacy/3.0/Roman/400/src/Oswald--400.ufo")
    builder = flatbuffers.Builder(0)

    attr_dict = GetAttrReference(builder, ufo)

    # Building the buffer by passing the references
    buf = BuildUfoBuffer(builder, ufo, attr_dict)

    with open("ufoff.bin", "wb") as outfile:
        outfile.write(buf)


if __name__ == "__main__":
    main()




