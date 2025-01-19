.PHONY: maturin
maturin:
	maturin develop -F python

.PHONY: ipython
ipython:
	./venv/bin/ipython3

.PHONY: jupyter
jupyter:
	./venv/bin/jupyter lab --notebook-dir=notebooks

.PHONY: jupyter-remote
jupyter-remote:
	./venv/bin/jupyter lab --notebook-dir=notebooks --ip=0.0.0.0
