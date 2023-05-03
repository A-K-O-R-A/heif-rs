use heif_macros::define_box;

type IPMPDescriptor = u8;

// BaseBox + FullBox
#[define_box("iinf", full)]
pub struct IPMPControlBox<'a> {
    pub tool_list: IPMPToolListDescriptor,
    pub ipmp_descriptors: Vec<IPMPDescriptor>,
}

#[derive(Debug)]
pub struct IPMPToolListDescriptor {
    pub tag: u8,
    pub num_tools: u8,
    pub tools: Vec<IPMPTool>,
}

#[derive(Debug)]
pub struct IPMPTool {
    pub tool_id: u128,
    pub is_alt_group: u8,
    pub is_parametric: u8,
    pub num_alternates: u8,
    pub specific_tool_id: u128,
    pub tool_param_desc: u8,
    pub num_urls: u8,
    pub tool_urls: u8,
}
