## ```mcomp filename ```

```mcomp  
let terms = ["rhai", "#TEST"];

let results = find_docs(terms);

let md = "| Results |\n";
md += "| --- |";

for result in results {
  md += "\n| " + result + " |";
}

md 
```

```mcomp time() ```

```mcomp
let results = find_backlink(current_file);

let md = "| Results |\n";
md += "| --- |";

for result in results {
  md += "\n| " + result + " |";
}

md
```
