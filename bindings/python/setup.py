#!/usr/bin/env python
from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    rust_extensions=[RustExtension("tidext.tidext", binding=Binding.PyO3)],
)
