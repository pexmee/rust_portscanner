
#[derive(Debug)]
pub enum PortParseError{
    StartPortLarger(),
    EndPortSmaller(),
    StartPortOutOfRange(),
    EndPortOutOfRange(),
    ParseError()
}

pub fn port_parser(port_range_str: String) -> eyre::Result<(u16, u16), PortParseError>{
    /*
    Responsible for parsing a user supplied port range. 
    Returns Err if it failed to do so. 
    */
    if port_range_str.is_empty(){
        return Ok((1,65535)) // Default to 1-65535
    }
    let port_range_vec: Vec<&str> = port_range_str.split("-").collect();
    let start_port =  match port_range_vec[0].parse::<u16>(){
        Ok(p) => p,
        Err(_)=> return Err(PortParseError::ParseError()),  
    };

    let end_port = match port_range_vec[1].parse::<u16>(){
        Ok(p) => p,
        Err(_) => return Err(PortParseError::ParseError()),
    };
    
    if start_port > end_port{
        return Err(PortParseError::StartPortLarger())
    } else if end_port < start_port{
        return Err(PortParseError::EndPortSmaller())
    } else if start_port < 1 || start_port > 65534{
        return Err(PortParseError::StartPortOutOfRange())
    } else if end_port < 2{ // We don't need to check if end_port is larger than 65535 due to u16
        return Err(PortParseError::EndPortOutOfRange())
    }
     
    Ok((start_port, end_port))
    
}