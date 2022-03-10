# bin2intel

Converts a binary file to an Intel HEX format file for PROM programming

# Usage

Takes a binaryry file on stdiand writes an [Intel HEX](https://en.wikipedia.org/wiki/Intel_HEX) format versionof it on stdout. The output format has 32 bytes per line. The address offset is assumed to be 0000.

    $ bin2intel < 32kprom.bin > 32kprom.hex


