import struct

def main():
    print("Clean COFF Header Timestamp")
    pathToFile = input("Path to file: ").strip()
    
    try:
        with open(pathToFile, "rb+") as f:
            # Check DOS Header Signature ('MZ')
            if f.read(2) != b"MZ":
                print("Error: Not a valid DOS/PE executable.")
                return

            # Go to offset 0x3C to read e_lfanew (PE Header Offset)
            f.seek(0x3C)
            lfanew_bytes = f.read(4)
            if len(lfanew_bytes) < 4:
                print("Error: Corrupted DOS header.")
                return

            pe_offset = struct.unpack('<I', lfanew_bytes)[0]

            # Verify PE Signature ("PE\0\0")
            f.seek(pe_offset)
            pe_sig = f.read(4)
            if pe_sig != b'PE\x00\x00':
                print("Error: Invalid PE header signature.")
                return

            # COFF Header starts right after PE Signature (+4 bytes)
            # TimeDateStamp is located at offset +4 inside COFF Header
            # Total offset: pe_offset + 4 (PE Sig) + 4 (Machine + Sections) = pe_offset + 8
            timestamp_offset = pe_offset + 8

            # Seek to TimeDateStamp and overwrite with 0x00000000
            f.seek(timestamp_offset)
            f.write(struct.pack("<I", 0))
            print("[+] TimeDateStamp successfully stripped (set to 0x00000000)!")

    except FileNotFoundError:
        print("Error: File not found. Try another path.")
    except IOError as e:
        print(f"I/O Error: {e}")

if __name__ == "__main__":
    main()