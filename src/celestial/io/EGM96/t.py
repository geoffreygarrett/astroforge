import pandas as pd

df = pd.read_csv("EGM96", sep="\s+", header=None)
print(df)
print(df.to_csv("EGM96.csv", index=False))
