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

Python requires some self-made libraries to run the code, you can either download the pre-compiled versions [here](https://github.com/DIRM2705/GENIA/releases/download/GENIA-lib-0.1.2/genia_libs-0.1.2-cp313-cp313-win_amd64.whl) or compile them yourself using maturin.

### Installing using pre-compiled versions (download the wheel files first)
```bash
pip install -r requirements.txt
pip install path_to_lib/genia_libs-0.1.2-cp313-cp313-win_amd64.whl  
```

## Installing using maturin (compile the libraries yourself)
```bash
pip install -r requirements.txt
pip install maturin
cd libs
maturin develop -r
```