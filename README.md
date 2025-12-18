# Chaos Lab
visually simulate chaotic systems in rust.
e.g. Lorenz attractor, double pendulum, logistic map

## Setup

create a `.csv` in the cwd for logistic map data to write to. Set appropriate `path` variable in `main.rs`.
Addtionally, create a `lorenz.csv` for `lorenz.rs` to write to


## To Start
```shell
./scripts/start.sh
```

### Python Plotting
in your terminal:
```shell
python3 -m venv venv

source venv/bin/activate # Macos
venv\Scripts\activate.bat # Windows Command Prompt

pip3 install matplotlib

python3 /src/scripts/plot.py
```
