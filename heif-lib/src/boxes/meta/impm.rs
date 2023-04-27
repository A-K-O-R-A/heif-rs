type IPMPDescriptor = u8;

// BaseBox + FullBox
pub struct IPMPControlBox<'a> {
    pub size: u32,
    /// This will always be `"iinf"`
    pub box_type: &'a str,
    pub version: u8,
    pub flags: u32,
    // End of full box
    pub tool_list: IPMPToolListDescriptor,
    pub ipmp_descriptors: Vec<IPMPDescriptor>,
}

pub struct IPMPToolListDescriptor {
    pub tag: u8,
    pub num_tools: u8,
    pub tools: Vec<IPMPTool>,
}

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
