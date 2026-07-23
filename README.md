# GENIA

## Overview

## Working with the code

First, download the main branch of this repository or clone the repo through git using:

```bash
git clone git@github.com/DIRM2705/GENIA.git
```

You'll need Python 3.13 and Rust 1.90.0 to compile and execute all the contents of this repo

Create a venv in Python using

```bash
py -m venv .venv
.venv/Scripts/activate
```

Then, install the required dependencies using:
```bash
pip install -r requirements.txt
```

Python requires some self-made libraries to run the code, they got installed automatically when you ran the previous command. However, if you wish to install older versions you may do it by selecting the desired version from the [releases page](https://github.com/DIRM2705/GENIA/releases) or compile them yourself using maturin.

## Installing the libraries from the releases page

To install the libraries from the releases page, you can use pip to install the wheel files directly. Replace `<link-to-wheel-file>` with the link to the wheel file and `<sha256>` with the corresponding SHA256 hash for that version.

```bash
pip install <link-to-wheel-file>#sha256:<sha256>
```

## Compiling the libraries yourself via maturin

To compile the libraries yourself, you can use maturin. First, install maturin using pip:

```bash
pip install maturin
```

Then, navigate to the directory containing the library's source code and run:
```bash
cd libs
maturin develop
```
This command will build the library on debug mode and install it in your current Python environment. If you desire to build the library in release mode, you can use the following command instead:
```bash
maturin develop -r
```