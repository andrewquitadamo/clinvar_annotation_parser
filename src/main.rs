use std::{error::Error, io, process};
use std::fs::File;
use serde::Deserialize;
use flate2::read::GzDecoder;
use std::io::{Read, BufReader, BufRead};
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(non_snake_case)]
struct ClinVarRecord {
    #[serde(rename="#AlleleID")]
    allele_id: i32,
    Type: String,
    Name: String,
//    GeneID: i32,
    GeneSymbol: String,
//    HGNC_ID: String,
    ClinicalSignificance: String,
//    ClinSigSimple: String,
//    LastEvaluated: String,
//    #[serde(rename="RS# (dbSNP)")]
//    rs_num_dbsnp: String,
//    #[serde(rename="nsv/esv (dbVar)")]
//    nsv_esv_dbVar: String,
//    RCVaccession: String,
    PhenotypeIDS: String,
    PhenotypeList: String,
//    Origin: String,
//    OriginSimple: String,
    Assembly: String,
    ChromosomeAccession: String,
    Chromosome: String,
    Start: i32,
    Stop: i32,
    ReferenceAllele: String,
    AlternateAllele: String,
    Cytogenetic: String,
    ReviewStatus: String,
//    NumberSubmitters: String,
//    Guidelines: String,
//    TestedInGTR: String,
    OtherIDs: String,
//    SubmitterCategories: String,
    VariationID: String,
    PositionVCF: String,
    ReferenceAlleleVCF: String,
    AlternateAlleleVCF: String,
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
    let mut tsv_reader = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(reader_box); println!("{:?}", tsv_reader.headers());
    for result in tsv_reader.deserialize() {
        let record: ClinVarRecord = result?;
        if gene_set.contains(&record.GeneSymbol) && record.Assembly == "GRCh38" {
              if record.Name.contains("c.") {
                  let rec_name_fields: Vec<&str> = record.Name.split(":").collect();
                  let mut c_dot_raw = "";
                  if rec_name_fields.len() > 1 {
                      c_dot_raw = rec_name_fields[1];
                  }
                  else {
                      c_dot_raw = rec_name_fields[0];
                  }
                  let mut c_dot: &str = "NA";
                  let mut p_dot = "NA";
                  if c_dot_raw.contains("p.") {
                      (c_dot, p_dot) = c_dot_raw.split_once("(").unwrap();
                      let p_dot = &str::replace(p_dot, ")", "");
                      println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", record.GeneSymbol, record.Name, c_dot, p_dot, record.Chromosome, record.Start, record.Stop, record.ReferenceAlleleVCF, record.AlternateAlleleVCF, record.PositionVCF, record.Type, record.ClinicalSignificance, record.allele_id, record.PhenotypeIDS, record.PhenotypeList, record.ReviewStatus, record.Assembly)
                  }
                  else {
                      c_dot = c_dot_raw;
                      println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", record.GeneSymbol, record.Name, c_dot, p_dot, record.Chromosome, record.Start, record.Stop, record.ReferenceAlleleVCF, record.AlternateAlleleVCF, record.PositionVCF, record.Type, record.ClinicalSignificance, record.allele_id, record.PhenotypeIDS, record.PhenotypeList, record.ReviewStatus, record.Assembly)
                  }
              }
              else {
                  println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", record.GeneSymbol, record.Name, "NA", "NA", record.Chromosome, record.Start, record.Stop, record.ReferenceAlleleVCF, record.AlternateAlleleVCF, record.PositionVCF, record.Type, record.ClinicalSignificance, record.allele_id, record.PhenotypeIDS, record.PhenotypeList, record.ReviewStatus, record.Assembly)
              }
        }
    }
    Ok(())
}
