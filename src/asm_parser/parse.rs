// type GenError = Box<dyn std::error::Error>;

// pub fn parse_line(line: &str) -> Result<Vec<u8>, GenError> {
//     if line.trim().is_empty() || line.trim().starts_with(';') {
//         return Ok(vec![]);
//     }
//     let line_code = line.split(';').collect::<Vec<&str>>();

//     let tokens = line_code[0].split(&[',', ' ']).collect::<Vec<&str>>();

//     let op = tokens[0];

//     if tokens.len() == 1 {
//         let opc = get_opcode(op)?;
//         return Ok(vec![opc]);
//     }

//     let args = &tokens[1..];

//     let mut output = vec![get_opcode(op)?];
//     let arg_bytes = get_argbytes(op);
//     for a in tokens[1..].iter().copied() {
//         if a.starts_with("i") {
//             let be = u16::from_str_radix(&a[2..], 16).unwrap().to_le_bytes();

//             output.push(be[0]);
//         } else {
//         }
//     }

//     return Ok(output);
// }

// fn get_opcode(op: &str) -> Result<u8, GenError> {
//     todo!()
// }

// fn get_argbytes(op: &str) -> u8 {
//     todo!()
// }
