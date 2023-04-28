use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn define_box(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let atts: Vec<_> = attr.into_iter().collect();
    let att_str = attr.to_string().replace(" ", "");
    let atts: Vec<_> = att_str.split(",").collect();

    let box_type = atts.get(0);
    let first_attr = atts.get(1);
    let second_attr = atts.get(2);

    match (box_type, first_attr, second_attr) {
        (Some(_), None, None) => {}
        (Some(_), Some(_), None) => {}
        (Some(_), Some(_), Some(_)) => {}
        _ => panic!("Invalid set of arguments"),
    }

    let box_type = box_type.unwrap().replace("\"", "");

    if box_type.len() != 4 || !box_type.is_ascii() {
        panic!("Invalid box type, must be four characters long and utf8")
    }

    println!("A: {} {}", box_type, first_attr.unwrap());

    let (full_box, container) = match (first_attr, second_attr) {
        (Some(&"full"), None) => (true, false),
        (Some(&"container"), None) => (false, true),
        (Some(&"full"), Some(&"container")) => (true, true),
        _ => panic!("Invalid set of arguments"),
    };

    println!("atts: {:?}", atts);
    let str_rep = item.to_string();

    println!("tms: {}", str_rep);

    let mut replace_string = String::from("{\n");

    replace_string += "    /// The size of the entire box in bytes\n";
    replace_string += "    pub size: u32,\n";
    replace_string +=
        format!("    /// This specifies the box type with 4 utf8 charcters, for this box it will always be `\"{box_type}\"`")
            .as_ref();
    replace_string += "    pub box_type: &'a str,\n";
    replace_string += "    // END OF GENERIC BOX DEFINITION\n";

    if full_box {
        replace_string += "    /// The fucntion of this property is unknown as of yet\n";
        replace_string += "    pub version: u8,\n";
        replace_string += "    /// Even though this is a u32 it consists only of 3 bytes\n";
        replace_string += "    /// when it's parsed, so the first 4 bits will always be 0\n";
        replace_string += "    pub flags: u32,\n";
        replace_string += "    // END OF FULL BOX DEFINITION\n";
    }

    if container {
        replace_string += "    /// The remaining data of the box will be parsed\n";
        replace_string += "    /// as a list of generic boxes\n";
        replace_string += "    pub children: Vec<BaseBox<'a>>,\n";
    }

    str_rep.replace("{", &replace_string).parse().unwrap()
}
