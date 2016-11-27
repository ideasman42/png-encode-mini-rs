
PNG Encode Mini
===============

Tiny PNG writing utility, without any dependency on ``zlib`` or ``libpng``,
useful when only basic functionality is required.


Exposes a single method ``write`` which takes ``RGBA`` pixel-data, with width and height arguments.


Tests
-----

Currently the tests use ImageMagick ``convert`` and ``compare`` commands
to validate the output images can be read.

