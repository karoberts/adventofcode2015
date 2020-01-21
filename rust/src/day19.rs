use super::utils;

fn recur(curmol:&String, steps:usize, min_steps:&mut usize, repls:&Vec<(&str,&str)>, done:&mut bool) -> usize 
{
    if curmol.find('e').is_some() || steps > *min_steps {
        return 99999999;
    }

    let mut possibles : Vec<(usize, String)> = vec!();
    for r in repls.iter() {
        let pos = curmol.find(r.1);
        if pos.is_none() {
            continue;
        }
        let ppos = pos.unwrap();
        let mut replaced : String = String::with_capacity(curmol.len() + r.0.len());
        replaced.push_str(&curmol[..ppos]);
        replaced.push_str(r.0);
        replaced.push_str(&curmol[ppos+r.1.len()..]);

        possibles.push((ppos, replaced));
    }

    let mut amts : Vec<usize> = vec!();
    possibles.sort_by_key(|x| std::cmp::Reverse(x.0));
    for p in possibles.iter() {
        if p.1 == "e" {
            *done = true;
            return steps + 1;
        }
        else if p.1.len() <= curmol.len() {
            let ret = recur(&p.1, steps + 1, min_steps, repls, done);
            if *done {
                return ret;
            }
            amts.push( ret );
        }
    }

    return *amts.iter().min().unwrap_or(&99999999);
}

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

    for r in repls.iter() {
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

    let mut sorted_repls = repls.clone();
    sorted_repls.sort_by_key(|a| std::cmp::Reverse(a.1.len()));

    let mut min_steps = start.len();
    let mut done = false;

    let steps = recur(&start.to_owned(), 0, &mut min_steps, &sorted_repls, &mut done);
    println!("day19-2: {}", steps);
}