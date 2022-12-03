---
jupytext:
  formats: ipynb,md:myst
  text_representation:
    extension: .md
    format_name: myst
    format_version: 0.13
    jupytext_version: 1.14.1
kernelspec:
  display_name: Python 3 (ipykernel)
  language: python
  name: python3
---

# Benchmark Results

This notebook shows the results of running the benchmark program on various systems.

+++

## Setup

Let's import some libraries we will need:

```{code-cell} ipython3
from pathlib import Path
import pandas as pd
import matplotlib.pyplot as plt
```

And import the results:

```{code-cell} ipython3
rdir = Path('results')
```

```{code-cell} ipython3
results = pd.concat({
    f.stem: pd.read_table(f, sep='\t')
    for f in rdir.glob('*.tsv')
}, names=['System']).reset_index('System').reset_index(drop=True)
```

## Result Table

Let's compute a result table, showing MiB/s for each algorithm:

```{code-cell} ipython3
tbl = results.pivot(index='System', columns='hash', values='MiB/s')
tbl
```

And a barchart for fun:

```{code-cell} ipython3
tbl.T.plot.barh()
plt.xlabel('MiB/s')
plt.ylabel('Hash')
plt.show()
```

```{code-cell} ipython3

```
