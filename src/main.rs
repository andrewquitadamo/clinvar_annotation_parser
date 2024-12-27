use clap::Parser;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Read, Write};

fn parse_genelist(genelist_filename: String) -> HashSet<String> {
    let mut gene_set = HashSet::new();
    let file = File::open(genelist_filename).expect("Failed to open genelist");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        gene_set.insert(line.expect("Failed to parse gene"));
    }
    return gene_set;
}

fn default_val() -> String {
    "NA".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(non_snake_case)]
struct ClinVarRecord {
    GeneSymbol: String,
    #[serde(rename = "#AlleleID")]
    allele_id: i32,
    Type: String,
    Name: String,
    #[serde(default = "default_val")]
    c_dot: String,
    #[serde(default = "default_val")]
    p_dot: String,
    ClinicalSignificance: String,
    PhenotypeIDS: String,
    PhenotypeList: String,
    Assembly: String,
    Chromosome: String,
    Start: i32,
    Stop: i32,
    ReviewStatus: String,
    PositionVCF: String,
    ReferenceAlleleVCF: String,
    AlternateAlleleVCF: String,
    OtherIDs: String,
    //    GeneID: i32,
    //    HGNC_ID: String,
    //    ClinSigSimple: String,
    //    LastEvaluated: String,
    //    #[serde(rename="RS# (dbSNP)")]
    //    rs_num_dbsnp: String,
    //    #[serde(rename="nsv/esv (dbVar)")]
    //    nsv_esv_dbVar: String,
    //    RCVaccession: String,
    //    Origin: String,
    //    OriginSimple: String,
    //    ChromosomeAccession: String,
    //    ReferenceAllele: String,
    //    AlternateAllele: String,
    //    Cytogenetic: String,
    //    NumberSubmitters: String,
    //    Guidelines: String,
    //    TestedInGTR: String,
    //    SubmitterCategories: String,
    //    VariationID: String,
    //    SomaticClinicalImpact: String,
    //    SomaticClinicalImpactLastEvaluated: String,
    //    ReviewStatusClinicalImpact: String,
    //    Oncogenicity: String,
    //    OncogenicityLastEvaluated: String,
    //    ReviewStatusOncogenicity: String,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// ClinVar Variant TSV
    #[arg(long, short = 'i', required = true)]
    input: String,

    /// Optional filename for list of genes to filter on
    #[arg(long, short = 'l')]
    genelist: Option<String>,

    /// Output filename. If not provided will print to STDOUT.
    #[arg(long, short = 'o')]
    output: Option<String>,

    /// Reference genome version.
    #[clap(value_enum)]
    #[arg(long, short='r', default_value_t=ReferenceGenome::Hg38)]
    reference: ReferenceGenome,
}

#[derive(clap::ValueEnum, Clone, Debug, Eq, PartialEq)]
enum ReferenceGenome {
    Hg19,
    Hg38,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let filename = cli.input;
    let mut use_gene_list = false;
    let mut gene_set = HashSet::new();
    if let Some(genelist_filename) = cli.genelist {
        gene_set = parse_genelist(genelist_filename);
        use_gene_list = true;
    }
    let file = File::open(filename.clone()).expect("Failed to open input file");
    let reader_box: Box<dyn Read> = if filename.ends_with(".gz") {
        Box::new(GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader_box);
    let writer: Box<dyn Write> = match cli.output {
        Some(filename) => {
            let file = File::create(filename)?;
            Box::new(file)
        }
        None => Box::new(stdout().lock()),
    };

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(writer);
    let ref_genome = if cli.reference == ReferenceGenome::Hg38 {
        "GRCh38"
    } else {
        "GRCh37"
    };
    for result in tsv_reader.deserialize() {
        let mut record: ClinVarRecord = result?;
        if use_gene_list && !gene_set.contains(&record.GeneSymbol) {
            continue;
        }
        if record.Assembly == ref_genome {
            if record.Name.contains("c.") {
                let rec_name_fields: Vec<&str> = record.Name.split(":").collect();

                let c_dot_raw: &str = if rec_name_fields.len() > 1 {
                    rec_name_fields[1]
                } else {
                    rec_name_fields[0]
                };

                if c_dot_raw.contains("p.") {
                    let (c_dot, p_dot) = c_dot_raw.split_once("(").unwrap();
                    let p_dot = &str::replace(p_dot, ")", "");
                    record.c_dot = c_dot.to_string();
                    record.p_dot = p_dot.to_string();
                    wtr.serialize(record)?;
                } else {
                    let c_dot = c_dot_raw;
                    record.c_dot = c_dot.to_string();
                    wtr.serialize(record)?;
                }
            } else {
                wtr.serialize(record)?;
            }
        }
    }
    wtr.flush()?;
    Ok(())
}
