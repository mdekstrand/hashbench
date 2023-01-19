---
jupyter:
  jupytext:
    formats: ipynb,md
    text_representation:
      extension: .md
      format_name: markdown
      format_version: '1.3'
      jupytext_version: 1.14.4
  kernelspec:
    display_name: Python 3
    language: python
    name: python3
---

# Benchmark Results

This notebook shows the results of running the benchmark program on various systems.

## Setup

Let's import some libraries we will need:

```python
from pathlib import Path
import pandas as pd
import matplotlib.pyplot as plt
```

And import the results:

```python
rdir = Path('results')
```

```python
results = pd.concat({
    f.stem: pd.read_table(f, sep='\t')
    for f in rdir.glob('*.tsv')
}, names=['System']).reset_index('System').reset_index(drop=True)
```

## Result Table

Let's compute a result table, showing MiB/s for each algorithm:

```python
tbl = results.pivot(index='System', columns='hash', values='MiB/s')
tbl.style.format(precision=1).set_table_attributes('class="dataframe table table-striped"')
```

And a barchart for fun:

```python
tbl.T.plot.barh()
plt.xlabel('MiB/s')
plt.ylabel('Hash')
plt.show()
```

The i7-11xx processors have Intel's SHA hash extensions which RustCrypto can use
for SHA-1 and SHA-256, which explains the substantially higher performance of
those hashes on that chip, even though it is a laptop CPU (compared to the
desktop i9 in the test processors).
