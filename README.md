ClinVar Variant Summary parser written in Rust
---

Code to parse the ClinVar variant summary TSV. Adds fields for C dot and P dot anotations.  
Can handle both gzipped files and plaintext. Output can either be printed to stdout or saved to a file.

Variant summary TSVs can be found at: [https://ftp.ncbi.nlm.nih.gov/pub/clinvar/tab_delimited/](https://ftp.ncbi.nlm.nih.gov/pub/clinvar/tab_delimited/)

```
Usage: clinvar_annotation_parser [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>          ClinVar Variant TSV
  -l, --genelist <GENELIST>    Optional filename for list of genes to filter on
  -o, --output <OUTPUT>        Output filename. If not provided will print to STDOUT
  -r, --reference <REFERENCE>  Reference genome version [default: hg38] [possible values: hg19, hg38]
  -h, --help                   Print help
  -V, --version                Print version
```

As written will output GeneSymbol, #AlleleID, Type, Name, C dot, P dot, ClinicalSignificance, PhenotypeIDS, PhenotypeList, Assembly, Chromosome, Start, Stop, ReviewStatus, PositionVCF, ReferenceAlleleVCF, AlternateAlleleVCF, OtherIDs.  
Other fields can be included by uncommenting out the field name in the `ClinVarRecord` struct and recompiling. 

Build from source
---
You will need Rust installed, along with the Cargo package manager. The compiled binary will be in `/target/release/clinvar_annotation_parser`.
```
git clone https://github.com/andrewquitadamo/clinvar_annotation_parser.git
cargo build --release
```

Install from source
---
This will install `clinvar_annotation_parser`, and make it available to your local `PATH`. The install location will most likely be `$HOME/.cargo/bin`. 
```
cargo install --git https://github.com/andrewquitadamo/clinvar_annotation_parser.git
```
