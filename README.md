# Cohertia

## Working with the code

First, download the main branch of this repository or clone the repo through git using:

```bash
git clone git@github.com/DIRM2705/Cohertia
```

You'll need Python 3.13 and Rust 1.90.0 to compile and execute all the contents of this repo

Create a venv in Python using

```bash
py -m venv .venv
.venv/Scripts/activate
```

Then, install the requirements for Python
```bash
pip install -r requirements.txt
```

Move to each library with the change directory command, and execute maturin. For example
```bash
cd gower
maturin develop
cd ..
```
This will install all the libraries created exclusively for this project