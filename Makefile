.PHONY: maturin
maturin:
	maturin develop -F python

.PHONY: ipython
ipython:
	./venv/bin/ipython3

.PHONY: jupyter
jupyter:
	./venv/bin/jupyter lab

.PHONY: jupyter-remote
jupyter-remote:
	./venv/bin/jupyter lab --ip=0.0.0.0

.PHONY: setup
setup:
	python3 -m venv venv
	./venv/bin/pip3 install -r requirements.txt
	echo "Please run \"source ./venv/bin/activate\" on your shell"
