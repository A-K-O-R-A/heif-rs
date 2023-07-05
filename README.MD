# High Efficiency Image File Format

This is a work in progress parser of the High Efficiency Image File Format commonly known as HEIF. The goal of this project is not to implement any encodings like the commonly used HEIC / H.265, instead it just parses the file and returns the individual boxes with the metadata of the image.

### Motivation

The motivation of this project is to parse a HEIF file taken in portrait mode with a iPhone and then be able to replace the depth mask, to controll the depth effect of apples so called portrait fotos, which can be used to create impressive visual effects on lock screens.
