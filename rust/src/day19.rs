use super::utils;

pub fn _run()
{
    let repls = vec![
        ("Al", "ThF"),
        ("Al", "ThRnFAr"),
        ("B", "BCa"),
        ("B", "TiB"),
        ("B", "TiRnFAr"),
        ("Ca", "CaCa"),
        ("Ca", "PB"),
        ("Ca", "PRnFAr"),
        ("Ca", "SiRnFYFAr"),
        ("Ca", "SiRnMgAr"),
        ("Ca", "SiTh"),
        ("F", "CaF"),
        ("F", "PMg"),
        ("F", "SiAl"),
        ("H", "CRnAlAr"),
        ("H", "CRnFYFYFAr"),
        ("H", "CRnFYMgAr"),
        ("H", "CRnMgYFAr"),
        ("H", "HCa"),
        ("H", "NRnFYFAr"),
        ("H", "NRnMgAr"),
        ("H", "NTh"),
        ("H", "OB"),
        ("H", "ORnFAr"),
        ("Mg", "BF"),
        ("Mg", "TiMg"),
        ("N", "CRnFAr"),
        ("N", "HSi"),
        ("O", "CRnFYFAr"),
        ("O", "CRnMgAr"),
        ("O", "HP"),
        ("O", "NRnFAr"),
        ("O", "OTi"),
        ("P", "CaP"),
        ("P", "PTi"),
        ("P", "SiRnFAr"),
        ("Si", "CaSi"),
        ("Th", "ThCa"),
        ("Ti", "BP"),
        ("Ti", "TiTi"),
        ("e", "HF"),
        ("e", "NAl"),
        ("e", "OMg")
        ];

    let start = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";

    let mut unique_results : utils::HashSetFnv<String> = fastset!();

    for r in repls {
        let mut pos = start.find(r.0);
        while pos.is_some() {
            let ppos = pos.unwrap();
            let mut replaced : String = String::with_capacity(start.len() + r.1.len());
            replaced.push_str(&start[..ppos]);
            replaced.push_str(r.1);
            replaced.push_str(&start[ppos+r.0.len()..]);
            unique_results.insert(replaced);

            match start[ppos+1..].find(r.0) {
                None => break,
                Some(p) => pos = Some(1 + p + ppos)
            }
        }
    }

    println!("day19-1: {}", unique_results.len());
}