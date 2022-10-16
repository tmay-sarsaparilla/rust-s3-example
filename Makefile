.PHONY: setup
setup:
	python -m venv venv
	./venv/Scripts/python -m pip install --upgrade pip
	./venv/Scripts/pip install pre-commit
	./venv/Scripts/pre-commit install
