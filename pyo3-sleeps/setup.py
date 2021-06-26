from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="pyo3_sleeps",
    version="0.0.1",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    rust_extensions=[
        RustExtension('pyo3_sleeps', "Cargo.toml", debug=False),
    ],
    include_package_data=True,
    zip_safe=False,
)
