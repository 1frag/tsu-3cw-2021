from setuptools import setup, Extension

lib_module = Extension(
    "c_api_bindings",
    sources=["lib.c"]
)

setup(
    name="c_api_bindings",
    ext_modules=[lib_module],
    version="0.0.1",
)
