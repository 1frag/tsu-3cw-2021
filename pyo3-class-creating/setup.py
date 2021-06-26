from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="pyo3_class_creating",
    version="0.0.1",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    rust_extensions=[
        RustExtension('pyo3_class_creating', "Cargo.toml", debug=False),
    ],
    include_package_data=True,
    zip_safe=False,
)
