PHONY: test readme clean

test:
	reset
	wasm-pack test --chrome

readme: 
	pdflatex README.tex

clean:
	rm -rf target
