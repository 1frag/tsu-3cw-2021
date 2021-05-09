from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="using_pyo3",
    version="0.0.1",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    rust_extensions=[
        RustExtension('using_pyo3', "Cargo.toml", debug=False),
    ],
    include_package_data=True,
    zip_safe=False,
)
