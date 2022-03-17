
from ufoLib2 import Font
import flatbuffers
from FlatFont.Ufo import FontInfo
from enum import IntEnum
import fontTools.ufoLib
SCALARS = [int, float, bool] # Other scalars needed # Enum needed
NON_SCALARS = [str, list] # Other none-scalars needed

#fontTools.ufoLib.UFOWriter()
with open("ufoff.bin", "rb") as infile:
    buf = infile.read()
buf = bytearray(buf)

ufo_info = Font.info()

#def main():
#    ufo_info = FontInfo.FontInfo.GetRootAs(buf, 0)
    
    
