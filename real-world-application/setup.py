from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="real_world_application",
    version="0.0.1",
    classifiers=[
        "Programming Language :: Python",
        "Programming Language :: Rust",
    ],
    rust_extensions=[
        RustExtension('real_world_application', "Cargo.toml", debug=False),
    ],
    include_package_data=True,
    zip_safe=False,
)
