use std::collections::HashMap;

use gemini_client_rs::types::{FunctionDeclaration, FunctionParameters, ParameterProperty, ParameterPropertyInteger};

// pub fn add_declaration()->FunctionDeclaration{

//      let mut add_parameters: HashMap<String, ParameterProperty> =    HashMap::new();
//         add_parameters.insert(
//             "number1".to_string(), 
//             ParameterProperty::Integer(
//                 ParameterPropertyInteger{
//                     description:Some("pass first number here in the add function".to_string())
//                 }
//             )
//         );
//         add_parameters.insert(
//             "number2".to_string(), 
//             ParameterProperty::Integer(
//                 ParameterPropertyInteger{
//                     description:Some("pass second number here in the add function".to_string())
//                 }
//             )
//         );
//         FunctionDeclaration{
//             name:"add".to_string(),
//             description:"this function performs addition of two integers and accepts only two parameters".to_string(),
//             parameters: Some(FunctionParameters{
//                 parameter_type: "OBJECT".to_string(),
//                 properties: add_parameters,
//                 required: Some(vec![
//                     "number1".to_string(), 
//                     "number2".to_string()
//                     ])
//             }),
//             response:None
//         }
// }

// pub fn multiply_declaration()->FunctionDeclaration{

//      let mut multiply_parameters: HashMap<String, ParameterProperty> =    HashMap::new();
//         multiply_parameters.insert(
//             "number1".to_string(), 
//             ParameterProperty::Integer(
//                 ParameterPropertyInteger{
//                     description:Some("pass first number here in the multiply function".to_string())
//                 }
//             )
//         );
//         multiply_parameters.insert(
//             "number2".to_string(), 
//             ParameterProperty::Integer(
//                 ParameterPropertyInteger{
//                     description:Some("pass second number here in the multiply function".to_string())
//                 }
//             )
//         );
//         FunctionDeclaration{
//             name:"multiply".to_string(),
//             description:"this function performs multiplication of two integers and accepts only two parameters".to_string(),
//             parameters: Some(FunctionParameters{
//                 parameter_type: "OBJECT".to_string(),
//                 properties: multiply_parameters,
//                 required: Some(vec![
//                     "number1".to_string(), 
//                     "number2".to_string()
//                     ])
//             }),
//             response:None
//         }
// }
