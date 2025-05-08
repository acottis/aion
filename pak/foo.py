import zipfile
import zlib

file = r"C:\Users\adamc\Downloads\AION.Encdec\AION Encdec\bin\data.zip"

z = zipfile.ZipFile(file)
z = z.getinfo(z.namelist()[0])

with open(file, "rb") as f:
    f.seek(30 + 0x17)

    raw = f.read(0x135)

    # d = zlib.decompress(raw)
    d = zlib.decompressobj(-15)
    d = d.decompress(raw)
    print(str(d))

# print(hex(f.compress_size))
# print(hex(f.compress_type))
# print(hex(f.CRC))
# print(f.date_time)
# print(hex(f.file_size))
