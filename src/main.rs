use std::error::Error;
use std::fs::File;
use serde::{Deserialize, Serialize};
use flate2::read::GzDecoder;
use std::io::{Read, BufReader, BufRead, stdout};
use std::collections::HashSet;
use std::env::args;

fn parse_genelist(genelist_filename: &String) -> HashSet<String> {
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
    #[serde(rename="#AlleleID")]
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let genelist_filename: &String = &args[1];
    let gene_set = parse_genelist(genelist_filename);
    let filename = "variant_summary.txt.gz";
    let file = File::open(filename).expect("Failed to open input file");
    let reader_box: Box<dyn Read> = if filename.ends_with(".gz") {Box::new(GzDecoder::new(file))} else {Box::new(file)};
    let mut tsv_reader = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(reader_box); 
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_writer(stdout());
    for result in tsv_reader.deserialize() {
        let mut record: ClinVarRecord = result?;
        if gene_set.contains(&record.GeneSymbol) && record.Assembly == "GRCh38" {
              if record.Name.contains("c.") {
                  let rec_name_fields: Vec<&str> = record.Name.split(":").collect();

                  let c_dot_raw: &str = if rec_name_fields.len() > 1 {
                      rec_name_fields[1]
                  }
                  else {
                      rec_name_fields[0]
                  };

                  if c_dot_raw.contains("p.") {
                      let (c_dot, p_dot) = c_dot_raw.split_once("(").unwrap();
                      let p_dot = &str::replace(p_dot, ")", "");
                      record.c_dot = c_dot.to_string();
                      record.p_dot = p_dot.to_string();
                      wtr.serialize(record)?;
                  }
                  else {
                      let c_dot = c_dot_raw;
                      record.c_dot = c_dot.to_string();
                      wtr.serialize(record)?;
                  }
              }
              else {
                    wtr.serialize(record)?;
              }
        }
    }
    wtr.flush()?;
    Ok(())
}
