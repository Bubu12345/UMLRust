extern crate regex;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct KlasseStruct {
    klassen_name: String,
    klassen_attribute: String,
	klassen_methoden: String,
}
pub fn parse_diagram(code: &str) -> i32 {

	let mut array: [&str; 2] = ["test", "test2"];
	let mut counter = 0;
	for s in code.split(|c| c == '{' || c == '}') {

		if counter < 2 {
			
			array[counter] = s;
			counter += 1;
		} else {
			counter += 1;
		}
	}
	if array[0] == "Klassendiagram " {
		
		return 1;
	} else {
		return 0;
	}
}

pub fn regex_parser<'r >(code: &'r str) -> Vec<KlasseStruct> {

	
	let regex_klasse = Regex::new(r"^Klassendiagram\s\{\s(?P<klassenName>\w+):\sAttribute=\s(?P<attribute>[static\s\w*\sInteger,]+);\s(?P<methode>[public\sstatic\s\w*\(\w*:\sint\):\sint,]+)\}").unwrap();
	let mut klassen_vec = Vec::new();
	
	for cap in regex_klasse.captures_iter(code) {
		

		
	


		klassen_vec.push(
				KlasseStruct {
					klassen_name: String::from(&cap[1]).as_str().to_owned(),
					klassen_attribute: String::from(&cap[2]).as_str().to_owned(),
					klassen_methoden: String::from(&cap[3]).as_str().to_owned(),
				},
			);
	}
	return klassen_vec;
}

pub fn klasse_zu_model(vec: Vec<KlasseStruct>) {

	let mut model = String::new();
	model.push_str(" --------\n");
	let mut test_klassen_name = String::from("| ");
	test_klassen_name.push_str(vec[0].klassen_name.as_str());
	test_klassen_name.push_str(" |\n");
	model.push_str(test_klassen_name.as_str());
	model.push_str(" --------\n");
	
	println!("{}", model);
	
	
}

 pub fn split_attribute<'r>(atr: &'r str) ->  Vec<&'r str> {

	let mut atr_vec = Vec::new();
	for s in atr.split(',') {

		atr_vec.push(s);
	}
	
	return atr_vec;
} 

fn main() {

	let code_text = "Klassendiagram { klasse1: Attribute= static atr1 Integer,static atr2 Integer; public static methode1(par1: int): int,public static methode2(par2: int): int}";
	let mut new_vec = Vec::new();
	println!("String: {}", code_text);
	let diagram_type = parse_diagram(code_text);
	if diagram_type == 1 {
		println!("Diagramtyp: Klassendiagram");
		new_vec = regex_parser(code_text);
		//println!("Klassenname: {}\nAttribute: {}\nMethode: {}", new_vec[0].klassen_name, new_vec[0].klassen_attribute.to_string(), new_vec[0].klassen_methoden.as_str());
	} else {
		println!("Fehlerhafter Diagramtyp!");
	}
	
	klasse_zu_model(new_vec);
}
