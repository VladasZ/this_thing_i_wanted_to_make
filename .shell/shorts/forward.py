#!/usr/bin/env python3

import os

os.system("""ssh \
-R 3588:127.0.0.1:3588 \
-R 3589:127.0.0.1:3589 \
-L 9092:127.0.0.1:9092 \
-L 3030:127.0.0.1:3030 \
-L 4566:127.0.0.1:4566 \
-L 4571:127.0.0.1:4571 \
-L 5432:127.0.0.1:5432 \
vladas@v""")
