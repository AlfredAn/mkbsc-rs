.PHONY: all

out/%.svg: out/%.dot
	dot -Tsvg $< > $@

all: $(addsuffix .svg,$(basename $(wildcard out/*.dot)))
